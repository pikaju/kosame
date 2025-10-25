use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident, Path, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::path_ext::PathExt;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(from);

    custom_keyword!(join);
    custom_keyword!(inner);
    custom_keyword!(left);
    custom_keyword!(right);
    custom_keyword!(full);
    custom_keyword!(natural);
    custom_keyword!(cross);
}

pub struct From {
    pub _from: kw::from,
    pub table: Path,
}

impl From {
    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Self::peek(input).then(|| input.parse()).transpose()
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::from)
    }
}

impl Parse for From {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _from: input.parse()?,
            table: input.parse()?,
        })
    }
}

impl ToTokens for From {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let table = self.table.to_call_site(1);
        quote! { ::kosame::repr::clause::From::new(&#table::TABLE) }.to_tokens(tokens);
    }
}

pub struct TableAlias {
    pub _as_token: Option<Token![as]>,
    pub name: Ident,
    pub columns: Option<TableAliasColumns>,
}

impl Parse for TableAlias {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _as_token: input.peek(Token![as]).then(|| input.parse()).transpose()?,
            name: input.parse()?,
            columns: input
                .peek(syn::token::Paren)
                .then(|| input.parse())
                .transpose()?,
        })
    }
}

pub struct TableAliasColumns {
    pub _paren_token: syn::token::Paren,
    pub columns: Punctuated<Ident, Token![,]>,
}

impl Parse for TableAliasColumns {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _paren_token: parenthesized!(content in input),
            columns: input.parse_terminated(Ident::parse, Token![,])?,
        })
    }
}

pub enum JoinType {
    Inner(kw::inner, kw::join),
    Left(kw::left, kw::join),
    Right(kw::right, kw::join),
    Full(kw::full, kw::join),
}

impl Parse for JoinType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::inner) {
            Ok(Self::Inner(input.parse()?, input.parse()?))
        } else if lookahead.peek(kw::left) {
            Ok(Self::Left(input.parse()?, input.parse()?))
        } else if lookahead.peek(kw::right) {
            Ok(Self::Right(input.parse()?, input.parse()?))
        } else if lookahead.peek(kw::full) {
            Ok(Self::Full(input.parse()?, input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

pub enum FromItem {
    Table {
        table: Path,
        alias: Option<TableAlias>,
    },
}
