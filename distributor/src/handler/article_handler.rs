use crate::{
    caching::keys::{
        READ_LIST_ARTICLE_CACHE_KEY, READ_LIST_ARTICLE_SEARCH_TERM_CACHE_KEY_PREFIX,
        READ_LIST_ARTICLE_SITE_CACHE_KEY_PREFIX,
    },
    tonic_proto_out::{
        ArticleMessage, ReadArticleListBySearchtermRequest, ReadArticleListBySearchtermResponse,
        ReadArticleListBySiteRequest, ReadArticleListBySiteResponse, ReadArticleListRequest,
        ReadArticleListResponse,
    },
};
use futures::stream::StreamExt;
use mongodb::bson::{doc, Document};
use redis::AsyncCommands;
use sunburst_connection::{mongo_connection::connect_mongo, redis_connection};
use sunburst_models::article::Article;

use crate::utils::config::{MONGO_CONN_STRING, REDIS_CONN_STRING};

pub async fn handle_read_list(
    _request: ReadArticleListRequest,
) -> Result<ReadArticleListResponse, mongodb::error::Error> {
    let mut red_conn = redis_connection::connect_redis(&REDIS_CONN_STRING)
        .await
        .expect("Could not establish redis connection");

    let current_cache_contents: Option<String> = red_conn
        .get(READ_LIST_ARTICLE_CACHE_KEY)
        .await
        .expect("Could not retrieve cache contents");

    let articles = match current_cache_contents {
        Some(v) => {
            serde_json::from_str(&v).expect("Could not parse cache contents to article list")
        }
        None => {
            let articles = read_articles_filter(None).await?;
            cache_articles(&articles, READ_LIST_ARTICLE_CACHE_KEY)
                .await
                .expect("Could not cache articles for handle read list");
            articles
        }
    };

    let response = ReadArticleListResponse {
        articles: articles_to_article_messages(&articles).await,
        msg: "Ok".to_owned(),
    };

    Ok(response)
}

pub fn formatted_search_term_cache_key(search_term: &str) -> String {
    format!(
        "{}{}",
        READ_LIST_ARTICLE_SEARCH_TERM_CACHE_KEY_PREFIX, search_term
    )
}

pub fn formatted_site_cache_key(site: &str) -> String {
    format!("{}{}", READ_LIST_ARTICLE_SITE_CACHE_KEY_PREFIX, site)
}

pub async fn handle_read_list_by_searchterm(
    request: ReadArticleListBySearchtermRequest,
) -> Result<ReadArticleListBySearchtermResponse, mongodb::error::Error> {
    let mut red_conn = redis_connection::connect_redis(&REDIS_CONN_STRING)
        .await
        .expect("Could not establish redis connection");

    let cache_key = formatted_search_term_cache_key(&request.search_term);

    let current_cache_contents: Option<String> = red_conn
        .get(&cache_key)
        .await
        .expect("Could not retrieve cache contents");

    let articles = match current_cache_contents {
        Some(v) => {
            serde_json::from_str(&v).expect("Could not parse cache contents to article list")
        }
        None => {
            let articles = read_articles_filter(Some(doc! {
                "name": &format!("/{}/i",request.search_term)
            }))
            .await?;
            cache_articles(&articles, &cache_key)
                .await
                .expect("Could not cache articles for handle read list");
            articles
        }
    };

    let response = ReadArticleListBySearchtermResponse {
        articles: articles_to_article_messages(&articles).await,
        msg: "Ok".to_owned(),
    };

    Ok(response)
}

pub async fn handle_read_article_list_by_site(
    request: ReadArticleListBySiteRequest,
) -> Result<ReadArticleListBySiteResponse, mongodb::error::Error> {
    let mut red_conn = redis_connection::connect_redis(&REDIS_CONN_STRING)
        .await
        .expect("Could not establish redis connection");

    let cache_key = formatted_site_cache_key(&request.site);

    let current_cache_contents: Option<String> = red_conn
        .get(&cache_key)
        .await
        .expect("Could not retrieve cache contents");

    let articles = match current_cache_contents {
        Some(v) => {
            serde_json::from_str(&v).expect("Could not parse cache contents to article list")
        }
        None => {
            let articles = read_articles_filter(Some(doc! {
                "site": &format!("{}",request.site)
            }))
            .await?;
            cache_articles(&articles, &cache_key)
                .await
                .expect("Could not cache articles for handle read list");
            articles
        }
    };

    let response = ReadArticleListBySiteResponse {
        articles: articles_to_article_messages(&articles).await,
        msg: "Ok".to_owned(),
    };

    Ok(response)
}

async fn articles_to_article_messages(articles: &[Article]) -> Vec<ArticleMessage> {
    articles
        .into_iter()
        .map(|article| ArticleMessage {
            name: article.name.clone(),
            site: article.site.clone(),
            url: article.url.clone(),
            language: article.language.clone(),
            scrape_date: article.scrape_date.to_string(),
            submission_date: match article.submission_date {
                Some(v) => Some(v.to_string()),
                None => None,
            },
            tags_and_categories: article.tags_and_categories.clone(),
        })
        .collect()
}

pub async fn read_articles_filter(
    filter: Option<Document>,
) -> Result<Vec<Article>, mongodb::error::Error> {
    let collection = connect_mongo(&MONGO_CONN_STRING)
        .await?
        .default_database()
        .unwrap()
        .collection::<Article>("articles");

    let mut articles: Vec<Article> = Vec::new();
    let mut cursor = collection.find(filter, None).await?;

    while let Some(article) = cursor.next().await {
        let article = article?;
        articles.push(article);
    }
    Ok(articles)
}

pub async fn cache_articles(
    articles: &[Article],
    cache_key: &str,
) -> Result<(), redis::RedisError> {
    let mut red_conn = redis_connection::connect_redis(&REDIS_CONN_STRING).await?;
    let value = serde_json::to_string(&articles).unwrap();
    // This might get too big...
    let _: () = red_conn.set(cache_key, &value).await?;

    Ok(())
}
