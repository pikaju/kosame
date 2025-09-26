use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::{expr::Expr, query::limit::Limit};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(order);
    custom_keyword!(by);

    custom_keyword!(asc);
    custom_keyword!(desc);

    custom_keyword!(nulls);
    custom_keyword!(first);
    custom_keyword!(last);
}

pub struct OrderBy {
    _order: kw::order,
    _by: kw::by,
    entries: Punctuated<OrderByEntry, Token![,]>,
}

impl OrderBy {
    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Self::peek(input).then(|| input.parse()).transpose()
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::order)
    }
}

impl Parse for OrderBy {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _order: input.parse()?,
            _by: input.parse()?,
            entries: {
                let mut punctuated = Punctuated::new();
                while !input.is_empty() && !Limit::peek(input) {
                    punctuated.push(input.parse()?);
                    if input.is_empty() || Limit::peek(input) {
                        break;
                    }
                    punctuated.push_punct(input.parse()?);
                }
                if punctuated.is_empty() {
                    return Err(syn::Error::new(
                        input.span(),
                        "order by clause cannot be empty",
                    ));
                }
                punctuated
            },
        })
    }
}

impl ToTokens for OrderBy {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let entries = self.entries.iter().map(OrderByEntry::to_token_stream);
        quote! { ::kosame::query::OrderBy::new(&[#(#entries),*]) }.to_tokens(tokens)
    }
}

pub struct OrderByEntry {
    expr: Expr,
    dir: Option<OrderByDir>,
    nulls: Option<OrderByNulls>,
}

impl Parse for OrderByEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr: input.parse()?,
            dir: input.call(OrderByDir::parse_optional)?,
            nulls: input.call(OrderByNulls::parse_optional)?,
        })
    }
}

impl ToTokens for OrderByEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = &self.expr;
        let dir = match self.dir {
            Some(OrderByDir::Asc(_)) => quote! { Some(::kosame::query::OrderByDir::Asc) },
            Some(OrderByDir::Desc(_)) => quote! { Some(::kosame::query::OrderByDir::Desc) },
            None => quote! { None },
        };
        let nulls = match self.nulls {
            Some(OrderByNulls::First(..)) => quote! { Some(::kosame::query::OrderByNulls::First) },
            Some(OrderByNulls::Last(..)) => quote! { Some(::kosame::query::OrderByNulls::Last) },
            None => quote! { None },
        };

        quote! { ::kosame::query::OrderByEntry::new(#expr, #dir, #nulls) }.to_tokens(tokens);
    }
}

pub enum OrderByDir {
    Asc(kw::asc),
    Desc(kw::desc),
}

impl OrderByDir {
    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Self::peek(input).then(|| input.parse()).transpose()
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::asc) || input.peek(kw::desc)
    }
}

impl Parse for OrderByDir {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::asc) {
            Ok(Self::Asc(input.parse()?))
        } else if lookahead.peek(kw::desc) {
            Ok(Self::Desc(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

pub enum OrderByNulls {
    First(kw::nulls, kw::first),
    Last(kw::nulls, kw::last),
}

impl OrderByNulls {
    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Self::peek(input).then(|| input.parse()).transpose()
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::nulls)
    }
}

impl Parse for OrderByNulls {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let nulls = input.parse()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::first) {
            Ok(Self::First(nulls, input.parse()?))
        } else if lookahead.peek(kw::last) {
            Ok(Self::Last(nulls, input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}
