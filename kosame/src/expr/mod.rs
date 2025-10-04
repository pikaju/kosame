mod binary;
mod bind_param;
mod call;
mod cast;
mod column_ref;
mod lit;
mod paren;
mod unary;

pub use binary::{BinOp, Binary};
pub use bind_param::BindParam;
pub use call::Call;
pub use cast::Cast;
pub use column_ref::ColumnRef;
pub use lit::Lit;
pub use paren::Paren;
pub use unary::{Unary, UnaryOp};

use crate::sql;

pub enum Expr {
    Binary(Binary),
    BindParam(BindParam),
    Call(Call),
    Cast(Cast),
    ColumnRef(ColumnRef),
    Lit(Lit),
    Paren(Paren),
    Unary(Unary),
}

impl Expr {
    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        match self {
            Self::Binary(inner) => inner.fmt_sql(formatter),
            Self::BindParam(inner) => inner.fmt_sql(formatter),
            Self::Call(inner) => inner.fmt_sql(formatter),
            Self::Cast(inner) => inner.fmt_sql(formatter),
            Self::ColumnRef(inner) => inner.fmt_sql(formatter),
            Self::Lit(inner) => inner.fmt_sql(formatter),
            Self::Paren(inner) => inner.fmt_sql(formatter),
            Self::Unary(inner) => inner.fmt_sql(formatter),
        }
    }
}
