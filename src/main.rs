#[macro_use]
extern crate serde;
extern crate serde_json;

extern crate postgres;
extern crate toml;
extern crate uuid;

mod config;
mod database;
mod requests;
mod threading;

use std::sync::{Arc, Mutex};

use config::init_config;
use config::ConfigStruct;

use database::Db;
use threading::ThreadPool;

use requests::handle_connection;

use std::net::TcpListener;

fn main() {
    let config: ConfigStruct = init_config();

    let listener = TcpListener::bind(config.server.address).unwrap();
    let pool = ThreadPool::new(4);

    // Wrap the DB connection in a motha-flippin MUTEX because threads
    let conn = Arc::new(Mutex::new(Some(Db::init(&config.postgres.connection))));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let conn = Arc::clone(&conn);

        pool.execute(move || {
            let conn = conn.lock().unwrap();
            handle_connection(stream, conn);
        });
    }
}
