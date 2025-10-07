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

pub struct Column {
    name: &'static str,
    alias: Option<&'static str>,
}

impl Column {
    pub const fn new(name: &'static str, alias: Option<&'static str>) -> Self {
        Self { name, alias }
    }

    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub const fn alias(&self) -> Option<&'static str> {
        self.alias
    }
}

pub struct Relation {
    name: &'static str,
    source_table: &'static str,
    source_columns: &'static [&'static Column],
    target_table: &'static str,
    target_columns: &'static [&'static Column],
}

impl Relation {
    pub const fn new(
        name: &'static str,
        source_table: &'static str,
        source_columns: &'static [&'static Column],
        target_table: &'static str,
        target_columns: &'static [&'static Column],
    ) -> Self {
        Self {
            name,
            source_table,
            source_columns,
            target_table,
            target_columns,
        }
    }

    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn source_table(&self) -> &'static str {
        self.source_table
    }

    pub const fn source_columns(&self) -> &'static [&'static Column] {
        self.source_columns
    }

    pub const fn target_table(&self) -> &'static str {
        self.target_table
    }

    pub const fn target_columns(&self) -> &'static [&'static Column] {
        self.target_columns
    }

    pub fn column_pairs(&self) -> impl Iterator<Item = (&Column, &Column)> {
        self.source_columns
            .iter()
            .zip(self.target_columns)
            .map(|(a, b)| (*a, *b))
    }
}
