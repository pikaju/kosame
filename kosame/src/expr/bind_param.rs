use crate::{dialect::Dialect, sql_writer::SqlFormatter};

pub struct BindParam {
    param: &'static crate::query::BindParam,
}

impl BindParam {
    pub const fn new(param: &'static crate::query::BindParam) -> Self {
        Self { param }
    }

    pub fn fmt_sql<D: Dialect>(&self, formatter: &mut SqlFormatter<D>) -> std::fmt::Result {
        formatter.write_bind_param(self.param.name(), self.param.ordinal())
    }
}
