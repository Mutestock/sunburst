use crate::connection::mongo_connection::connect_mongo;
use mongodb::results::InsertManyResult;
use rs_models::article::Article;

pub async fn article_insert_many(
    articles: &[Article],
) -> Result<InsertManyResult, mongodb::error::Error> {
    let conn = connect_mongo().await?;
    let collection = conn
        .default_database()
        .expect("mongo db in scraper did not have a default database")
        .collection::<Article>("articles");
    let res = collection.insert_many(articles, None).await?;

    Ok(res)
}
