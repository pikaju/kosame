use std::ops::{Deref, DerefMut};

// pub type ManyToOne<T> = Option<T>;
// pub type OneToMany<T> = Vec<T>;

#[derive(Debug, Default)]
pub struct ManyToOne<T>(pub Option<T>);

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
pub struct OneToMany<T>(pub Vec<T>);

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
