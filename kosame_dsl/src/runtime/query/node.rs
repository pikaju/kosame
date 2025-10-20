use crate::runtime::{clause::*, schema::Table};

use super::*;

pub struct Node {
    table: &'static Table,
    star: bool,
    fields: &'static [Field],
    r#where: Option<Where>,
    order_by: Option<OrderBy>,
    limit: Option<Limit>,
    offset: Option<Offset>,
}

impl Node {
    pub const fn new(
        table: &'static Table,
        star: bool,
        fields: &'static [Field],
        r#where: Option<Where>,
        order_by: Option<OrderBy>,
        limit: Option<Limit>,
        offset: Option<Offset>,
    ) -> Self {
        Self {
            table,
            star,
            fields,
            r#where,
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

    pub const fn r#where(&self) -> Option<&Where> {
        self.r#where.as_ref()
    }

    pub const fn order_by(&self) -> Option<&OrderBy> {
        self.order_by.as_ref()
    }

    pub const fn limit(&self) -> Option<&Limit> {
        self.limit.as_ref()
    }

    pub const fn offset(&self) -> Option<&Offset> {
        self.offset.as_ref()
    }
}
