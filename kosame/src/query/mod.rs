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
}
