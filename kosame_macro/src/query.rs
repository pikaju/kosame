use quote::{ToTokens, quote};
use syn::{
    Ident, Token, braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct Query {
    root: QueryNode,
}

impl Parse for Query {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            root: input.parse()?,
        })
    }
}

impl ToTokens for Query {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let node = &self.root;
        let table = &node.name;

        let column_names = node
            .fields
            .iter()
            .filter_map(|field| match field {
                QueryField::Column(column) => Some(quote! { #column }),
                QueryField::Relation(_) => None,
            })
            .collect::<Vec<_>>();

        let field_names = node
            .fields
            .iter()
            .map(|field| match field {
                QueryField::Column(column) => quote! { #column },
                QueryField::Relation(relation) => {
                    let name = &relation.name;
                    quote! { #name }
                }
            })
            .collect::<Vec<_>>();

        let field_paths = node.fields.iter().map(|field| match field {
            QueryField::Column(column) => quote! {
                #table::columns::#column
            },
            QueryField::Relation(relation) => {
                let name = &relation.name;
                quote! {
                    #table::relations::#name
                }
            }
        });
        let query_string = format!(
            "select {} from {{}}",
            field_names
                .iter()
                .map(|_| "{}".to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );

        quote! {
            {
                mod _internal {
                    use super::#table;
                    #(use super::#table::columns_and_relations::#field_names;)*

                    #[derive(Default)]
                    pub struct Row {
                        #(pub #column_names: super::#table::columns::#column_names::Type,)*
                    }
                }

                let query: String = format!(#query_string, #(#field_paths::NAME),*, #table::NAME);

                let row: _internal::Row = Default::default();
                (row, query)
            }
        }
        .to_tokens(tokens);
    }
}

pub struct QueryNode {
    name: syn::Path,
    brace: syn::token::Brace,
    fields: Punctuated<QueryField, Token![,]>,
}

impl Parse for QueryNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            name: input.parse()?,
            brace: braced!(content in input),
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
