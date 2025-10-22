use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident, Token, parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
};

use super::{Expr, Visitor};

pub struct Cast {
    _cast: kw::cast,
    _paren: Paren,
    value: Box<Expr>,
    _as: Token![as],
    data_type: Ident,
}

impl Cast {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::cast)
    }

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        self.value.accept(visitor);
    }
}

impl Parse for Cast {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _cast: input.parse()?,
            _paren: parenthesized!(content in input),
            value: content.parse()?,
            _as: content.parse()?,
            data_type: content.parse()?,
        })
    }
}

impl ToTokens for Cast {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = &self.value;
        let data_type = &self.data_type.to_string();
        quote! {
            ::kosame::expr::Cast::new(&#value, #data_type)
        }
        .to_tokens(tokens);
    }
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(cast);
}
