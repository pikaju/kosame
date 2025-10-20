use crate::runtime::expr::Expr;

pub struct Column {
    pub name: &'static str,
    pub data_type: &'static str,
    pub primary_key: bool,
    pub not_null: bool,
    pub default: Option<&'static Expr>,
}

impl Column {
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub const fn data_type(&self) -> &'static str {
        self.data_type
    }

    #[inline]
    pub const fn primary_key(&self) -> bool {
        self.primary_key
    }

    #[inline]
    pub const fn not_null(&self) -> bool {
        self.not_null
    }

    #[inline]
    pub const fn default(&self) -> Option<&'static Expr> {
        self.default
    }
}
