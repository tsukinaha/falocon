use proc_macro2::TokenStream;
use quote::quote;

pub struct DocsGenerator;

impl DocsGenerator {
    pub fn generate(description: Option<&str>) -> TokenStream {
        if let Some(desc) = description
            && !desc.trim().is_empty()
        {
            let clean_desc = desc
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join(" ");

            return quote! {
                #[doc = #clean_desc]
            };
        }

        quote! {}
    }
}
