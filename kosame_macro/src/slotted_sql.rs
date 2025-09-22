use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub struct SlottedSqlBuilder {
    segments: Vec<TokenStream>,
    buffer: String,
}

impl SlottedSqlBuilder {
    pub fn new() -> Self {
        Self {
            segments: vec![],
            buffer: String::new(),
        }
    }

    pub fn append_str(&mut self, string: &str) {
        self.buffer += string;
    }

    pub fn append_slot(&mut self, slot: impl ToTokens) {
        self.flush();
        self.segments.push(slot.to_token_stream());
    }

    fn flush(&mut self) {
        if !self.buffer.is_empty() {
            self.segments.push(self.buffer.to_token_stream());
            self.buffer.clear();
        }
    }

    pub fn build(mut self) -> TokenStream {
        self.flush();
        let segments = self.segments;
        quote! {
            ::kosame::concatcp!(#(#segments),*)
        }
    }
}
