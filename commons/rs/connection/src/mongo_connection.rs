use mongodb::{options::ClientOptions, Client};
use sunburst_utils::config::{is_containerized, is_production, is_testing, Config};

pub fn construct_connection_string(config: &Config) -> String {
    let db = {
        if is_production() {
            &config.database.prod_db_name
        } else if is_testing() {
            &config.database.test_db_name
        } else {
            &config.database.dev_db_name
        }
    };
    let host = match is_containerized() {
        true => &config.database.containerized.host,
        false => &config.database.local.host,
    };
    let port = match is_containerized() {
        true => config.database.containerized.port,
        false => config.database.local.port,
    };
    let username = match is_containerized() {
        true => &config.database.containerized.username,
        false => &config.database.local.username,
    };
    let password = match is_containerized() {
        true => &config.database.containerized.password,
        false => &config.database.local.password,
    };
    format!(
        "mongodb://{}:{}@{}:{}/{}?authSource=admin",
        username, password, host, port, db
    )
}

pub async fn make_mongo_options(conn_string: &str) -> Result<ClientOptions, mongodb::error::Error> {
    ClientOptions::parse(conn_string).await
}

pub async fn connect_mongo(conn_string: &str) -> Result<Client, mongodb::error::Error> {
    Client::with_options(make_mongo_options(conn_string).await?)
}
