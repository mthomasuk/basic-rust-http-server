use super::postgres::{Connection, TlsMode};
use super::uuid::Uuid;

use super::serde_json;
use super::serde_json::json;

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn init(conn_string: &str) -> Db {
        let conn = Connection::connect(conn_string, TlsMode::None).unwrap();
        let db = Db { conn };
        db
    }
    pub fn get_users(self) -> Vec<serde_json::Value> {
        let mut users = Vec::new();
        for row in &self.conn.query("SELECT id, email FROM users", &[]).unwrap() {
            let user_id: Uuid = row.get("id");
            let user_email: String = row.get("email");

            let user = json!({
                "id": user_id.hyphenated().to_string(),
                "email": user_email,
            });
            users.push(user);
        }
        users
    }
}
