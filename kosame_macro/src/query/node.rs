use crate::clause::peek_clause;
use crate::expr::Visitor;
use crate::quote_option::QuoteOption;
use crate::{
    clause::{Limit, Offset, OrderBy, Where},
    row::Row,
};

use super::star::Star;
use super::*;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Path, PathSegment, Token, braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct Node {
    _brace: syn::token::Brace,
    star: Option<Star>,
    fields: Punctuated<Field, Token![,]>,
    r#where: Option<Where>,
    order_by: Option<OrderBy>,
    limit: Option<Limit>,
    offset: Option<Offset>,
}

impl Node {
    pub fn accept_expr<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        for field in &self.fields {
            match field {
                Field::Relation { node, .. } => node.accept_expr(visitor),
                Field::Expr { expr, .. } => expr.accept(visitor),
                _ => {}
            }
        }

        if let Some(r#where) = &self.r#where {
            r#where.expr().accept(visitor);
        }

        if let Some(order_by) = &self.order_by {
            order_by.accept_expr(visitor);
        }

        if let Some(limit) = &self.limit {
            limit.expr().accept(visitor);
        }

        if let Some(offset) = &self.offset {
            offset.expr().accept(visitor);
        }
    }

    pub fn to_row_tokens(
        &self,
        tokens: &mut TokenStream,
        query: &Query,
        node_path: &QueryNodePath,
    ) {
        let table_path = node_path.resolve(&query.table);
        tokens.extend(self.to_autocomplete_module_tokens(
            node_path.to_module_name("autocomplete_row"),
            &table_path,
        ));

        let row = {
            let table_path = table_path.to_call_site(1);

            let star_field = self
                .star
                .as_ref()
                .and_then(|star| star.alias.is_some().then(|| star.to_row_field(&table_path)));

            Row::new(
                query.outer_attrs.clone(),
                node_path.to_struct_name("Row"),
                star_field
                    .into_iter()
                    .chain(
                        self.fields
                            .iter()
                            .map(|field| field.to_row_field(&table_path, node_path)),
                    )
                    .collect(),
            )
        };

        if let Some(star) = &self.star
            && star.alias.is_none()
        {
            let table_path = table_path.to_call_site(1);
            quote! {
                #table_path::star! {
                    (#table_path)
                    #row
                }
            }
            .to_tokens(tokens);
        } else {
            row.to_tokens(tokens);
        }

        // Recursively call to_tokens on child nodes.
        for field in &self.fields {
            if let Field::Relation { name, node, .. } = field {
                let mut node_path = node_path.clone();
                node_path.append(name.clone());
                node.to_row_tokens(tokens, query, &node_path);
            }
        }
    }

    fn to_autocomplete_module_tokens(
        &self,
        module_name: impl ToTokens,
        table_path: &Path,
    ) -> TokenStream {
        let table_path = table_path.to_call_site(2);
        let mut module_rows = vec![];

        for field in self.fields.iter() {
            let name = match field {
                Field::Column { name, .. } => name,
                Field::Relation { name, .. } => name,
                Field::Expr { .. } => continue,
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

    pub fn to_query_node_tokens(
        &self,
        tokens: &mut TokenStream,
        query: &Query,
        node_path: QueryNodePath,
    ) {
        let table_path = node_path.resolve(&query.table);
        let table_path_call_site = table_path.to_call_site(1);

        let scope_module = {
            let table_path = node_path.resolve(&query.table);
            let table_path_call_site = table_path.to_call_site(2);
            quote! {
                mod scope {
                    pub(super) use super::params;
                    pub(super) use #table_path_call_site::*;
                }
            }
        };

        let mut fields = vec![];
        for field in &self.fields {
            match field {
                Field::Column { name, alias, .. } => {
                    let alias = QuoteOption(alias.as_ref().map(|alias| alias.ident().to_string()));
                    fields.push(quote! {
                        ::kosame::repr::query::Field::Column {
                            column: &#table_path_call_site::columns::#name::COLUMN,
                            alias: #alias
                        }
                    });
                }
                Field::Relation {
                    name, node, alias, ..
                } => {
                    let alias = QuoteOption(alias.as_ref().map(|alias| alias.ident().to_string()));

                    let node_path = node_path.clone().appended(name.clone());

                    let mut relation_path = table_path.clone();
                    relation_path
                        .segments
                        .push(Ident::new("relations", Span::call_site()).into());
                    relation_path.segments.push(PathSegment::from(name.clone()));

                    let mut tokens = TokenStream::new();
                    node.to_query_node_tokens(&mut tokens, query, node_path);

                    let relation_path = relation_path.to_call_site(1);

                    fields.push(quote! {
                        ::kosame::repr::query::Field::Relation {
                            relation: &#relation_path::RELATION,
                            node: #tokens,
                            alias: #alias
                        }
                    });
                }
                Field::Expr { expr, alias, .. } => {
                    let alias = alias.ident().to_string();

                    fields.push(quote! {
                        {
                            #scope_module
                            ::kosame::repr::query::Field::Expr {
                                expr: #expr,
                                alias: #alias
                            }
                        }
                    });
                }
            }
        }

        let star = self.star.is_some();

        let r#where = QuoteOption(self.r#where.as_ref());
        let order_by = QuoteOption(self.order_by.as_ref());
        let limit = QuoteOption(self.limit.as_ref());
        let offset = QuoteOption(self.offset.as_ref());

        quote! {
            {
                #scope_module
                ::kosame::repr::query::Node::new(
                    &#table_path_call_site::TABLE,
                    #star,
                    &[#(#fields),*],
                    #r#where,
                    #order_by,
                    #limit,
                    #offset,
                )
            }
        }
        .to_tokens(tokens);
    }
}

impl Parse for Node {
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

        let mut fields = Punctuated::<Field, _>::new();
        while !content.is_empty() {
            if peek_clause(input) {
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
                    "column references are not allowed after `*`",
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
            r#where: content.call(Where::parse_optional)?,
            order_by: content.call(OrderBy::parse_optional)?,
            limit: content.call(Limit::parse_optional)?,
            offset: content.call(Offset::parse_optional)?,
        })
    }
}
