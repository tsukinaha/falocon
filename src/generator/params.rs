use super::create_rust_safe_ident;
use heck::ToSnakeCase;
use openapiv3::ParameterSchemaOrContent::Content;
use openapiv3::ParameterSchemaOrContent::Schema;
use openapiv3::{Parameter, ReferenceOr};
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;

pub struct ParamsGenerator<'a> {
    name: &'a Ident,
    params: &'a [ReferenceOr<Parameter>],
}

type ParamStruct = TokenStream;

type ParamsInPathFields = TokenStream;

type ParamsReplaceFields = TokenStream;

impl<'a> ParamsGenerator<'a> {
    pub fn new(name: &'a Ident, params: &'a [ReferenceOr<Parameter>]) -> Self {

        Self { name, params }
    }

    pub fn generate(
        &self,
    ) -> Result<(ParamStruct, ParamsInPathFields, ParamsReplaceFields), String> {

        let mut fields = TokenStream::new();

        let mut in_path_fields = TokenStream::new();

        let mut replace_fields = TokenStream::new();

        let struct_ident = self.name;

        for params in self.params {

            let ReferenceOr::Item(param) = params else {

                continue;
            };

            if let openapiv3::Parameter::Query { parameter_data, .. } = &param {

                let field_name = &parameter_data.name;

                let serde_name = field_name;

                let field_ident = create_rust_safe_ident(&field_name.to_snake_case());

                let field_type = match &parameter_data.format {
                    Schema(schema) => {

                        let openapiv3::ReferenceOr::Item(schema) = schema else {

                            // params with references are not supported in this context
                            return Err("Unsupported parameter schema".to_string());
                        };

                        let rust_type = super::TypesGenerator::new(schema).generate()?;

                        if schema.schema_data.nullable {

                            quote! { Option<#rust_type> }
                        } else {

                            quote! { #rust_type }
                        }
                    }
                    Content(_) => {

                        return Err("Content parameters are not supported".to_string());
                    }
                };

                let doc_comment =
                    super::DocsGenerator::generate(parameter_data.description.as_deref());

                fields.extend(quote! {
                    #doc_comment
                    #[serde(rename = #serde_name)]
                    pub #field_ident: #field_type,
                });
            } else if let openapiv3::Parameter::Path { parameter_data, .. } = &param {

                let field_name = &parameter_data.name;

                let field_ident = format_ident!("{}", field_name);

                let field_name = create_rust_safe_ident(&field_name.to_snake_case());

                let field_type = match &parameter_data.format {
                    Schema(schema) => {

                        let openapiv3::ReferenceOr::Item(schema) = schema else {

                            // params with references are not supported in this context
                            return Err("Unsupported parameter schema".to_string());
                        };

                        let rust_type = super::TypesGenerator::new(schema).generate()?;

                        if schema.schema_data.nullable {

                            quote! { Option<#rust_type> }
                        } else {

                            quote! { #rust_type }
                        }
                    }
                    Content(_) => {

                        return Err("Content parameters are not supported".to_string());
                    }
                };

                let doc_comment =
                    super::DocsGenerator::generate(parameter_data.description.as_deref());

                in_path_fields.extend(quote! {
                    #doc_comment
                    pub #field_name: #field_type,
                });

                let replace_ident = format!("{{{field_ident}}}");

                replace_fields.extend(quote! {
                    .replace(
                        #replace_ident,
                        &self.#field_name.to_string()
                    )
                });
            }
        }

        if fields.is_empty() {

            Ok((quote! {}, in_path_fields, replace_fields))
        } else {

            Ok((
                quote! {
                    #[derive(Debug, Clone, Serialize, Deserialize)]
                    pub struct #struct_ident {
                        #fields
                    }
                },
                in_path_fields,
                replace_fields,
            ))
        }
    }
}
