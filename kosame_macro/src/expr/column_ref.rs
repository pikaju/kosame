use super::Visitor;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
};

pub struct ColumnRef {
    correlation: Option<Correlation>,
    name: Ident,
}

impl ColumnRef {
    pub fn accept<'a>(&'a self, _visitor: &mut impl Visitor<'a>) {}

    pub fn span(&self) -> Span {
        if let Some(correlation) = &self.correlation {
            correlation
                .name
                .span()
                .join(self.name.span())
                .expect("same file")
        } else {
            self.name.span()
        }
    }
}

impl Parse for ColumnRef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident1 = input.parse::<Ident>()?;
        if input.peek(Token![.]) {
            let correlation = Correlation {
                name: ident1,
                _period_token: input.parse()?,
            };
            Ok(Self {
                correlation: Some(correlation),
                name: input.parse()?,
            })
        } else {
            Ok(Self {
                correlation: None,
                name: ident1,
            })
        }
    }
}

impl ToTokens for ColumnRef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        match &self.correlation {
            Some(correlation) => {
                let correlation = &correlation.name;
                quote! {
                    ::kosame::repr::expr::ColumnRef::new(
                        Some(scope::#correlation::TABLE_NAME),
                        scope::#correlation::columns::#name::COLUMN_NAME
                    )
                }
                .to_tokens(tokens)
            }
            None => quote! {
                ::kosame::repr::expr::ColumnRef::new(
                    ::core::option::Option::None,
                    scope::#name::COLUMN_NAME
                )
            }
            .to_tokens(tokens),
        }
    }
}

pub struct Correlation {
    name: Ident,
    _period_token: Token![.],
}
