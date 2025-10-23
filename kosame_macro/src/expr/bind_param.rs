use super::Visitor;
use proc_macro2::{Span, TokenStream};
use quote::{IdentFragment, ToTokens, quote};
use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

pub struct BindParam {
    colon: Token![:],
    name: Ident,
}

impl BindParam {
    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        visitor.visit_bind_param(self);
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(Token![:])
    }
}

impl Spanned for BindParam {
    fn span(&self) -> Span {
        self.colon.span.join(self.name.span())
    }
}

impl Parse for BindParam {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            colon: input.parse()?,
            name: input.parse()?,
        })
    }
}

impl ToTokens for BindParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        quote! { scope::params::#name::BIND_PARAM }.to_tokens(tokens)
    }
}
