use crate::utils::config::{get_config, is_containerized, is_production, is_testing, Config};
use lazy_static;
use mongodb::{options::ClientOptions, Client};

lazy_static::lazy_static! {
    static ref CONFIG: Config = get_config();
    static ref DB: &'static str = {
        if is_production(){
            &CONFIG.database.prod_db_name
        }
        else if is_testing() {
            &CONFIG.database.test_db_name
        }
        else {
            &CONFIG.database.dev_db_name
        }
    };
    static ref HOST: &'static str = match is_containerized(){
        true => &CONFIG.database.containerized.host,
        false => &CONFIG.database.local.host,
    };
    static ref PORT: u16 = match is_containerized(){
        true => CONFIG.database.containerized.port,
        false => CONFIG.database.local.port,
    };
    static ref USERNAME: &'static str = match is_containerized(){
        true => &CONFIG.database.containerized.username,
        false => &CONFIG.database.local.username,
    };
    static ref PASSWORD: &'static str = match is_containerized(){
        true => &CONFIG.database.containerized.password,
        false => &CONFIG.database.local.password,
    };
    static ref CONN_STRING: String = format!("mongodb://{}:{}@{}:{}/{}?authSource=admin",
        USERNAME.to_string(),
        PASSWORD.to_string(),
        HOST.to_string(),
        PORT.to_string(),
        DB.to_string()
    );

    static ref TEST_MODE_CONN_STRING: String = format!("mongodb://{}:{}@{}:{}/{}?authSource=admin",
        USERNAME.to_string(),
        PASSWORD.to_string(),
        HOST.to_string(),
        PORT.to_string(),
        &CONFIG.database.test_db_name
    );
}

pub async fn make_mongo_options() -> Result<ClientOptions, mongodb::error::Error> {
    ClientOptions::parse(CONN_STRING.to_string()).await
}

pub async fn connect_mongo() -> Result<Client, mongodb::error::Error> {
    Client::with_options(make_mongo_options().await?)
}
