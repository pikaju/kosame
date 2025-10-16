mod record_array;

pub use record_array::*;

use crate::{Error, driver::Connection, params::Params};

use super::*;

pub trait Runner {
    #[doc(hidden)]
    fn run<'a, C, Q>(
        &self,
        connection: &mut C,
        query: &Q,
    ) -> impl Future<Output = Result<Vec<Q::Row>, Error<C>>>
    where
        C: Connection,
        Q: Query + ?Sized,
        <Q as Query>::Params: Params<C::Params<'a>>,
        for<'b> <Q as Query>::Row: From<&'b C::Row>;
}
