mod record_array;

pub use record_array::*;

use crate::{dbms::Connection, params::Params};

use super::*;

pub trait QueryRunner {
    #[doc(hidden)]
    fn execute<'a, C, Q>(
        &self,
        connection: &mut C,
        query: &Q,
    ) -> impl Future<Output = Result<Vec<Q::Row>, C::Error>>
    where
        C: Connection,
        Q: Query + ?Sized,
        <Q as Query>::Params: Params<C::Params<'a>>,
        for<'b> <Q as Query>::Row: From<&'b C::Row>;
}
