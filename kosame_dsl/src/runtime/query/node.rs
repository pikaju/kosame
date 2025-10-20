use crate::runtime::{expr::Expr, schema::Table};

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

    pub const fn table(&self) -> &'static Table {
        self.table
    }

    pub const fn star(&self) -> bool {
        self.star
    }

    pub const fn fields(&self) -> &'static [QueryField] {
        self.fields
    }

    pub const fn filter(&self) -> Option<&Expr> {
        self.filter.as_ref()
    }

    pub const fn order_by(&self) -> Option<&OrderBy> {
        self.order_by.as_ref()
    }

    pub const fn limit(&self) -> Option<&Expr> {
        self.limit.as_ref()
    }

    pub const fn offset(&self) -> Option<&Expr> {
        self.offset.as_ref()
    }
}
