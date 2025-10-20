use super::Node;
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
        node: Node,
        alias: Option<&'static str>,
    },
    Expr {
        expr: Expr,
        alias: &'static str,
    },
}
