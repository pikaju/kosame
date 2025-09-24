pub(crate) mod as_ident;
pub(crate) mod docs;
pub(crate) mod path_ext;
pub(crate) mod query;
pub(crate) mod row_struct;
pub(crate) mod schema;
pub(crate) mod slotted_sql;

use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_error]
#[proc_macro]
pub fn table(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as schema::table::Table);
    quote! { #input }.into()
}

#[proc_macro_error]
#[proc_macro]
pub fn query(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as query::Query);
    quote! { #input }.into()
}
