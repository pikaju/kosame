use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Ident,
    parse::{Parse, ParseStream},
    parse_quote,
};

use crate::{alias::Alias, row::RowField};

pub struct Star {
    pub attrs: Vec<Attribute>,
    pub star: syn::token::Star,
    pub alias: Option<Alias>,
}

impl Star {
    pub fn to_row_field(&self, table_path: impl ToTokens) -> RowField {
        let additional_attrs = [
            parse_quote! { #[star] },
            #[cfg(feature = "serde")]
            parse_quote! { #[serde(flatten)] },
        ];

        RowField::new(
            self.attrs
                .iter()
                .chain(additional_attrs.iter())
                .cloned()
                .collect(),
            match &self.alias {
                Some(as_name) => as_name.ident().clone(),
                None => Ident::new("_star", Span::call_site()),
            },
            quote! { #table_path::Select },
        )
    }
}

impl Parse for Star {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            star: input.parse()?,
            alias: input.call(Alias::parse_optional)?,
        })
    }
}
