use std::{
    hash::{Hash, Hasher},
    sync::atomic::Ordering,
};

use crate::{
    attribute::{CustomMeta, MetaLocation},
    row::{Row, RowField},
};

use super::{column::Column, relation::Relation};
use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{
    Attribute, Ident, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

mod kw {
    syn::custom_keyword!(create);
    syn::custom_keyword!(table);
}

pub struct Table {
    _token_stream: TokenStream,

    pub _inner_attrs: Vec<Attribute>,
    pub _outer_attrs: Vec<Attribute>,

    pub _create: kw::create,
    pub _table: kw::table,
    pub name: Ident,

    pub _paren: syn::token::Paren,

    pub columns: Punctuated<Column, Token![,]>,

    pub _semi: Token![;],

    pub relations: Punctuated<Relation, Token![,]>,
}

impl Parse for Table {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _token_stream: input.fork().parse()?,
            _inner_attrs: {
                let attrs = Attribute::parse_inner(input)?;
                CustomMeta::parse_attrs(&attrs, MetaLocation::TableInner)?;
                attrs
            },
            _outer_attrs: {
                let attrs = Attribute::parse_outer(input)?;
                CustomMeta::parse_attrs(&attrs, MetaLocation::TableOuter)?;
                attrs
            },
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name.to_string();
        let rust_name = Ident::new(
            &self.name.to_string().to_case(Case::Snake),
            self.name.span(),
        );

        let columns = self.columns.iter();
        let relations = self.relations.iter();

        let column_names = self
            .columns
            .iter()
            .map(Column::rust_name)
            .collect::<Vec<_>>();
        let relation_names = self
            .relations
            .iter()
            .map(|relation| &relation.name)
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

        let inject_macro = {
            static AUTO_INCREMENT: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let increment = AUTO_INCREMENT.fetch_add(1, Ordering::Relaxed);
            let file = self.name.span().file();
            let line_column = self.name.span().start();
            let hash = {
                let mut hasher = std::hash::DefaultHasher::new();
                file.hash(&mut hasher);
                line_column.line.hash(&mut hasher);
                line_column.column.hash(&mut hasher);
                increment.hash(&mut hasher);
                hasher.finish()
            };
            let unique_macro_name = format_ident!("__kosame_inject_{}", hash);

            let token_stream = &self._token_stream;

            quote! {
                #[macro_export]
                macro_rules! #unique_macro_name {
                    (
                        $(#![$acc:meta])*
                        ($($child:tt)*) {
                            $($content:tt)*
                        }
                        ($($table_path:tt)*)
                    ) => {
                        $($child)* {
                            #![kosame(__table($($table_path)* = #token_stream))]
                            $(#![$acc])*

                            $($content)*
                        }
                    }
                }

                pub use #unique_macro_name as inject;
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
                    pub use super::columns::*;
                    pub use super::relations::*;
                }

                pub const TABLE_NAME: &str = #name;
                pub const TABLE: ::kosame::repr::schema::Table<'_> = ::kosame::repr::schema::Table::new(
                    #name,
                    &[#(&columns::#column_names::COLUMN),*],
                    &[#(&relations::#relation_names::RELATION),*],
                );

                #select_struct

                #inject_macro
            }
        }
        .to_tokens(tokens);
    }
}
