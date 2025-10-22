#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected number of rows in result set")]
    RowCount,
    #[error("SQL formatting failed")]
    FmtSql(
        #[from]
        #[source]
        kosame_sql::Error,
    ),
    #[error("driver error: {0}")]
    Driver(
        #[from]
        #[source]
        Box<dyn std::error::Error>,
    ),
}

pub type Result<T> = std::result::Result<T, Error>;
