use syn::parse::{Parse, ParseStream};

use crate::expr::Expr;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(limit);
}

pub struct LimitClause {
    _limit: kw::limit,
    expr: Expr,
}

impl LimitClause {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Self::peek(input).then(|| input.parse()).transpose()
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::limit)
    }
}

impl Parse for LimitClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _limit: input.parse()?,
            expr: input.parse()?,
        })
    }
}
