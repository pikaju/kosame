#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "tokio-postgres")]
pub mod tokio_postgres;

#[cfg(any(feature = "postgres", feature = "tokio-postgres"))]
#[doc(hidden)]
pub mod postgres_types;

use crate::sql;

pub trait Connection {
    type Dialect: sql::Dialect;
    type Params<'a>;
    type Row;
    type Error: std::error::Error + 'static;

    fn query(
        &mut self,
        sql: &str,
        params: &Self::Params<'_>,
    ) -> impl Future<Output = Result<Vec<Self::Row>, Self::Error>> + Send;
}
