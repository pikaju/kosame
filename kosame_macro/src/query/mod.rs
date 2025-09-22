mod field;
mod node;
mod relation_path;

use convert_case::Casing;
use field::QueryField;
use node::QueryNode;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use relation_path::RelationPath;
use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
};

use crate::{keywords::AsIdent, slotted_sql::SlottedSqlBuilder};

pub struct Query {
    table: syn::Path,
    body: QueryNode,
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

        let mut recurse_tokens = proc_macro2::TokenStream::new();
        let mut slotted_sql_builder = SlottedSqlBuilder::new();

        fn recurse(
            tokens: &mut proc_macro2::TokenStream,
            slotted_sql_builder: &mut SlottedSqlBuilder,
            table: &syn::Path,
            relation_path: RelationPath,
            node: &QueryNode,
        ) {
            let mut relation_path_tokens = proc_macro2::TokenStream::new();
            for field in relation_path.segments() {
                Token![::](Span::call_site()).to_tokens(&mut relation_path_tokens);
                Ident::new("relations", Span::call_site()).to_tokens(&mut relation_path_tokens);
                Token![::](Span::call_site()).to_tokens(&mut relation_path_tokens);
                field.to_tokens(&mut relation_path_tokens);
                Token![::](Span::call_site()).to_tokens(&mut relation_path_tokens);
                Ident::new("target_table", Span::call_site()).to_tokens(&mut relation_path_tokens);
            }

            let struct_name = relation_path.to_struct_name("Row");
            let mut struct_fields = vec![];

            let internal_module_name = relation_path.to_module_name("row");
            let mut internal_module_rows = vec![];

            slotted_sql_builder.append_str("select ");

            for (index, field) in node.fields().iter().enumerate() {
                let mut struct_field_tokens = proc_macro2::TokenStream::new();
                let mut internal_module_row_tokens = proc_macro2::TokenStream::new();

                match field {
                    QueryField::Column { name } => {
                        let column_module = quote! {
                            super::#table #relation_path_tokens::columns::#name
                        };
                        quote! {
                            #name: #column_module::Type
                        }
                        .to_tokens(&mut struct_field_tokens);

                        quote! {
                            use super::super::#table #relation_path_tokens::columns_and_relations::#name;
                        }
                        .to_tokens(&mut internal_module_row_tokens);

                        slotted_sql_builder.append_slot(quote! { #column_module::NAME });
                    }
                    QueryField::Relation { name, .. } => {
                        let mut relation_path = relation_path.clone();
                        relation_path.append(name.clone());
                        let inner_type = relation_path.to_struct_name("Row");
                        let wrapper_type = quote! {
                            super::#table #relation_path_tokens::relations::#name::Wrapper
                        };

                        quote! {
                            #name: #wrapper_type<#inner_type>
                        }
                        .to_tokens(&mut struct_field_tokens);

                        quote! {
                            use super::super::#table #relation_path_tokens::relations::#name;
                        }
                        .to_tokens(&mut internal_module_row_tokens);
                    }
                }

                struct_fields.push(struct_field_tokens);
                internal_module_rows.push(internal_module_row_tokens);

                if index < node.fields().len() - 1 {
                    slotted_sql_builder.append_str(", ");
                }
            }

            let root_impl = relation_path
                .is_empty()
                .then(|| node.to_from_row_impl(&struct_name));

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

            for field in node.fields().iter() {
                if let QueryField::Relation { name, node } = field {
                    let mut relation_path = relation_path.clone();
                    relation_path.append(name.clone());
                    recurse(tokens, slotted_sql_builder, table, relation_path, node);
                }
            }
        }

        recurse(
            &mut recurse_tokens,
            &mut slotted_sql_builder,
            table,
            RelationPath::new(),
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
                        }
                    }
                }
        }
        .to_tokens(tokens);
    }
}
