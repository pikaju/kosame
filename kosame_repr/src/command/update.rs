use std::fmt::Write;

use crate::{clause::*, schema::Table};

pub struct Update<'a> {
    table: &'a Table<'a>,
    set: Set<'a>,
    from: Option<From<'a>>,
    r#where: Option<Where<'a>>,
    returning: Option<Returning<'a>>,
}

impl<'a> Update<'a> {}

impl kosame_sql::FmtSql for Update<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        formatter.write_str("update ")?;
        formatter.write_ident(self.table.name())?;
        self.set.fmt_sql(formatter)?;

        if let Some(from) = &self.from {
            from.fmt_sql(formatter)?;
        }
        if let Some(r#where) = &self.r#where {
            r#where.fmt_sql(formatter)?;
        }
        if let Some(returning) = &self.returning {
            returning.fmt_sql(formatter)?;
        }

        Ok(())
    }
}
