use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute,
    parse::{Parse, ParseStream},
};

use crate::{
    clause::{self, From, GroupBy, Having, Limit, Offset, OrderBy, Where},
    quote_option::QuoteOption,
    scope::Scope,
    visitor::Visitor,
};

pub struct Select {
    pub attrs: Vec<Attribute>,
    pub select: clause::Select,
    pub from: Option<From>,
    pub r#where: Option<Where>,
    pub group_by: Option<GroupBy>,
    pub having: Option<Having>,
    pub order_by: Option<OrderBy>,
    pub limit: Option<Limit>,
    pub offset: Option<Offset>,
}

impl Select {
    pub fn peek(input: ParseStream) -> bool {
        let input = input.fork();
        let attrs = input.call(Attribute::parse_outer);
        if attrs.is_err() {
            return false;
        }
        clause::Select::peek(&input)
    }

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        self.select.accept(visitor);
        if let Some(inner) = self.from.as_ref() {
            inner.accept(visitor)
        }
        if let Some(inner) = self.r#where.as_ref() {
            inner.accept(visitor)
        }
        if let Some(inner) = self.group_by.as_ref() {
            inner.accept(visitor)
        }
        if let Some(inner) = self.having.as_ref() {
            inner.accept(visitor)
        }
        if let Some(inner) = self.order_by.as_ref() {
            inner.accept(visitor)
        }
        if let Some(inner) = self.limit.as_ref() {
            inner.accept(visitor)
        }
        if let Some(inner) = self.offset.as_ref() {
            inner.accept(visitor)
        }
    }
}

impl Parse for Select {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            select: input.parse()?,
            from: input.call(From::parse_optional)?,
            r#where: input.call(Where::parse_optional)?,
            group_by: input.call(GroupBy::parse_optional)?,
            having: input.call(Having::parse_optional)?,
            order_by: input.call(OrderBy::parse_optional)?,
            limit: input.call(Limit::parse_optional)?,
            offset: input.call(Offset::parse_optional)?,
        })
    }
}

impl ToTokens for Select {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let select = &self.select;
        let from = QuoteOption(self.from.as_ref());
        let r#where = QuoteOption(self.r#where.as_ref());
        let group_by = QuoteOption(self.group_by.as_ref());
        let having = QuoteOption(self.having.as_ref());
        let order_by = QuoteOption(self.order_by.as_ref());
        let limit = QuoteOption(self.limit.as_ref());
        let offset = QuoteOption(self.offset.as_ref());

        let scope = Scope::new(self.from.as_ref().map(|from| &from.item));

        quote! {
            {
                const select: ::kosame::repr::command::Select<'static> = ::kosame::repr::command::Select::new(
                    #select,
                    #from,
                    #r#where,
                    #group_by,
                    #having,
                    #order_by,
                    #limit,
                    #offset,
                );

                #scope

                select
            }
        }
        .to_tokens(tokens);
    }
}
