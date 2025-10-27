#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "tokio-postgres")]
pub mod tokio_postgres;

#[cfg(any(feature = "postgres", feature = "tokio-postgres"))]
#[doc(hidden)]
pub mod postgres_types;

pub trait Connection {
    type Dialect: kosame_sql::Dialect;
    type Params<'a>;
    type Row;
    type Error: std::error::Error + 'static;

    fn exec(
        &mut self,
        sql: &str,
        params: &Self::Params<'_>,
    ) -> impl Future<Output = Result<u64, Self::Error>> + Send;

    fn query(
        &mut self,
        sql: &str,
        params: &Self::Params<'_>,
    ) -> impl Future<Output = Result<Vec<Self::Row>, Self::Error>> + Send;
}
