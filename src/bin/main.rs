extern crate uuid;
extern crate web_serve;

use uuid::Uuid;

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use web_serve::Db;
use web_serve::ThreadPool;

#[derive(Debug)]
struct Request {
    method: String,
    path: String,
    host: String,
    user_agent: String,
    headers: Vec<String>,
    body: String,
}

struct User {
    id: Uuid,
    email: String,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    let conn = Db::init("postgresql://postgres:postgres@localhost:5432/test-db");

    query_db(conn);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn query_db(db: Db) {
    for row in &Db::query(db, "SELECT id, email FROM users") {
        let user_id: Uuid = row.get("id");
        let user = User {
            id: user_id,
            email: row.get("email"),
        };
        println!(
            "Found user!\nid  = {}\nwith email = {}\n",
            user.id, user.email
        );
    }
}

fn parse_request(buf: &[u8]) -> Request {
    let stream_string = String::from_utf8_lossy(&buf[..]);

    let mut splitted = stream_string.split(" ");
    let mut split_vec = splitted.collect::<Vec<&str>>();

    let method = String::from(split_vec[0]);
    let path = String::from(split_vec[1]);

    splitted = stream_string.split("\r\n");
    split_vec = splitted.collect::<Vec<&str>>();

    let host = String::from(split_vec[1]).replace("Host: ", "");
    let user_agent = String::from(split_vec[2]).replace("User-Agent: ", "");

    let headers = parse_headers(&split_vec);
    let body = parse_body(&split_vec);

    let parsed_request = Request {
        method,
        path,
        host,
        user_agent,
        headers,
        body,
    };

    parsed_request
}

fn parse_headers(rvec: &Vec<&str>) -> Vec<String> {
    // Copy vector so you can mutate it safely
    let mut new_vec = rvec.clone();

    new_vec.remove(0);
    new_vec.remove(1);

    // Find empty vec entry, indicates the start of the body
    let header_end_index = new_vec.iter().position(|x| *x == "").unwrap();

    // Remove empty string indicating end of headers
    new_vec.remove(header_end_index);

    // Now header_end_index is the start of the body
    new_vec.remove(header_end_index);

    // Cannot have a Vec<&str> in a struct because &str size cannot be known
    // at compile time, so we have to convert it into a Vec<String> here to
    // be passed back into the struct
    let returned_vec: Vec<String> = new_vec.iter().map(|s| String::from(&**s)).collect();
    returned_vec
}

fn parse_body(rvec: &Vec<&str>) -> String {
    // Copy vector so you can mutate it safely
    let mut new_vec = rvec.clone();

    // Find empty vec entry, indicates the start of the body
    let body_start_index = new_vec.iter().position(|x| *x == "").unwrap();

    // Remove empty string indicating end of headers
    new_vec.remove(body_start_index);

    // .replace is used to remove empty data left over from
    // buffer initialisation
    String::from(new_vec[body_start_index]).replace("\u{0}", "")
}

fn handle_connection(mut stream: TcpStream) {
    // Arbitrary buffer length - hopefully long enough to capture all
    // headers, even if there's shit-loads of them
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let request_obj = parse_request(&buffer);

    println!("{:?}", request_obj);

    let (status_line, filename) = if request_obj.method == "GET" && request_obj.path == "/" {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
