extern crate postgres;
extern crate uuid;

use self::uuid::Uuid;
use database::postgres::{Connection, TlsMode};

pub struct Db {
    pub conn: Connection,
}

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
}

impl Db {
    pub fn init(conn_string: &str) -> Db {
        let conn = Connection::connect(conn_string, TlsMode::None).unwrap();
        let db = Db { conn };
        db
    }
    pub fn get_users(self) -> Vec<super::database::User> {
        let mut users = Vec::new();
        for row in &self.conn.query("SELECT id, email FROM users", &[]).unwrap() {
            let user_id: Uuid = row.get("id");
            let user = User {
                id: user_id,
                email: row.get("email"),
            };
            users.push(user);
        }
        users
    }
}
