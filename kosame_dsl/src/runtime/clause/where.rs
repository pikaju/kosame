use std::fmt::Write;

use crate::{runtime::expr::Expr, sql};

pub struct Where {
    expr: Expr,
}

impl Where {
    #[inline]
    pub const fn new(expr: Expr) -> Self {
        Self { expr }
    }

    #[inline]
    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        formatter.write_str(" where ")?;
        self.expr.fmt_sql(formatter)?;
        Ok(())
    }

    #[inline]
    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}
