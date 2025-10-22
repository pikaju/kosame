#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected number of rows in result set")]
    RowCount,
    #[error("{0}")]
    Connection(
        #[from]
        #[source]
        Box<dyn std::error::Error>,
    ),
}

pub type Result<T> = std::result::Result<T, Error>;
