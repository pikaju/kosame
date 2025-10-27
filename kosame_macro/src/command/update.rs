use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Path,
    parse::{Parse, ParseStream},
};

use crate::{
    clause::*, path_ext::PathExt, quote_option::QuoteOption, scope::Scope, visitor::Visitor,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(update);
}

pub struct Update {
    pub attrs: Vec<Attribute>,
    pub _update_kw: kw::update,
    pub table: Path,
    pub set: Set,
    pub from: Option<From>,
    pub r#where: Option<Where>,
    pub returning: Option<Returning>,
}

impl Update {
    pub fn peek(input: ParseStream) -> bool {
        let input = input.fork();
        let attrs = input.call(Attribute::parse_outer);
        if attrs.is_err() {
            return false;
        }
        input.peek(kw::update)
    }

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        visitor.visit_table_ref(&self.table);
        self.set.accept(visitor);
        if let Some(inner) = &self.r#where {
            inner.accept(visitor)
        }
        if let Some(inner) = &self.returning {
            inner.accept(visitor)
        }
    }
}

impl Parse for Update {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            _update_kw: input.parse()?,
            table: input.parse()?,
            set: input.parse()?,
            from: input.call(From::parse_optional)?,
            r#where: input.call(Where::parse_optional)?,
            returning: input.call(Returning::parse_optional)?,
        })
    }
}

impl ToTokens for Update {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let table = &self.table.to_call_site(1);
        let set = &self.set;
        let from = QuoteOption(self.from.as_ref());
        let r#where = QuoteOption(self.r#where.as_ref());
        let returning = QuoteOption(self.returning.as_ref());

        let scope = Scope::new(
            std::iter::once(&FromItem::Table {
                table: self.table.clone(),
                alias: None,
            })
            .chain(self.from.as_ref().map(|from| &from.item)),
        );

        quote! {
            {
                #scope

                ::kosame::repr::command::Update::new(
                    &#table::TABLE,
                    #set,
                    #from,
                    #r#where,
                    #returning,
                )
            }
        }
        .to_tokens(tokens);
    }
}
