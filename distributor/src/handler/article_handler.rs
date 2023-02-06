use crate::tonic_proto_out::{
    ArticleMessage, ReadArticleListBySearchtermRequest, ReadArticleListBySearchtermResponse,
    ReadArticleListRequest, ReadArticleListResponse,
};
use futures::stream::StreamExt;
use mongodb::bson::doc;
use sunburst_connection::mongo_connection::connect_mongo;
use sunburst_models::article::Article;

use crate::utils::config::MONGO_CONN_STRING;

pub async fn handle_read_list(
    _request: ReadArticleListRequest,
) -> Result<ReadArticleListResponse, mongodb::error::Error> {
    let collection = connect_mongo(&MONGO_CONN_STRING)
        .await?
        .default_database()
        .unwrap()
        .collection::<Article>("article");

    let mut article_messages: Vec<ArticleMessage> = Vec::new();
    let mut cursor = collection.find(None, None).await?;

    while let Some(article) = cursor.next().await {
        let article = article?;
        article_messages.push(ArticleMessage {
            name: article.name,
            site: article.site,
            url: article.url,
            language: article.language,
            scrape_date: article.scrape_date.to_string(),
            submission_date: match article.submission_date {
                Some(v) => Some(v.to_string()),
                None => None,
            },
            tags_and_categories: article.tags_and_categories,
        })
    }
    let response = ReadArticleListResponse {
        articles: article_messages,
        msg: "Ok".to_owned(),
    };

    Ok(response)
}

pub async fn handle_read_list_by_searchterm(
    request: ReadArticleListBySearchtermRequest,
) -> Result<ReadArticleListBySearchtermResponse, mongodb::error::Error> {
    let collection = connect_mongo(&MONGO_CONN_STRING)
        .await?
        .default_database()
        .unwrap()
        .collection::<Article>("article");

    let mut article_messages: Vec<ArticleMessage> = Vec::new();
    let mut cursor = collection
        .find(
            doc! {
                "name": &format!("/{}/i",request.search_term)
            },
            None,
        )
        .await?;

    while let Some(article) = cursor.next().await {
        let article = article?;
        article_messages.push(ArticleMessage {
            name: article.name,
            site: article.site,
            url: article.url,
            language: article.language,
            scrape_date: article.scrape_date.to_string(),
            submission_date: match article.submission_date {
                Some(v) => Some(v.to_string()),
                None => None,
            },
            tags_and_categories: article.tags_and_categories,
        })
    }
    let response = ReadArticleListBySearchtermResponse {
        articles: article_messages,
        msg: "Ok".to_owned(),
    };

    Ok(response)
}
