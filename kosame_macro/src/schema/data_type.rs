use proc_macro_error::abort;
use quote::{ToTokens, quote};
use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

#[allow(dead_code)]
pub struct DataType {
    name: Ident,
}

impl Parse for DataType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
        })
    }
}

impl ToTokens for DataType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self.name.to_string().as_str() {
            "int" => quote! { i32 },
            "text" => quote! { ::std::string::String },
            _ => {
                abort!(
                    self.name.span(),
                    "cannot determine rust type for unrecognized database type {}, requires type override",
                    self.name,
                );
                quote! { () }
            }
        }
        .to_tokens(tokens);
    }
}
