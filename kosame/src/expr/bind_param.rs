pub struct BindParam {
    name: &'static str,
}

impl BindParam {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub fn to_sql_string(&self, buf: &mut String) {
        *buf += "$1";
    }
}
