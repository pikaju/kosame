use std::fmt::{Display, Write};

use quote::{ToTokens, quote};
use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

use crate::{
    data_type::DataType,
    docs::{Docs, ToDocsTokens},
    keywords,
};

pub struct Column {
    name: Ident,
    data_type: DataType,
    not_null: Option<keywords::NotNull>,
    primary_key: Option<keywords::PrimaryKey>,
}
impl Column {
    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }
}

impl Parse for Column {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let r#type = input.parse()?;

        Ok(Self {
            name,
            data_type: r#type,
            not_null: None,
            primary_key: None,
        })
    }
}

impl ToTokens for Column {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let name_string = name.to_string();
        let data_type = &self.data_type;
        let docs = self.to_docs_token_stream();

        quote! {
            // #docs
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
        Ok(())
    }
}
