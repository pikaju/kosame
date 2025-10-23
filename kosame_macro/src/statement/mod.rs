use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Attribute, Ident,
    parse::{Parse, ParseStream},
};

use crate::{command::Command, row::Row};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(select);
}

pub struct Statement {
    _inner_attrs: Vec<Attribute>,
    command: Command,
}

impl Parse for Statement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _inner_attrs: input.call(Attribute::parse_inner)?,
            command: input.parse()?,
        })
    }
}

impl ToTokens for Statement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let command = &self.command;
        let fields = command.fields();
        let row = Row::new(
            command.attrs().to_owned(),
            Ident::new("Row", Span::call_site()),
            fields.iter().map(|field| field.to_row_field()).collect(),
        );
        quote! {
            #row
            #command
        }
        .to_tokens(tokens);
    }
}
