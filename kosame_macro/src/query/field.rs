use proc_macro2::Span;
use syn::{
    Attribute, Ident,
    parse::{Parse, ParseStream},
};

use super::QueryNode;

pub enum QueryField {
    Column {
        attrs: Vec<Attribute>,
        name: Ident,
    },
    Relation {
        attrs: Vec<Attribute>,
        name: Ident,
        node: QueryNode,
    },
}

impl QueryField {
    pub fn name(&self) -> &Ident {
        match self {
            Self::Column { name, .. } => name,
            Self::Relation { name, .. } => name,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Column { name, .. } => name.span(),
            Self::Relation { name, .. } => name.span(),
        }
    }

    /// Returns `true` if the query field is [`Column`].
    ///
    /// [`Column`]: QueryField::Column
    #[must_use]
    pub fn is_column(&self) -> bool {
        matches!(self, Self::Column { .. })
    }
}

impl Parse for QueryField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let name = input.parse()?;
        if input.peek(syn::token::Brace) {
            Ok(Self::Relation {
                attrs,
                name,
                node: input.parse()?,
            })
        } else {
            Ok(Self::Column { attrs, name })
        }
    }
}
