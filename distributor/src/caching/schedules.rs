use std::time::Duration;

use mongodb::bson::doc;

use crate::caching::keys::*;
use crate::handler::article_handler::{make_article_count, read_articles_filter};
use crate::utils::config::CONFIG;

use super::caching_tools::cache_something;

pub async fn cache_common_queries_sequence() {
    tokio::task::spawn(async {
        loop {
            let articles = read_articles_filter(None)
                .await
                .expect("Could not read articles for the cache sequence");

            cache_something(&articles, READ_LIST_ARTICLE_CACHE_KEY)
                .await
                .expect("Could not cache articles for cache sequence");

            for search_term in CONFIG.cache.cached_common_article_search_terms.iter() {
                let articles = read_articles_filter(Some(doc! {
                    "name": {"$regex": search_term, "$options": "i" }
                }))
                .await
                .expect("Could not read articles for the cache sequence");

                cache_something(&articles, &formatted_search_term_cache_key(search_term))
                    .await
                    .expect("Could not cache articles for cache sequence");
            }

            for site in CONFIG.cache.cached_sites.iter() {
                let articles = read_articles_filter(Some(doc! {
                    "site": site
                }))
                .await
                .expect("Could not read articles for the cache sequence");

                cache_something(&articles, &formatted_site_cache_key(site))
                    .await
                    .expect("Could not cache articles for cache sequence");
            }

            for site in CONFIG.cache.cached_sites.iter() {
                for search_term in CONFIG.cache.cached_common_article_search_terms.iter() {
                    let article_count = make_article_count(site, search_term)
                        .await
                        .expect("Could not make article count for the cache sequence");
                    let cache_key = formatted_cnt_search_site_cache_key(search_term, site);
                    cache_something(article_count, &cache_key)
                        .await
                        .expect("Could not cache article count");
                }
            }

            tokio::time::sleep(Duration::from_secs(
                CONFIG.cache.cache_common_articles_refresh_rate_seconds,
            ))
            .await;
        }
    });
}
