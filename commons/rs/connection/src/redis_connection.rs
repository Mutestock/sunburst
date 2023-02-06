use sunburst_utils::config::{is_containerized, Config};

pub fn construct_connection_string(config: &Config) -> String {
    let host = match is_containerized() {
        true => &config.cache.containerized.host,
        false => &config.cache.local.host,
    };
    let port = match is_containerized() {
        true => config.cache.containerized.port,
        false => config.cache.local.port,
    };
    let password = match is_containerized() {
        true => &config.cache.containerized.password,
        false => &config.cache.local.password,
    };
    format!("redis://:{}@{}:{}", password, host, port)
}

pub async fn connect_redis(redis_url: &str) -> Result<redis::aio::Connection, redis::RedisError> {
    redis::Client::open(redis_url)
        .unwrap()
        .get_tokio_connection()
        .await
}
