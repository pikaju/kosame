use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use super::Limit;
use crate::{
    clause::{Having, Offset, OrderBy},
    expr::{Expr, Visitor},
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(group);
    custom_keyword!(by);
}

pub struct GroupBy {
    _group: kw::group,
    _by: kw::by,
    entries: Punctuated<GroupByEntry, Token![,]>,
}

impl GroupBy {
    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Self::peek(input).then(|| input.parse()).transpose()
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::group) && input.peek2(kw::by)
    }

    pub fn accept_expr<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        for entry in &self.entries {
            entry.expr.accept(visitor);
        }
    }
}

impl Parse for GroupBy {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _group: input.parse()?,
            _by: input.parse()?,
            entries: {
                let mut punctuated = Punctuated::new();
                let quit_cond = |input: ParseStream| {
                    input.is_empty()
                        || Having::peek(input)
                        || OrderBy::peek(input)
                        || Limit::peek(input)
                        || Offset::peek(input)
                };
                while !quit_cond(input) {
                    punctuated.push(input.parse()?);
                    if quit_cond(input) {
                        break;
                    }
                    punctuated.push_punct(input.parse()?);
                }
                if punctuated.is_empty() {
                    return Err(syn::Error::new(
                        input.span(),
                        "group by clause cannot be empty",
                    ));
                }
                punctuated
            },
        })
    }
}

impl ToTokens for GroupBy {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let entries = self.entries.iter().map(GroupByEntry::to_token_stream);
        quote! { ::kosame::repr::clause::GroupBy::new(&[#(#entries),*]) }.to_tokens(tokens)
    }
}

pub struct GroupByEntry {
    expr: Expr,
}

impl Parse for GroupByEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr: input.parse()?,
        })
    }
}

impl ToTokens for GroupByEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = &self.expr;
        quote! { ::kosame::repr::clause::GroupByEntry::new(#expr) }.to_tokens(tokens);
    }
}
