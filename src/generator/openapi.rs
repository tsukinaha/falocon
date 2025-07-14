use std::collections::HashMap;

use openapiv3::{OpenAPI, ReferenceOr};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{PathsGenerator, generator::StructsGenerator};

pub struct OpenAPIGenerator {
    openapi: OpenAPI,
}

impl OpenAPIGenerator {
    pub fn from_json(data: &str) -> Self {

        Self {
            openapi: serde_json::from_str(data).expect("Could not deserialize input"),
        }
    }

    pub fn gen_types(&self) -> TokenStream {

        let mut output = TokenStream::new();

        let Some(components) = &self.openapi.components else {

            return output;
        };

        output.extend(quote! {
            use serde::{Deserialize, Serialize};
            use std::collections::HashMap;
        });

        for (name, schema) in &components.schemas {

            let ReferenceOr::Item(schema) = schema else {

                continue;
            };

            output.extend(StructsGenerator::new(name, schema).generate());
        }

        output
    }

    pub fn gen_methods(&self) -> HashMap<String, TokenStream> {

        let mut output = HashMap::new();

        for path in self.openapi.paths.paths.keys() {

            let ReferenceOr::Item(path_item) = &self.openapi.paths.paths[path] else {

                continue;
            };

            if let Ok(map) = PathsGenerator::new(path, path_item).generate() {

                output.extend(map);
            }
        }

        output
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test_openapi_generator() {

        let data = include_str!("../../tests/openapi.json");

        let generator = super::OpenAPIGenerator::from_json(data);

        let structs = generator.gen_types();

        dbg!(structs.to_string());
    }
}
