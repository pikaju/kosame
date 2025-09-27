pub struct BindParam {
    param: &'static crate::query::BindParam,
}

impl BindParam {
    pub const fn new(param: &'static crate::query::BindParam) -> Self {
        Self { param }
    }

    pub fn to_sql_string(&self, buf: &mut String) {
        *buf += "$";
        *buf += &self.param.ordinal().to_string();
    }
}
