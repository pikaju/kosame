use crate::Connection;

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error<C: Connection> {
    #[error("unexpected number of rows returned from query")]
    RowCount,
    #[error("{0}")]
    Connection(C::Error),
}
