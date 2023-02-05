use scraper::connection::mongo_connection::connect_mongo;


#[tokio::test]
async fn test_mongo_connection() {
    // This forces db interaction. Consider it a "does not panic" test.
    let client = connect_mongo().await.unwrap();
    let session = client.start_session(None).await.unwrap();
    let id_doc = session.id();
    id_doc.is_empty();
}