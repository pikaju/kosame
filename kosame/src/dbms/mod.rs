#[cfg(feature = "mssql")]
pub mod mssql;

#[cfg(feature = "mysql")]
pub mod mysql;

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "sqlite")]
pub mod sqlite;

use std::fmt::Write;

use crate::query::BindParamOrdinal;

pub trait Dialect {
    fn ident_esc() -> (&'static str, &'static str);
    fn fmt_bind_param(
        formatter: &mut impl Write,
        name: &str,
        ordinal: BindParamOrdinal,
    ) -> std::fmt::Result;
}

pub trait Connection {
    type Dialect: Dialect;
    type Params<'a>;
    type Row;
    type Error;

    async fn query(
        &mut self,
        sql: &str,
        params: &Self::Params<'_>,
    ) -> Result<Vec<Self::Row>, Self::Error>;
}
