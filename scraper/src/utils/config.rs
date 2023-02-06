use sunburst_utils::config::{read_config, Config};


const CONF_RELATIVE_PATH: &'static str = "../config.toml";

pub fn get_config() -> Config {
    read_config(&CONF_RELATIVE_PATH.to_string())
}
