use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub trait Docs {
    fn docs(&self) -> String;
}

pub trait ToDocsTokens {
    fn to_docs_token_stream(&self) -> TokenStream;
}

impl<T: Docs> ToDocsTokens for T {
    fn to_docs_token_stream(&self) -> TokenStream {
        let docs_string = self.docs();
        quote! {
            #[doc = #docs_string]
        }
        .into_token_stream()
    }
}
