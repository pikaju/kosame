use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident, Path,
    parse::{Parse, ParseStream},
};

use crate::{alias::Alias, schema::column::Column, type_override::TypeOverride};

pub struct ColumnOverride {
    name: Ident,
    alias: Option<Alias>,
    type_override: Option<TypeOverride>,
}

impl ColumnOverride {
    pub fn name(&self) -> &Ident {
        &self.name
    }
}

impl Parse for ColumnOverride {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            alias: Alias::parse_optional(input)?,
            type_override: TypeOverride::parse_optional(input)?,
        })
    }
}

pub struct ColumnWithOverride<'a> {
    column: &'a Column,
    column_override: Option<&'a ColumnOverride>,
}

impl<'a> ColumnWithOverride<'a> {
    pub fn new(column: &'a Column, column_override: Option<&'a ColumnOverride>) -> Self {
        Self {
            column,
            column_override,
        }
    }

    pub fn name_or_alias(&self) -> &Ident {
        self.alias().unwrap_or(self.column.name())
    }

    pub fn alias(&self) -> Option<&Ident> {
        self.column_override
            .and_then(|column_override| column_override.alias.as_ref().map(|alias| alias.ident()))
    }

    pub fn type_or_override(&self) -> TokenStream {
        self.type_override()
            .map(ToTokens::to_token_stream)
            .unwrap_or_else(|| self.column.data_type_auto())
    }

    pub fn type_override(&self) -> Option<&Path> {
        self.column_override.and_then(|column_override| {
            column_override
                .type_override
                .as_ref()
                .map(|type_override| type_override.type_path())
        })
    }
}

impl ToTokens for ColumnWithOverride<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.name_or_alias();
        let column_name = self.column.name();
        let column_name_string = column_name.to_string();
        let alias = match self.alias() {
            Some(ident) => {
                let string = ident.to_string();
                quote! { Some(#string) }
            }
            None => quote! { None },
        };
        let data_type = self.type_or_override();

        quote! {
            pub mod #name {
                pub const COLUMN: ::kosame::schema::Column = ::kosame::schema::Column::new(#column_name_string, #alias);
                pub type Type = #data_type;
            }
        }
        .to_tokens(tokens);
    }
}
