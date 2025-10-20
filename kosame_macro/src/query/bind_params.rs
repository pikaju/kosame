use crate::expr;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

pub struct BindParamsBuilder<'a> {
    params: Vec<&'a Ident>,
}

impl BindParamsBuilder<'_> {
    pub fn new() -> Self {
        Self { params: Vec::new() }
    }
}

impl<'a> BindParamsBuilder<'a> {
    pub fn build(self) -> BindParams<'a> {
        BindParams::new(self.params)
    }
}

impl<'a> expr::Visitor<'a> for BindParamsBuilder<'a> {
    fn visit_bind_param(&mut self, bind_param: &'a expr::BindParam) {
        if !self.params.contains(&bind_param.name()) {
            self.params.push(bind_param.name());
        }
    }
}

pub struct BindParams<'a> {
    params: Vec<&'a Ident>,
}

impl<'a> BindParams<'a> {
    fn new(params: Vec<&'a Ident>) -> Self {
        Self { params }
    }

    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    pub fn to_closure_token_stream(&self, module_name: &Ident) -> TokenStream {
        let mut rename_vars = vec![];
        let mut struct_fields = vec![];
        for (ordinal, name) in self.params.iter().enumerate() {
            let renamed = format_ident!("bind_param_{}", ordinal);
            rename_vars.push(quote! { let #renamed = &#name; });
            struct_fields.push(quote! { #name: #renamed });
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
        for (ordinal, name) in self.params.iter().enumerate() {
            let ordinal = ordinal as u32;
            let name_string = name.to_string();
            modules.push(quote! {
                pub(super) mod #name {
                    pub const BIND_PARAM: ::kosame::expr::BindParam = ::kosame::expr::BindParam::new(#name_string, #ordinal);
                }
            });
        }

        let mut fields = vec![];
        for name in &self.params {
            fields.push(quote! {
                #name: &'a (dyn ::kosame::driver::postgres_types::ToSql + ::std::marker::Sync)
            });
        }
        let fields_len = fields.len();
        let field_names = &self.params;

        let lifetime = (fields_len > 0).then(|| quote! { <'a> });

        quote! {
            mod params {
                #(#modules)*
            }

            #[derive(Debug, Clone)]
            pub struct Params #lifetime {
                #(pub #fields),*
            }
        }
        .to_tokens(tokens);

        #[cfg(feature = "postgres-types")]
        quote! {
            impl<'a> ::kosame::params::Params<Vec<&'a (dyn ::kosame::driver::postgres_types::ToSql + ::std::marker::Sync + 'a)>> for Params #lifetime {
                fn to_driver(&self) -> Vec<&'a (dyn ::kosame::driver::postgres_types::ToSql + ::std::marker::Sync + 'a)> {
                    vec![#(self.#field_names),*]
                }
            }
        }.to_tokens(tokens);
    }
}
