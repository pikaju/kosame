use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Ident, Token, braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct Query {
    table: syn::Path,
    body: QueryNodeBody,
}

impl Parse for Query {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            table: input.parse()?,
            body: input.parse()?,
        })
    }
}

impl ToTokens for Query {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let table = &self.table;
        let body = &self.body;

        fn field_path_to_struct_name(field_path: &[Ident]) -> Ident {
            let struct_name = field_path
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("");
            Ident::new(&("Row".to_owned() + &struct_name), Span::call_site())
        }

        let mut recurse_tokens = proc_macro2::TokenStream::new();
        fn recurse(
            tokens: &mut proc_macro2::TokenStream,
            table: &syn::Path,
            field_path: Vec<Ident>,
            body: &QueryNodeBody,
        ) {
            for field in body.fields.iter() {
                if let QueryField::Relation(relation) = field {
                    let field_path = field_path
                        .iter()
                        .cloned()
                        .chain(std::iter::once(relation.name.clone()))
                        .collect::<Vec<_>>();

                    recurse(tokens, table, field_path, &relation.body);
                }
            }

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
            let struct_fields = vec![];

            let columns = body
                .fields
                .iter()
                .filter_map(|field| match field {
                    QueryField::Column(column) => Some(column),
                    QueryField::Relation(_) => None,
                })
                .collect::<Vec<_>>();
            let relations = body
                .fields
                .iter()
                .filter_map(|field| match field {
                    QueryField::Column(_) => None,
                    QueryField::Relation(relation) => Some(relation),
                })
                .collect::<Vec<_>>();

            let column_names = columns.iter().map(|column| quote! { #column });
            let column_types = columns.iter().map(|column| {
                quote! {
                    super::#table #field_path_tokens::columns::#column::Type
                }
            });

            let relation_names = relations.iter().map(|relation| {
                let name = &relation.name;
                quote! { #name }
            });
            let relation_types = relations.iter().map(|relation| {
                let mut field_path = field_path.clone();
                field_path.push(relation.name.clone());
                field_path_to_struct_name(&field_path)
            });

            quote! {
                #[derive(Default, Debug)]
                pub struct #struct_name {
                    #(pub #column_names: #column_types,)*
                    #(pub #relation_names: #relation_types,)*
                }
            }
            .to_tokens(tokens);
        }

        recurse(&mut recurse_tokens, table, vec![], body);

        let field_names = body
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

        let field_paths = body.fields.iter().map(|field| match field {
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
                    #recurse_tokens

                    use super::#table;
                    #(use super::#table::columns_and_relations::#field_names;)*
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
