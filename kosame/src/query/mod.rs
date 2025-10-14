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
    ) -> impl Future<Output = Result<Vec<<Self as crate::query::Query>::Row>, C::Error>>
    where
        C: crate::Connection,
        <Self as crate::query::Query>::Params: crate::params::Params<C::Params<'c>>,
        for<'b> <Self as crate::query::Query>::Row: From<&'b C::Row>;
}
