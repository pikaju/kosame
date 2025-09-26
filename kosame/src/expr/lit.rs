pub enum Lit {
    Int(i64),
    Float(f64),
    Str(&'static str),
    Bool(bool),
}

impl Lit {
    pub fn to_sql_string(&self, buf: &mut String) {
        match self {
            Self::Int(inner) => *buf += &inner.to_string(),
            Self::Float(inner) => *buf += &inner.to_string(),
            Self::Str(inner) => {
                *buf += "'";
                *buf += inner;
                *buf += "'";
            }
            Self::Bool(inner) => *buf += &inner.to_string(),
        }
    }
}
