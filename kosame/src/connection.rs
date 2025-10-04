pub trait Connection {
    type Params<'a>;
    type Row;
    type Error;

    async fn query(
        &mut self,
        sql: &str,
        params: &Self::Params<'_>,
    ) -> Result<Vec<Self::Row>, Self::Error>;
}
