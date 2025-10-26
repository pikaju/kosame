use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Attribute, Ident,
    parse::{Parse, ParseStream},
};

use crate::{
    alias::Alias,
    attribute::{CustomMeta, MetaLocation},
    bind_params::BindParamsBuilder,
    command::Command,
    row::Row,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(select);
}

pub struct Statement {
    pub _inner_attrs: Vec<Attribute>,
    pub command: Command,
    pub alias: Option<Alias>,
}

impl Parse for Statement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _inner_attrs: {
                let attrs = input.call(Attribute::parse_inner)?;
                CustomMeta::parse_attrs(&attrs, MetaLocation::StatementInner)?;
                attrs
            },
            command: input.parse()?,
            alias: input.call(Alias::parse_optional)?,
        })
    }
}

impl ToTokens for Statement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let module_name = match &self.alias {
            Some(alias) => &alias.ident,
            None => &Ident::new("internal", Span::call_site()),
        };

        let bind_params = {
            let mut builder = BindParamsBuilder::new();
            self.command.accept(&mut builder);
            builder.build()
        };
        let closure_tokens = self
            .alias
            .is_none()
            .then(|| bind_params.to_closure_token_stream(module_name));

        let command = &self.command;
        let fields = command.fields();
        let row = Row::new(
            command.attrs().to_owned(),
            Ident::new("Row", Span::call_site()),
            fields.iter().map(|field| field.to_row_field()).collect(),
        );

        let lifetime = (!bind_params.is_empty()).then_some(quote! { <'a> });

        let module_tokens = quote! {
            pub mod #module_name {
                #row

                #bind_params

                pub struct Statement #lifetime {
                    params: Params #lifetime,
                }

                impl #lifetime Statement #lifetime {
                    pub fn new(params: Params #lifetime) -> Self { Self { params } }
                }

                impl #lifetime ::kosame::statement::Statement for Statement #lifetime {
                    type Params = Params #lifetime;
                    type Row = Row;

                    const REPR: ::kosame::repr::command::Command<'static> = #command;

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

                    #module_name::Statement::new(closure)
                }
            }
            .to_tokens(tokens);
        }
    }
}
