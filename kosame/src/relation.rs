use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde-full", derive(serde::Deserialize))]
pub struct ZeroOrOne<T>(Option<T>);

impl<T> ZeroOrOne<T> {
    pub(crate) fn new(inner: Option<T>) -> Self {
        Self(inner)
    }

    pub fn into_option(self) -> Option<T> {
        self.0
    }
}

impl<T> Deref for ZeroOrOne<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ZeroOrOne<T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde-full", derive(serde::Deserialize))]
pub struct Many<T>(Vec<T>);

impl<T> Deref for Many<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Many<T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl<T> Many<T> {
    pub(crate) fn new(inner: Vec<T>) -> Self {
        Self(inner)
    }

    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}
