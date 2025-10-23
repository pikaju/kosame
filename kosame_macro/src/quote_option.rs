use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub struct QuoteOption<T>(pub Option<T>);

impl<T> ToTokens for QuoteOption<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.0 {
            Some(inner) => quote! { ::std::option::Option::Some(#inner) },
            None => quote! { ::std::option::Option::None },
        }
        .to_tokens(tokens);
    }
}
