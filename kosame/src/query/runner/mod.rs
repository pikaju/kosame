mod record_array;

pub use record_array::*;

use crate::connection::Connection;

use super::*;

pub trait QueryRunner {
    #[doc(hidden)]
    async fn execute<C, Q>(&self, connection: &mut C, query: &Q) -> Result<Vec<Q::Row>, C::Error>
    where
        C: Connection,
        Q: Query + ?Sized,
        for<'a> <Q as Query>::Row: From<&'a C::Row>;
}
