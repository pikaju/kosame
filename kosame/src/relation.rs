use std::ops::{Deref, DerefMut};

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde-serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize", derive(serde::Deserialize))]
pub struct ManyToOne<T>(Option<T>);

impl<T> ManyToOne<T> {
    pub(crate) fn new(inner: Option<T>) -> Self {
        Self(inner)
    }

    pub fn into_option(self) -> Option<T> {
        self.0
    }
}

impl<T> Deref for ManyToOne<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ManyToOne<T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde-serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize", derive(serde::Deserialize))]
pub struct OneToMany<T>(Vec<T>);

impl<T> Deref for OneToMany<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for OneToMany<T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl<T> OneToMany<T> {
    pub(crate) fn new(inner: Vec<T>) -> Self {
        Self(inner)
    }

    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}
