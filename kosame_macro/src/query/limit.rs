use syn::{
    LitInt,
    parse::{Parse, ParseStream},
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(limit);
}

pub struct LimitClause {
    _limit: kw::limit,
    by: LimitBy,
}

impl LimitClause {
    pub fn by(&self) -> &LimitBy {
        &self.by
    }

    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Ok(if input.peek(kw::limit) {
            Some(input.parse()?)
        } else {
            None
        })
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::limit)
    }
}

impl Parse for LimitClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _limit: input.parse()?,
            by: input.parse()?,
        })
    }
}

pub enum LimitBy {
    Literal(LitInt),
}

impl LimitBy {
    pub fn to_sql_string(&self) -> &str {
        match self {
            Self::Literal(lit) => lit.base10_digits(),
        }
    }
}

impl Parse for LimitBy {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self::Literal(input.parse()?))
    }
}
