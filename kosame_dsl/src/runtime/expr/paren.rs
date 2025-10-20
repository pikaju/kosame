use std::fmt::Write;

use crate::sql;

use super::Expr;

pub struct Paren {
    expr: &'static Expr,
}

impl Paren {
    #[inline]
    pub const fn new(expr: &'static Expr) -> Self {
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
