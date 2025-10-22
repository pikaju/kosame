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
