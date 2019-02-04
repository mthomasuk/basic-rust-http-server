use super::postgres::{Connection, TlsMode};

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn init(conn_string: &str) -> Db {
        let conn = Connection::connect(conn_string, TlsMode::None).unwrap();
        let db = Db { conn };
        db
    }
}
