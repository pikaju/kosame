use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Path, Token};

pub struct RecordStruct {
    name: Ident,
    fields: Vec<RecordStructField>,
}

impl RecordStruct {
    pub fn new(name: Ident, fields: Vec<RecordStructField>) -> Self {
        Self { name, fields }
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
            impl From<::postgres::Row> for #name {
                fn from(row: ::postgres::Row) -> Self {
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
                #name: {
                    let (field, length) = ::kosame::pg::internal::record_field_from_sql(&reader)?;
                    reader = &reader[length..];
                    field
                }
            }
        });

        quote! {
            impl<'a> ::kosame::pg::internal::FromSql<'a> for #name {
                fn accepts(ty: &::kosame::pg::internal::Type) -> bool {
                    ty.name() == "_record"
                }

                fn from_sql(
                    ty: &::kosame::pg::internal::Type,
                    raw: &[u8],
                ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
                    let mut reader = raw;
                    let column_count = ::kosame::pg::internal::int4_from_sql(&reader[..4])?;
                    reader = &reader[4..];
                    assert_eq!(column_count, #field_count);

                    Ok(Self {
                        #(#fields),*
                    })
                }
            }
        }
        .to_tokens(tokens);
    }
}

impl ToTokens for RecordStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let fields = &self.fields;

        let derives = [
            quote! { Debug },
            #[cfg(feature = "serde-serialize")]
            quote! { ::serde::Serialize },
            #[cfg(feature = "serde-deserialize")]
            quote! { ::serde::Deserialize },
        ];

        quote! {
            #[derive(#(#derives),*)]
            pub struct #name {
                #(pub #fields,)*
            }
        }
        .to_tokens(tokens);

        self.to_from_row_impl(tokens);
        self.to_from_sql_impl(tokens);
    }
}

pub struct RecordStructField {
    name: Ident,
    r#type: TokenStream,
}

impl RecordStructField {
    pub fn new(name: Ident, r#type: TokenStream) -> Self {
        Self { name, r#type }
    }
}

impl ToTokens for RecordStructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.name.to_tokens(tokens);
        syn::token::Colon::default().to_tokens(tokens);
        self.r#type.to_tokens(tokens);
    }
}
