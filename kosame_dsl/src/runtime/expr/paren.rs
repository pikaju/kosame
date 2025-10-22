use std::fmt::Write;

use crate::sql;

use super::Expr;

pub struct Paren<'a> {
    expr: &'a Expr<'a>,
}

impl<'a> Paren<'a> {
    #[inline]
    pub const fn new(expr: &'a Expr) -> Self {
        Self { expr }
    }

    #[inline]
    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        formatter.write_str("(")?;
        self.expr.fmt_sql(formatter)?;
        formatter.write_str(")")?;
        Ok(())
    }
}
