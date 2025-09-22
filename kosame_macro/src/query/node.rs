use proc_macro2::TokenStream;
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
