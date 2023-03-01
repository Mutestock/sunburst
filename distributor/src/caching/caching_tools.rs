use redis::AsyncCommands;
use serde::Serialize;
use sunburst_connection::redis_connection;

use crate::utils::config::{CONFIG, REDIS_CONN_STRING};

pub async fn cache_something(
    something: impl Serialize,
    cache_key: &str,
) -> Result<(), redis::RedisError> {
    let mut red_conn = redis_connection::connect_redis(&REDIS_CONN_STRING).await?;
    let value = serde_json::to_string(&something).unwrap();
    let _: () = red_conn.set(cache_key, &value).await?;
    let _: () = red_conn
        .expire(cache_key, CONFIG.cache.cache_key_lifetime_seconds as usize)
        .await?;

    Ok(())
}
