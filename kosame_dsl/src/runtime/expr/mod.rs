mod binary;
mod bind_param;
mod call;
mod cast;
mod column_ref;
mod lit;
mod paren;
mod unary;

pub use binary::*;
pub use bind_param::*;
pub use call::*;
pub use cast::*;
pub use column_ref::*;
pub use lit::*;
pub use paren::*;
pub use unary::*;

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
