use std::fmt::Display;

use super::{column::Column, relation::Relation};
use crate::{
    docs::{Docs, ToDocsTokens},
    row_struct::{RowStruct, RowStructField},
};
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct Table {
    _create_table: CreateTable,
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
            _create_table: input.parse()?,
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

        let columns = self.columns.iter();
        let relations = self
            .relations
            .iter()
            .map(|relation| relation.to_token_stream(&self.name));

        let column_names = self.columns.iter().map(Column::name);
        let column_names2 = column_names.clone();
        let relation_names = self.relations.iter().map(Relation::name);
        let relation_names2 = relation_names.clone();

        let select_struct = RowStruct::new(
            vec![],
            Ident::new("Select", Span::call_site()),
            self.columns
                .iter()
                .map(|column| {
                    RowStructField::new(vec![], column.name().clone(), column.data_type_auto())
                })
                .collect(),
        );
        let insert_struct = RowStruct::new(
            vec![],
            Ident::new("Insert", Span::call_site()),
            self.columns
                .iter()
                .map(|column| {
                    RowStructField::new(
                        vec![],
                        column.name().clone(),
                        column.data_type_not_null().to_token_stream(),
                    )
                })
                .collect(),
        );
        let update_struct = RowStruct::new(
            vec![],
            Ident::new("Update", Span::call_site()),
            self.columns
                .iter()
                .map(|column| {
                    RowStructField::new(vec![], column.name().clone(), column.data_type_nullable())
                })
                .collect(),
        );

        let docs = self.to_docs_token_stream();

        quote! {
            // #docs
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

                pub const TABLE: ::kosame::schema::Table = ::kosame::schema::Table::new(
                    #name_string,
                    &[#(&columns::#column_names2::COLUMN),*],
                    &[#(&relations::#relation_names2::RELATION),*],
                );

                #select_struct
                #insert_struct
                #update_struct
            }
        }
        .to_tokens(tokens);
    }
}

impl Docs for Table {
    fn docs(&self) -> String {
        let name = &self.name;
        format!(
            "## {name} (Kosame Table)

```sql
{self}
```"
        )
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("create table ")?;
        Display::fmt(&self.name, f)?;
        f.write_str(" (\n")?;
        for column in &self.columns {
            f.write_str("    ")?;
            column.fmt(f)?;
            f.write_str(",\n")?;
        }
        f.write_str(");\n")?;
        f.write_str("\n\n")?;
        for relation in &self.relations {
            Display::fmt(relation, f)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

mod kw {
    syn::custom_keyword!(create);
    syn::custom_keyword!(table);
}

pub struct CreateTable {
    _create: kw::create,
    _table: kw::table,
}

impl Parse for CreateTable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _create: input.parse()?,
            _table: input.parse()?,
        })
    }
}
