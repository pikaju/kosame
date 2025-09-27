use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Attribute, Ident};

pub struct RowStruct {
    attrs: Vec<Attribute>,
    name: Ident,
    fields: Vec<RowStructField>,
}

impl RowStruct {
    pub fn new(attrs: Vec<Attribute>, name: Ident, fields: Vec<RowStructField>) -> Self {
        Self {
            attrs,
            name,
            fields,
        }
    }

    fn to_from_row_impl(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let fields = self.fields.iter().enumerate().map(|(index, field)| {
            let name = &field.name;
            quote! {
                #name: row.get(#index)
            }
        });

        quote! {
            impl From<&::kosame::postgres::internal::Row> for #name {
                fn from(row: &::kosame::postgres::internal::Row) -> Self {
                    Self {
                        #(#fields),*
                    }
                }
            }
        }
        .to_tokens(tokens);
    }

    fn to_from_sql_impl(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let field_count = self.fields.len() as i32;
        let fields = self.fields.iter().map(|field| {
            let name = &field.name;
            quote! {
                #name: ::kosame::postgres::internal::record_field_from_sql(&raw, &mut offset)?
            }
        });

        quote! {
            impl<'a> ::kosame::postgres::internal::FromSql<'a> for #name {
                fn accepts(ty: &::kosame::postgres::internal::Type) -> bool {
                    ty.name() == "record"
                }

                fn from_sql(
                    ty: &::kosame::postgres::internal::Type,
                    raw: &[u8],
                ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
                    let column_count = ::kosame::postgres::internal::int4_from_sql(&raw[..4])?;
                    assert_eq!(column_count, #field_count);

                    let mut offset = 4;

                    Ok(Self {
                        #(#fields),*
                    })
                }
            }
        }
        .to_tokens(tokens);
    }
}

impl ToTokens for RowStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let fields = &self.fields;

        let attrs = &self.attrs;

        let derives = [
            quote! { Debug },
            #[cfg(feature = "serde-serialize")]
            quote! { ::serde::Serialize },
            #[cfg(feature = "serde-deserialize")]
            quote! { ::serde::Deserialize },
        ];

        quote! {
            #[derive(#(#derives),*)]
            #(#attrs)*
            pub struct #name {
                #(#fields,)*
            }
        }
        .to_tokens(tokens);

        self.to_from_row_impl(tokens);
        self.to_from_sql_impl(tokens);
    }
}

pub struct RowStructField {
    attrs: Vec<Attribute>,
    name: Ident,
    r#type: TokenStream,
}

impl RowStructField {
    pub fn new(attrs: Vec<Attribute>, name: Ident, r#type: TokenStream) -> Self {
        Self {
            attrs,
            name,
            r#type,
        }
    }
}

impl ToTokens for RowStructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for attribute in &self.attrs {
            attribute.to_tokens(tokens);
        }
        syn::token::Pub::default().to_tokens(tokens);
        self.name.to_tokens(tokens);
        syn::token::Colon::default().to_tokens(tokens);
        self.r#type.to_tokens(tokens);
    }
}
