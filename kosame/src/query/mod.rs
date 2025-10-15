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

use crate::{
    Error,
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

    fn execute<'c, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl crate::query::Runner + ?Sized),
    ) -> impl Future<Output = Result<Vec<<Self as crate::query::Query>::Row>, Error<C>>>
    where
        C: crate::Connection,
        <Self as crate::query::Query>::Params: crate::params::Params<C::Params<'c>>,
        for<'b> <Self as crate::query::Query>::Row: From<&'b C::Row>;

    fn execute_one<'c, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl crate::query::Runner + ?Sized),
    ) -> impl Future<Output = Result<<Self as crate::query::Query>::Row, Error<C>>>
    where
        C: crate::Connection,
        <Self as crate::query::Query>::Params: crate::params::Params<C::Params<'c>>,
        for<'b> <Self as crate::query::Query>::Row: From<&'b C::Row>,
    {
        async {
            self.execute_opt(connection, runner)
                .await
                .and_then(|res| res.ok_or(Error::RowCount))
        }
    }

    fn execute_opt<'c, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl crate::query::Runner + ?Sized),
    ) -> impl Future<Output = Result<Option<<Self as crate::query::Query>::Row>, Error<C>>>
    where
        C: crate::Connection,
        <Self as crate::query::Query>::Params: crate::params::Params<C::Params<'c>>,
        for<'b> <Self as crate::query::Query>::Row: From<&'b C::Row>,
    {
        async {
            self.execute(connection, runner).await.and_then(|res| {
                let mut iter = res.into_iter();
                let row = iter.next();
                if iter.next().is_some() {
                    return Err(Error::RowCount);
                }
                Ok(row)
            })
        }
    }
}
