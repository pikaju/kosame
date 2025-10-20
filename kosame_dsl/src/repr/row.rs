use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Attribute, Ident};

pub struct RowStruct {
    attrs: Vec<Attribute>,
    name: Ident,
    fields: Vec<RowStructField>,
}

impl RowStruct {
    pub fn new(attrs: Vec<Attribute>, name: Ident, fields: Vec<RowStructField>) -> Self {
        Self {
            attrs,
            name,
            fields,
        }
    }
}

impl ToTokens for RowStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let attrs = &self.attrs;
        let name = &self.name;
        let fields = &self.fields;

        let derives = [
            quote! { ::kosame::Row },
            quote! { Debug },
            #[cfg(feature = "serde")]
            quote! { ::serde::Serialize },
            #[cfg(feature = "serde-full")]
            quote! { ::serde::Deserialize },
        ];

        quote! {
            #[derive(#(#derives),*)]
            #(#attrs)*
            pub struct #name {
                #(#fields,)*
            }
        }
        .to_tokens(tokens);
    }
}

pub struct RowStructField {
    attrs: Vec<Attribute>,
    name: Ident,
    r#type: TokenStream,
}

impl RowStructField {
    pub fn new(attrs: Vec<Attribute>, name: Ident, r#type: TokenStream) -> Self {
        Self {
            attrs,
            name,
            r#type,
        }
    }
}

impl ToTokens for RowStructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for attribute in &self.attrs {
            attribute.to_tokens(tokens);
        }
        syn::token::Pub::default().to_tokens(tokens);
        self.name.to_tokens(tokens);
        syn::token::Colon::default().to_tokens(tokens);
        self.r#type.to_tokens(tokens);
    }
}
