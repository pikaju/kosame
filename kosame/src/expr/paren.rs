use std::fmt::Write;

use crate::{dbms::Dialect, sql_formatter::SqlFormatter};

use super::Expr;

pub struct Paren {
    expr: &'static Expr,
}

impl Paren {
    pub const fn new(expr: &'static Expr) -> Self {
        Self { expr }
    }

    pub fn fmt_sql<D: Dialect>(&self, formatter: &mut SqlFormatter<D>) -> std::fmt::Result {
        formatter.write_str("(")?;
        self.expr.fmt_sql(formatter)?;
        formatter.write_str(")")?;
        Ok(())
    }
}
