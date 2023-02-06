use std::{env, fs};

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub database: Database,
    pub scraper: Scraper,
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

#[derive(Serialize, Deserialize)]
pub struct Scraper {
    pub user_agent: String,
    pub profiles: Profiles,
}

#[derive(Serialize, Deserialize)]
pub struct Profiles {
    pub tv2: Tv2,
}

#[derive(Serialize, Deserialize)]
pub struct Tv2 {
    pub request_frequency_seconds: u64,
    pub url: String,
}

pub fn read_config(path: &str) -> Config{
    let config_file_contents =
        fs::read_to_string(path).expect("Failed to read config file");
    toml::from_str(&config_file_contents).expect("Could not parse config file contents")
}

pub fn is_containerized() -> bool {
    env::var("CONTAINERIZED").is_ok()
}

pub fn is_production() -> bool {
    env::var("PRODUCTION").is_ok()
}

pub fn is_testing() -> bool {
    let generic_testing = env::var("TESTING").is_ok();
    let scraper_testing = env::var("TEST_MODE_SCRAPER").is_ok();
    generic_testing || scraper_testing
}

pub fn set_testing_mode() {
    env::set_var("TEST_MODE_SCRAPER", "1");
}
