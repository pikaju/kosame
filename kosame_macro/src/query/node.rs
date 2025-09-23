use super::*;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Path, Token, braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct QueryNode {
    _brace: syn::token::Brace,
    fields: Punctuated<QueryField, Token![,]>,
}

impl QueryNode {
    pub fn fields(&self) -> &Punctuated<QueryField, Token![,]> {
        &self.fields
    }

    pub fn to_autocomplete_module(
        &self,
        module_name: impl ToTokens,
        table_path: &Path,
    ) -> TokenStream {
        let table_path = table_path.to_call_site(2);
        let mut module_rows = vec![];

        for field in self.fields.iter() {
            let name = match field {
                QueryField::Column { name } => name,
                QueryField::Relation { name, .. } => name,
            };
            module_rows.push(quote! {
                use #table_path::columns_and_relations::#name;
            });
        }

        quote! {
            mod #module_name {
                #(#module_rows)*
            }
        }
    }

    pub fn to_struct_definition(
        &self,
        struct_name: impl ToTokens,
        table_path: &Path,
        relation_path: &RelationPath,
    ) -> TokenStream {
        let table_path = table_path.to_call_site(1);
        let mut struct_fields = vec![];

        for field in self.fields.iter() {
            let tokens = match field {
                QueryField::Column { name } => {
                    quote! {
                        #name: #table_path::columns::#name::Type
                    }
                }
                QueryField::Relation { name, .. } => {
                    let mut relation_path = relation_path.clone();
                    relation_path.append(name.clone());
                    let inner_type = relation_path.to_struct_name("Row");
                    quote! {
                        #name: #table_path::relations::#name::Wrapper<#inner_type>
                    }
                }
            };
            struct_fields.push(tokens);
        }

        quote! {
            #[derive(Default, Debug)]
            pub struct #struct_name {
                #(pub #struct_fields,)*
            }
        }
    }

    pub fn to_from_row_impl(&self, struct_name: impl ToTokens) -> TokenStream {
        let fields = self.fields.iter().enumerate().map(|(index, field)| {
            let name = field.name();
            quote! {
                #name: row.get(#index)
            }
        });

        quote! {
            impl From<::postgres::Row> for #struct_name {
                fn from(row: ::postgres::Row) -> Self {
                    Self {
                        #(#fields),*
                    }
                }
            }
        }
    }
}

impl Parse for QueryNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _brace: braced!(content in input),
            fields: content.parse_terminated(QueryField::parse, Token![,])?,
        })
    }
}
