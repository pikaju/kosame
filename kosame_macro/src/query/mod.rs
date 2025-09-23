mod field;
mod node;
mod relation_path;

use field::QueryField;
use node::QueryNode;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use relation_path::RelationPath;
use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

use crate::{keywords::AsIdent, path_ext::PathExt, slotted_sql::SlottedSqlBuilder};

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

        let mut node_tokens = proc_macro2::TokenStream::new();
        let mut slotted_sql_builder = SlottedSqlBuilder::new();

        fn recursive_to_tokens(
            node_tokens: &mut proc_macro2::TokenStream,
            slotted_sql_builder: &mut SlottedSqlBuilder,
            table: &syn::Path,
            relation_path: RelationPath,
            node: &QueryNode,
        ) {
            let current_table_path = {
                let mut path = table.clone();
                for field in relation_path.segments() {
                    path.segments
                        .push(Ident::new("relations", Span::call_site()).into());
                    path.segments.push(field.clone().into());
                    path.segments
                        .push(Ident::new("target_table", Span::call_site()).into());
                }
                path
            };

            node.to_tokens(node_tokens, &current_table_path, &relation_path);

            for field in node.fields().iter() {
                if let QueryField::Relation { name, node } = field {
                    let mut relation_path = relation_path.clone();
                    relation_path.append(name.clone());
                    recursive_to_tokens(
                        node_tokens,
                        slotted_sql_builder,
                        table,
                        relation_path,
                        node,
                    );
                }
            }
        }

        recursive_to_tokens(
            &mut node_tokens,
            &mut slotted_sql_builder,
            table,
            RelationPath::new(),
            body,
        );

        self.body
            .to_sql_select(&mut slotted_sql_builder, &self.table, RelationPath::new());

        let module_name = self
            .as_name
            .as_ref()
            .map_or(quote! { internal }, |as_name| {
                as_name.ident().to_token_stream()
            });
        let sql_tokens = slotted_sql_builder.build();

        quote! {
                mod #module_name {
                    #node_tokens

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
