use super::Node;
use crate::dsl::{
    alias::Alias, expr::Expr, path_ext::PathExt, query::node_path::QueryNodePath,
    row_struct::RowStructField, type_override::TypeOverride,
};
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Ident, Path, Token,
    parse::{Parse, ParseStream},
    parse_quote,
};

pub enum QueryField {
    Column {
        attrs: Vec<Attribute>,
        name: Ident,
        alias: Option<Alias>,
        type_override: Option<TypeOverride>,
    },
    Relation {
        attrs: Vec<Attribute>,
        name: Ident,
        node: Box<Node>,
        alias: Option<Alias>,
    },
    Expr {
        attrs: Vec<Attribute>,
        expr: Expr,
        alias: Alias,
        type_override: TypeOverride,
    },
}

impl QueryField {
    pub fn name(&self) -> &Ident {
        match self {
            Self::Column { name, .. } => name,
            Self::Relation { name, .. } => name,
            Self::Expr { alias, .. } => alias.ident(),
        }
    }

    pub fn alias(&self) -> Option<&Alias> {
        match self {
            Self::Column { alias, .. } => alias.as_ref(),
            Self::Relation { alias, .. } => alias.as_ref(),
            Self::Expr { alias, .. } => Some(alias),
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Column { name, .. } => name.span(),
            Self::Relation { name, .. } => name.span(),
            Self::Expr { alias, .. } => alias.ident().span(),
        }
    }

    /// Returns `true` if the query field is [`Column`].
    ///
    /// [`Column`]: QueryField::Column
    #[must_use]
    pub fn is_column(&self) -> bool {
        matches!(self, Self::Column { .. })
    }

    pub fn to_row_struct_field(
        &self,
        table_path: &Path,
        node_path: &QueryNodePath,
    ) -> RowStructField {
        match self {
            QueryField::Column {
                attrs,
                name,
                alias,
                type_override,
                ..
            } => {
                let alias_or_name = alias
                    .as_ref()
                    .map(|alias| alias.ident())
                    .unwrap_or(name)
                    .clone();

                let type_override_or_default = type_override
                    .as_ref()
                    .map(|type_override| type_override.type_path().to_call_site(1))
                    .unwrap_or_else(|| parse_quote! { #table_path::columns::#name::Type });

                RowStructField::new(
                    attrs.clone(),
                    alias_or_name,
                    type_override_or_default.to_token_stream(),
                )
            }
            QueryField::Relation {
                attrs, name, alias, ..
            } => {
                let alias_or_name = alias
                    .as_ref()
                    .map(|alias| alias.ident())
                    .unwrap_or(name)
                    .clone();

                let mut node_path = node_path.clone();
                node_path.append(name.clone());
                let inner_type = node_path.to_struct_name("Row");

                RowStructField::new(
                    attrs.clone(),
                    alias_or_name,
                    quote! { #table_path::relations::#name::Type<#inner_type> },
                )
            }
            QueryField::Expr {
                attrs,
                alias,
                type_override,
                ..
            } => RowStructField::new(
                attrs.clone(),
                alias.ident().clone(),
                type_override.type_path().to_call_site(1).to_token_stream(),
            ),
        }
    }
}

impl Parse for QueryField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let fork = input.fork();
        let ident = fork.parse::<Ident>();

        if input.peek2(syn::token::Brace) {
            Ok(Self::Relation {
                attrs,
                name: input.parse()?,
                node: input.parse()?,
                alias: input.call(Alias::parse_optional)?,
            })
        } else if ident.is_ok()
            && (fork.peek(Token![,])
                || Alias::peek(&fork)
                || TypeOverride::peek(&fork)
                || fork.is_empty())
        {
            Ok(Self::Column {
                attrs,
                name: input.parse()?,
                alias: input.call(Alias::parse_optional)?,
                type_override: input.call(TypeOverride::parse_optional)?,
            })
        } else {
            Ok(Self::Expr {
                attrs,
                expr: input.parse()?,
                alias: input.parse()?,
                type_override: input.parse()?,
            })
        }
    }
}
