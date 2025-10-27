use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::{clause::peek_clause, expr::Expr, visitor::Visitor};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(set);
    custom_keyword!(default);
}

pub struct Set {
    _set_kw: kw::set,
    items: Punctuated<SetItem, Token![,]>,
}

impl Set {
    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Self::peek(input).then(|| input.parse()).transpose()
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::set)
    }

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        for item in &self.items {
            item.accept(visitor);
        }
    }
}

impl Parse for Set {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _set_kw: input.parse()?,
            items: {
                let mut items = Punctuated::<SetItem, _>::new();
                while !input.is_empty() {
                    if peek_clause(input) {
                        break;
                    }

                    items.push(input.parse()?);

                    if !input.peek(Token![,]) {
                        break;
                    }
                    items.push_punct(input.parse()?);
                }

                items
            },
        })
    }
}

impl ToTokens for Set {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let items = self.items.iter();
        quote! {
            ::kosame::repr::clause::Set::new(&[#(#items),*])
        }
        .to_tokens(tokens);
    }
}

pub enum SetItem {
    Default {
        column: Ident,
        _eq_token: Token![=],
        _default_kw: kw::default,
    },
    Expr {
        column: Ident,
        _eq_token: Token![=],
        expr: Expr,
    },
}

impl SetItem {
    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        match self {
            Self::Default { .. } => {}
            Self::Expr { expr, .. } => {
                expr.accept(visitor);
            }
        }
    }
}

impl Parse for SetItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let column: Ident = input.parse()?;
        let eq_token = input.parse()?;

        if input.peek(kw::default) {
            Ok(Self::Default {
                column,
                _eq_token: eq_token,
                _default_kw: input.parse()?,
            })
        } else {
            Ok(Self::Expr {
                column,
                _eq_token: eq_token,
                expr: input.parse()?,
            })
        }
    }
}

impl ToTokens for SetItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Default { column, .. } => {
                let column = column.to_string();
                quote! {
                    ::kosame::repr::clause::SetItem::Default { column: #column }
                }
            }
            Self::Expr { column, expr, .. } => {
                let column = column.to_string();
                quote! {
                    ::kosame::repr::clause::SetItem::Expr { column: #column, expr: #expr }
                }
            }
        }
        .to_tokens(tokens);
    }
}
