use std::{env, fs};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub database: Database,
    pub scraper: Scraper,
    pub distributor: Distributor,
    pub cache: Cache,
    pub rest: Rest,
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

#[derive(Serialize, Deserialize)]
pub struct Distributor {
    pub dev: DistDev,
    pub prod: DistProd,
    pub containerized: DistContainerized,
}

#[derive(Serialize, Deserialize)]
pub struct DistDev {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct DistProd {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct DistContainerized {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub container_name: String,
    pub cached_common_article_search_terms: Vec<String>,
    pub cache_common_articles_refresh_rate_seconds: u64,
    pub cached_sites: Vec<String>,
    pub local: CacheLocal,
    pub containerized: CacheContainerized,
}

#[derive(Serialize, Deserialize)]
pub struct CacheLocal {
    pub host: String,
    pub port: u16,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CacheContainerized {
    pub host: String,
    pub port: u16,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Rest {
    pub rs: RsRest,
}

#[derive(Serialize, Deserialize)]
pub struct RsRest {
    pub container_name: String,
    pub dev: RsRestDev,
    pub prod: RsRestProd,
    pub containerized: RsRestContainerized,
}

#[derive(Serialize, Deserialize)]
pub struct RsRestDev {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct RsRestProd {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct RsRestContainerized {
    pub host: String,
    pub port: u16,
}

pub fn read_config(path: &str) -> Config {
    let config_file_contents = fs::read_to_string(path).expect("Failed to read config file");
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
