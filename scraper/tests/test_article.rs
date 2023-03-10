use chrono::Local;
use mongodb::bson::doc;
use scraper::{
    handlers::article_handler::article_insert_many,
    utils::config::{CONFIG, MONGO_CONN_STRING},
};
use sunburst_connection::mongo_connection::connect_mongo;
use sunburst_models::article::Article;
use sunburst_utils::config::set_testing_mode;

#[tokio::test]
async fn test_article_insert_many() -> Result<(), mongodb::error::Error> {
    set_testing_mode();
    let db = connect_mongo(&MONGO_CONN_STRING)
        .await
        .unwrap()
        .default_database()
        .unwrap();
    assert_eq!(db.name(), &CONFIG.database.test_db_name);
    let collection = db.collection::<Article>("articles");
    assert_eq!(collection.count_documents(None, None).await?, 0);
    let articles_to_insert = vec![
        Article::new(
            "gnerf",
            "flerp",
            "https://flerp.com/gnerf",
            "nerglish",
            &Local::now().naive_local(),
            None,
            vec![],
        ),
        Article::new(
            "smurf",
            "flerp",
            "https://flerp.com/smurf",
            "nerglish",
            &Local::now().naive_local(),
            None,
            vec!["smerk".to_owned()],
        ),
        Article::new(
            "meef",
            "flerp",
            "https://flerp.com/meef",
            "nerglish",
            &Local::now().naive_local(),
            None,
            vec![
                "gnerklish".to_owned(),
                "nsssseeeeeeef".to_owned(),
                "arcanite reaper".to_owned(),
            ],
        ),
    ];

    article_insert_many(&articles_to_insert).await?;
    assert_eq!(collection.count_documents(None, None).await?, 3);

    collection
        .delete_many(
            doc! {
                "site": "flerp"
            },
            None,
        )
        .await
        .unwrap();
    assert_eq!(collection.count_documents(None, None).await?, 0);
    Ok(())
}
