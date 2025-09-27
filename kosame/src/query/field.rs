use super::*;

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
