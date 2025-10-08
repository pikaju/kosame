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
    target_table: syn::Path,
    _target_paren: syn::token::Paren,
    target_columns: Punctuated<Ident, Token![,]>,
}

impl Relation {
    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn to_token_stream(&self) -> TokenStream {
        let name = &self.name;
        let name_string = name.to_string();

        let target = &self.target_table;
        let target_path = target.to_call_site(3);

        let source_columns = self.source_columns.iter();
        let source_columns2 = source_columns.clone();
        let target_columns = self.target_columns.iter();
        let target_columns2 = target_columns.clone();

        let relation_type = match self.arrow {
            Arrow::ManyToOne(_) => quote! { ::kosame::relation::ManyToOne<T> },
            Arrow::OneToMany(_) => quote! { ::kosame::relation::OneToMany<T> },
        };

        let docs = self.to_docs_token_stream();

        quote! {
            // #docs
            pub mod #name {
                pub use #target_path as target_table;

                pub mod source_columns {
                    #(pub use super::super::super::columns::#source_columns;)*
                }

                pub mod target_columns {
                    #(pub use super::target_table::columns::#target_columns;)*
                }

                pub const RELATION: ::kosame::schema::Relation = ::kosame::schema::Relation::new(
                    #name_string,
                    super::super::NAME,
                    &[#(&source_columns::#source_columns2::COLUMN),*],
                    target_table::NAME,
                    &[#(&target_columns::#target_columns2::COLUMN),*],
                );

                pub type Type<T> = #relation_type;
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
            target_table: input.parse()?,
            _target_paren: parenthesized!(dest_content in input),
            target_columns: dest_content.parse_terminated(Ident::parse, Token![,])?,
        };

        if result.source_columns.is_empty() {
            emit_error!(
                result._source_paren.span.span(),
                "at least one column must be specified for relation `{}`",
                result.name
            );
        }
        if result.source_columns.len() != result.target_columns.len() {
            emit_error!(
                result._target_paren.span.span(),
                "number of columns must match on both side of the relation `{}`",
                result.name
            );
        }

        Ok(result)
    }
}

#[allow(unused)]
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
        f.write_str(&self.target_table.to_token_stream().to_string())?;
        f.write_str(" (")?;
        f.write_str(
            &self
                .target_columns
                .iter()
                .map(|column| column.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        )?;
        f.write_str(")")?;
        Ok(())
    }
}
