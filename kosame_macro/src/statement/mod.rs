use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute,
    parse::{Parse, ParseStream},
};

use crate::command::Command;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(select);
}

pub struct Statement {
    _inner_attrs: Vec<Attribute>,
    command: Command,
}

impl Parse for Statement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _inner_attrs: input.call(Attribute::parse_inner)?,
            command: input.parse()?,
        })
    }
}

impl ToTokens for Statement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let command = &self.command;
        quote! {
            #command
        }
        .to_tokens(tokens);
    }
}
