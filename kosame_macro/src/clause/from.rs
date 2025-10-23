use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Path,
    parse::{Parse, ParseStream},
};

use crate::path_ext::PathExt;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(from);
}

pub struct From {
    _from: kw::from,
    table: Path,
}

impl From {
    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Self::peek(input).then(|| input.parse()).transpose()
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::from)
    }
}

impl Parse for From {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _from: input.parse()?,
            table: input.parse()?,
        })
    }
}

impl ToTokens for From {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let table = self.table.to_call_site(1);
        quote! { ::kosame::repr::clause::From::new(&#table::TABLE) }.to_tokens(tokens);
    }
}
