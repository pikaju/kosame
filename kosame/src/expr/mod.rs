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

use crate::{dbms::Dialect, sql_formatter::SqlFormatter};

pub enum Expr {
    Binary(Binary),
    BindParam(BindParam),
    ColumnRef(ColumnRef),
    Lit(Lit),
    Paren(Paren),
}

impl Expr {
    pub fn fmt_sql<D: Dialect>(&self, formatter: &mut SqlFormatter<D>) -> std::fmt::Result {
        match self {
            Self::Binary(inner) => inner.fmt_sql(formatter),
            Self::BindParam(inner) => inner.fmt_sql(formatter),
            Self::ColumnRef(inner) => inner.fmt_sql(formatter),
            Self::Lit(inner) => inner.fmt_sql(formatter),
            Self::Paren(inner) => inner.fmt_sql(formatter),
        }
    }
}
