mod bind_params;
mod field;
mod filter;
mod limit;
mod node;
mod node_path;
mod offset;
mod order_by;
mod star;

use field::QueryField;
use filter::Filter;
use limit::Limit;
use node::QueryNode;
use node_path::QueryNodePath;
use offset::Offset;
use order_by::OrderBy;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Attribute, Ident,
    parse::{Parse, ParseStream},
};

use crate::{alias::Alias, path_ext::PathExt, query::bind_params::BindParamsBuilder};

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
        let module_name = match &self.alias {
            Some(alias) => alias.ident(),
            None => &Ident::new("internal", Span::call_site()),
        };

        let bind_params = {
            let mut builder = BindParamsBuilder::new();
            self.body.accept_expr(&mut builder);
            builder.build()
        };
        let closure_tokens = self
            .alias
            .is_none()
            .then(|| bind_params.to_closure_token_stream(module_name));

        let node_tokens = {
            let mut tokens = proc_macro2::TokenStream::new();
            self.body
                .to_row_struct_tokens(&mut tokens, self, &QueryNodePath::new());
            tokens
        };

        let query_node = {
            let mut tokens = TokenStream::new();
            self.body
                .to_query_node_tokens(&mut tokens, self, QueryNodePath::new());
            tokens
        };

        let lifetime = (!bind_params.is_empty()).then_some(quote! { <'a> });

        let module_tokens = quote! {
            mod #module_name {
                #node_tokens

                #bind_params

                pub struct Query #lifetime {
                    params: Params #lifetime,
                }

                impl #lifetime Query #lifetime {
                    pub fn new(params: Params #lifetime) -> Self { Self { params } }
                    pub fn params(&self) -> &Params #lifetime { &self.params }
                    pub fn from_row(&self, row: &::kosame::postgres::internal::Row) -> Row {
                        row.into()
                    }
                }

                impl #lifetime ::kosame::query::Query for Query #lifetime {
                    type Params = Params #lifetime;
                    type Row = Row;

                    const ROOT: ::kosame::query::QueryNode = #query_node;

                    fn params(&self) -> &Self::Params {
                        &self.params
                    }
                }
            }
        };

        if self.alias.is_some() {
            module_tokens.to_tokens(tokens);
        } else {
            quote! {
                {
                    #closure_tokens

                    #module_tokens

                    let query = #module_name::Query::new(closure);
                    query
                }
            }
            .to_tokens(tokens);
        }
    }
}
