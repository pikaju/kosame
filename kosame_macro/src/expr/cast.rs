use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Ident, Token, parenthesized,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::Paren,
};

use super::{Expr, Visitor};

pub struct Cast {
    pub cast: kw::cast,
    pub paren: Paren,
    pub value: Box<Expr>,
    pub _as: Token![as],
    pub data_type: Ident,
}

impl Cast {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::cast)
    }

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        self.value.accept(visitor);
    }

    pub fn span(&self) -> Span {
        self.cast
            .span
            .join(self.paren.span.span())
            .expect("same file")
    }
}

impl Parse for Cast {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            cast: input.parse()?,
            paren: parenthesized!(content in input),
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
            ::kosame::repr::expr::Cast::new(&#value, #data_type)
        }
        .to_tokens(tokens);
    }
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(cast);
}
