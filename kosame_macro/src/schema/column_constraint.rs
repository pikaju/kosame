use std::{fmt::Display, ops::Deref};

use syn::{
    Token,
    parse::{Parse, ParseStream},
};

pub struct ColumnConstraints(Vec<ColumnConstraint>);

impl ColumnConstraints {
    pub fn has_not_null(&self) -> bool {
        self.0.iter().any(ColumnConstraint::is_not_null)
    }

    pub fn has_primary_key(&self) -> bool {
        self.0.iter().any(ColumnConstraint::is_primary_key)
    }
}

impl Parse for ColumnConstraints {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut constraints = vec![];
        while !input.is_empty() && !input.peek(Token![,]) {
            constraints.push(input.parse()?);
        }
        Ok(Self(constraints))
    }
}

impl Deref for ColumnConstraints {
    type Target = Vec<ColumnConstraint>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
pub enum ColumnConstraint {
    NotNull(NotNull),
    PrimaryKey(PrimaryKey),
}

impl ColumnConstraint {
    /// Returns `true` if the column constraint is [`NotNull`].
    ///
    /// [`NotNull`]: ColumnConstraint::NotNull
    #[must_use]
    pub fn is_not_null(&self) -> bool {
        matches!(self, Self::NotNull(..))
    }

    /// Returns `true` if the column constraint is [`PrimaryKey`].
    ///
    /// [`PrimaryKey`]: ColumnConstraint::PrimaryKey
    #[must_use]
    pub fn is_primary_key(&self) -> bool {
        matches!(self, Self::PrimaryKey(..))
    }
}

impl Parse for ColumnConstraint {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::not) {
            Ok(Self::NotNull(input.parse()?))
        } else if lookahead.peek(kw::primary) {
            Ok(Self::PrimaryKey(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Display for ColumnConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotNull(_) => f.write_str("not null")?,
            Self::PrimaryKey(_) => f.write_str("primary key")?,
        };
        Ok(())
    }
}

mod kw {
    syn::custom_keyword!(not);
    syn::custom_keyword!(null);

    syn::custom_keyword!(default);

    syn::custom_keyword!(primary);
    syn::custom_keyword!(key);

    syn::custom_keyword!(references);
}

pub struct NotNull {
    _not: kw::not,
    _null: kw::null,
}

impl Parse for NotNull {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _not: input.parse()?,
            _null: input.parse()?,
        })
    }
}

pub struct PrimaryKey {
    _primary: kw::primary,
    _key: kw::key,
}

impl Parse for PrimaryKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _primary: input.parse()?,
            _key: input.parse()?,
        })
    }
}
