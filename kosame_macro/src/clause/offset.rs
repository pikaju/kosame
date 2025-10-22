use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};

use crate::expr::Expr;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(offset);
}

pub struct Offset {
    _offset: kw::offset,
    expr: Expr,
}

impl Offset {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Self::peek(input).then(|| input.parse()).transpose()
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::offset)
    }
}

impl Parse for Offset {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _offset: input.parse()?,
            expr: input.parse()?,
        })
    }
}

impl ToTokens for Offset {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = &self.expr;
        quote! { ::kosame::repr::clause::Offset::new(#expr) }.to_tokens(tokens);
    }
}
