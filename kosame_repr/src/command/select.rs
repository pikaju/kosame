use crate::{clause::*, command::Fields};

pub struct Select<'a> {
    fields: Fields<'a>,
    r#where: Option<Where<'a>>,
    group_by: Option<GroupBy<'a>>,
    having: Option<Having<'a>>,
    order_by: Option<OrderBy<'a>>,
    limit: Option<Limit<'a>>,
    offset: Option<Offset<'a>>,
}

impl<'a> Select<'a> {
    #[inline]
    pub const fn fields(&self) -> &Fields<'_> {
        &self.fields
    }

    #[inline]
    pub const fn r#where(&self) -> Option<&Where<'_>> {
        self.r#where.as_ref()
    }

    #[inline]
    pub const fn group_by(&self) -> Option<&GroupBy<'_>> {
        self.group_by.as_ref()
    }

    #[inline]
    pub const fn having(&self) -> Option<&Having<'_>> {
        self.having.as_ref()
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
