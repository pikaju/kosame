use std::sync::atomic::Ordering;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::Ident;

use crate::repr::row::{Row, RowField};

use super::*;

pub struct Table {
    name: String,
    rust_name: Ident,

    columns: Vec<Column>,
    relations: Vec<Relation>,
}

#[cfg(feature = "dsl")]
impl From<crate::dsl::schema::Table> for Table {
    fn from(value: crate::dsl::schema::Table) -> Self {
        use convert_case::{Case, Casing};

        let rust_name = Ident::new(
            &value.name.to_string().to_case(Case::Snake),
            value.name.span(),
        );

        Self {
            name: value.name.to_string(),
            rust_name,
            columns: value.columns.into_iter().map(Column::from).collect(),
            relations: value.relations.into_iter().map(Relation::from).collect(),
        }
    }
}

impl ToTokens for Table {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let rust_name = &self.rust_name;

        let columns = &self.columns;
        let relations = &self.relations;

        let column_names = self
            .columns
            .iter()
            .map(Column::rust_name)
            .collect::<Vec<_>>();
        let relation_names = self
            .relations
            .iter()
            .map(Relation::name)
            .collect::<Vec<_>>();

        let select_struct = Row::new(
            vec![],
            Ident::new("Select", Span::call_site()),
            self.columns
                .iter()
                .map(|column| {
                    let column = column.rust_name();
                    RowField::new(vec![], column.clone(), quote! { columns::#column::Type })
                })
                .collect(),
        );

        let star_macro = {
            static UNIQUE_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let unique_macro_name = quote::format_ident!(
                "__kosame_star_{}",
                UNIQUE_ID.fetch_add(1, Ordering::Relaxed).to_string()
            );

            let fields = self.columns.iter().map(|column| {
                let column_name = column.rust_name();
                RowField::new(
                    vec![],
                    column_name.clone(),
                    quote! { $($table_path)* ::columns::#column_name::Type },
                )
            });

            quote! {
                #[macro_export]
                macro_rules! #unique_macro_name {
                    (
                        ($($table_path:tt)*)
                        $(#[$meta:meta])* pub struct $name:ident { $($tokens:tt)* }
                    ) => {
                        $(#[$meta])*
                        pub struct $name {
                            #(#fields,)*
                            $($tokens)*
                        }
                    }
                }

                pub use #unique_macro_name as star;
            }
        };

        quote! {
            pub mod #rust_name {
                pub mod columns {
                    #(#columns)*
                }

                pub mod relations {
                    #(#relations)*
                }

                pub mod columns_and_relations {
                    #(pub use super::columns::#column_names;)*
                    #(pub use super::relations::#relation_names;)*
                }

                pub const NAME: &str = #name;
                pub const TABLE: ::kosame::schema::Table = ::kosame::schema::Table::new(
                    #name,
                    &[#(&columns::#column_names::COLUMN),*],
                    &[#(&relations::#relation_names::RELATION),*],
                );

                #select_struct

                #star_macro
            }
        }
        .to_tokens(tokens);
    }
}
