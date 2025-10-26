use std::collections::HashSet;

use syn::Path;

use crate::visitor::Visitor;

pub struct TableRefs<'a> {
    table_refs: HashSet<&'a Path>,
}

impl<'a> TableRefs<'a> {
    pub fn new() -> Self {
        Self {
            table_refs: HashSet::new(),
        }
    }

    pub fn build(self) -> HashSet<&'a Path> {
        self.table_refs
    }
}

impl<'a> Visitor<'a> for TableRefs<'a> {
    fn visit_table_ref(&mut self, table_ref: &'a Path) {
        self.table_refs.insert(table_ref);
    }
}
