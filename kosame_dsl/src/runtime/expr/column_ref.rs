use crate::runtime::schema::Column;

pub struct ColumnRef<'a> {
    column: &'a Column<'a>,
}

impl<'a> ColumnRef<'a> {
    #[inline]
    pub const fn new(column: &'a Column) -> Self {
        Self { column }
    }
}

impl kosame_sql::FmtSql for ColumnRef<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        formatter.write_ident(self.column.name())
    }
}
