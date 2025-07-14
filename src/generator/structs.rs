use heck::ToPascalCase;
use openapiv3::{SchemaKind, Type};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::generator::{
    docs::DocsGenerator, enums::EnumsGenerator, fields::FieldsGenerator, types::TypesGenerator,
};

pub struct StructsGenerator<'a> {
    name: &'a str,
    schema: &'a openapiv3::Schema,
}

impl<'a> StructsGenerator<'a> {
    pub fn new(name: &'a str, schema: &'a openapiv3::Schema) -> Self {

        Self { name, schema }
    }

    pub fn generate(&self) -> Result<TokenStream, String> {

        let struct_name = self.name.to_pascal_case();

        let struct_ident = format_ident!("{}", struct_name);

        let doc_comment = DocsGenerator::generate(self.schema.schema_data.description.as_deref());

        match &self.schema.schema_kind {
            SchemaKind::Type(Type::Object(obj)) => {

                let fields = FieldsGenerator::new(&struct_name, obj).generate()?;

                Ok(quote! {
                    #doc_comment
                    #[derive(Debug, Clone, Serialize, Deserialize)]
                    pub struct #struct_ident {
                        #fields
                    }
                })
            }
            SchemaKind::Type(Type::String(schema)) if !schema.enumeration.is_empty() => {

                let variants = EnumsGenerator::new(schema).generate()?;

                Ok(quote! {
                    #doc_comment
                    #[derive(Debug, Clone, Serialize, Deserialize)]
                    pub enum #struct_ident {
                        #variants
                    }
                })
            }
            _ => {

                let rust_type = TypesGenerator::new(self.schema).generate()?;

                Ok(quote! {
                    #doc_comment
                    pub type #struct_ident = #rust_type;
                })
            }
        }
    }
}
