use std::fmt::Write;

use super::Expr;
use crate::sql;

pub struct Call<'a> {
    function: &'a str,
    params: &'a [&'a Expr<'a>],
}

impl<'a> Call<'a> {
    #[inline]
    pub const fn new(function: &'a str, params: &'a [&'a Expr]) -> Self {
        Self { function, params }
    }

    #[inline]
    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        formatter.write_ident(self.function)?;
        formatter.write_str("(")?;
        for (index, param) in self.params.iter().enumerate() {
            param.fmt_sql(formatter)?;
            if index != self.params.len() - 1 {
                formatter.write_str(", ")?;
            }
        }
        formatter.write_str(")")?;
        Ok(())
    }
}
