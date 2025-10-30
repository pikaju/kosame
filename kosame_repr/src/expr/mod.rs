mod binary;
mod bind_param;
mod call;
mod cast;
mod column_ref;
mod lit;
mod paren;
mod raw;
mod unary;

pub use binary::*;
pub use bind_param::*;
pub use call::*;
pub use cast::*;
pub use column_ref::*;
pub use lit::*;
pub use paren::*;
pub use raw::*;
pub use unary::*;

pub enum Expr<'a> {
    Binary(Binary<'a>),
    BindParam(BindParam<'a>),
    Call(Call<'a>),
    Cast(Cast<'a>),
    ColumnRef(ColumnRef<'a>),
    Lit(Lit),
    Paren(Paren<'a>),
    Raw(Raw<'a>),
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
            Raw
            Unary
        )
    };
}

impl kosame_sql::FmtSql for Expr<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
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
