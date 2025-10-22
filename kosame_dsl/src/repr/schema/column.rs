use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Path, parse_quote};

use crate::lang::{
    self,
    attribute::{CustomMeta, MetaLocation},
    path_ext::PathExt,
};

pub struct Column {
    name: String,
    rust_name: Ident,

    data_type: String,
    rust_type_not_null: Path,
    rust_type_nullable: Path,
    rust_type_auto: Path,

    not_null: bool,
    primary_key: bool,
    default: Option<TokenStream>,
}

impl Column {
    pub fn rust_name(&self) -> &Ident {
        &self.rust_name
    }
}

impl From<lang::schema::Column> for Column {
    fn from(value: lang::schema::Column) -> Self {
        let meta = CustomMeta::parse_attrs(&value.attrs, MetaLocation::Column)
            .expect("custom meta should be checked earlier");

        let rust_name = match meta.rename {
            Some(rename) => rename.value,
            None => Ident::new(
                &value.name.to_string().to_case(Case::Snake),
                value.name.span(),
            ),
        };

        let data_type = value.data_type;
        let rust_type_not_null = match meta.type_override {
            Some(type_override) => type_override.value.to_call_site(3),
            None => parse_quote! { #data_type },
        };
        let rust_type_nullable: Path = parse_quote! { Option<#data_type> };
        let rust_type_auto: Path = if value.constraints.not_null().is_none()
            && value.constraints.primary_key().is_none()
        {
            rust_type_nullable.clone()
        } else {
            rust_type_not_null.clone()
        };

        Self {
            name: value.name.to_string(),
            rust_name,
            data_type: data_type.name().to_string(),
            rust_type_not_null,
            rust_type_nullable,
            rust_type_auto,
            not_null: value.constraints.not_null().is_some(),
            primary_key: value.constraints.primary_key().is_some(),
            default: value.constraints.default().map(|default| {
                let expr = default.expr();
                expr.to_token_stream()
            }),
        }
    }
}

impl ToTokens for Column {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let rust_name = &self.rust_name;

        let data_type = &self.data_type;
        let rust_type_not_null = &self.rust_type_not_null;
        let rust_type_nullable = &self.rust_type_nullable;
        let rust_type_auto = &self.rust_type_auto;

        let not_null = self.not_null;
        let primary_key = self.primary_key;
        let default = match &self.default {
            Some(default) => quote! { Some(&#default) },
            None => quote! { None },
        };

        quote! {
            pub mod #rust_name {
                pub const COLUMN: ::kosame::schema::Column = ::kosame::schema::Column {
                    name: #name,
                    data_type: #data_type,
                    not_null: #not_null,
                    primary_key: #primary_key,
                    default: #default,
                };
                pub type TypeNotNull = #rust_type_not_null;
                pub type TypeNullable = #rust_type_nullable;
                pub type Type = #rust_type_auto;
            }
        }
        .to_tokens(tokens);
    }
}
