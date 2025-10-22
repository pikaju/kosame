use crate::sql;

pub struct BindParam<'a> {
    name: &'a str,
    ordinal: u32,
}

impl<'a> BindParam<'a> {
    #[inline]
    pub const fn new(name: &'a str, ordinal: u32) -> Self {
        Self { name, ordinal }
    }

    #[inline]
    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        formatter.write_bind_param(self.name, self.ordinal)
    }
}
