use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Path};

use crate::{
    clause::{self, FromItem},
    path_ext::PathExt,
};

#[derive(Default, Clone)]
pub struct Scope {
    items: Vec<ScopeItem>,
}

impl Scope {
    pub fn new(from: Option<&clause::From>) -> Self {
        let mut items = vec![];
        if let Some(from) = from {
            fn collect(items: &mut Vec<ScopeItem>, item: &FromItem) {
                match item {
                    FromItem::Table { table, alias } => {
                        if let Some(alias) = alias
                            && let Some(columns) = &alias.columns
                        {
                            items.push(ScopeItem::CustomTable {
                                correlation: alias.name.clone(),
                                columns: columns.columns.iter().cloned().collect(),
                            });
                            items.push(ScopeItem::SpreadColumns(alias.name.clone()));
                        } else {
                            items.push(ScopeItem::Table {
                                table: table.clone(),
                                alias: alias.as_ref().map(|alias| alias.name.clone()),
                            });
                            let name = alias
                                .as_ref()
                                .map(|alias| alias.name.clone())
                                .unwrap_or_else(|| {
                                    table
                                        .segments
                                        .last()
                                        .expect("path cannot be empty")
                                        .ident
                                        .clone()
                                });
                            items.push(ScopeItem::SpreadColumns(name));
                        }
                    }
                    FromItem::Join { left, right, .. } => {
                        collect(items, left);
                        collect(items, right);
                    }
                }
            }
            collect(&mut items, &from.item);
        }
        Self { items }
    }
}

impl ToTokens for Scope {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let items = &self.items;
        quote! {
            mod scope {
                #(#items)*
            }
        }
        .to_tokens(tokens);
    }
}

#[derive(Clone)]
enum ScopeItem {
    Table {
        table: Path,
        alias: Option<Ident>,
    },
    CustomTable {
        correlation: Ident,
        columns: Vec<Ident>,
    },
    SpreadColumns(Ident),
}

impl ToTokens for ScopeItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ScopeItem::Table { table, alias } => {
                if let Some(alias) = alias {
                    let table = table.to_call_site(4);
                    let table_name = alias.to_string();
                    quote! {
                        pub(super) mod #alias {
                            pub const TABLE_NAME: &str = #table_name;
                            pub mod columns {
                                pub use #table::columns::*;
                            }
                        }
                    }
                } else {
                    let table = table.to_call_site(2);
                    quote! { pub(super) use #table; }
                }
            }
            ScopeItem::CustomTable {
                correlation,
                columns,
            } => {
                let table_name = correlation.to_string();
                let column_strings = columns.iter().map(|column| column.to_string());
                quote! {
                    pub(super) mod #correlation {
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
            ScopeItem::SpreadColumns(table) => {
                quote! { pub(super) use #table::columns::*; }
            }
        }
        .to_tokens(tokens);
    }
}
