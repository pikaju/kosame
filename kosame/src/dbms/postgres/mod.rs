use std::fmt::Write;

use tokio_postgres::Client;

use crate::{dbms::Connection, query::BindParamOrdinal};

#[doc(hidden)]
pub mod internal;

pub enum Dialect {}

impl crate::sql::Dialect for Dialect {
    fn ident_esc() -> (&'static str, &'static str) {
        ("\"", "\"")
    }

    fn fmt_bind_param(
        formatter: &mut impl Write,
        _name: &str,
        ordinal: BindParamOrdinal,
    ) -> std::fmt::Result {
        write!(formatter, "${}", ordinal + 1)
    }
}

impl Connection for Client {
    type Dialect = Dialect;
    type Params<'a> = Vec<&'a (dyn postgres_types::ToSql + std::marker::Sync + 'a)>;
    type Row = tokio_postgres::Row;
    type Error = tokio_postgres::Error;

    fn query(
        &mut self,
        sql: &str,
        params: &Self::Params<'_>,
    ) -> impl Future<Output = Result<Vec<Self::Row>, Self::Error>> + Send {
        Client::query(self, sql, params)
    }
}
