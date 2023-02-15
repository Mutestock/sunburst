use lazy_static;
use sunburst_utils::config::{is_containerized, is_production, read_config, Config};

lazy_static::lazy_static! {
    static ref CONF_RELATIVE_PATH: String = "../../config.toml".to_owned();
    pub static ref CONFIG: Config = read_config(&CONF_RELATIVE_PATH.to_string());
    pub static ref DISTRIBUTOR_URL: String = {

        let host: &str;
        let port: u16;

        if is_containerized(){
            host = &CONFIG.distributor.containerized.host;
            port = CONFIG.distributor.containerized.port;
        }
        else if is_production(){
            host = &CONFIG.distributor.prod.host;
            port = CONFIG.distributor.prod.port;
        }
        else{
            host = &CONFIG.distributor.dev.host;
            port = CONFIG.distributor.dev.port;
        }
        format!("http://{}:{}", host, port)
    };
    pub static ref RS_REST_URL: String = {

        let host: &str;
        let port: u16;

        if is_production() || is_containerized(){
            host = &CONFIG.rest.rs.prod.host;
            port = CONFIG.rest.rs.prod.port;
        }
        else{
            host = &CONFIG.rest.rs.dev.host;
            port = CONFIG.rest.rs.dev.port;
        }
        format!("{}:{}", host, port)
    };
}
