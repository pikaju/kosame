use quote::{ToTokens, quote};
use syn::{
    Ident, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct Relation {
    name: Ident,
    _colon: Token![:],
    _source_paren: syn::token::Paren,
    source_columns: Punctuated<Ident, Token![,]>,
    arrow: Arrow,
    dest_table: syn::Path,
    _dest_paren: syn::token::Paren,
    dest_columns: Punctuated<Ident, Token![,]>,
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
        Ok(Self {
            name: input.parse()?,
            _colon: input.parse()?,
            _source_paren: parenthesized!(source_content in input),
            source_columns: source_content.parse_terminated(Ident::parse, Token![,])?,
            arrow: input.parse()?,
            dest_table: input.parse()?,
            _dest_paren: parenthesized!(dest_content in input),
            dest_columns: dest_content.parse_terminated(Ident::parse, Token![,])?,
        })
    }
}

impl ToTokens for Relation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let name_string = name.to_string();

        let target = &self.dest_table;
        let target_path = quote! { super::super::super::#target};

        let source_columns = self.source_columns.iter();
        let dest_columns = self.dest_columns.iter();

        quote! {
            /// kosame relation
            pub mod #name {
                pub const NAME: &str = #name_string;

                pub mod source_columns {
                    #(pub use super::super::super::columns::#source_columns;)*
                }

                pub mod target_table {
                    pub use super::#target_path::*;
                }

                pub mod target_columns {
                    #(pub use super::target_table::columns::#dest_columns;)*
                }
            }
        }
        .to_tokens(tokens);
    }
}

enum Arrow {
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
