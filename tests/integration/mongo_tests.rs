use crate::integration::common::build_test_env;
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::{Client, Collection};
use snacks_awesome_shortener::models::mongo_short::MongoShortUrl;

const DATABASE: &str = "shorts";
const COLLECTION: &str = "short_url";
const TEST_SHORTENED_URL: &str = "https://hltv.org";

#[tokio::test]
async fn post_short_mongo_document_created() {
    let test_env = build_test_env(true, true).await;
    let mut client_options = ClientOptions::parse(test_env.mongo_url).await.unwrap();
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let mongo_client = Client::with_options(client_options).unwrap();

    let resp = reqwest::Client::new()
        .post(format!("http://127.0.0.1:{}/short", test_env.app_port))
        .json(&serde_json::json!({ "url": TEST_SHORTENED_URL }))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let short_url = body.get("short_url").unwrap().as_str().unwrap();

    let mongo_collection: Collection<MongoShortUrl> =
        mongo_client.database(DATABASE).collection(COLLECTION);

    let result = mongo_collection
        .find_one(doc! { "_id": short_url})
        .await
        .unwrap();

    assert!(result.is_some());
}

#[tokio::test]
async fn post_short_mongo_document_created_and_matching() {
    let test_env = build_test_env(true, true).await;
    let mut client_options = ClientOptions::parse(test_env.mongo_url).await.unwrap();
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let mongo_client = Client::with_options(client_options).unwrap();

    let resp = reqwest::Client::new()
        .post(format!("http://127.0.0.1:{}/short", test_env.app_port))
        .json(&serde_json::json!({ "url": TEST_SHORTENED_URL }))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let short_url = body.get("short_url").unwrap().as_str().unwrap();

    let mongo_collection: Collection<MongoShortUrl> =
        mongo_client.database(DATABASE).collection(COLLECTION);

    let result = mongo_collection
        .find_one(doc! { "_id": short_url})
        .await
        .unwrap().unwrap();

    assert_eq!(result.long_url, TEST_SHORTENED_URL.to_string());
}