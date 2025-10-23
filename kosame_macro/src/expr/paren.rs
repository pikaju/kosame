use super::Expr;
use super::Visitor;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
};

pub struct Paren {
    paren: syn::token::Paren,
    expr: Box<Expr>,
}

impl Paren {
    pub fn accept<'a>(&'a self, _visitor: &mut impl Visitor<'a>) {}
}

impl Spanned for Paren {
    fn span(&self) -> Span {
        self.paren.span()
    }
}

impl Parse for Paren {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            paren: parenthesized!(content in input),
            expr: content.parse()?,
        })
    }
}

impl ToTokens for Paren {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let expr = &self.expr;
        quote! {
            ::kosame::repr::expr::Paren::new(&#expr)
        }
        .to_tokens(tokens);
    }
}
