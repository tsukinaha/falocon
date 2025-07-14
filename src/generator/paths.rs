use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use heck::{ToPascalCase, ToSnakeCase};
use openapiv3::{Operation, PathItem};
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;

use crate::{ParamsGenerator, generator::docs::DocsGenerator};

pub struct PathsGenerator<'a> {
    name: &'a str,
    path_item: &'a PathItem,
}

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        match self {
            Method::Get => write!(f, "GET"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Delete => write!(f, "DELETE"),
            Method::Patch => write!(f, "PATCH"),
            Method::Head => write!(f, "HEAD"),
            Method::Options => write!(f, "OPTIONS"),
        }
    }
}

type OperationName = String;

type OperationTurple = (OperationName, TokenStream);

pub type OperationMap = HashMap<OperationName, TokenStream>;

impl<'a> PathsGenerator<'a> {
    pub fn new(name: &'a str, path_item: &'a PathItem) -> Self {

        Self { name, path_item }
    }

    pub fn generate(&self) -> Result<OperationMap, String> {

        let mut output = OperationMap::new();

        if let Some(get) = &self.path_item.get {

            let (op_name, token) = self.gent(get, Method::Get)?;
            output.insert(op_name, token);
        }

        if let Some(post) = &self.path_item.post {

            let (op_name, token) = self.gent(post, Method::Post)?;
            output.insert(op_name, token);
        }

        if let Some(put) = &self.path_item.put {

            let (op_name, token) = self.gent(put, Method::Put)?;
            output.insert(op_name, token);
        }

        if let Some(delete) = &self.path_item.delete {

            let (op_name, token) = self.gent(delete, Method::Delete)?;
            output.insert(op_name, token);
        }

        if let Some(patch) = &self.path_item.patch {

            let (op_name, token) = self.gent(patch, Method::Patch)?;
            output.insert(op_name, token);
        }

        if let Some(head) = &self.path_item.head {

            let (op_name, token) = self.gent(head, Method::Head)?;
            output.insert(op_name, token);
        }

        if let Some(options) = &self.path_item.options {

            let (op_name, token) = self.gent(options, Method::Options)?;
            output.insert(op_name, token);
        }

        Ok(output)
    }

    fn gent(&self, op: &Operation, method: Method) -> Result<OperationTurple, String> {

        if op.deprecated {

            return Err("Operation is deprecated".to_string());
        }

        let struct_name = op
            .operation_id
            .as_ref()
            .expect("Operation ID is required")
            .to_pascal_case();

        let struct_ident = format_ident!("{}", struct_name);

        let params_struct_name = format_ident!("{}Params", struct_name);

        let doc_comment = DocsGenerator::generate(op.description.as_deref());

        let (params, inpath_fields, replace_fields) =
            ParamsGenerator::new(&params_struct_name, &op.parameters).generate()?;

        let (params, param_fn, param_struct_field, param_type) =
            params_and_fn(params, params_struct_name);

        let path_fn = path_fn(replace_fields);

        let method_ident = format_ident!("{}", method.to_string().to_uppercase());

        let path = self.name;

        let response_name_type = op
            .responses
            .default
            .as_ref()
            .and_then(|r| match r {
                openapiv3::ReferenceOr::Reference { reference } => reference
                    .strip_prefix("#/components/schemas/")
                    .map(|name| name.to_pascal_case()),
                _ => None,
            })
            .map(|pascal| quote! { type Response = #pascal; })
            .unwrap_or_else(|| quote! { type Response = (); });

        let body_name = op
            .request_body
            .as_ref()
            .and_then(|rb| match rb {
                openapiv3::ReferenceOr::Reference { reference } => reference
                    .strip_prefix("#/components/schemas/")
                    .map(|name| name.to_pascal_case()),
                _ => None,
            })
            .map(|pascal| format_ident!("{}", pascal));

        let (body, body_fn, body_type) = body_and_fn(body_name);

        // TODO: Headers and Cookies
        Ok((
            struct_name.to_snake_case(),
            quote! {
                use crate::Request;
                use reqwest::Method;
                use serde::{Deserialize, Serialize};
                use std::borrow::Cow;

                #doc_comment
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct #struct_ident {
                    #inpath_fields
                    #body
                    #param_struct_field
                }

                #params

                impl Request for #struct_ident {
                    #response_name_type
                    #body_type
                    #param_type

                    const METHOD: Method = Method::#method_ident;
                    const PATH: &'static str = #path;

                    #body_fn

                    #param_fn

                    #path_fn
                }
            },
        ))
    }
}

type ParamStruct = TokenStream;

type ParamsFn = TokenStream;

type ParamsFieldInStruct = TokenStream;

type ParamsTypeRequest = TokenStream;

pub fn params_and_fn(
    params: TokenStream,
    struct_name: Ident,
) -> (
    ParamStruct,
    ParamsFn,
    ParamsFieldInStruct,
    ParamsTypeRequest,
) {

    if params.is_empty() {

        return (
            quote! {},
            quote! {},
            quote! {},
            quote! {
                type Params = ();
            },
        );
    };

    (
        params,
        quote! {
            fn params(&self) -> Option<&Self::Params> {
                Some(&self.params)
            }
        },
        quote! {
            pub params: #struct_name,
        },
        quote! {
            type Params = #struct_name;
        },
    )
}

type PathFn = TokenStream;

pub fn path_fn(replace_fields: TokenStream) -> PathFn {

    if replace_fields.is_empty() {

        return quote! {};
    }

    quote! {
        fn path(&self) -> Cow<'static, str> {
            let path = Self::PATH
                #replace_fields;

            Cow::Owned(path)
        }
    }
}

type BodyInStructField = TokenStream;

type BodyFn = TokenStream;

type BodyTypeRequest = TokenStream;

pub fn body_and_fn(struct_name: Option<Ident>) -> (BodyInStructField, BodyFn, BodyTypeRequest) {

    let Some(struct_name) = struct_name else {

        return (
            quote! {},
            quote! {},
            quote! {
                type Body = ();
            },
        );
    };

    (
        quote! {
            pub body: #struct_name,
        },
        quote! {
            fn body(&self) -> Option<&Self::Body> {
                Some(&self.body)
            }
        },
        quote! {
            type Body = #struct_name;
        },
    )
}
