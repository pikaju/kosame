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
    pub fn name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub fn columns(&self) -> &'static [&'static Column] {
        self.columns
    }

    #[inline]
    pub fn relations(&self) -> &'static [&'static Relation] {
        self.relations
    }
}

pub struct Column {
    name: &'static str,
}

impl Column {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }
}

pub struct Relation {
    name: &'static str,
    join_condition: &'static str,
}

impl Relation {
    pub const fn new(name: &'static str, join_condition: &'static str) -> Self {
        Self {
            name,
            join_condition,
        }
    }

    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub fn join_condition(&self) -> &'static str {
        self.join_condition
    }
}
