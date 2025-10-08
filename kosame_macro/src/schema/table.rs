use super::{column::Column, field_spec::FieldSpec, relation::Relation};
use crate::{
    row_struct::{RowStruct, RowStructField},
    schema::column_override::{ColumnOverride, ColumnWithOverride},
};
use proc_macro_error::emit_error;
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
    _create: kw::create,
    _table: kw::table,
    _paren: syn::token::Paren,

    name: Ident,
    columns: Punctuated<Column, Token![,]>,

    _semi: Token![;],

    field_specs: Punctuated<FieldSpec, Token![,]>,
}

impl Table {
    fn columns(&self) -> impl Iterator<Item = ColumnWithOverride<'_>> {
        self.columns.iter().map(|column| {
            ColumnWithOverride::new(
                column,
                self.field_specs
                    .iter()
                    .find_map(|field_spec| match field_spec {
                        FieldSpec::ColumnOverride(column_override)
                            if column_override.name() == column.name() =>
                        {
                            Some(column_override)
                        }
                        _ => None,
                    }),
            )
        })
    }

    fn relations(&self) -> impl Iterator<Item = &Relation> {
        self.field_specs
            .iter()
            .filter_map(|field_spec| match field_spec {
                FieldSpec::Relation(relation) => Some(relation),
                _ => None,
            })
    }

    fn unmatched_column_overrides(&self) -> impl Iterator<Item = &ColumnOverride> {
        self.field_specs
            .iter()
            .filter_map(|field_spec| match field_spec {
                FieldSpec::ColumnOverride(column_override) => (!self
                    .columns
                    .iter()
                    .any(|column| column.name() == column_override.name()))
                .then_some(column_override),
                _ => None,
            })
    }
}

impl Parse for Table {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _create: input.parse()?,
            _table: input.parse()?,
            name: input.parse()?,
            _paren: syn::parenthesized!(content in input),
            columns: content.parse_terminated(Column::parse, Token![,])?,
            _semi: input.parse()?,
            field_specs: input.parse_terminated(FieldSpec::parse, Token![,])?,
        })
    }
}

impl ToTokens for Table {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let name_string = name.to_string();

        for unmatched_column_override in self.unmatched_column_overrides() {
            emit_error!(
                unmatched_column_override.name().span(),
                "column override {} does not match any column name",
                unmatched_column_override.name()
            );
        }

        let columns = self.columns().collect::<Vec<_>>();
        let relations = self.relations().map(|relation| relation.to_token_stream());

        let column_names = columns
            .iter()
            .map(ColumnWithOverride::name_or_alias)
            .collect::<Vec<_>>();
        let relation_names = self.relations().map(Relation::name).collect::<Vec<_>>();

        let select_struct = RowStruct::new(
            vec![],
            Ident::new("Select", Span::call_site()),
            self.columns()
                .map(|column| {
                    RowStructField::new(
                        vec![],
                        column.name_or_alias().clone(),
                        column.type_or_override(),
                    )
                })
                .collect(),
        );

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

                pub const NAME: &str = #name_string;
                pub const TABLE: ::kosame::schema::Table = ::kosame::schema::Table::new(
                    #name_string,
                    &[#(&columns::#column_names::COLUMN),*],
                    &[#(&relations::#relation_names::RELATION),*],
                );

                #select_struct
            }
        }
        .to_tokens(tokens);
    }
}
