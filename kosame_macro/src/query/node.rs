use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Token, braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use super::QueryField;

struct Context {}

pub struct QueryNode {
    _brace: syn::token::Brace,
    fields: Punctuated<QueryField, Token![,]>,
}

impl QueryNode {
    pub fn fields(&self) -> &Punctuated<QueryField, Token![,]> {
        &self.fields
    }

    pub fn to_from_row_impl(&self, struct_name: impl ToTokens) -> TokenStream {
        let fields = self.fields.iter().enumerate().map(|(index, field)| {
            let name = field.name();
            quote! {
                #name: row.get(#index)
            }
        });

        quote! {
            impl From<::postgres::Row> for #struct_name {
                fn from(row: ::postgres::Row) -> Self {
                    Self {
                        #(#fields),*
                    }
                }
            }
        }
    }

    fn to_tokens_with_cx(&self, tokens: &mut TokenStream, cx: Context) {}
}

impl Parse for QueryNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _brace: braced!(content in input),
            fields: content.parse_terminated(QueryField::parse, Token![,])?,
        })
    }
}
