use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Path};

pub struct Column {
    name: String,
    data_type: String,
    rust_name: Ident,
    rust_type_not_null: Path,
    rust_type_nullable: Path,
    rust_type_auto: Path,
}

impl Column {
    pub fn rust_name(&self) -> &Ident {
        &self.rust_name
    }
}

#[cfg(feature = "dsl")]
impl From<crate::dsl::schema::Column> for Column {
    fn from(value: crate::dsl::schema::Column) -> Self {
        use convert_case::{Case, Casing};
        use syn::parse_quote;

        use crate::dsl::path_ext::PathExt;

        let rust_name = match value.attrs.rename() {
            Some(name) => Ident::new(&name.value(), name.span()),
            None => Ident::new(
                &value.name.to_string().to_case(Case::Snake),
                value.name.span(),
            ),
        };

        let data_type = value.data_type;
        let rust_type_not_null = match value.attrs.type_override() {
            Some(path) => path.to_call_site(3),
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
            data_type: data_type.name().to_string(),
            rust_name,
            rust_type_not_null,
            rust_type_nullable,
            rust_type_auto,
        }
    }
}

impl ToTokens for Column {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let data_type = &self.data_type;
        let rust_name = &self.rust_name;

        let rust_type_not_null = &self.rust_type_not_null;
        let rust_type_nullable = &self.rust_type_nullable;
        let rust_type_auto = &self.rust_type_auto;

        quote! {
            pub mod #rust_name {
                pub const COLUMN: ::kosame::schema::Column = ::kosame::schema::Column {
                    name: #name,
                    data_type: #data_type,
                };
                pub type TypeNotNull = #rust_type_not_null;
                pub type TypeNullable = #rust_type_nullable;
                pub type Type = #rust_type_auto;
            }
        }
        .to_tokens(tokens);
    }
}
