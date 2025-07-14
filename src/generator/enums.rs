use heck::ToPascalCase;
use openapiv3::StringType;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct EnumsGenerator<'a> {
    schema: &'a StringType,
}

impl<'a> EnumsGenerator<'a> {
    pub fn new(schema: &'a StringType) -> Self {

        Self { schema }
    }

    pub fn generate(&self) -> Result<TokenStream, String> {

        let mut variants = TokenStream::new();

        for value in &self.schema.enumeration {

            if let Some(variant_str) = value.as_ref().map(|v| v.as_str()) {

                let variant_name = format_ident!("{}", variant_str.to_pascal_case());

                variants.extend(quote! {
                    #[serde(rename = #variant_str)]
                    #variant_name,
                });
            }
        }

        Ok(variants)
    }
}
