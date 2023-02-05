use std::env;

use scraper::{connection::mongo_connection::connect_mongo, utils::config::{get_config, set_testing_mode}};



#[tokio::test]
async fn test_mongo_connection() {
    // This forces db interaction. Consider it a "does not panic" test.
    set_testing_mode();
    let client = connect_mongo().await.unwrap();
    let session = client.start_session(None).await.unwrap();
    let id_doc = session.id();
    id_doc.is_empty();
}

#[tokio::test]
async fn test_mongo_client_has_default_database() {
    set_testing_mode();
    let client = connect_mongo().await.unwrap();
    let default_database = client.default_database();
    assert_eq!(default_database.is_some(), true);
    assert_eq!(default_database.unwrap().name(), get_config().database.test_db_name);
}