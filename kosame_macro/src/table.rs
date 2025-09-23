use std::fmt::{Display, Write};

use quote::{ToTokens, quote};
use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::{
    column::Column,
    docs::{Docs, ToDocsTokens},
    keywords,
    relation::Relation,
};

pub struct Table {
    _create_table: keywords::CreateTable,
    _paren: syn::token::Paren,

    name: Ident,
    columns: Punctuated<Column, Token![,]>,

    _semi: Token![;],

    relations: Punctuated<Relation, Token![,]>,
}

impl Parse for Table {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _create_table: input.parse()?,
            name: input.parse()?,
            _paren: syn::parenthesized!(content in input),
            columns: content.parse_terminated(Column::parse, Token![,])?,
            _semi: input.parse()?,
            relations: input.parse_terminated(Relation::parse, Token![,])?,
        })
    }
}

impl ToTokens for Table {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let name_string = name.to_string();
        let columns = self.columns.iter();
        let relations = self.relations.iter();
        let column_names = self.columns.iter().map(Column::name);
        let relation_names = self.relations.iter().map(Relation::name);

        let docs = self.to_docs_token_stream();

        quote! {
            // #docs
            pub mod #name {
                pub const NAME: &str = #name_string;

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
            }
        }
        .to_tokens(tokens);
    }
}

impl Docs for Table {
    fn docs(&self) -> String {
        let name = &self.name;
        format!(
            "## {name} (Kosame Table)

```sql
{self}
```"
        )
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("create table ")?;
        Display::fmt(&self.name, f)?;
        f.write_str(" (\n")?;
        for column in &self.columns {
            f.write_str("    ")?;
            column.fmt(f)?;
            f.write_str(",\n")?;
        }
        f.write_str(");\n")?;
        f.write_str("\n\n")?;
        for relation in &self.relations {
            Display::fmt(relation, f)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}
