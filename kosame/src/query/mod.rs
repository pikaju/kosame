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
    connection::Connection,
    expr::Expr,
    params::Params,
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

    async fn execute<'a, C>(
        &self,
        connection: &mut C,
        runner: &mut (impl QueryRunner + ?Sized),
    ) -> Result<Vec<Self::Row>, C::Error>
    where
        C: Connection,
        Self::Params: Params<C::Params<'a>>,
        for<'b> Self::Row: From<&'b C::Row>,
    {
        runner.execute(connection, self).await
    }
}
