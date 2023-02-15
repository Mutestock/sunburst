use crate::{
    caching::{
        caching_tools::cache_something,
        keys::{
            formatted_cnt_search_site_cache_key, formatted_search_term_cache_key,
            formatted_site_cache_key, READ_LIST_ARTICLE_CACHE_KEY,
        },
    },
    tonic_proto_out::{
        ArticleMessage, ReadArticleCountBySearchSiteRequest, ReadArticleCountBySearchSiteResponse,
        ReadArticleListBySearchtermRequest, ReadArticleListBySearchtermResponse,
        ReadArticleListBySiteRequest, ReadArticleListBySiteResponse, ReadArticleListRequest,
        ReadArticleListResponse,
    },
};
use futures::stream::StreamExt;
use mongodb::bson::{doc, Document};
use redis::AsyncCommands;

use sunburst_connection::{mongo_connection::connect_mongo, redis_connection};
use sunburst_models::{article::Article, article_count::ArticleCount};

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
            cache_something(&articles, READ_LIST_ARTICLE_CACHE_KEY)
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
            cache_something(&articles, &cache_key)
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
                "site": request.site
            }))
            .await?;
            cache_something(&articles, &cache_key)
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

pub async fn handle_read_article_count_by_search_site(
    request: ReadArticleCountBySearchSiteRequest,
) -> Result<ReadArticleCountBySearchSiteResponse, mongodb::error::Error> {
    let mut red_conn = redis_connection::connect_redis(&REDIS_CONN_STRING)
        .await
        .expect("Could not establish redis connection");

    let cache_key = formatted_cnt_search_site_cache_key(&request.search_term, &request.site);
    let current_cache_contents: Option<String> = red_conn
        .get(&cache_key)
        .await
        .expect("Could not receive cache contents");

    let article_count: ArticleCount = match current_cache_contents {
        Some(v) => {
            serde_json::from_str(&v).expect("Could not parse cache contents to article count")
        }
        None => {
            let article_count = make_article_count(&request.site, &request.search_term).await?;
            article_count
        }
    };

    Ok(ReadArticleCountBySearchSiteResponse {
        total: article_count.total,
        cnt_contained_search_term: article_count.cnt_contained_search_term,
        cnt_not_contained_search_term: article_count.cnt_not_contained_search_term,
    })
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

pub async fn make_article_count(
    site: &str,
    search_term: &str,
) -> Result<ArticleCount, mongodb::error::Error> {
    let articles = read_articles_filter(Some(doc! {
        "site": site
    }))
    .await?;
    let total = articles.len();
    let articles_with_search_term = articles
        .into_iter()
        .filter(|x| {
            x.name.contains(search_term) || x.tags_and_categories.contains(&search_term.to_owned())
        })
        .collect::<Vec<Article>>();
    Ok(ArticleCount::new(
        total as i64,
        articles_with_search_term.len() as i64,
        (total - articles_with_search_term.len()) as i64,
    ))
}
