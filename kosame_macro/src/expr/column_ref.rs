use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

pub struct ColumnRef {
    name: Ident,
}

impl ToTokens for ColumnRef {
    fn to_tokens(&self, tokens: &mut TokenStream) {}
}

impl Parse for ColumnRef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
        })
    }
}
