pub(crate) mod column;
pub(crate) mod data_type;
pub(crate) mod docs;
pub(crate) mod keywords;
pub(crate) mod query;
pub(crate) mod relation;
pub(crate) mod slotted_sql;
pub(crate) mod table;

use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn table(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as table::Table);
    quote! { #input }.into()
}

#[proc_macro]
pub fn query(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as query::Query);
    quote! { #input }.into()
}
