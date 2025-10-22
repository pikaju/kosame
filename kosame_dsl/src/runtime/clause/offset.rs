use std::fmt::Write;

use crate::{runtime::expr::Expr, sql};

pub struct Offset<'a> {
    expr: Expr<'a>,
}

impl<'a> Offset<'a> {
    #[inline]
    pub const fn new(expr: Expr<'a>) -> Self {
        Self { expr }
    }

    #[inline]
    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        formatter.write_str(" offset ")?;
        self.expr.fmt_sql(formatter)?;
        Ok(())
    }
}
