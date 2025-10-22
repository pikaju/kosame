use crate::runtime::expr::Expr;

pub struct Column<'a> {
    pub name: &'a str,
    pub data_type: &'a str,
    pub primary_key: bool,
    pub not_null: bool,
    pub default: Option<&'a Expr<'a>>,
}

impl<'a> Column<'a> {
    #[inline]
    pub const fn name(&self) -> &str {
        self.name
    }

    #[inline]
    pub const fn data_type(&self) -> &str {
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
    pub const fn default(&self) -> Option<&Expr<'_>> {
        self.default
    }
}
