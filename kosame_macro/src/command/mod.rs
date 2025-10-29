mod delete;
mod insert;
mod select;
mod update;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute,
    parse::{Parse, ParseStream},
};

pub use delete::*;
pub use insert::*;
pub use select::*;
pub use update::*;

use crate::{clause::Fields, keyword, visitor::Visitor};

pub enum Command {
    Delete(Delete),
    Insert(Insert),
    Select(Box<Select>),
    Update(Update),
}

impl Command {
    pub fn attrs(&self) -> &[Attribute] {
        match self {
            Self::Delete(inner) => &inner.attrs,
            Self::Insert(inner) => &inner.attrs,
            Self::Select(inner) => &inner.attrs,
            Self::Update(inner) => &inner.attrs,
        }
    }

    pub fn fields(&self) -> Option<&Fields> {
        match self {
            Self::Delete(inner) => None,
            Self::Insert(inner) => None,
            Self::Select(inner) => Some(&inner.select.fields),
            Self::Update(inner) => None,
        }
    }

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        match self {
            Self::Delete(inner) => inner.accept(visitor),
            Self::Insert(inner) => inner.accept(visitor),
            Self::Select(inner) => inner.accept(visitor),
            Self::Update(inner) => inner.accept(visitor),
        }
    }
}

impl Parse for Command {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if Delete::peek(input) {
            Ok(Self::Delete(input.parse()?))
        } else if Insert::peek(input) {
            Ok(Self::Insert(input.parse()?))
        } else if Select::peek(input) {
            Ok(Self::Select(input.parse()?))
        } else if Update::peek(input) {
            Ok(Self::Update(input.parse()?))
        } else {
            keyword::group_command::error(input);
        }
    }
}

impl ToTokens for Command {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Delete(inner) => quote! {
                ::kosame::repr::command::Command::Delete(#inner)
            },
            Self::Insert(inner) => quote! {
                ::kosame::repr::command::Command::Insert(#inner)
            },
            Self::Select(inner) => quote! {
                ::kosame::repr::command::Command::Select(#inner)
            },
            Self::Update(inner) => quote! {
                ::kosame::repr::command::Command::Update(#inner)
            },
        }
        .to_tokens(tokens)
    }
}
