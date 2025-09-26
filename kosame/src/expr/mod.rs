mod binary;
mod column_ref;
mod lit;
mod paren;

pub use binary::{BinOp, Binary};
pub use column_ref::ColumnRef;
pub use lit::Lit;
pub use paren::Paren;

pub enum Expr {
    Binary(Binary),
    ColumnRef(ColumnRef),
    Lit(Lit),
    Paren(Paren),
}

impl Expr {
    pub fn to_sql_string(&self, buf: &mut String) {
        match self {
            Self::Binary(inner) => inner.to_sql_string(buf),
            Self::ColumnRef(inner) => inner.to_sql_string(buf),
            Self::Lit(inner) => inner.to_sql_string(buf),
            Self::Paren(inner) => inner.to_sql_string(buf),
        }
    }
}

macro_rules! impl_from {
    ($type:ident) => {
        impl From<$type> for Expr {
            fn from(v: $type) -> Self {
                Self::$type(v)
            }
        }
    };
}

impl_from!(Binary);
impl_from!(ColumnRef);
impl_from!(Lit);
impl_from!(Paren);
