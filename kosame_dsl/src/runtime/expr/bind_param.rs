use crate::sql;

pub struct BindParam {
    name: &'static str,
    ordinal: u32,
}

impl BindParam {
    pub const fn new(name: &'static str, ordinal: u32) -> Self {
        Self { name, ordinal }
    }

    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        formatter.write_bind_param(self.name, self.ordinal)
    }
}
