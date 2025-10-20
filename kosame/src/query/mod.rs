mod runner;

pub use kosame_dsl::runtime::query::*;
pub use runner::*;

use crate::{Error, driver::Connection, params::Params};
use pollster::FutureExt;

pub trait Query {
    type Params: std::fmt::Debug;
    type Row;

    const ROOT: Node;

    fn root(&self) -> &'static Node {
        &Self::ROOT
    }

    fn params(&self) -> &Self::Params;

    fn exec<'c, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl Runner + ?Sized),
    ) -> impl Future<Output = Result<Vec<Self::Row>, Error<C>>>
    where
        C: Connection,
        Self::Params: Params<C::Params<'c>>,
        for<'b> Self::Row: From<&'b C::Row>,
    {
        async { runner.run(connection, self).await }
    }

    fn exec_one<'c, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl Runner + ?Sized),
    ) -> impl Future<Output = Result<Self::Row, Error<C>>>
    where
        C: Connection,
        Self::Params: Params<C::Params<'c>>,
        for<'b> Self::Row: From<&'b C::Row>,
    {
        async {
            self.exec_opt(connection, runner)
                .await
                .and_then(|res| res.ok_or(Error::RowCount))
        }
    }

    fn exec_opt<'c, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl Runner + ?Sized),
    ) -> impl Future<Output = Result<Option<Self::Row>, Error<C>>>
    where
        C: Connection,
        Self::Params: Params<C::Params<'c>>,
        for<'b> Self::Row: From<&'b C::Row>,
    {
        async {
            self.exec(connection, runner).await.and_then(|res| {
                let mut iter = res.into_iter();
                let row = iter.next();
                if row.is_some() && iter.next().is_some() {
                    return Err(Error::RowCount);
                }
                Ok(row)
            })
        }
    }

    fn exec_sync<'c, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl Runner + ?Sized),
    ) -> Result<Vec<Self::Row>, Error<C>>
    where
        C: Connection,
        Self::Params: Params<C::Params<'c>>,
        for<'b> Self::Row: From<&'b C::Row>,
    {
        self.exec(connection, runner).block_on()
    }

    fn exec_one_sync<'c, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl Runner + ?Sized),
    ) -> Result<Self::Row, Error<C>>
    where
        C: Connection,
        Self::Params: Params<C::Params<'c>>,
        for<'b> Self::Row: From<&'b C::Row>,
    {
        self.exec_one(connection, runner).block_on()
    }

    fn exec_opt_sync<'c, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl Runner + ?Sized),
    ) -> Result<Option<Self::Row>, Error<C>>
    where
        C: Connection,
        Self::Params: Params<C::Params<'c>>,
        for<'b> Self::Row: From<&'b C::Row>,
    {
        self.exec_opt(connection, runner).block_on()
    }
}
