use std::time::Duration;

use mongodb::bson::doc;

use crate::caching::keys::*;
use crate::handler::article_handler::{cache_articles, read_articles_filter, formatted_search_term_cache_key};
use crate::utils::config::CONFIG;

pub async fn start_sequence() {
    tokio::task::spawn(async {
        loop {
            let articles = read_articles_filter(None)
                .await
                .expect("Could not read articles for the cache sequence");

            cache_articles(&articles, READ_LIST_ARTICLE_CACHE_KEY)
                .await
                .expect("Could not cache articles for cache sequence");

            for search_term in CONFIG.cache.cached_common_article_search_terms.iter() {
                let articles = read_articles_filter(Some(doc! {
                    "name": &format!("/{}/i",search_term)
                }))
                .await
                .expect("Could not read articles for the cache sequence");

                cache_articles(&articles, &formatted_search_term_cache_key(search_term))
                    .await
                    .expect("Could not cache articles for cache sequence");
            }

            tokio::time::sleep(Duration::from_secs(
                CONFIG.cache.cache_common_articles_refresh_rate_seconds,
            ))
            .await;
        }
    });
}
