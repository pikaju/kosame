use std::sync::atomic::Ordering;

use super::{column::Column, relation::Relation};
use crate::{
    dsl::attribute::ParsedAttributes,
    repr::row::{Row, RowField},
};
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

mod kw {
    syn::custom_keyword!(create);
    syn::custom_keyword!(table);
}

pub struct Table {
    attrs: ParsedAttributes,

    _create: kw::create,
    _table: kw::table,
    _paren: syn::token::Paren,

    name: Ident,
    columns: Punctuated<Column, Token![,]>,

    _semi: Token![;],

    relations: Punctuated<Relation, Token![,]>,
}

impl Parse for Table {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            attrs: input.parse()?,
            _create: input.parse()?,
            _table: input.parse()?,
            name: input.parse()?,
            _paren: syn::parenthesized!(content in input),
            columns: content.parse_terminated(Column::parse, Token![,])?,
            _semi: input.parse()?,
            relations: input.parse_terminated(Relation::parse, Token![,])?,
        })
    }
}

impl ToTokens for Table {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let name_string = name.to_string();

        let columns = self.columns.iter().collect::<Vec<_>>();
        let relations = self
            .relations
            .iter()
            .map(|relation| relation.to_token_stream());

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
            pub mod #name {
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

                pub const NAME: &str = #name_string;
                pub const TABLE: ::kosame::schema::Table = ::kosame::schema::Table::new(
                    #name_string,
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
