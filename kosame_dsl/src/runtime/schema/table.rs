use super::*;

pub struct Table<'a> {
    name: &'a str,
    columns: &'a [&'a Column<'a>],
    relations: &'a [&'a Relation<'a>],
}

impl<'a> Table<'a> {
    pub const fn new(
        name: &'a str,
        columns: &'a [&'a Column],
        relations: &'a [&'a Relation],
    ) -> Self {
        Self {
            name,
            columns,
            relations,
        }
    }

    #[inline]
    pub const fn name(&self) -> &str {
        self.name
    }

    #[inline]
    pub const fn columns(&self) -> &[&Column<'_>] {
        self.columns
    }

    #[inline]
    pub const fn relations(&self) -> &[&Relation<'_>] {
        self.relations
    }
}
