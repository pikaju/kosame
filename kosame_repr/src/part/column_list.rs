use std::fmt::Write;

pub struct ColumnList<'a> {
    columns: &'a [&'a str],
}

impl<'a> ColumnList<'a> {
    #[inline]
    pub const fn new(columns: &'a [&'a str]) -> Self {
        Self { columns }
    }

    #[inline]
    pub const fn columns(&self) -> &'a [&'a str] {
        self.columns
    }
}

impl kosame_sql::FmtSql for ColumnList<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        formatter.write_str(" (")?;
        for (index, alias) in self.columns.iter().enumerate() {
            formatter.write_ident(alias)?;
            if index != self.columns.len() - 1 {
                formatter.write_str(", ")?;
            }
        }
        formatter.write_str(")")?;
        Ok(())
    }
}
