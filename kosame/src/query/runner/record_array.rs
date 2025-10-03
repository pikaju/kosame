use std::fmt::Write;

use crate::{
    connection::Connection,
    dialect::{Dialect, Postgres},
    sql_writer::SqlFormatter,
};

use super::*;

pub struct RecordArrayRunner {}

impl QueryRunner for RecordArrayRunner {
    async fn execute<C, Q>(&self, connection: &mut C, query: &Q) -> Result<Vec<Q::Row>, C::Error>
    where
        C: Connection,
        Q: Query + ?Sized,
        for<'a> <Q as Query>::Row: From<&'a C::Row>,
    {
        let mut sql = String::new();
        let mut formatter = SqlFormatter::<Postgres>::new(&mut sql);
        fmt_node_sql(&mut formatter, query.root(), None)
            .expect("SQL string formatting should never fail");
        println!("==== Query ====");
        println!("{}", sql);
        println!("{:?}", query.params());

        let rows = connection.query(&sql).await?;
        Ok(rows.iter().map(<Q as Query>::Row::from).collect())
    }
}

fn fmt_node_sql<D: Dialect>(
    formatter: &mut SqlFormatter<D>,
    node: &QueryNode,
    relation: Option<&Relation>,
) -> std::fmt::Result {
    formatter.write_str("select ")?;

    if relation.is_some() {
        formatter.write_str("row(")?;
    }

    if node.star() {
        formatter.write_str("row(")?;
        for (index, column) in node.table().columns().iter().enumerate() {
            formatter.write_ident(column.name())?;
            if index != node.table().columns().len() - 1 {
                formatter.write_str(", ")?;
            }
        }
        formatter.write_str(")")?;
        if !node.fields().is_empty() {
            formatter.write_str(", ")?;
        }
    }

    for (index, field) in node.fields().iter().enumerate() {
        match field {
            QueryField::Column { column, .. } => {
                formatter.write_ident(column.name())?;
            }
            QueryField::Relation { node, relation, .. } => {
                formatter.write_str("array(")?;
                fmt_node_sql::<D>(formatter, node, Some(relation))?;
                formatter.write_str(")")?;
            }
            QueryField::Expr { expr, .. } => {
                expr.fmt_sql(formatter)?;
            }
        }
        if index != node.fields().len() - 1 {
            formatter.write_str(", ")?;
        }
    }

    if relation.is_some() {
        formatter.write_str(")")?;
    }

    formatter.write_str(" from ")?;
    formatter.write_ident(node.table().name())?;

    if relation.is_some() || node.filter().is_some() {
        formatter.write_str(" where ")?;
    }

    if relation.is_some() && node.filter().is_some() {
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

    if relation.is_some() && node.filter().is_some() {
        formatter.write_str(") and (")?;
    }

    if let Some(filter) = &node.filter() {
        filter.fmt_sql(formatter)?;
    }

    if relation.is_some() && node.filter().is_some() {
        formatter.write_str(")")?;
    }

    if let Some(order_by) = &node.order_by() {
        order_by.fmt_sql(formatter)?;
    }

    if let Some(limit) = &node.limit() {
        formatter.write_str(" limit ")?;
        limit.fmt_sql(formatter)?;
    }

    if let Some(offset) = &node.offset() {
        formatter.write_str(" offset ")?;
        offset.fmt_sql(formatter)?;
    }

    Ok(())
}
