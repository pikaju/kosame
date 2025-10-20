use super::Visitor;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

pub struct ColumnRef {
    name: Ident,
}

impl ColumnRef {
    pub fn accept<'a>(&'a self, _visitor: &mut impl Visitor<'a>) {}
}

impl Parse for ColumnRef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
        })
    }
}

impl ToTokens for ColumnRef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        quote! {
            ::kosame::expr::ColumnRef::new(
                &scope::columns::#name::COLUMN
            )
        }
        .to_tokens(tokens)
    }
}
