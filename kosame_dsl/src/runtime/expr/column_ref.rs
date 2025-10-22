use crate::{runtime::schema::Column, sql};

pub struct ColumnRef<'a> {
    column: &'a Column<'a>,
}

impl<'a> ColumnRef<'a> {
    #[inline]
    pub const fn new(column: &'a Column) -> Self {
        Self { column }
    }

    #[inline]
    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        formatter.write_ident(self.column.name())
    }
}
