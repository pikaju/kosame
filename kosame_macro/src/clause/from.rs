use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident, Path, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::{
    clause::peek_clause, command::Select, expr::Expr, path_ext::PathExt, quote_option::QuoteOption,
    visitor::Visitor,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(from);

    custom_keyword!(join);
    custom_keyword!(inner);
    custom_keyword!(left);
    custom_keyword!(right);
    custom_keyword!(full);
    custom_keyword!(on);

    custom_keyword!(natural);
    custom_keyword!(cross);

    custom_keyword!(lateral);
}

pub struct From {
    pub _from: kw::from,
    pub item: FromItem,
}

impl From {
    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Self::peek(input).then(|| input.parse()).transpose()
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::from)
    }

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        self.item.accept(visitor);
    }
}

impl Parse for From {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _from: input.parse()?,
            item: input.parse()?,
        })
    }
}

impl ToTokens for From {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let item = &self.item;
        quote! { ::kosame::repr::clause::From::new(#item) }.to_tokens(tokens);
    }
}

pub struct TableAlias {
    pub _as_token: Option<Token![as]>,
    pub name: Ident,
    pub columns: Option<TableAliasColumns>,
}

impl TableAlias {
    fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        if input.is_empty() || peek_clause(input) {
            return Ok(None);
        }
        macro_rules! check {
            ($kw:expr) => {
                if input.peek($kw) {
                    return Ok(None);
                }
            };
        }
        check!(kw::inner);
        check!(kw::left);
        check!(kw::right);
        check!(kw::full);
        check!(kw::on);

        check!(kw::natural);
        check!(kw::cross);

        Ok(Some(input.parse()?))
    }
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

impl ToTokens for TableAlias {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name.to_string();
        let columns = QuoteOption(self.columns.as_ref());
        quote! {
            ::kosame::repr::clause::TableAlias::new(#name, #columns)
        }
        .to_tokens(tokens);
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
            columns: content.parse_terminated(Ident::parse, Token![,])?,
        })
    }
}

impl ToTokens for TableAliasColumns {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let columns = self.columns.iter().map(|column| column.to_string());
        quote! {
            &[#(#columns),*]
        }
        .to_tokens(tokens);
    }
}

#[allow(unused)]
pub enum JoinType {
    Inner(kw::inner, kw::join),
    Left(kw::left, kw::join),
    Right(kw::right, kw::join),
    Full(kw::full, kw::join),
}

impl JoinType {
    fn peek(input: ParseStream) -> bool {
        macro_rules! check {
            ($kw:expr) => {
                if input.peek($kw) {
                    return true;
                }
            };
        }
        check!(kw::inner);
        check!(kw::left);
        check!(kw::right);
        check!(kw::full);
        false
    }
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

impl ToTokens for JoinType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Inner(..) => quote! { ::kosame::repr::clause::JoinType::Inner },
            Self::Left(..) => quote! { ::kosame::repr::clause::JoinType::Left },
            Self::Right(..) => quote! { ::kosame::repr::clause::JoinType::Right },
            Self::Full(..) => quote! { ::kosame::repr::clause::JoinType::Full },
        }
        .to_tokens(tokens);
    }
}

pub struct On {
    pub _on_token: kw::on,
    pub expr: Expr,
}

impl Parse for On {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _on_token: input.parse()?,
            expr: input.parse()?,
        })
    }
}

pub enum FromItem {
    Table {
        table: Path,
        alias: Option<TableAlias>,
    },
    Subquery {
        lateral_kw: Option<kw::lateral>,
        _paren_token: syn::token::Paren,
        select: Box<Select>,
        alias: Option<TableAlias>,
    },
    Join {
        left: Box<FromItem>,
        join_type: JoinType,
        right: Box<FromItem>,
        on: On,
    },
    NaturalJoin {
        _natural_kw: kw::natural,
        left: Box<FromItem>,
        join_type: JoinType,
        right: Box<FromItem>,
    },
    CrossJoin {
        left: Box<FromItem>,
        _cross_kw: kw::cross,
        _join_kw: kw::join,
        right: Box<FromItem>,
    },
}

