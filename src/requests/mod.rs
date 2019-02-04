use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::MutexGuard;

use database::Db;

#[derive(Debug)]
struct Request {
    method: String,
    path: String,
    host: String,
    user_agent: String,
    headers: Vec<String>,
    body: String,
}

#[derive(Debug)]
enum Response {
    S(String),
    J(Vec<serde_json::Value>),
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

fn handle_routing(
    method: &str,
    path: &str,
    mut conn_guard: MutexGuard<Option<Db>>,
) -> (String, Response) {
    match path.as_ref() {
        "/" => {
            if method == "GET" {
                return (
                    "HTTP/1.1 200 OK\r\n\r\n".to_string(),
                    Response::S(fs::read_to_string("templates/index.html").unwrap()),
                );
            } else {
                return (
                    "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string(),
                    Response::S(fs::read_to_string("templates/404.html").unwrap()),
                );
            }
        }
        "/guests" => {
            if method == "GET" {
                let guests = *conn_guard.take().unwrap().get_guests();
                return ("HTTP/1.1 200 OK\r\n\r\n".to_string(), Response::J(guests));
            } else {
                return (
                    "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string(),
                    Response::S("Not found".to_string()),
                );
            }
        }
        _ => (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string(),
            Response::S(fs::read_to_string("templates/404.html").unwrap()),
        ),
    }
}

pub fn handle_connection(mut stream: TcpStream, conn_guard: MutexGuard<Option<Db>>) {
    // Arbitrary buffer length - hopefully long enough to capture all
    // headers, even if there's shit-loads of them
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let request_obj = parse_request(&buffer);
    println!("{:?}", request_obj);

    let (status_line, contents) =
        handle_routing(&request_obj.method, &request_obj.path, conn_guard);
    let response = format!("{}{:#?}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
