use syn::parse::{Parse, ParseStream};

use super::{column_override::ColumnOverride, relation::Relation};

pub enum FieldSpec {
    ColumnOverride(ColumnOverride),
    Relation(Relation),
}

impl Parse for FieldSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek3(syn::token::Paren) {
            Ok(Self::Relation(input.parse()?))
        } else {
            Ok(Self::ColumnOverride(input.parse()?))
        }
    }
}