impl FromItem {
    fn parse_prefix(input: ParseStream) -> syn::Result<Self> {
        let lateral_kw = input.peek(kw::lateral).then(|| input.parse()).transpose()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::token::Paren) {
            let content;
            Ok(Self::Subquery {
                lateral_kw,
                _paren_token: parenthesized!(content in input),
                select: content.parse()?,
                alias: input.call(TableAlias::parse_optional)?,
            })
        } else if lookahead.peek(Ident) {
            Ok(Self::Table {
                table: input.parse()?,
                alias: input.call(TableAlias::parse_optional)?,
            })
        } else {
            Err(lookahead.error())
        }
    }

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        match self {
            Self::Table { table, .. } => {
                visitor.visit_table_ref(table);
            }
            Self::Subquery { select, .. } => {
                select.accept(visitor);
            }
            Self::Join {
                left, right, on, ..
            } => {
                on.expr.accept(visitor);
                left.accept(visitor);
                right.accept(visitor);
            }
            Self::NaturalJoin { left, right, .. } => {
                left.accept(visitor);
                right.accept(visitor);
            }
            Self::CrossJoin { left, right, .. } => {
                left.accept(visitor);
                right.accept(visitor);
            }
        }
    }
}

impl Parse for FromItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut item = Self::parse_prefix(input)?;
        loop {
            if JoinType::peek(input) {
                item = FromItem::Join {
                    left: Box::new(item),
                    join_type: input.parse()?,
                    right: Box::new(Self::parse_prefix(input)?),
                    on: input.parse()?,
                };
                continue;
            }
            if input.peek(kw::natural) {
                item = FromItem::NaturalJoin {
                    _natural_kw: input.parse()?,
                    left: Box::new(item),
                    join_type: input.parse()?,
                    right: Box::new(Self::parse_prefix(input)?),
                };
            }
            if input.peek(kw::cross) {
                item = FromItem::CrossJoin {
                    left: Box::new(item),
                    _cross_kw: input.parse()?,
                    _join_kw: input.parse()?,
                    right: Box::new(Self::parse_prefix(input)?),
                };
            }
            break;
        }
        Ok(item)
    }
}

impl ToTokens for FromItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Table { table, alias } => {
                let table = table.to_call_site(1);
                let alias = QuoteOption(alias.as_ref());
                quote! {
                    ::kosame::repr::clause::FromItem::Table {
                        table: &#table::TABLE,
                        alias: #alias,
                    }
                }
            }
            Self::Subquery {
                lateral_kw: _lateral_kw,
                select,
                alias,
                ..
            } => {
                let lateral = _lateral_kw.is_some();
                let alias = QuoteOption(alias.as_ref());
                quote! {
                    ::kosame::repr::clause::FromItem::Subquery {
                        lateral: #lateral,
                        select: &#select,
                        alias: #alias,
                    }
                }
            }
            Self::Join {
                left,
                join_type,
                right,
                on,
            } => {
                let on = &on.expr;
                quote! {
                    ::kosame::repr::clause::FromItem::Join {
                        left: &#left,
                        join_type: #join_type,
                        right: &#right,
                        on: #on,
                    }
                }
            }
            Self::NaturalJoin {
                left,
                join_type,
                right,
                ..
            } => {
                quote! {
                    ::kosame::repr::clause::FromItem::NaturalJoin {
                        left: &#left,
                        join_type: #join_type,
                        right: &#right,
                    }
                }
            }
            Self::CrossJoin { left, right, .. } => {
                quote! {
                    ::kosame::repr::clause::FromItem::CrossJoin {
                        left: &#left,
                        right: &#right,
                    }
                }
            }
        }
        .to_tokens(tokens);
    }
}
