mod bind_param;
mod field;
mod node;
mod order_by;
mod runner;

pub use bind_param::*;
pub use field::*;
pub use node::*;
pub use order_by::*;
pub use runner::*;

use pollster::FutureExt;

use crate::{
    Error,
    driver::Connection,
    expr::Expr,
    schema::{Column, Relation, Table},
};

pub trait Query {
    type Params: std::fmt::Debug;
    type Row;

    const ROOT: QueryNode;

    fn root(&self) -> &'static QueryNode {
        &Self::ROOT
    }

    fn params(&self) -> &Self::Params;

    fn exec<'c, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl crate::query::Runner + ?Sized),
    ) -> impl Future<Output = Result<Vec<<Self as crate::query::Query>::Row>, Error<C>>>
    where
        C: Connection,
        Self::Params: crate::params::Params<C::Params<'c>>,
        for<'b> Self::Row: From<&'b C::Row>,
    {
        async { runner.run(connection, self).await }
    }

    fn exec_one<'c, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl crate::query::Runner + ?Sized),
    ) -> impl Future<Output = Result<<Self as crate::query::Query>::Row, Error<C>>>
    where
        C: Connection,
        Self::Params: crate::params::Params<C::Params<'c>>,
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
        runner: &mut (impl crate::query::Runner + ?Sized),
    ) -> impl Future<Output = Result<Option<<Self as crate::query::Query>::Row>, Error<C>>>
    where
        C: Connection,
        Self::Params: crate::params::Params<C::Params<'c>>,
        for<'b> Self::Row: From<&'b C::Row>,
    {
        async {
            self.exec(connection, runner).await.and_then(|res| {
                let mut iter = res.into_iter();
                let row = iter.next();
                if iter.next().is_some() {
                    return Err(Error::RowCount);
                }
                Ok(row)
            })
        }
    }

    fn exec_sync<'c, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl crate::query::Runner + ?Sized),
    ) -> Result<Vec<<Self as crate::query::Query>::Row>, Error<C>>
    where
        C: Connection,
        Self::Params: crate::params::Params<C::Params<'c>>,
        for<'b> Self::Row: From<&'b C::Row>,
    {
        self.exec(connection, runner).block_on()
    }

    fn exec_one_sync<'c, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl crate::query::Runner + ?Sized),
    ) -> Result<<Self as crate::query::Query>::Row, Error<C>>
    where
        C: Connection,
        Self::Params: crate::params::Params<C::Params<'c>>,
        for<'b> Self::Row: From<&'b C::Row>,
    {
        self.exec_one(connection, runner).block_on()
    }

    fn exec_opt_sync<'c, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl crate::query::Runner + ?Sized),
    ) -> Result<Option<<Self as crate::query::Query>::Row>, Error<C>>
    where
        C: Connection,
        Self::Params: crate::params::Params<C::Params<'c>>,
        for<'b> Self::Row: From<&'b C::Row>,
    {
        self.exec_opt(connection, runner).block_on()
    }
}
