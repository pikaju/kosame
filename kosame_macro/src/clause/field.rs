use proc_macro_error::abort;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
};

use crate::{
    alias::Alias,
    clause::{Limit, Offset, OrderBy, Where},
    expr::Expr,
    path_ext::PathExt,
    row::RowField,
    type_override::TypeOverride,
};

pub struct Field {
    attrs: Vec<Attribute>,
    expr: Expr,
    alias: Option<Alias>,
    type_override: Option<TypeOverride>,
}

impl Field {
    pub fn to_row_field(&self) -> RowField {
        let Some(alias) = self.alias.as_ref() else {
            abort!(self.expr.span(), "field requires alias using `as my_alias`");
        };
        let Some(type_override) = self.type_override.as_ref() else {
            abort!(
                self.expr.span(),
                "field requires type override using `: RustType`"
            );
        };
        RowField::new(
            self.attrs.clone(),
            alias.ident().clone(),
            type_override.type_path().to_call_site(1).to_token_stream(),
        )
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            expr: input.parse()?,
            alias: input.call(Alias::parse_optional)?,
            type_override: input.call(TypeOverride::parse_optional)?,
        })
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = &self.expr;
        quote! {
            ::kosame::repr::clause::Field::new(#expr, None)
        }
        .to_tokens(tokens)
    }
}

pub struct Fields(Punctuated<Field, Token![,]>);

impl Fields {
    pub fn iter(&self) -> impl Iterator<Item = &Field> {
        self.0.iter()
    }
}

impl Parse for Fields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut fields = Punctuated::<Field, _>::new();

        while !input.is_empty() {
            if Where::peek(input)
                || OrderBy::peek(input)
                || Limit::peek(input)
                || Offset::peek(input)
            {
                break;
            }

            fields.push(input.parse()?);

            if !input.peek(Token![,]) {
                break;
            }
            fields.push_punct(input.parse()?);
        }

        Ok(Self(fields))
    }
}

impl ToTokens for Fields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fields = self.0.iter();
        quote! {
            ::kosame::repr::clause::Fields::new(&[
                #(#fields),*
            ])
        }
        .to_tokens(tokens)
    }
}
