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
