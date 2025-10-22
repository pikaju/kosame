use std::fmt::Write;

use crate::expr::Expr;

pub struct OrderBy<'a> {
    entries: &'a [OrderByEntry<'a>],
}

impl<'a> OrderBy<'a> {
    #[inline]
    pub const fn new(entries: &'a [OrderByEntry]) -> Self {
        Self { entries }
    }
}

impl kosame_sql::FmtSql for OrderBy<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        formatter.write_str(" order by ")?;
        for (index, entry) in self.entries.iter().enumerate() {
            entry.fmt_sql(formatter)?;
            if index != self.entries.len() - 1 {
                formatter.write_str(", ")?;
            }
        }
        Ok(())
    }
}

pub struct OrderByEntry<'a> {
    expr: Expr<'a>,
    dir: Option<OrderByDir>,
    nulls: Option<OrderByNulls>,
}

impl<'a> OrderByEntry<'a> {
    #[inline]
    pub const fn new(expr: Expr<'a>, dir: Option<OrderByDir>, nulls: Option<OrderByNulls>) -> Self {
        Self { expr, dir, nulls }
    }
}

impl kosame_sql::FmtSql for OrderByEntry<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        self.expr.fmt_sql(formatter)?;
        match self.dir {
            Some(OrderByDir::Asc) => formatter.write_str(" asc")?,
            Some(OrderByDir::Desc) => formatter.write_str(" desc")?,
            None => {}
        }
        match self.nulls {
            Some(OrderByNulls::First) => formatter.write_str(" nulls first")?,
            Some(OrderByNulls::Last) => formatter.write_str(" nulls last")?,
            None => {}
        }
        Ok(())
    }
}

pub enum OrderByDir {
    Asc,
    Desc,
}

pub enum OrderByNulls {
    First,
    Last,
}
