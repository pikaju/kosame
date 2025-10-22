use crate::lang::attribute::{CustomMeta, MetaLocation};

use super::{column::Column, relation::Relation};
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
    pub macro_attrs: Vec<Attribute>,
    pub attrs: Vec<Attribute>,

    pub _create: kw::create,
    pub _table: kw::table,
    pub _paren: syn::token::Paren,

    pub name: Ident,
    pub columns: Punctuated<Column, Token![,]>,

    pub _semi: Token![;],

    pub relations: Punctuated<Relation, Token![,]>,
}

impl Parse for Table {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            macro_attrs: {
                let attrs = Attribute::parse_inner(input)?;
                CustomMeta::parse_attrs(&attrs, MetaLocation::TableMacro)?;
                attrs
            },
            attrs: {
                let attrs = Attribute::parse_outer(input)?;
                CustomMeta::parse_attrs(&attrs, MetaLocation::Table)?;
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
