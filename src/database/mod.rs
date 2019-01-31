extern crate postgres;

use database::postgres::rows::Rows;
use database::postgres::{Connection, TlsMode};

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn init(conn_string: &str) -> Db {
        let conn = Connection::connect(conn_string, TlsMode::None).unwrap();
        let db = Db { conn };
        db
    }
    pub fn query(self, query_string: &str) -> Rows {
        self.conn.query(query_string, &[]).unwrap()
    }
}
