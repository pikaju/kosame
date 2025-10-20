use crate::runtime::{expr::Expr, schema::Table};

use super::*;

pub struct Node {
    table: &'static Table,
    star: bool,
    fields: &'static [Field],
    filter: Option<Expr>,
    order_by: Option<OrderBy>,
    limit: Option<Expr>,
    offset: Option<Expr>,
}

impl Node {
    pub const fn new(
        table: &'static Table,
        star: bool,
        fields: &'static [Field],
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

    pub const fn fields(&self) -> &'static [Field] {
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
