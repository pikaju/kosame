use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Path;

use crate::{
    clause::{self, FromItem},
    path_ext::PathExt,
};

#[derive(Default, Clone)]
pub struct Scope {
    tables: Vec<Path>,
}

impl Scope {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn shadow(&mut self, scope: &Scope) {
        for table in &scope.tables {
            if !self.tables.iter().any(|existing_table| {
                existing_table
                    .segments
                    .last()
                    .expect("paths cannot be empty")
                    .ident
                    == table.segments.last().expect("paths cannot be empty").ident
            }) {
                self.tables.push(table.clone());
            }
        }
    }

    pub fn shadowed(&self, scope: &Scope) -> Self {
        let mut result = self.clone();
        result.shadow(scope);
        result
    }
}

impl From<&clause::From> for Scope {
    fn from(value: &clause::From) -> Self {
        let mut tables = vec![];
        fn collect(tables: &mut Vec<Path>, item: &FromItem) {
            match item {
                FromItem::Table { table, .. } => {
                    tables.push(table.clone());
                }
                FromItem::Join { left, right, .. } => {
                    collect(tables, left);
                    collect(tables, right);
                }
            }
        }
        collect(&mut tables, &value.item);
        Self { tables }
    }
}

impl ToTokens for Scope {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let items = self
            .tables
            .iter()
            .map(|path| path.to_call_site(2))
            .collect::<Vec<_>>();
        quote! {
            mod scope {
                #(pub(super) use #items;)*
                #(pub(super) use #items::columns::*;)*
            }
        }
        .to_tokens(tokens);
    }
}
