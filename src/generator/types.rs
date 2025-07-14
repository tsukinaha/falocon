use heck::ToPascalCase;
use openapiv3::{ReferenceOr, SchemaKind, Type};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct TypesGenerator<'a> {
    schema: &'a openapiv3::Schema,
}

impl<'a> TypesGenerator<'a> {
    pub fn new(schema: &'a openapiv3::Schema) -> Self {
        Self { schema }
    }

    pub fn generate(&self) -> Result<TokenStream, String> {
        match &self.schema.schema_kind {
            SchemaKind::Type(Type::String(_)) => Ok(quote! { String }),
            SchemaKind::Type(Type::Integer(int_schema)) => {
                if let openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int64) =
                    int_schema.format
                {
                    Ok(quote! { i64 })
                } else {
                    Ok(quote! { i32 })
                }
            }

            SchemaKind::Type(Type::Number(num_schema)) => {
                if let openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::NumberFormat::Double) =
                    num_schema.format
                {
                    Ok(quote! { f64 })
                } else {
                    Ok(quote! { f32 })
                }
            }

            SchemaKind::Type(Type::Boolean(_)) => Ok(quote! { bool }),
            SchemaKind::Type(Type::Array(array_schema)) => {
                if let Some(items) = &array_schema.items {
                    let item_type = match items {
                        ReferenceOr::Reference { reference } => {
                            if let Some(type_name) = reference.strip_prefix("#/components/schemas/")
                            {
                                let type_ident = format_ident!("{}", type_name.to_pascal_case());

                                quote! { #type_ident }
                            } else {
                                quote! { serde_json::Value }
                            }
                        }
                        ReferenceOr::Item(schema) => Self::new(schema).generate()?,
                    };

                    Ok(quote! { Vec<#item_type> })
                } else {
                    Ok(quote! { Vec<serde_json::Value> })
                }
            }
            SchemaKind::Type(Type::Object(_)) => Ok(quote! { HashMap<String, serde_json::Value> }),
            _ => Ok(quote! { serde_json::Value }),
        }
    }
}
