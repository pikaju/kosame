use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Path, Token,
    parse::{Parse, ParseStream},
};

pub struct ColonType {
    colon: Token![:],
    type_path: Path,
}

impl ColonType {
    pub fn type_path(&self) -> &Path {
        &self.type_path
    }

    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Ok(if input.peek(Token![:]) {
            Some(input.parse()?)
        } else {
            None
        })
    }
}

impl Parse for ColonType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            colon: input.parse()?,
            type_path: input.parse()?,
        })
    }
}

impl ToTokens for ColonType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.colon.to_tokens(tokens);
        self.type_path.to_tokens(tokens);
    }
}
