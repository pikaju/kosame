use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
};

pub struct BindParam {
    _colon: Token![:],
    name: Ident,
}

impl BindParam {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Token![:])
    }
}

impl Parse for BindParam {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _colon: input.parse()?,
            name: input.parse()?,
        })
    }
}

impl ToTokens for BindParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name.to_string();
        quote! {
            ::kosame::expr::BindParam::new(
                #name
            )
        }
        .to_tokens(tokens)
    }
}
