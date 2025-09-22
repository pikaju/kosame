use convert_case::Casing;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Ident, Token, braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::{keywords::AsIdent, slotted_sql::SlottedSqlBuilder};

pub struct Query {
    table: syn::Path,
    body: QueryNodeBody,
    as_name: Option<AsIdent>,
}

impl Parse for Query {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            table: input.parse()?,
            body: input.parse()?,
            as_name: if input.is_empty() {
                None
            } else {
                Some(input.parse()?)
            },
        })
    }
}

impl ToTokens for Query {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let table = &self.table;
        let body = &self.body;

        fn field_path_to_struct_name(field_path: &[Ident]) -> Ident {
            Ident::new(
                &field_path_to_module_name(field_path)
                    .to_string()
                    .to_case(convert_case::Case::Pascal),
                Span::call_site(),
            )
        }
        fn field_path_to_module_name(field_path: &[Ident]) -> Ident {
            let mut module_name = "row".to_string();
            for segment in field_path {
                module_name += "_";
                module_name += &segment.to_string();
            }
            Ident::new(&module_name, Span::call_site())
        }

        let mut recurse_tokens = proc_macro2::TokenStream::new();
        let mut slotted_sql_builder = SlottedSqlBuilder::new();

        fn recurse(
            tokens: &mut proc_macro2::TokenStream,
            slotted_sql_builder: &mut SlottedSqlBuilder,
            table: &syn::Path,
            field_path: Vec<Ident>,
            body: &QueryNodeBody,
        ) {
            let mut field_path_tokens = proc_macro2::TokenStream::new();
            for field in &field_path {
                Token![::](Span::call_site()).to_tokens(&mut field_path_tokens);
                Ident::new("relations", Span::call_site()).to_tokens(&mut field_path_tokens);
                Token![::](Span::call_site()).to_tokens(&mut field_path_tokens);
                field.to_tokens(&mut field_path_tokens);
                Token![::](Span::call_site()).to_tokens(&mut field_path_tokens);
                Ident::new("target_table", Span::call_site()).to_tokens(&mut field_path_tokens);
            }

            let struct_name = field_path_to_struct_name(&field_path);
            let mut struct_fields = vec![];

            let internal_module_name = field_path_to_module_name(&field_path);
            let mut internal_module_rows = vec![];

            slotted_sql_builder.append_str("select ");

            for (index, field) in body.fields.iter().enumerate() {
                let mut struct_field_tokens = proc_macro2::TokenStream::new();
                let mut internal_module_row_tokens = proc_macro2::TokenStream::new();

                match field {
                    QueryField::Column(column) => {
                        let column_module = quote! {
                            super::#table #field_path_tokens::columns::#column
                        };
                        quote! {
                            #column: #column_module::Type
                        }
                        .to_tokens(&mut struct_field_tokens);

                        quote! {
                            use super::super::#table #field_path_tokens::columns_and_relations::#column;
                        }
                        .to_tokens(&mut internal_module_row_tokens);

                        slotted_sql_builder.append_slot(quote! { #column_module::NAME });
                    }
                    QueryField::Relation(relation) => {
                        let mut field_path = field_path.clone();
                        field_path.push(relation.name.clone());
                        let name = &relation.name;
                        let inner_type = field_path_to_struct_name(&field_path);
                        let wrapper_type = quote! {
                            super::#table #field_path_tokens::relations::#name::Wrapper
                        };

                        quote! {
                            #name: #wrapper_type<#inner_type>
                        }
                        .to_tokens(&mut struct_field_tokens);

                        quote! {
                            use super::super::#table #field_path_tokens::relations::#name;
                        }
                        .to_tokens(&mut internal_module_row_tokens);
                    }
                }

                struct_fields.push(struct_field_tokens);
                internal_module_rows.push(internal_module_row_tokens);

                if index < body.fields.len() - 1 {
                    slotted_sql_builder.append_str(", ");
                }
            }

            let root_impl = if field_path.is_empty() {
                let fields = body.fields.iter().enumerate().map(|(index, field)| {
                    let name = match field {
                        QueryField::Column(name) => name,
                        QueryField::Relation(node) => &node.name,
                    };
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
            } else {
                quote! {}
            };

            quote! {
                mod #internal_module_name {
                    #(#internal_module_rows)*
                }

                #[derive(Default, Debug)]
                pub struct #struct_name {
                    #(pub #struct_fields,)*
                }

                #root_impl
            }
            .to_tokens(tokens);

            for field in body.fields.iter() {
                if let QueryField::Relation(relation) = field {
                    let field_path = field_path
                        .iter()
                        .cloned()
                        .chain(std::iter::once(relation.name.clone()))
                        .collect::<Vec<_>>();

                    recurse(
                        tokens,
                        slotted_sql_builder,
                        table,
                        field_path,
                        &relation.body,
                    );
                }
            }
        }

        recurse(
            &mut recurse_tokens,
            &mut slotted_sql_builder,
            table,
            vec![],
            body,
        );

        let module_name = self
            .as_name
            .as_ref()
            .map_or(quote! { internal }, |as_name| {
                as_name.ident().to_token_stream()
            });
        let sql_tokens = slotted_sql_builder.build();

        quote! {
                mod #module_name {
                    #recurse_tokens

                    pub struct Query {
                    }

                    impl Query {
                        pub fn sql_string(&self) -> String {
                            #sql_tokens
                            // format!(#query_string, #(#field_paths::NAME),*, super::#table::NAME)
                        }
                    }
                }
        }
        .to_tokens(tokens);
    }
}

pub struct QueryNode {
    name: Ident,
    body: QueryNodeBody,
}

impl Parse for QueryNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            body: input.parse()?,
        })
    }
}

pub struct QueryNodeBody {
    _brace: syn::token::Brace,
    fields: Punctuated<QueryField, Token![,]>,
}

impl Parse for QueryNodeBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _brace: braced!(content in input),
            fields: content.parse_terminated(QueryField::parse, Token![,])?,
        })
    }
}

pub enum QueryField {
    Column(Ident),
    Relation(QueryNode),
}

impl Parse for QueryField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Brace) {
            Ok(Self::Relation(input.parse()?))
        } else {
            Ok(Self::Column(input.parse()?))
        }
    }
}
