use proc_macro_error::abort;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Ident, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::{
    alias::Alias, clause::peek_clause, data_type::InferredType, expr::Expr, path_ext::PathExt,
    row::RowField, type_override::TypeOverride, visitor::Visitor,
};

pub struct Field {
    pub attrs: Vec<Attribute>,
    pub expr: Expr,
    pub alias: Option<Alias>,
    pub type_override: Option<TypeOverride>,
}

impl Field {
    pub fn to_row_field(&self) -> RowField {
        let Some(name) = self.infer_name() else {
            abort!(
                self.expr.span(),
                "field name cannot be inferred";
                help = "consider adding an alias using `as my_alias`"
            );
        };
        let Some(inferred_type) = self.infer_type() else {
            abort!(
                self.expr.span(),
                "field requires type override using `: RustType`"
            );
        };
        RowField::new(
            self.attrs.clone(),
            name.clone(),
            inferred_type.to_call_site(1).to_token_stream(),
        )
    }

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        self.expr.accept(visitor);
    }

    pub fn infer_name(&self) -> Option<&Ident> {
        self.alias
            .as_ref()
            .map(|alias| &alias.ident)
            .or_else(|| self.expr.infer_name())
    }

    pub fn infer_type(&self) -> Option<InferredType> {
        self.type_override
            .as_ref()
            .map(|type_override| InferredType::RustType(type_override.type_path.clone()))
            .or_else(|| self.expr.infer_type())
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

pub struct Fields(pub Punctuated<Field, Token![,]>);

impl Fields {
    pub fn iter(&self) -> impl Iterator<Item = &Field> {
        self.0.iter()
    }

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        for field in self.iter() {
            field.accept(visitor);
        }
    }
}

impl Parse for Fields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut fields = Punctuated::<Field, _>::new();

        while !input.is_empty() {
            if peek_clause(input) {
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
