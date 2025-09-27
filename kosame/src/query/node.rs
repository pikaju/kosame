use crate::{dialect::Dialect, sql_writer::SqlFormatter};
use std::fmt::Write;

use super::*;

pub struct QueryNode {
    table: &'static Table,
    star: bool,
    fields: &'static [QueryField],
    filter: Option<Expr>,
    order_by: Option<OrderBy>,
    limit: Option<Expr>,
    offset: Option<Expr>,
}

impl QueryNode {
    pub const fn new(
        table: &'static Table,
        star: bool,
        fields: &'static [QueryField],
        filter: Option<Expr>,
        order_by: Option<OrderBy>,
        limit: Option<Expr>,
        offset: Option<Expr>,
    ) -> Self {
        Self {
            table,
            star,
            fields,
            filter,
            order_by,
            limit,
            offset,
        }
    }

    pub fn fmt_sql<D: Dialect>(
        &self,
        formatter: &mut SqlFormatter<D>,
        relation: Option<&Relation>,
    ) -> std::fmt::Result {
        formatter.write_str("select ")?;

        if relation.is_some() {
            formatter.write_str("row(")?;
        }

        if self.star {
            formatter.write_str("row(")?;
            for (index, column) in self.table.columns().iter().enumerate() {
                formatter.write_ident(column.name())?;
                if index != self.table.columns().len() - 1 {
                    formatter.write_str(", ")?;
                }
            }
            formatter.write_str(")")?;
            if !self.fields.is_empty() {
                formatter.write_str(", ")?;
            }
        }

        for (index, field) in self.fields.iter().enumerate() {
            match field {
                QueryField::Column { column, .. } => {
                    formatter.write_ident(column.name())?;
                }
                QueryField::Relation { node, relation, .. } => {
                    formatter.write_str("array(")?;
                    node.fmt_sql::<D>(formatter, Some(relation))?;
                    formatter.write_str(")")?;
                }
                QueryField::Expr { expr, .. } => {
                    expr.fmt_sql(formatter)?;
                }
            }
            if index != self.fields.len() - 1 {
                formatter.write_str(", ")?;
            }
        }

        if relation.is_some() {
            formatter.write_str(")")?;
        }

        formatter.write_str(" from ")?;
        formatter.write_ident(self.table.name())?;

        if relation.is_some() || self.filter.is_some() {
            formatter.write_str(" where ")?;
        }

        if relation.is_some() && self.filter.is_some() {
            formatter.write_str("(")?;
        }

        if let Some(relation) = relation {
            for (index, (source_column, target_column)) in relation.column_pairs().enumerate() {
                formatter.write_ident(relation.source_table())?;
                formatter.write_str(".")?;
                formatter.write_ident(source_column.name())?;
                formatter.write_str(" = ")?;
                formatter.write_ident(relation.target_table())?;
                formatter.write_str(".")?;
                formatter.write_ident(target_column.name())?;
                if index != relation.source_columns().len() - 1 {
                    formatter.write_str(" and ")?;
                }
            }
        }

        if relation.is_some() && self.filter.is_some() {
            formatter.write_str(") and (")?;
        }

        if let Some(filter) = &self.filter {
            filter.fmt_sql(formatter)?;
        }

        if relation.is_some() && self.filter.is_some() {
            formatter.write_str(")")?;
        }

        if let Some(order_by) = &self.order_by {
            order_by.fmt_sql(formatter)?;
        }

        if let Some(limit) = &self.limit {
            formatter.write_str(" limit ")?;
            limit.fmt_sql(formatter)?;
        }

        if let Some(offset) = &self.offset {
            formatter.write_str(" offset ")?;
            offset.fmt_sql(formatter)?;
        }

        Ok(())
    }
}
