use crate::{dialect::Dialect, schema::Column, sql_writer::SqlFormatter};

pub struct ColumnRef {
    column: &'static Column,
}

impl ColumnRef {
    pub const fn new(column: &'static Column) -> Self {
        Self { column }
    }

    pub fn fmt_sql<D: Dialect>(&self, formatter: &mut SqlFormatter<D>) -> std::fmt::Result {
        formatter.write_ident(self.column.name())
    }
}
