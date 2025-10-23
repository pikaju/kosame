use std::fmt::Write;

use crate::expr::Expr;

pub struct GroupBy<'a> {
    entries: &'a [GroupByEntry<'a>],
}

impl<'a> GroupBy<'a> {
    #[inline]
    pub const fn new(entries: &'a [GroupByEntry]) -> Self {
        Self { entries }
    }
}

impl kosame_sql::FmtSql for GroupBy<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        formatter.write_str(" group by ")?;
        for (index, entry) in self.entries.iter().enumerate() {
            entry.fmt_sql(formatter)?;
            if index != self.entries.len() - 1 {
                formatter.write_str(", ")?;
            }
        }
        Ok(())
    }
}

pub struct GroupByEntry<'a> {
    expr: Expr<'a>,
}

impl<'a> GroupByEntry<'a> {
    #[inline]
    pub const fn new(expr: Expr<'a>) -> Self {
        Self { expr }
    }
}

impl kosame_sql::FmtSql for GroupByEntry<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        self.expr.fmt_sql(formatter)?;
        Ok(())
    }
}
