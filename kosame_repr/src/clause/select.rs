use crate::clause::Fields;

pub struct Select<'a> {
    fields: Fields<'a>,
}

impl<'a> Select<'a> {
    #[inline]
    pub const fn new(fields: Fields<'a>) -> Self {
        Self { fields }
    }

    #[inline]
    pub const fn fields(&self) -> &Fields<'a> {
        &self.fields
    }
}
