use crate::{runtime::schema::Column, sql};

pub struct ColumnRef {
    column: &'static Column,
}

impl ColumnRef {
    pub const fn new(column: &'static Column) -> Self {
        Self { column }
    }

    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        formatter.write_ident(self.column.name())
    }
}
