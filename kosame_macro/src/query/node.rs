use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

use super::QueryNodeBody;

pub struct QueryNode {
    name: Ident,
    body: QueryNodeBody,
}

impl QueryNode {
    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn body(&self) -> &QueryNodeBody {
        &self.body
    }
}

impl Parse for QueryNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            body: input.parse()?,
        })
    }
}
