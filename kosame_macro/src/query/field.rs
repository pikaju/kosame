use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

use super::QueryNodeBody;

pub enum QueryField {
    Column { name: Ident },
    Relation { name: Ident, body: QueryNodeBody },
}

impl QueryField {
    pub fn name(&self) -> &Ident {
        match self {
            Self::Column { name } => name,
            Self::Relation { name, .. } => name,
        }
    }
}

impl Parse for QueryField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        if input.peek(syn::token::Brace) {
            Ok(Self::Relation {
                name,
                body: input.parse()?,
            })
        } else {
            Ok(Self::Column { name })
        }
    }
}
