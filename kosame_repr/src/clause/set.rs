use std::fmt::Write;

use crate::expr::Expr;

pub struct Set<'a> {
    items: &'a [SetItem<'a>],
}

impl<'a> Set<'a> {
    #[inline]
    pub const fn new(items: &'a [SetItem<'a>]) -> Self {
        Self { items }
    }

    #[inline]
    pub const fn items(&self) -> &'a [SetItem<'a>] {
        self.items
    }
}

impl kosame_sql::FmtSql for Set<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        formatter.write_str(" set ")?;
        for (index, item) in self.items.iter().enumerate() {
            item.fmt_sql(formatter)?;
            if index < self.items.len() - 1 {
                formatter.write_str(", ")?;
            }
        }
        Ok(())
    }
}

pub enum SetItem<'a> {
    Default { column: &'a str },
    Expr { column: &'a str, expr: Expr<'a> },
}

impl kosame_sql::FmtSql for SetItem<'_> {
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
