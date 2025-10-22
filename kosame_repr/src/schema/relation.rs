use super::*;

pub struct Relation<'a> {
    name: &'a str,
    source_table: &'a str,
    source_columns: &'a [&'a Column<'a>],
    target_table: &'a str,
    target_columns: &'a [&'a Column<'a>],
}

impl<'a> Relation<'a> {
    pub const fn new(
        name: &'a str,
        source_table: &'a str,
        source_columns: &'a [&'a Column],
        target_table: &'a str,
        target_columns: &'a [&'a Column],
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
    pub const fn name(&self) -> &str {
        self.name
    }

    #[inline]
    pub const fn source_table(&self) -> &str {
        self.source_table
    }

    #[inline]
    pub const fn source_columns(&self) -> &[&Column<'_>] {
        self.source_columns
    }

    #[inline]
    pub const fn target_table(&self) -> &str {
        self.target_table
    }

    #[inline]
    pub const fn target_columns(&self) -> &[&Column<'_>] {
        self.target_columns
    }

    #[inline]
    pub fn column_pairs(&self) -> impl Iterator<Item = (&Column<'_>, &Column<'_>)> {
        self.source_columns
            .iter()
            .zip(self.target_columns)
            .map(|(a, b)| (*a, *b))
    }
}
