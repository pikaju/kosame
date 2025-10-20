use crate::dsl::attribute::ParsedAttributes;

use super::{column_constraint::ColumnConstraints, data_type::DataType};
use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

pub struct Column {
    pub attrs: ParsedAttributes,
    pub name: Ident,
    pub data_type: DataType,
    pub constraints: ColumnConstraints,
}

impl Parse for Column {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs: ParsedAttributes = input.parse()?;
        attrs.require_no_global()?;
        let name = input.parse()?;
        let data_type = input.parse()?;

        Ok(Self {
            attrs,
            name,
            data_type,
            constraints: input.parse()?,
        })
    }
}
