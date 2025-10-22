use super::Node;
use crate::{
    expr::Expr,
    schema::{Column, Relation},
};

pub enum Field<'a> {
    Column {
        column: &'a Column<'a>,
        alias: Option<&'a str>,
    },
    Relation {
        relation: &'a Relation<'a>,
        node: Node<'a>,
        alias: Option<&'a str>,
    },
    Expr {
        expr: Expr<'a>,
        alias: &'a str,
    },
}
