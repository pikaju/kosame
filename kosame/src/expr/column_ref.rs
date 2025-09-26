use crate::schema::Column;

pub struct ColumnRef {
    column: &'static Column,
}

impl ColumnRef {
    pub fn to_sql_string(&self, buf: &mut String) {
        *buf += self.column.name();
    }
}
