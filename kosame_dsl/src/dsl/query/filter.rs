use syn::{
    Token,
    parse::{Parse, ParseStream},
};

use crate::dsl::expr::Expr;

pub struct Filter {
    _where: Token![where],
    expr: Expr,
}

impl Filter {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Self::peek(input).then(|| input.parse()).transpose()
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(Token![where])
    }
}

impl Parse for Filter {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _where: input.parse()?,
            expr: input.parse()?,
        })
    }
}
