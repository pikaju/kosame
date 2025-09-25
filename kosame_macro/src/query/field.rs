use super::QueryNode;
use crate::{
    alias::Alias, path_ext::PathExt, query::node_path::QueryNodePath, row_struct::RowStructField,
    type_override::TypeOverride,
};
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Ident, Path,
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
        node: QueryNode,
        alias: Option<Alias>,
    },
}

impl QueryField {
    pub fn name(&self) -> &Ident {
        match self {
            Self::Column { name, .. } => name,
            Self::Relation { name, .. } => name,
        }
    }

    pub fn alias(&self) -> Option<&Alias> {
        match self {
            Self::Column { alias, .. } => alias.as_ref(),
            Self::Relation { alias, .. } => alias.as_ref(),
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Column { name, .. } => name.span(),
            Self::Relation { name, .. } => name.span(),
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
        }
    }
}

impl Parse for QueryField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let name = input.parse()?;
        if input.peek(syn::token::Brace) {
            Ok(Self::Relation {
                attrs,
                name,
                node: input.parse()?,
                alias: input.call(Alias::parse_optional)?,
            })
        } else {
            Ok(Self::Column {
                attrs,
                name,
                alias: input.call(Alias::parse_optional)?,
                type_override: input.call(TypeOverride::parse_optional)?,
            })
        }
    }
}
