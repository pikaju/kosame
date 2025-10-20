use std::fmt::Write;

use crate::sql;

use super::Expr;

pub struct Cast {
    value: &'static Expr,
    data_type: &'static str,
}

impl Cast {
    #[inline]
    pub const fn new(value: &'static Expr, data_type: &'static str) -> Self {
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
