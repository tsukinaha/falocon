use heck::{ToPascalCase, ToSnakeCase};
use openapiv3::{ObjectType, ReferenceOr};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashSet;

use crate::generator::{docs::DocsGenerator, types::TypesGenerator};

pub struct FieldsGenerator<'a> {
    struct_name: &'a str,
    obj: &'a ObjectType,
}

impl<'a> FieldsGenerator<'a> {
    pub fn new(struct_name: &'a str, obj: &'a ObjectType) -> Self {

        Self { struct_name, obj }
    }

    pub fn generate(&self) -> Result<TokenStream, String> {

        let mut fields = TokenStream::new();

        let required_fields: HashSet<String> = self.obj.required.iter().cloned().collect();

        for (field_name, field_schema_ref) in &self.obj.properties {

            let snake_case_name = field_name.to_snake_case();

            let field_ident = create_rust_safe_ident(&snake_case_name);

            // Generate field documentation and type
            let (field_type, field_doc) = match field_schema_ref {
                ReferenceOr::Reference { reference } => {

                    if let Some(type_name) = reference.strip_prefix("#/components/schemas/") {

                        let type_ident = format_ident!("{}", type_name.to_pascal_case());

                        let ty = if type_name == self.struct_name {

                            quote! { Box<#type_ident> }
                        } else {

                            quote! { #type_ident }
                        };

                        (ty, quote! {})
                    } else {

                        (quote! { serde_json::Value }, quote! {})
                    }
                }
                ReferenceOr::Item(schema) => {

                    let rust_type = TypesGenerator::new(schema).generate()?;

                    let doc_comment =
                        DocsGenerator::generate(schema.schema_data.description.as_deref());

                    (rust_type, doc_comment)
                }
            };

            let field_type = if required_fields.contains(field_name) {

                field_type
            } else {

                quote! { Option<#field_type> }
            };

            let serde_attr = if field_name != &field_name.to_snake_case() {

                quote! { #[serde(rename = #field_name)] }
            } else {

                quote! {}
            };

            fields.extend(quote! {
                #field_doc
                #serde_attr
                pub #field_ident: #field_type,
            });
        }

        Ok(fields)
    }
}

pub fn is_rust_keyword(name: &str) -> bool {

    matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
            | "try"
    )
}

pub fn create_rust_safe_ident(name: &str) -> Ident {

    if is_rust_keyword(name) {

        // Special handling for keywords that cannot be raw identifiers
        match name {
            "self" => format_ident!("self_"),
            "Self" => format_ident!("Self_"),
            _ => format_ident!("r#{}", name),
        }
    } else {

        format_ident!("{}", name)
    }
}
