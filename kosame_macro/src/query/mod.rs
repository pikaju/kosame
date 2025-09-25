mod field;
mod limit;
mod node;
mod node_path;
mod star;

use field::QueryField;
use node::QueryNode;
use node_path::QueryNodePath;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Ident,
    parse::{Parse, ParseStream},
};

use crate::{alias::Alias, path_ext::PathExt, slotted_sql::SlottedSqlBuilder};

pub struct Query {
    attrs: Vec<Attribute>,
    table: syn::Path,
    body: QueryNode,
    alias: Option<Alias>,
}

impl Parse for Query {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            table: input.parse()?,
            body: input.parse()?,
            alias: input.call(Alias::parse_optional)?,
        })
    }
}

impl ToTokens for Query {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let module_name = self
            .alias
            .as_ref()
            .map_or(quote! { internal }, |alias| alias.ident().to_token_stream());

        let node_tokens = {
            let mut tokens = proc_macro2::TokenStream::new();
            self.body
                .to_tokens(&mut tokens, self, &self.table, &QueryNodePath::new());
            tokens
        };

        let sql_tokens = {
            let mut tokens = TokenStream::new();
            self.body
                .to_query_node(&mut tokens, &self.table, QueryNodePath::new(), None);
            quote! {
                #tokens.to_sql_string(None)
            }
        };

        quote! {
                mod #module_name {
                    #node_tokens

                    pub struct Query {
                    }

                    impl Query {
                        pub fn to_sql_string(&self) -> String {
                            #sql_tokens
                        }
                    }
                }
        }
        .to_tokens(tokens);
    }
}
