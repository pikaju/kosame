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
}
