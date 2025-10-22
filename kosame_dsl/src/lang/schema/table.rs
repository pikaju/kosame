use std::{
    hash::{Hash, Hasher},
    sync::atomic::Ordering,
};

use crate::{
    lang::attribute::{CustomMeta, MetaLocation},
    lang::row::{Row, RowField},
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
    pub inner_attrs: Vec<Attribute>,
    pub outer_attrs: Vec<Attribute>,

    pub create: kw::create,
    pub table: kw::table,
    pub paren: syn::token::Paren,

    pub name: Ident,
    pub columns: Punctuated<Column, Token![,]>,

    pub semi: Token![;],

    pub relations: Punctuated<Relation, Token![,]>,
}

impl Parse for Table {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            inner_attrs: {
                let attrs = Attribute::parse_inner(input)?;
                CustomMeta::parse_attrs(&attrs, MetaLocation::TableInner)?;
                attrs
            },
            outer_attrs: {
                let attrs = Attribute::parse_outer(input)?;
                CustomMeta::parse_attrs(&attrs, MetaLocation::TableOuter)?;
                attrs
            },
            create: input.parse()?,
            table: input.parse()?,
            name: input.parse()?,
            paren: syn::parenthesized!(content in input),
            columns: content.parse_terminated(Column::parse, Token![,])?,
            semi: input.parse()?,
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

        let star_macro = {
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
            let unique_macro_name = format_ident!("__kosame_star_{}", hash);

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
                pub const TABLE: ::kosame::repr::schema::Table<'_> = ::kosame::repr::schema::Table::new(
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
