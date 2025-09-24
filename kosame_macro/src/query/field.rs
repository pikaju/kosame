use proc_macro2::Span;
use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

use super::QueryNode;

pub enum QueryField {
    Column { name: Ident },
    Relation { name: Ident, node: QueryNode },
}

impl QueryField {
    pub fn name(&self) -> Option<&Ident> {
        match self {
            Self::Column { name } => Some(name),
            Self::Relation { name, .. } => Some(name),
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Column { name } => name.span(),
            Self::Relation { name, .. } => name.span(),
        }
    }
}

impl Parse for QueryField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        if input.peek(syn::token::Brace) {
            Ok(Self::Relation {
                name,
                node: input.parse()?,
            })
        } else {
            Ok(Self::Column { name })
        }
    }
}
