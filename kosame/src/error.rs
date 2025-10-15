use crate::Connection;

#[derive(Clone)]
pub enum Error<C: Connection> {
    RowCount,
    Connection(C::Error),
}

impl<C> std::fmt::Debug for Error<C>
where
    C: Connection,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RowCount => write!(f, "RowCount"),
            Error::Connection(err) => write!(f, "Connection({:?})", err),
        }
    }
}

impl<C> std::fmt::Display for Error<C>
where
    C: Connection,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::RowCount => {
                write!(f, "unexpected number of rows in result set")
            }
            Error::Connection(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

impl<C> std::error::Error for Error<C> where C: Connection {}
