use std::fmt::Display;

use super::data_type::DataType;
use crate::docs::{Docs, ToDocsTokens};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

pub struct Column {
    name: Ident,
    data_type: DataType,
    constraints: super::column_constraint::ColumnConstraints,
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
        if !self.constraints.has_not_null() && !self.constraints.has_primary_key() {
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

impl ToTokens for Column {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let name_string = name.to_string();
        let data_type = &self.data_type_auto();
        let docs = self.to_docs_token_stream();

        quote! {
            #docs
            pub mod #name {
                pub const NAME: &str = #name_string;
                pub type Type = #data_type;
            }
        }
        .to_tokens(tokens);
    }
}

impl Docs for Column {
    fn docs(&self) -> String {
        let name = &self.name;
        format!(
            "## {name} (Kosame Column)

```sql
{self}
```"
        )
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.name, f)?;
        f.write_str(" ")?;
        Display::fmt(&self.data_type, f)?;
        for constraint in self.constraints.iter() {
            f.write_str(" ")?;
            Display::fmt(&constraint, f)?;
        }
        Ok(())
    }
}
