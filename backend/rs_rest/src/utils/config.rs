use sunburst_utils::config::{read_config, Config};
use lazy_static;

lazy_static::lazy_static!{
    static ref CONF_RELATIVE_PATH: String = "../config.toml".to_owned();
    pub static ref CONFIG: Config = read_config(&CONF_RELATIVE_PATH.to_string());
}
 