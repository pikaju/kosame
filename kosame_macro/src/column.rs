use quote::{ToTokens, quote};
use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

use crate::keywords;

pub struct Column {
    name: Ident,
    r#type: Ident,
    not_null: Option<keywords::NotNull>,
    primary_key: Option<keywords::PrimaryKey>,
}

impl Parse for Column {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let r#type = input.parse()?;

        Ok(Self {
            name,
            r#type,
            not_null: None,
            primary_key: None,
        })
    }
}

impl ToTokens for Column {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let name_string = name.to_string();
        quote! {
            /// kosame column
            pub mod #name {
                const NAME: &str = #name_string;
            }
        }
        .to_tokens(tokens);
    }
}
