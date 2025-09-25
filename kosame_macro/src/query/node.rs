use crate::row_struct::RowStruct;

use super::limit::LimitClause;
use super::star::Star;
use super::*;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Path, PathSegment, Token, braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct QueryNode {
    _brace: syn::token::Brace,
    star: Option<Star>,
    fields: Punctuated<QueryField, Token![,]>,
    limit: Option<LimitClause>,
}

impl QueryNode {
    pub fn to_tokens(
        &self,
        tokens: &mut TokenStream,
        query: &Query,
        table_path: &Path,
        node_path: &QueryNodePath,
    ) {
        tokens.extend(
            self.to_autocomplete_module(node_path.to_module_name("autocomplete_row"), table_path),
        );

        let row_struct = {
            let table_path = table_path.to_call_site(1);

            let star_field = self
                .star
                .iter()
                .map(|star| star.to_row_struct_field(&table_path));

            RowStruct::new(
                query.attrs.clone(),
                node_path.to_struct_name("Row"),
                star_field
                    .chain(
                        self.fields
                            .iter()
                            .map(|field| field.to_row_struct_field(&table_path, node_path)),
                    )
                    .collect(),
            )
        };

        row_struct.to_tokens(tokens);

        // Recursively call to_tokens on child nodes.
        for field in &self.fields {
            if let QueryField::Relation { name, node, .. } = field {
                let mut node_path = node_path.clone();
                node_path.append(name.clone());

                let mut table_path = table_path.clone();
                table_path
                    .segments
                    .push(Ident::new("relations", Span::call_site()).into());
                table_path.segments.push(PathSegment::from(name.clone()));
                table_path
                    .segments
                    .push(Ident::new("target_table", Span::call_site()).into());

                node.to_tokens(tokens, query, &table_path, &node_path);
            }
        }
    }

    fn to_autocomplete_module(&self, module_name: impl ToTokens, table_path: &Path) -> TokenStream {
        let table_path = table_path.to_call_site(2);
        let mut module_rows = vec![];

        for field in self.fields.iter() {
            let name = match field {
                QueryField::Column { name, .. } => name,
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

    pub fn to_sql_select(
        &self,
        builder: &mut SlottedSqlBuilder,
        table_path: &Path,
        node_path: QueryNodePath,
        join_condition: Option<&Path>,
    ) {
        let table_path_call_site = table_path.to_call_site(1);

        builder.append_str("select ");

        if !node_path.is_empty() {
            builder.append_str("array_agg(row(");
        }

        if self.star.is_some() {
            builder.append_str("row(");
            builder.append_slot(quote! { #table_path_call_site::ALL_FIELDS });
            builder.append_str(")");
            if !self.fields.is_empty() {
                builder.append_str(", ");
            }
        }

        for (index, field) in self.fields.iter().enumerate() {
            match field {
                QueryField::Column { name, .. } => {
                    builder.append_str(&name.to_string());
                    // For renamed columns:
                    // builder.append_slot(quote! {
                    //     #table_path_call_site::columns::#name::NAME
                    // });
                }
                QueryField::Relation { name, node, .. } => {
                    let mut node_path = node_path.clone();
                    node_path.append(name.clone());

                    let mut relation_path = table_path.clone();
                    relation_path
                        .segments
                        .push(Ident::new("relations", Span::call_site()).into());
                    relation_path.segments.push(PathSegment::from(name.clone()));

                    let mut table_path = relation_path.clone();
                    table_path
                        .segments
                        .push(Ident::new("target_table", Span::call_site()).into());

                    builder.append_str("(");
                    node.to_sql_select(builder, &table_path, node_path, Some(&relation_path));
                    builder.append_str(")");
                }
            }

            if index != self.fields.len() - 1 {
                builder.append_str(", ");
            }
        }

        if !node_path.is_empty() {
            builder.append_str("))");
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

        if let Some(join_condition) = join_condition {
            let path = join_condition.to_call_site(1);
            builder.append_str(" where ");
            builder.append_slot(quote! { #path::JOIN_CONDITION });
        }

        if let Some(limit) = &self.limit {
            builder.append_str(" limit ");
            builder.append_str(limit.by().to_sql_string());
        }
    }
}

impl Parse for QueryNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _brace = braced!(content in input);

        let star = if content.fork().parse::<Star>().is_ok() {
            let star = Some(content.parse()?);
            if !content.is_empty() {
                let _: Token![,] = content.parse()?;
            }
            star
        } else {
            None
        };

        let mut fields = Punctuated::<QueryField, _>::new();
        while !content.is_empty() {
            if LimitClause::peek(&content) {
                break;
            }

            fields.push(content.parse()?);

            if !content.peek(Token![,]) {
                break;
            }
            fields.push_punct(content.parse()?);
        }

        let mut existing = vec![];
        for field in &fields {
            let name = field.name();

            if field.is_column() && star.is_some() {
                return Err(syn::Error::new(
                    field.span(),
                    "only relation fields are allowed after `*`",
                ));
            }

            let name_string = field
                .alias()
                .map(|alias| alias.ident())
                .unwrap_or(name)
                .to_string();
            if existing.contains(&name_string) {
                return Err(syn::Error::new(
                    field.span(),
                    format!("duplicate field `{}`", name_string),
                ));
            }
            existing.push(name_string);
        }

        Ok(Self {
            _brace,
            star,
            fields,
            limit: content.call(LimitClause::parse_optional)?,
        })
    }
}
