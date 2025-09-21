use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};

mod kw {
    syn::custom_keyword!(int);
    syn::custom_keyword!(text);
}

#[allow(dead_code)]
pub enum DataType {
    Int(kw::int),
    Text(kw::text),
}

impl Parse for DataType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::int) {
            Ok(DataType::Int(input.parse()?))
        } else if lookahead.peek(kw::text) {
            Ok(DataType::Text(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for DataType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            DataType::Int(_) => {
                quote! { i32 }.to_tokens(tokens);
            }
            DataType::Text(_) => {
                quote! { ::std::string::String }.to_tokens(tokens);
            }
        }
    }
}
