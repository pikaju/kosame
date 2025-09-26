use syn::parse::{Parse, ParseStream};

use crate::expr::Expr;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(filter);
}

pub struct FilterClause {
    _filter: kw::filter,
    expr: Expr,
}

impl FilterClause {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Ok(if input.peek(kw::filter) {
            Some(input.parse()?)
        } else {
            None
        })
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::filter)
    }
}

impl Parse for FilterClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _filter: input.parse()?,
            expr: input.parse()?,
        })
    }
}
