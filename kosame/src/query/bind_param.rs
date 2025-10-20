pub struct BindParam {
    name: &'static str,
    ordinal: u32,
}

impl BindParam {
    pub const fn new(name: &'static str, ordinal: u32) -> Self {
        Self { name, ordinal }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }
}
