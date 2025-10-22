pub enum Driver {
    Postgres,
    TokioPostgres,
    Mysql,
    Rusqlite,
}

impl TryFrom<&str> for Driver {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, ()> {
        match value {
            "postgres" => Ok(Self::Postgres),
            "tokio-postgres" => Ok(Self::TokioPostgres),
            "mysql" => Ok(Self::Mysql),
            "rusqlite" => Ok(Self::Rusqlite),
            _ => Err(()),
        }
    }
}
