mod record_array;

pub use record_array::*;

use crate::{Error, driver::Connection, params::Params};

use super::*;

pub trait Runner {
    fn run<'a, C, Q>(
        &self,
        connection: &mut C,
        query: &Q,
    ) -> impl Future<Output = Result<Vec<Q::Row>, Error<C>>>
    where
        C: Connection,
        Q: Query + ?Sized,
        Q::Params: Params<C::Params<'a>>,
        for<'b> Q::Row: From<&'b C::Row>;
}
