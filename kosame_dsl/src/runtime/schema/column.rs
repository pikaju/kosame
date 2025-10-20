pub struct Column {
    pub name: &'static str,
    pub data_type: &'static str,
}

impl Column {
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }
}
