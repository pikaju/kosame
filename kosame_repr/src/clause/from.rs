use std::fmt::Write;

use crate::schema::Table;

pub struct From<'a> {
    table: &'a Table<'a>,
}

impl<'a> From<'a> {
    #[inline]
    pub const fn new(table: &'a Table<'a>) -> Self {
        Self { table }
    }

    #[inline]
    pub const fn table(&self) -> &'a Table<'a> {
        self.table
    }
}

impl kosame_sql::FmtSql for From<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        formatter.write_str(" from ")?;
        formatter.write_ident(self.table.name())?;

        Ok(())
    }
}
