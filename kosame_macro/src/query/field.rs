use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

use super::QueryNode;

pub enum QueryField {
    Column(Ident),
    Relation(QueryNode),
}

impl QueryField {
    /// Returns `true` if the query field is [`Column`].
    ///
    /// [`Column`]: QueryField::Column
    #[must_use]
    pub fn is_column(&self) -> bool {
        matches!(self, Self::Column(..))
    }

    /// Returns `true` if the query field is [`Relation`].
    ///
    /// [`Relation`]: QueryField::Relation
    #[must_use]
    pub fn is_relation(&self) -> bool {
        matches!(self, Self::Relation(..))
    }
}

impl Parse for QueryField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Brace) {
            Ok(Self::Relation(input.parse()?))
        } else {
            Ok(Self::Column(input.parse()?))
        }
    }
}
