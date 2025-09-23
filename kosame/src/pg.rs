pub mod internal {
    use fallible_iterator::FallibleIterator;
    pub use postgres_protocol::types::int4_from_sql;
    pub use postgres_types::{FromSql, Type};

    impl<'a, T> FromSql<'a> for crate::relation::OneToMany<T>
    where
        T: FromSql<'a>,
    {
        fn accepts(ty: &Type) -> bool {
            ty.name() == "_record"
        }

        fn from_sql(
            ty: &Type,
            raw: &'a [u8],
        ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
            if ty.name() != "_record" {
                panic!("expected _record type");
            };

            let array = postgres_protocol::types::array_from_sql(raw)?;
            if array.dimensions().count()? > 1 {
                return Err("array contains too many dimensions".into());
            }

            let inner = array
                .values()
                .map(|v| T::from_sql_nullable(ty, v))
                .collect()?;

            Ok(Self(inner))
        }
    }

    pub fn record_field_from_sql<'a, T>(
        buf: &'a [u8],
    ) -> Result<(T, usize), Box<dyn std::error::Error + Sync + Send>>
    where
        T: FromSql<'a>,
    {
        let oid = postgres_protocol::types::oid_from_sql(&buf[..4])? as u32;
        let Some(ty) = ::postgres_types::Type::from_oid(oid) else {
            panic!("unknown oid {}", oid);
        };
        let length = postgres_protocol::types::int4_from_sql(&buf[4..8])? as usize;

        Ok((T::from_sql(&ty, &buf[8..(8 + length)])?, 8 + length))
    }
}
