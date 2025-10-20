use super::QueryNode;
use crate::runtime::{
    expr::Expr,
    schema::{Column, Relation},
};

pub enum QueryField {
    Column {
        column: &'static Column,
        alias: Option<&'static str>,
    },
    Relation {
        relation: &'static Relation,
        node: QueryNode,
        alias: Option<&'static str>,
    },
    Expr {
        expr: Expr,
        alias: &'static str,
    },
}
