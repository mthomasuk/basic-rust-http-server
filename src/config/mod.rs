use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
pub struct ConfigStruct {
    pub server: ServerConf,
    pub postgres: PostgresConf,
}

#[derive(Deserialize, Debug)]
pub struct ServerConf {
    pub address: String,
}

#[derive(Deserialize, Debug)]
pub struct PostgresConf {
    pub connection: String,
}

pub fn init_config() -> ConfigStruct {
    let mut file = File::open("config/config.toml").unwrap();
    let mut toml_content = String::new();

    file.read_to_string(&mut toml_content).unwrap();

    let config: ConfigStruct = toml::from_str(&toml_content).unwrap();
    config
}
