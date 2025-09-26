use quote::{ToTokens, quote};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
};

use super::Expr;

pub struct Paren {
    _paren: syn::token::Paren,
    expr: Box<Expr>,
}

impl Parse for Paren {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _paren: parenthesized!(content in input),
            expr: content.parse()?,
        })
    }
}

impl ToTokens for Paren {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let expr = &self.expr;
        quote! {
            ::kosame::expr::Paren::new(&#expr)
        }
        .to_tokens(tokens);
    }
}
