use database::Db;
use std::fs;
use std::sync::MutexGuard;
use uuid::Uuid;

use request::Response;

use super::serde_json::json;

pub fn serve_index_page() -> (String, Response) {
    (
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n".to_string(),
        Response::S(fs::read_to_string("templates/index.html").unwrap()),
    )
}

pub fn serve_error_page() -> (String, Response) {
    (
        "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\n\r\n".to_string(),
        Response::S(fs::read_to_string("templates/404.html").unwrap()),
    )
}

pub fn serve_method_not_allowed() -> (String, Response) {
    (
        "HTTP/1.1 405 METHOD NOT ALLOWED\r\nContent-Type: text/html\r\n\r\n".to_string(),
        Response::S("Method not allowed".to_string()),
    )
}

pub fn serve_guests_json(conn: MutexGuard<Db>) -> (String, Response) {
    let mut guests = Vec::new();

    for row in &conn
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
        guests.push(guest.to_string());
    }

    return (
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n".to_string(),
        Response::J(guests),
    );
}

pub fn serve_guest_json(conn: MutexGuard<Db>, path: &str) -> (String, Response) {
    let id_vec: Vec<&str> = path.split("/").collect();
    let mut guests = Vec::new();

    for row in &conn
        .conn
        .query(
            "SELECT id, key, name FROM guests WHERE key = $1",
            &[&id_vec[2]],
        )
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
        guests.push(guest.to_string());
    }

    return (
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n".to_string(),
        Response::J(guests),
    );
}
