use crate::dsl::attribute::{CustomMeta, MetaLocation};

use super::{column_constraint::ColumnConstraints, data_type::DataType};
use syn::{
    Attribute, Ident,
    parse::{Parse, ParseStream},
};

pub struct Column {
    pub attrs: Vec<Attribute>,
    pub name: Ident,
    pub data_type: DataType,
    pub constraints: ColumnConstraints,
}

impl Parse for Column {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = Attribute::parse_outer(input)?;
        CustomMeta::parse_attrs(&attrs, MetaLocation::Column)?;
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
