use std::fmt::Write;

use super::Expr;

pub struct Call<'a> {
    function: &'a str,
    params: &'a [&'a Expr<'a>],
}

impl<'a> Call<'a> {
    #[inline]
    pub const fn new(function: &'a str, params: &'a [&'a Expr]) -> Self {
        Self { function, params }
    }
}

impl kosame_sql::FmtSql for Call<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
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
