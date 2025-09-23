use std::fmt::Display;

use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
};

use crate::{
    docs::{Docs, ToDocsTokens},
    path_ext::PathExt,
};

pub struct Relation {
    name: Ident,
    _colon: Token![:],
    _source_paren: syn::token::Paren,
    source_columns: Punctuated<Ident, Token![,]>,
    arrow: Arrow,
    dest_table: syn::Path,
    _dest_paren: syn::token::Paren,
    dest_columns: Punctuated<Ident, Token![,]>,
}

impl Relation {
    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn to_token_stream(&self, source_table: &Ident) -> TokenStream {
        let name = &self.name;
        let name_string = name.to_string();

        let source_table_string = source_table.to_string();
        let dest_table_string = self.dest_table.segments.last().unwrap().ident.to_string();

        let join_string = self
            .source_columns
            .iter()
            .zip(self.dest_columns.iter())
            .map(|(source_column, dest_column)| {
                String::new()
                    + &source_table_string
                    + "."
                    + &source_column.to_string()
                    + " = "
                    + &dest_table_string
                    + "."
                    + &dest_column.to_string()
            })
            .collect::<Vec<_>>()
            .join(" and ");

        let target = &self.dest_table;
        let target_path = target.to_call_site(4);

        let source_columns = self.source_columns.iter();
        let dest_columns = self.dest_columns.iter();

        let relation_type = match self.arrow {
            Arrow::ManyToOne(_) => quote! { ::kosame::relation::ManyToOne<T> },
            Arrow::OneToMany(_) => quote! { ::kosame::relation::OneToMany<T> },
        };

        let docs = self.to_docs_token_stream();

        quote! {
            // #docs
            pub mod #name {
                pub mod target_table {
                    pub use #target_path::*;
                }

                pub mod source_columns {
                    #(pub use super::super::super::columns::#source_columns;)*
                }

                pub mod target_columns {
                    #(pub use super::target_table::columns::#dest_columns;)*
                }

                pub const NAME: &str = #name_string;
                pub const JOIN_CONDITION: &str = #join_string;

                pub type Relation<T> = #relation_type;
            }
        }
    }
}

impl Parse for Relation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let source_content;
        let dest_content;
        let result = Self {
            name: input.parse()?,
            _colon: input.parse()?,
            _source_paren: parenthesized!(source_content in input),
            source_columns: source_content.parse_terminated(Ident::parse, Token![,])?,
            arrow: input.parse()?,
            dest_table: input.parse()?,
            _dest_paren: parenthesized!(dest_content in input),
            dest_columns: dest_content.parse_terminated(Ident::parse, Token![,])?,
        };

        if result.source_columns.is_empty() {
            emit_error!(
                result._source_paren.span.span(),
                "at least one column must be specified for relation `{}`",
                result.name
            );
        }
        if result.source_columns.len() != result.dest_columns.len() {
            emit_error!(
                result._dest_paren.span.span(),
                "number of columns must match on both side of the relation `{}`",
                result.name
            );
        }

        Ok(result)
    }
}

#[allow(dead_code)]
enum Arrow {
    ManyToOne(Token![=>]),
    OneToMany(Token![<=]),
}

impl Parse for Arrow {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![=>]) {
            Ok(Self::ManyToOne(input.parse()?))
        } else if lookahead.peek(Token![<=]) {
            Ok(Self::OneToMany(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Docs for Relation {
    fn docs(&self) -> String {
        let name = &self.name;
        format!(
            "## {name} (Kosame Relation)

```sql
{self}
```"
        )
    }
}

impl Display for Relation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.name, f)?;
        f.write_str(": (")?;
        f.write_str(
            &self
                .source_columns
                .iter()
                .map(|column| column.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        )?;
        f.write_str(") ")?;
        match self.arrow {
            Arrow::ManyToOne(_) => f.write_str("=>")?,
            Arrow::OneToMany(_) => f.write_str("<=")?,
        };
        f.write_str(" ")?;
        f.write_str(&self.dest_table.to_token_stream().to_string())?;
        f.write_str(" (")?;
        f.write_str(
            &self
                .dest_columns
                .iter()
                .map(|column| column.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        )?;
        f.write_str(")")?;
        Ok(())
    }
}
