use tokio_postgres::Client;

use crate::connection::Connection;

#[doc(hidden)]
pub mod internal;

impl Connection for Client {
    type Row = tokio_postgres::Row;
    type Error = tokio_postgres::Error;

    async fn query(&mut self, sql: &str) -> Result<Vec<Self::Row>, Self::Error> {
        Client::query(self, sql, &[]).await
    }
}
