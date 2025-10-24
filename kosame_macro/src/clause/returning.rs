use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};

use crate::clause::Fields;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(returning);
}

pub struct Returning {
    _returning: kw::returning,
    fields: Fields,
}

impl Returning {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::returning)
    }

    pub fn fields(&self) -> &Fields {
        &self.fields
    }
}

impl Parse for Returning {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _returning: input.parse()?,
            fields: input.parse()?,
        })
    }
}

impl ToTokens for Returning {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fields = &self.fields;
        quote! {
            ::kosame::repr::clause::Returning::new(#fields)
        }
        .to_tokens(tokens)
    }
}
