use std::fmt::Write;

use crate::sql;

use super::Expr;

pub struct Cast<'a> {
    value: &'a Expr<'a>,
    data_type: &'a str,
}

impl<'a> Cast<'a> {
    #[inline]
    pub const fn new(value: &'a Expr, data_type: &'a str) -> Self {
        Self { value, data_type }
    }

    #[inline]
    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        formatter.write_str("cast(")?;
        self.value.fmt_sql(formatter)?;
        formatter.write_str(" as ")?;
        formatter.write_str(self.data_type)?;
        formatter.write_str(")")?;
        Ok(())
    }
}
