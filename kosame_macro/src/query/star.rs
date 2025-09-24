use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Ident,
    parse::{Parse, ParseStream},
    parse_quote,
};

use crate::record_struct::RecordStructField;

pub struct Star {
    attrs: Vec<Attribute>,
    _star: syn::token::Star,
}

impl Star {
    pub fn to_record_struct_field(&self, table_path: impl ToTokens) -> RecordStructField {
        let additional_attrs = [
            #[cfg(any(feature = "serde-serialize", feature = "serde-deserialize"))]
            parse_quote! { #[serde(flatten)] },
        ];

        RecordStructField::new(
            self.attrs
                .iter()
                .chain(additional_attrs.iter())
                .cloned()
                .collect(),
            Ident::new("_star", Span::call_site()),
            quote! { #table_path::Select },
        )
    }
}

impl Parse for Star {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            _star: input.parse()?,
        })
    }
}
