#[cfg(feature = "mssql")]
pub mod mssql;

#[cfg(feature = "mysql")]
pub mod mysql;

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "sqlite")]
pub mod sqlite;

use crate::sql;

pub trait Connection {
    type Dialect: sql::Dialect;
    type Params<'a>;
    type Row;
    type Error;

    fn query(
        &mut self,
        sql: &str,
        params: &Self::Params<'_>,
    ) -> impl Future<Output = Result<Vec<Self::Row>, Self::Error>> + Send;
}
