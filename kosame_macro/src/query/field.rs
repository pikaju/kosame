use super::QueryNode;
use crate::{
    as_ident::AsIdent, as_type::AsType, query::node_path::QueryNodePath, row_struct::RowStructField,
};
use proc_macro2::Span;
use quote::quote;
use syn::{
    Attribute, Ident, Path,
    parse::{Parse, ParseStream},
};

pub enum QueryField {
    Column {
        attrs: Vec<Attribute>,
        name: Ident,
        alias: Option<AsIdent>,
        type_override: Option<AsType>,
    },
    Relation {
        attrs: Vec<Attribute>,
        name: Ident,
        node: QueryNode,
        alias: Option<AsIdent>,
        type_override: Option<AsType>,
    },
}

impl QueryField {
    pub fn name(&self) -> &Ident {
        match self {
            Self::Column { name, .. } => name,
            Self::Relation { name, .. } => name,
        }
    }

    pub fn alias(&self) -> Option<&AsIdent> {
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
                attrs, name, alias, ..
            } => {
                let alias_or_name = alias
                    .as_ref()
                    .map(|alias| alias.ident())
                    .unwrap_or(name)
                    .clone();

                RowStructField::new(
                    attrs.clone(),
                    alias_or_name,
                    quote! { #table_path::columns::#name::Type },
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
                    quote! { #table_path::relations::#name::Relation<#inner_type> },
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
                alias: input.call(AsIdent::parse_optional)?,
                type_override: input.call(AsType::parse_optional)?,
            })
        } else {
            Ok(Self::Column {
                attrs,
                name,
                alias: input.call(AsIdent::parse_optional)?,
                type_override: input.call(AsType::parse_optional)?,
            })
        }
    }
}
