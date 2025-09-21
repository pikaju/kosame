use quote::{ToTokens, quote};
use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::{column::Column, keywords};

pub struct Table {
    _create_table: keywords::CreateTable,
    _paren: syn::token::Paren,

    name: Ident,
    columns: Punctuated<Column, Token![,]>,

    _semi: Token![;],
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
        })
    }
}

impl ToTokens for Table {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let name_string = name.to_string();
        let columns = self.columns.iter();
        quote! {
            /// kosame table
            pub mod #name {
                const NAME: &str = #name_string;

                pub mod columns {
                    #(#columns)*
                }
            }
        }
        .to_tokens(tokens);
    }
}
