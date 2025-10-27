use std::fmt::Write;

use crate::expr::Expr;

pub struct Set<'a> {
    entries: &'a [SetEntry<'a>],
}

impl<'a> Set<'a> {
    #[inline]
    pub const fn new(entries: &'a [SetEntry<'a>]) -> Self {
        Self { entries }
    }

    #[inline]
    pub const fn entries(&self) -> &'a [SetEntry<'a>] {
        self.entries
    }
}

impl kosame_sql::FmtSql for Set<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        formatter.write_str(" set ")?;
        for (index, entry) in self.entries.into_iter().enumerate() {
            entry.fmt_sql(formatter)?;
            if index < self.entries.len() - 1 {
                formatter.write_str(", ")?;
            }
        }
        Ok(())
    }
}

pub enum SetEntry<'a> {
    Default { column: &'a str },
    Expr { column: &'a str, expr: Expr<'a> },
}

impl kosame_sql::FmtSql for SetEntry<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        match self {
            Self::Default { column } => {
                formatter.write_ident(column)?;
                formatter.write_str(" = default")?;
            }
            Self::Expr { column, expr } => {
                formatter.write_ident(column)?;
                formatter.write_str(" = ")?;
                expr.fmt_sql(formatter)?;
            }
        }
        Ok(())
    }
}
