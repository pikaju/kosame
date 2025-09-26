mod binary;
mod bind_param;
mod column_ref;
mod lit;
mod paren;

pub use binary::{BinOp, Binary};
pub use bind_param::BindParam;
pub use column_ref::ColumnRef;
pub use lit::Lit;
pub use paren::Paren;

pub enum Expr {
    Binary(Binary),
    BindParam(BindParam),
    ColumnRef(ColumnRef),
    Lit(Lit),
    Paren(Paren),
}

impl Expr {
    pub fn to_sql_string(&self, buf: &mut String) {
        match self {
            Self::Binary(inner) => inner.to_sql_string(buf),
            Self::BindParam(inner) => inner.to_sql_string(buf),
            Self::ColumnRef(inner) => inner.to_sql_string(buf),
            Self::Lit(inner) => inner.to_sql_string(buf),
            Self::Paren(inner) => inner.to_sql_string(buf),
        }
    }
}
