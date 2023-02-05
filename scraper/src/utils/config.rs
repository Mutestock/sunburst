use std::{env, fs};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub database: Database,
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub container_name: String,
    pub test_db_name: String,
    pub dev_db_name: String,
    pub prod_db_name: String,
    pub local: Local,
    pub containerized: Containerized,
}

#[derive(Serialize, Deserialize)]
pub struct Local {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Containerized {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

pub fn get_config() -> Config {
    let config_file_contents =
        fs::read_to_string("../config.toml").expect("Failed to read config file");
    toml::from_str(&config_file_contents).expect("Could not parse config file contents")
}

pub fn is_containerized() -> bool {
    env::var("CONTAINERIZED").is_ok()
}

pub fn is_production() -> bool {
    env::var("PRODUCTION").is_ok()
}

pub fn is_testing() -> bool {
    env::var("TESTING").is_ok()
}