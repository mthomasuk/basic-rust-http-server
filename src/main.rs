#[macro_use]
extern crate serde;

extern crate postgres;
extern crate toml;
extern crate uuid;

mod config;
mod database;
mod requests;
mod threading;

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
    let conn = Db::init(&config.postgres.connection);

    let users = Db::get_users(conn);
    println!("{:?}", users);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
