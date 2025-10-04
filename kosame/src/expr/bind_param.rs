use crate::sql;

pub struct BindParam {
    param: &'static crate::query::BindParam,
}

impl BindParam {
    pub const fn new(param: &'static crate::query::BindParam) -> Self {
        Self { param }
    }

    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        formatter.write_bind_param(self.param.name(), self.param.ordinal())
    }
}
