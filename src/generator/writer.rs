use std::{collections::HashMap, io::Write};

use super::OperationMap;
use proc_macro2::TokenStream;
use quote::format_ident;

const CARGO_TOML: &str = include_str!("../../template/Cargo.toml");

const RUSTFMT_TOML: &str = include_str!("../../template/rustfmt.toml");

const LIB_RS: &str = include_str!("../../template/src/lib.rs");

const REQUEST_RS: &str = include_str!("../../template/src/request.rs");

const ROUTE_RS: &str = include_str!("../../template/src/route.rs");

const CLIENT_RS: &str = include_str!("../../template/src/client.rs");

const ERROR_RS: &str = include_str!("../../template/src/error.rs");

pub struct CrateWriter<'a> {
    pub relative_path: &'a str,
    pub types: TokenStream,
    pub methods: HashMap<String, TokenStream>,
}

impl<'a> CrateWriter<'a> {
    pub fn new(relative_path: &'a str, types: TokenStream, methods: OperationMap) -> Self {

        Self {
            relative_path,
            types,
            methods,
        }
    }

    pub fn add_method(&mut self, name: String, token: TokenStream) {

        self.methods.insert(name, token);
    }

    pub fn write(&self) -> std::io::Result<()> {

        let path = std::path::Path::new(self.relative_path);

        std::fs::create_dir_all(path)?;

        let methods_mod_path = std::path::Path::new(path).join("src/methods");

        if !methods_mod_path.exists() {

            std::fs::create_dir_all(&methods_mod_path)?;
        }

        let src_path = std::path::Path::new(path).join("src");

        if !src_path.exists() {

            std::fs::create_dir_all(&src_path)?;
        }

        std::fs::write(std::path::Path::new(path).join("Cargo.toml"), CARGO_TOML)?;

        std::fs::write(
            std::path::Path::new(path).join("rustfmt.toml"),
            RUSTFMT_TOML,
        )?;

        std::fs::write(std::path::Path::new(path).join("src/lib.rs"), LIB_RS)?;

        std::fs::write(
            std::path::Path::new(path).join("src/request.rs"),
            REQUEST_RS,
        )?;

        std::fs::write(std::path::Path::new(path).join("src/route.rs"), ROUTE_RS)?;

        std::fs::write(std::path::Path::new(path).join("src/client.rs"), CLIENT_RS)?;

        std::fs::write(std::path::Path::new(path).join("src/error.rs"), ERROR_RS)?;

        std::fs::write(
            std::path::Path::new(path).join("src/types.rs"),
            self.types.to_string(),
        )?;

        let mut methods_mod_file = std::fs::File::create(methods_mod_path.join("mod.rs"))?;

        let mut methods_mod_token = TokenStream::new();

        methods_mod_token.extend(quote::quote! {
            use super::types::*;
        });

        for (name, token) in &self.methods {

            let file_name = format!("{name}.rs");

            let file_path = methods_mod_path.join(&file_name);

            let mut file = std::fs::File::create(file_path)?;

            file.write_all(token.to_string().as_bytes())?;

            let name_ident = format_ident!("{}", name);

            methods_mod_token.extend(quote::quote! {
                pub mod #name_ident;
                pub use #name_ident ::*;
            });
        }

        methods_mod_file.write_all(methods_mod_token.to_string().as_bytes())?;

        Ok(())
    }
}
