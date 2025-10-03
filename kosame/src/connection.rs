pub trait Connection {
    type Row;
    type Error;

    async fn query(&mut self, sql: &str) -> Result<Vec<Self::Row>, Self::Error>;
}
