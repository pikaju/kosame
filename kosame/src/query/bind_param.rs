pub type BindParamOrdinal = u32;

pub struct BindParam {
    name: &'static str,
    ordinal: BindParamOrdinal,
}

impl BindParam {
    pub const fn new(name: &'static str, ordinal: BindParamOrdinal) -> Self {
        Self { name, ordinal }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn ordinal(&self) -> BindParamOrdinal {
        self.ordinal
    }
}
