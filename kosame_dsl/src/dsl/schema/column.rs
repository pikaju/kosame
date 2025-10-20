use crate::{dsl::attribute::ParsedAttributes, dsl::path_ext::PathExt};

use super::{column_constraint::ColumnConstraints, data_type::DataType};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

pub struct Column {
    attrs: ParsedAttributes,
    name: Ident,
    data_type: DataType,
    constraints: ColumnConstraints,
}

impl Column {
    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn rust_name(&self) -> Ident {
        Ident::new(
            &self.name.to_string().to_case(Case::Snake),
            self.name.span(),
        )
    }

    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }

    fn rust_type(&self) -> TokenStream {
        let data_type = self.data_type();
        match self.attrs.type_override() {
            Some(path) => {
                let path = path.to_call_site(3);
                quote! { #path }
            }
            None => quote! { #data_type },
        }
    }

    fn rust_type_nullable(&self) -> TokenStream {
        let data_type = self.data_type();
        quote! { Option<#data_type> }
    }

    fn rust_type_auto(&self) -> TokenStream {
        if self.constraints.not_null().is_none() && self.constraints.primary_key().is_none() {
            self.rust_type_nullable()
        } else {
            self.rust_type()
        }
    }
}

impl Parse for Column {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs: ParsedAttributes = input.parse()?;
        attrs.require_no_global()?;
        let name = input.parse()?;
        let data_type = input.parse()?;

        Ok(Self {
            attrs,
            name,
            data_type,
            constraints: input.parse()?,
        })
    }
}

impl ToTokens for Column {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name_string = self.name.to_string();
        let rust_name = self.rust_name();
        let rust_name_string = rust_name.to_string();

        let rust_type_auto = self.rust_type_auto();
        let rust_type_nullable = self.rust_type_nullable();

        quote! {
            pub mod #rust_name {
                pub const COLUMN: ::kosame::schema::Column = ::kosame::schema::Column::new(#name_string, Some(#rust_name_string));
                pub type Type = #rust_type_auto;
                pub type TypeNullable = #rust_type_nullable;
            }
        }
        .to_tokens(tokens);
    }
}
