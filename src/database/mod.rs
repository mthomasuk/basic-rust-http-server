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
    pub fn get_guests(self) -> Box<Vec<serde_json::Value>> {
        let mut guests = Vec::new();

        for row in &self
            .conn
            .query("SELECT id, key, name FROM guests", &[])
            .unwrap()
        {
            let guests_id: Uuid = row.get("id");
            let guests_key: String = row.get("key");
            let guests_name: String = row.get("name");

            let guest = json!({
                "id": guests_id.hyphenated().to_string(),
                "key": guests_key,
                "name": guests_name,
            });
            guests.push(guest);
        }

        Box::new(guests)
    }
}
