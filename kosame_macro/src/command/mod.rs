mod select;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute,
    parse::{Parse, ParseStream},
};

pub use select::*;

use crate::{clause::Fields, visitor::Visitor};

pub enum Command {
    Select(Select),
}

impl Command {
    pub fn attrs(&self) -> &[Attribute] {
        match self {
            Self::Select(inner) => &inner.attrs,
        }
    }

    pub fn fields(&self) -> &Fields {
        match self {
            Self::Select(inner) => &inner.select.fields,
        }
    }

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        match self {
            Self::Select(inner) => inner.accept(visitor),
        }
    }
}

impl Parse for Command {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if Select::peek(input) {
            Ok(Self::Select(input.parse()?))
        } else {
            Err(syn::Error::new(input.span(), "expected command"))
        }
    }
}

impl ToTokens for Command {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Select(inner) => quote! {
                ::kosame::repr::command::Command::Select(#inner)
            },
        }
        .to_tokens(tokens)
    }
}
