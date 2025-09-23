use std::ops::{Deref, DerefMut};

// pub type ManyToOne<T> = Option<T>;
// pub type OneToMany<T> = Vec<T>;
//
pub type ManyToOne<T> = OneToMany<T>;

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
