use super::*;

pub struct Table {
    name: &'static str,
    columns: &'static [&'static Column],
    relations: &'static [&'static Relation],
}

impl Table {
    pub const fn new(
        name: &'static str,
        columns: &'static [&'static Column],
        relations: &'static [&'static Relation],
    ) -> Self {
        Self {
            name,
            columns,
            relations,
        }
    }

    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub const fn columns(&self) -> &'static [&'static Column] {
        self.columns
    }

    #[inline]
    pub const fn relations(&self) -> &'static [&'static Relation] {
        self.relations
    }
}
