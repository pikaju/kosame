pub(crate) mod alias;
pub(crate) mod docs;
pub(crate) mod expr;
pub(crate) mod path_ext;
pub(crate) mod query;
pub(crate) mod row_struct;
pub(crate) mod schema;
pub(crate) mod type_override;

use proc_macro_error::proc_macro_error;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_error]
#[proc_macro]
pub fn table(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as schema::table::Table);
    quote! { #input }.into()
}

#[proc_macro_error]
#[proc_macro]
pub fn query(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as query::Query);
    quote! { #input }.into()
}

#[proc_macro_error]
#[proc_macro_derive(Row)]
pub fn derive_row(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let name = &input.ident;
    let syn::Data::Struct(data) = input.data else {
        proc_macro_error::abort_call_site!("#[derive(Row)] can only be used on structs.");
    };

    let mut tokens = proc_macro2::TokenStream::new();

    #[cfg(feature = "postgres")]
    {
        let fields = data.fields.iter().enumerate().map(|(index, field)| {
            let name = &field.ident;
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
        .to_tokens(&mut tokens);
    }

    #[cfg(feature = "postgres")]
    {
        let field_count = data.fields.len() as i32;
        let fields = data.fields.iter().map(|field| {
            let name = &field.ident;
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
        .to_tokens(&mut tokens);
    }

    tokens.into()
}
