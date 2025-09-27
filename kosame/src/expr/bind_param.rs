pub struct BindParam {
    name: &'static str,
    ordinal: u32,
}

impl BindParam {
    pub const fn new(name: &'static str, ordinal: u32) -> Self {
        Self { name, ordinal }
    }

    pub fn to_sql_string(&self, buf: &mut String) {
        *buf += "$";
        *buf += &self.ordinal.to_string();
    }
}
