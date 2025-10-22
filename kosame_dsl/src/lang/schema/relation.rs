use proc_macro_error::emit_error;
use syn::{
    Ident, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
};

pub struct Relation {
    pub name: Ident,
    pub _colon: Token![:],
    pub _source_paren: syn::token::Paren,
    pub source_columns: Punctuated<Ident, Token![,]>,
    pub arrow: Arrow,
    pub target_table: syn::Path,
    pub _target_paren: syn::token::Paren,
    pub target_columns: Punctuated<Ident, Token![,]>,
}

impl Relation {
    pub fn name(&self) -> &Ident {
        &self.name
    }
}

impl Parse for Relation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let source_content;
        let dest_content;
        let result = Self {
            name: input.parse()?,
            _colon: input.parse()?,
            _source_paren: parenthesized!(source_content in input),
            source_columns: source_content.parse_terminated(Ident::parse, Token![,])?,
            arrow: input.parse()?,
            target_table: input.parse()?,
            _target_paren: parenthesized!(dest_content in input),
            target_columns: dest_content.parse_terminated(Ident::parse, Token![,])?,
        };

        if result.source_columns.is_empty() {
            emit_error!(
                result._source_paren.span.span(),
                "at least one column must be specified for relation `{}`",
                result.name
            );
        }
        if result.source_columns.len() != result.target_columns.len() {
            emit_error!(
                result._target_paren.span.span(),
                "number of columns must match on both side of the relation `{}`",
                result.name
            );
        }

        Ok(result)
    }
}

#[allow(unused)]
pub enum Arrow {
    ManyToOne(Token![=>]),
    OneToMany(Token![<=]),
}

impl Parse for Arrow {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![=>]) {
            Ok(Self::ManyToOne(input.parse()?))
        } else if lookahead.peek(Token![<=]) {
            Ok(Self::OneToMany(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}
