use super::{column_constraint::ColumnConstraints, data_type::DataType};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

pub struct Column {
    name: Ident,
    data_type: DataType,
    constraints: ColumnConstraints,
}

impl Column {
    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn data_type_not_null(&self) -> &DataType {
        &self.data_type
    }

    pub fn data_type_nullable(&self) -> TokenStream {
        let data_type = self.data_type_not_null();
        quote! { Option<#data_type> }
    }

    pub fn data_type_auto(&self) -> TokenStream {
        if self.constraints.not_null().is_none() && self.constraints.primary_key().is_none() {
            self.data_type_nullable()
        } else {
            self.data_type_not_null().to_token_stream()
        }
    }
}

impl Parse for Column {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let r#type = input.parse()?;

        Ok(Self {
            name,
            data_type: r#type,
            constraints: input.parse()?,
        })
    }
}
