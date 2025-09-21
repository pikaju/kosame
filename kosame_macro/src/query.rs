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
        let table = &node.table;
        let fields = node.fields.iter();
        let fields2 = node.fields.iter();
        let fields3 = node.fields.iter();

        let field_paths = node.fields.iter().map(|field| {
            quote! {
                #table::columns::#field
            }
        });
        let query_string = format!(
            "select {} from {{}}",
            fields3
                .map(|_| "{}".to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );

        quote! {
            {
                mod _internal {
                    use super::#table;
                    #(use super::#table::columns_and_relations::#fields;)*

                    #[derive(Default)]
                    pub struct Row {
                        #(pub #fields2: super::#table::columns::#fields2::Type,)*
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
    table: syn::Path,
    brace: syn::token::Brace,
    fields: Punctuated<Ident, Token![,]>,
}

impl Parse for QueryNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            table: input.parse()?,
            brace: braced!(content in input),
            fields: content.parse_terminated(Ident::parse, Token![,])?,
        })
    }
}
