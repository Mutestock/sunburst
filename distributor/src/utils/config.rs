use sunburst_utils::config::{read_config, Config};
use sunburst_connection::{mongo_connection, redis_connection};
use lazy_static;

lazy_static::lazy_static!{
    static ref CONF_RELATIVE_PATH: String = "../config.toml".to_owned();
    pub static ref CONFIG: Config = read_config(&CONF_RELATIVE_PATH.to_string());
    pub static ref MONGO_CONN_STRING: String = mongo_connection::construct_connection_string(&CONFIG);
    pub static ref REDIS_CONN_STRING: String = redis_connection::construct_connection_string(&CONFIG);
}
