use crate::{clause::*, schema::Table};

use super::*;

pub struct Node<'a> {
    table: &'a Table<'a>,
    star: bool,
    fields: &'a [Field<'a>],
    r#where: Option<Where<'a>>,
    order_by: Option<OrderBy<'a>>,
    limit: Option<Limit<'a>>,
    offset: Option<Offset<'a>>,
}

impl<'a> Node<'a> {
    pub const fn new(
        table: &'a Table<'a>,
        star: bool,
        fields: &'a [Field<'a>],
        r#where: Option<Where<'a>>,
        order_by: Option<OrderBy<'a>>,
        limit: Option<Limit<'a>>,
        offset: Option<Offset<'a>>,
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

    #[inline]
    pub const fn table(&self) -> &Table<'_> {
        self.table
    }

    #[inline]
    pub const fn star(&self) -> bool {
        self.star
    }

    #[inline]
    pub const fn fields(&self) -> &[Field<'a>] {
        self.fields
    }

    #[inline]
    pub const fn r#where(&self) -> Option<&Where<'_>> {
        self.r#where.as_ref()
    }

    #[inline]
    pub const fn order_by(&self) -> Option<&OrderBy<'_>> {
        self.order_by.as_ref()
    }

    #[inline]
    pub const fn limit(&self) -> Option<&Limit<'_>> {
        self.limit.as_ref()
    }

    #[inline]
    pub const fn offset(&self) -> Option<&Offset<'_>> {
        self.offset.as_ref()
    }
}
