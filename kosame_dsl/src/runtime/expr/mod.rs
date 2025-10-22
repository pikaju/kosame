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

pub enum Expr<'a> {
    Binary(Binary<'a>),
    BindParam(BindParam<'a>),
    Call(Call<'a>),
    Cast(Cast<'a>),
    ColumnRef(ColumnRef<'a>),
    Lit(Lit),
    Paren(Paren<'a>),
    Unary(Unary<'a>),
}

macro_rules! variants {
    ($macro:ident!()) => {
        $macro!(
            Binary
            BindParam
            Call
            Cast
            ColumnRef
            Lit
            Paren
            Unary
        )
    };
}

impl<'a> Expr<'a> {
    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        macro_rules! branches {
            ($($variant:ident)*) => {
                match self {
                    $(Self::$variant(inner) => inner.fmt_sql(formatter)),*
                }
            };
        }

        variants!(branches!())
    }
}
