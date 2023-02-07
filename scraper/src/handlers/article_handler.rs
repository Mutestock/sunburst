use mongodb::{
    bson::{doc, to_document},
    options::UpdateOptions,
};
use sunburst_connection::mongo_connection::connect_mongo;
use sunburst_models::article::Article;

use crate::utils::config::MONGO_CONN_STRING;

pub async fn article_insert_many(articles: &[Article]) -> Result<(), mongodb::error::Error> {
    let conn = connect_mongo(&MONGO_CONN_STRING).await?;
    let collection = conn
        .default_database()
        .expect("mongo db in scraper did not have a default database")
        .collection::<Article>("articles");
    let mut update_options = UpdateOptions::default();
    update_options.upsert = Some(true);

    for article in articles.iter() {
        collection
            .update_many(
                doc! {"name": &article.name},
                doc! {"$set": to_document(article)?},
                update_options.clone(),
            )
            .await?;
    }
    Ok(())
}
