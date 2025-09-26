use crate::{
    expr::Expr,
    schema::{Column, Relation, Table},
};

pub trait Query {
    const ROOT: QueryNode;
}

pub struct QueryNode {
    table: &'static Table,
    star: bool,
    fields: &'static [QueryField],
    limit: Option<Expr>,
}

impl QueryNode {
    pub const fn new(
        table: &'static Table,
        star: bool,
        fields: &'static [QueryField],
        limit: Option<Expr>,
    ) -> Self {
        Self {
            table,
            star,
            fields,
            limit,
        }
    }

    pub fn to_sql_string(&self, relation: Option<&Relation>) -> String {
        let mut result = "select ".to_string();

        if relation.is_some() {
            result += "row(";
        }

        if self.star {
            result += "row(";
            for (index, column) in self.table.columns().iter().enumerate() {
                result += column.name();
                if index != self.table.columns().len() - 1 {
                    result += ", ";
                }
            }
            result += ")";
            if !self.fields.is_empty() {
                result += ", ";
            }
        }

        for (index, field) in self.fields.iter().enumerate() {
            match field {
                QueryField::Column { column, .. } => {
                    result += column.name();
                }
                QueryField::Relation { node, relation, .. } => {
                    result += "array(";
                    result += &node.to_sql_string(Some(relation));
                    result += ")";
                }
                QueryField::Expr { expr, .. } => {
                    expr.to_sql_string(&mut result);
                }
            }
            if index != self.fields.len() - 1 {
                result += ", ";
            }
        }

        if relation.is_some() {
            result += ")";
        }

        result += " from ";
        result += self.table.name();

        if let Some(relation) = relation {
            result += " where ";
            for (index, (source_column, target_column)) in relation.column_pairs().enumerate() {
                result += relation.source_table();
                result += ".";
                result += source_column.name();
                result += " = ";
                result += relation.target_table();
                result += ".";
                result += target_column.name();
                if index != relation.source_columns().len() - 1 {
                    result += " and ";
                }
            }
        }

        if let Some(limit) = &self.limit {
            result += " limit ";
            limit.to_sql_string(&mut result);
        }

        result
    }
}

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
