use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Path};

use crate::{
    clause::{self, FromItem, TableAlias},
    path_ext::PathExt,
};

#[derive(Default, Clone)]
pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    tables: Vec<ScopeTable>,
}

impl<'a> Scope<'a> {
    pub fn new(from: Option<&clause::From>) -> Self {
        let mut tables = vec![];
        if let Some(from) = from {
            fn collect(tables: &mut Vec<ScopeTable>, item: &FromItem) {
                match item {
                    FromItem::Table { table, alias } => match alias {
                        Some(TableAlias {
                            name,
                            columns: Some(columns),
                            ..
                        }) => {
                            tables.push(ScopeTable::Custom {
                                correlation: name.clone(),
                                columns: columns.columns.iter().cloned().collect(),
                            });
                        }
                        Some(TableAlias {
                            name,
                            columns: None,
                            ..
                        }) => {
                            tables.push(ScopeTable::Aliased {
                                table: table.clone(),
                                alias: name.clone(),
                            });
                        }
                        None => {
                            tables.push(ScopeTable::Existing(table.clone()));
                        }
                    },
                    FromItem::Join { left, right, .. } => {
                        collect(tables, left);
                        collect(tables, right);
                    }
                }
            }
            collect(&mut tables, &from.item);
        }
        Self {
            parent: None,
            tables,
        }
    }
}

impl ToTokens for Scope<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tables = &self.tables;
        quote! {
            mod scope {
                pub mod tables {
                    #(#tables)*
                }
                pub mod columns {

                }
            }
        }
        .to_tokens(tokens);
    }
}

#[derive(Clone)]
enum ScopeTable {
    Existing(Path),
    Aliased {
        table: Path,
        alias: Ident,
    },
    Custom {
        correlation: Ident,
        columns: Vec<Ident>,
    },
}

impl ScopeTable {
    fn name(&self) -> &Ident {
        match self {
            Self::Existing(table) => &table.segments.last().expect("paths cannot be empty").ident,
            Self::Aliased { alias, .. } => alias,
            Self::Custom { correlation, .. } => correlation,
        }
    }
}

impl ToTokens for ScopeTable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ScopeTable::Existing(table) => {
                let table = table.to_call_site(3);
                quote! { pub use #table; }
            }
            ScopeTable::Aliased { table, alias } => {
                let table = table.to_call_site(4);
                let table_name = alias.to_string();
                quote! {
                    pub mod #alias {
                        pub const TABLE_NAME: &str = #table_name;
                        pub use #table::columns;
                    }
                }
            }
            ScopeTable::Custom {
                correlation,
                columns,
            } => {
                let table_name = correlation.to_string();
                let column_strings = columns.iter().map(|column| column.to_string());
                quote! {
                    pub mod #correlation {
                        pub const TABLE_NAME: &str = #table_name;
                        pub mod columns {
                            #(
                                pub mod #columns {
                                    pub const COLUMN_NAME: &str = #column_strings;
                                }
                            )*
                        }
                    }
                }
            }
        }
        .to_tokens(tokens);
    }
}
