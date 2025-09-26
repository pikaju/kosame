use crate::schema::Column;

pub struct ColumnRef {
    column: &'static Column,
}

impl ColumnRef {
    pub const fn new(column: &'static Column) -> Self {
        Self { column }
    }

    pub fn to_sql_string(&self, buf: &mut String) {
        *buf += self.column.name();
    }
}
