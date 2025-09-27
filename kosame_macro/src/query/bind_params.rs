use std::{cell::RefCell, collections::HashMap};

use crate::expr;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

pub type BindParamOrdinal = u32;

pub struct BindParamsBuilder<'a> {
    next_ordinal: BindParamOrdinal,
    params: HashMap<&'a Ident, BindParamOrdinal>,
}

impl BindParamsBuilder<'_> {
    pub fn new() -> Self {
        Self {
            next_ordinal: 1,
            params: HashMap::new(),
        }
    }
}

impl<'a> BindParamsBuilder<'a> {
    pub fn build(self) -> BindParams<'a> {
        BindParams::new(self.params)
    }
}

impl<'a> expr::Visitor<'a> for BindParamsBuilder<'a> {
    fn visit_bind_param(&mut self, bind_param: &'a expr::BindParam) {
        self.params.entry(bind_param.name()).or_insert_with(|| {
            let ordinal = self.next_ordinal;
            self.next_ordinal += 1;
            ordinal
        });
    }
}

pub struct BindParams<'a> {
    params: HashMap<&'a Ident, BindParamOrdinal>,
}

impl<'a> BindParams<'a> {
    fn new(params: HashMap<&'a Ident, BindParamOrdinal>) -> Self {
        Self { params }
    }

    pub fn to_closure_token_stream(&self, module_name: &Ident) -> TokenStream {
        let mut rename_vars = vec![];
        let mut struct_fields = vec![];
        for (ident, ordinal) in &self.params {
            let renamed = format_ident!("bind_param_{}", ordinal);
            rename_vars.push(quote! { let #renamed = &#ident; });
            struct_fields.push(quote! { #ident: #renamed });
        }

        quote! {
            #(#rename_vars)*

            let closure = #module_name::Params {
                #(#struct_fields,)*
            };
        }
    }
}

impl ToTokens for BindParams<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut modules = vec![];
        for (name, ordinal) in &self.params {
            let name_string = name.to_string();
            modules.push(quote! {
                pub(super) mod #name {
                    pub const BIND_PARAM: ::kosame::query::BindParam = ::kosame::query::BindParam::new(#name_string, #ordinal);
                }
            });
        }

        let mut fields = vec![];
        for name in self.params.keys() {
            fields.push(quote! {
                #name: &'a (dyn ::kosame::pg::internal::ToSql + ::std::marker::Sync)
            });
        }
        let fields_len = fields.len();

        let field_names = self.params.keys();

        quote! {
            mod params {
                #(#modules)*
            }

            pub struct Params<'a> {
                #(pub #fields),*
            }

            impl<'a> Params<'a> {
                pub fn array(&self) -> [&(dyn ::kosame::pg::internal::ToSql + ::std::marker::Sync); #fields_len] {
                    [#(self.#field_names),*]
                }
            }
        }
        .to_tokens(tokens);
    }
}
