use std::{cell::RefCell, collections::HashMap};

use crate::expr;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
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

    pub fn ordinal_of(&self, name: &Ident) -> Option<BindParamOrdinal> {
        self.params.get(name).copied()
    }
}

impl ToTokens for BindParams<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for (name, ordinal) in &self.params {
            let name_string = name.to_string();
            quote! {
                pub mod #name {
                    pub const BIND_PARAM: ::kosame::query::BindParam = ::kosame::query::BindParam::new(#name_string, #ordinal);
                }
            }.to_tokens(tokens);
        }
    }
}
