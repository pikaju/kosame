use std::fmt::Write;

use crate::{clause::*, schema::Table};

pub struct Insert<'a> {
    table: &'a Table<'a>,
    returning: Option<Returning<'a>>,
}

impl<'a> Insert<'a> {}

impl kosame_sql::FmtSql for Insert<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        formatter.write_str("insert into ")?;
        formatter.write_ident(self.table.name())?;

        unimplemented!();

        if let Some(returning) = &self.returning {
            returning.fmt_sql(formatter)?;
        }

        Ok(())
    }
}
