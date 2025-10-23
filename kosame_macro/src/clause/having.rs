use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Token,
    parse::{Parse, ParseStream},
};

use crate::expr::Expr;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(having);
}

pub struct Having {
    _having: kw::having,
    expr: Expr,
}

impl Having {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Self::peek(input).then(|| input.parse()).transpose()
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(Token![where])
    }
}

impl Parse for Having {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _having: input.parse()?,
            expr: input.parse()?,
        })
    }
}

impl ToTokens for Having {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = &self.expr;
        quote! { ::kosame::repr::clause::Having::new(#expr) }.to_tokens(tokens);
    }
}
