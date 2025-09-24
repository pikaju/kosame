use super::QueryNode;
use crate::{query::node_path::QueryNodePath, record_struct::RecordStructField};
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
    },
    Relation {
        attrs: Vec<Attribute>,
        name: Ident,
        node: QueryNode,
    },
}

impl QueryField {
    pub fn name(&self) -> &Ident {
        match self {
            Self::Column { name, .. } => name,
            Self::Relation { name, .. } => name,
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

    pub fn to_record_struct_field(
        &self,
        table_path: &Path,
        node_path: &QueryNodePath,
    ) -> RecordStructField {
        match self {
            QueryField::Column { attrs, name, .. } => RecordStructField::new(
                attrs.clone(),
                name.clone(),
                quote! { #table_path::columns::#name::Type },
            ),
            QueryField::Relation { attrs, name, .. } => {
                let mut node_path = node_path.clone();
                node_path.append(name.clone());
                let inner_type = node_path.to_struct_name("Row");
                RecordStructField::new(
                    attrs.clone(),
                    name.clone(),
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
            })
        } else {
            Ok(Self::Column { attrs, name })
        }
    }
}
