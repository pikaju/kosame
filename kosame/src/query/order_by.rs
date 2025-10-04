use std::fmt::Write;

use crate::{dbms::Dialect, expr::Expr, sql_formatter::SqlFormatter};

pub struct OrderBy {
    entries: &'static [OrderByEntry],
}

impl OrderBy {
    pub const fn new(entries: &'static [OrderByEntry]) -> Self {
        Self { entries }
    }

    pub fn fmt_sql<D: Dialect>(&self, formatter: &mut SqlFormatter<D>) -> std::fmt::Result {
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

pub struct OrderByEntry {
    expr: Expr,
    dir: Option<OrderByDir>,
    nulls: Option<OrderByNulls>,
}

impl OrderByEntry {
    pub const fn new(expr: Expr, dir: Option<OrderByDir>, nulls: Option<OrderByNulls>) -> Self {
        Self { expr, dir, nulls }
    }

    pub fn fmt_sql<D: Dialect>(&self, formatter: &mut SqlFormatter<D>) -> std::fmt::Result {
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
