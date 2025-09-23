use proc_macro2::Span;
use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
};

use super::QueryNode;

pub enum QueryField {
    Column { name: Ident },
    Relation { name: Ident, node: QueryNode },
    Star(Token![*]),
}

impl QueryField {
    pub fn name(&self) -> Option<&Ident> {
        match self {
            Self::Column { name } => Some(name),
            Self::Relation { name, .. } => Some(name),
            Self::Star(_) => None,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Column { name } => name.span(),
            Self::Relation { name, .. } => name.span(),
            Self::Star(token) => token.span,
        }
    }
}

impl Parse for QueryField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![*]) {
            return Ok(Self::Star(input.parse()?));
        }

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
