pub struct Column {
    name: &'static str,
    alias: Option<&'static str>,
}

impl Column {
    pub const fn new(name: &'static str, alias: Option<&'static str>) -> Self {
        Self { name, alias }
    }

    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub const fn alias(&self) -> Option<&'static str> {
        self.alias
    }
}
