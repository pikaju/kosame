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
    use crate::autocomplete::custom_keyword;

    custom_keyword!(insert);
    custom_keyword!(into);
}

pub struct Insert {
    pub attrs: Vec<Attribute>,
    pub _insert_kw: kw::insert,
    pub _into_kw: kw::into,
    pub table: Path,
    pub values: Values,
    pub returning: Option<Returning>,
}

impl Insert {
    pub fn peek(input: ParseStream) -> bool {
        let input = input.fork();
        let attrs = input.call(Attribute::parse_outer);
        if attrs.is_err() {
            return false;
        }
        input.peek(kw::insert)
    }

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        visitor.visit_table_ref(&self.table);
        self.values.accept(visitor);
        if let Some(inner) = &self.returning {
            inner.accept(visitor)
        }
    }
}

impl Parse for Insert {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            _insert_kw: input.parse()?,
            _into_kw: input.parse()?,
            table: input.parse()?,
            values: input.parse()?,
            returning: input.call(Returning::parse_optional)?,
        })
    }
}

impl ToTokens for Insert {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let table = &self.table.to_call_site(1);
        let values = &self.values;
        let returning = QuoteOption(self.returning.as_ref());

        let scope = Scope::new(std::iter::once(&FromItem::Table {
            table: self.table.clone(),
            alias: None,
        }));

        quote! {
            {
                #scope

                ::kosame::repr::command::Insert::new(
                    &#table::TABLE,
                    {
                        mod scope {}
                        #values
                    },
                    #returning,
                )
            }
        }
        .to_tokens(tokens);
    }
}
