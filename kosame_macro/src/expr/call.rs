use super::{Expr, Visitor};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct Call {
    function: Ident,
    _paren: syn::token::Paren,
    params: Punctuated<Expr, Token![,]>,
}

impl Call {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Ident) && input.peek2(syn::token::Paren)
    }

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        for param in self.params.iter() {
            param.accept(visitor);
        }
    }
}

impl Parse for Call {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            function: input.parse()?,
            _paren: parenthesized!(content in input),
            params: content.parse_terminated(Expr::parse, Token![,])?,
        })
    }
}

impl ToTokens for Call {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let function_name = &self.function.to_string();
        let params = self.params.iter();
        quote! {
            ::kosame::repr::expr::Call::new(
                #function_name,
                &[#(&#params),*]
            )
        }
        .to_tokens(tokens)
    }
}
