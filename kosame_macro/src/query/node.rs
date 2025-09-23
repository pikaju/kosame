use super::*;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Path, PathSegment, Token, braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct QueryNode {
    _brace: syn::token::Brace,
    fields: Punctuated<QueryField, Token![,]>,
}

impl QueryNode {
    pub fn to_tokens(
        &self,
        tokens: &mut TokenStream,
        table_path: &Path,
        relation_path: &RelationPath,
    ) {
        let struct_name = relation_path.to_struct_name("Row");

        tokens.extend(
            self.to_autocomplete_module(
                relation_path.to_module_name("autocomplete_row"),
                table_path,
            ),
        );
        tokens.extend(self.to_struct_definition(&struct_name, table_path, relation_path));

        if relation_path.is_empty() {
            tokens.extend(self.to_from_row_impl(&struct_name));
        } else {
            tokens.extend(self.to_from_sql_impl(&struct_name));
        }

        // Recursively call to_tokens on child nodes.
        for field in &self.fields {
            if let QueryField::Relation { name, node } = field {
                let mut relation_path = relation_path.clone();
                relation_path.append(name.clone());

                let mut table_path = table_path.clone();
                table_path
                    .segments
                    .push(Ident::new("relations", Span::call_site()).into());
                table_path.segments.push(PathSegment::from(name.clone()));
                table_path
                    .segments
                    .push(Ident::new("target_table", Span::call_site()).into());

                node.to_tokens(tokens, &table_path, &relation_path);
            }
        }
    }

    fn to_autocomplete_module(&self, module_name: impl ToTokens, table_path: &Path) -> TokenStream {
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

    fn to_struct_definition(
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

    fn to_from_row_impl(&self, struct_name: impl ToTokens) -> TokenStream {
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

    fn to_from_sql_impl(&self, struct_name: impl ToTokens) -> TokenStream {
        let field_count = self.fields.len() as i32;
        let fields = self.fields.iter().enumerate().map(|(index, field)| {
            let name = field.name();
            quote! {
                #name: {
                    let (field, length) = ::kosame::pg::internal::record_field_from_sql(&reader)?;
                    reader = &reader[length..];
                    field
                }
            }
        });

        quote! {
            impl<'a> ::kosame::pg::internal::FromSql<'a> for #struct_name {
                fn accepts(ty: &::kosame::pg::internal::Type) -> bool {
                    ty.name() == "_record"
                }

                fn from_sql(
                    ty: &::kosame::pg::internal::Type,
                    raw: &[u8],
                ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
                    println!("{:?}", raw);
                    let mut reader = raw;
                    let column_count = ::kosame::pg::internal::int4_from_sql(&reader[..4])?;
                    reader = &reader[4..];
                    assert_eq!(column_count, #field_count);

                    Ok(Self {
                        #(#fields),*
                    })
                }
            }
        }
    }

    pub fn to_sql_select(
        &self,
        builder: &mut SlottedSqlBuilder,
        table_path: &Path,
        relation_path: RelationPath,
    ) {
        let table_path_call_site = table_path.to_call_site(1);

        builder.append_str("select ");

        if !relation_path.is_empty() {
            builder.append_str("row(");
        }

        for (index, field) in self.fields.iter().enumerate() {
            match field {
                QueryField::Column { name } => {
                    builder.append_str(&name.to_string());
                    // For renamed columns:
                    // builder.append_slot(quote! {
                    //     #table_path_call_site::columns::#name::NAME
                    // });
                }
                QueryField::Relation { name, node } => {
                    let mut relation_path = relation_path.clone();
                    relation_path.append(name.clone());

                    let mut table_path = table_path.clone();
                    table_path
                        .segments
                        .push(Ident::new("relations", Span::call_site()).into());
                    table_path.segments.push(PathSegment::from(name.clone()));
                    table_path
                        .segments
                        .push(Ident::new("target_table", Span::call_site()).into());

                    builder.append_str("array[(");
                    node.to_sql_select(builder, &table_path, relation_path);
                    builder.append_str(")]");
                }
            }

            if index != self.fields.len() - 1 {
                builder.append_str(", ");
            }
        }

        if !relation_path.is_empty() {
            builder.append_str(")");
        }

        builder.append_str(" from ");

        // builder.append_str(
        //     &table_path_call_site
        //         .segments
        //         .last()
        //         .unwrap()
        //         .ident
        //         .to_string(),
        // );
        //
        // For renamed tables:
        builder.append_slot(quote! { #table_path_call_site::NAME });
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
