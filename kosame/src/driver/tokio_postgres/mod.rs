use std::fmt::Write;

use crate::{driver::Connection, query::BindParamOrdinal};

impl Connection for tokio_postgres::Client {
    type Dialect = crate::dbms::postgres::Dialect;
    type Params<'a> = Vec<&'a (dyn postgres_types::ToSql + std::marker::Sync + 'a)>;
    type Row = tokio_postgres::Row;
    type Error = tokio_postgres::Error;

    async fn query(
        &mut self,
        sql: &str,
        params: &Self::Params<'_>,
    ) -> Result<Vec<Self::Row>, Self::Error> {
        tokio_postgres::Client::query(self, sql, params).await
    }
}

impl Connection for tokio_postgres::Transaction<'_> {
    type Dialect = crate::dbms::postgres::Dialect;
    type Params<'a> = Vec<&'a (dyn postgres_types::ToSql + std::marker::Sync + 'a)>;
    type Row = tokio_postgres::Row;
    type Error = tokio_postgres::Error;

    async fn query(
        &mut self,
        sql: &str,
        params: &Self::Params<'_>,
    ) -> Result<Vec<Self::Row>, Self::Error> {
        tokio_postgres::Transaction::<'_>::query(self, sql, params).await
    }
}
