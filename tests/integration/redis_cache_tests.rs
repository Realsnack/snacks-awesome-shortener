use redis::{AsyncCommands, RedisResult};
use snacks_awesome_shortener::models::short_url::ShortUrl;
use crate::integration::common::{build_test_env, BASE_URL, SHORT_ENDPOINT, TEST_SHORTENED_URL};

#[tokio::test]
async fn post_short_redis_key_created() {
    let test_env = build_test_env(true, true).await;
    let redis_client = redis::Client::open(test_env.redis_url).unwrap();
    let mut connection = match redis_client.get_multiplexed_async_connection().await {
        Ok(connection) => connection,
        Err(e) => {
            panic!("Redis connection error: {}", e);
        }
    };

    let resp = reqwest::Client::new()
        .post(format!("{}:{}/{}", BASE_URL, test_env.app_port, SHORT_ENDPOINT))
        .json(&serde_json::json!({ "url": TEST_SHORTENED_URL }))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let short_url = body.get("short_url").unwrap().as_str().unwrap();

    let result: RedisResult<String> = connection.get(short_url).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn post_short_redis_key_created_and_matching() {
    let test_env = build_test_env(true, true).await;
    let redis_client = redis::Client::open(test_env.redis_url).unwrap();
    let mut connection = match redis_client.get_multiplexed_async_connection().await {
        Ok(connection) => connection,
        Err(e) => {
            panic!("Redis connection error: {}", e);
        }
    };

    let resp = reqwest::Client::new()
        .post(format!("{}:{}/{}", BASE_URL, test_env.app_port, SHORT_ENDPOINT))
        .json(&serde_json::json!({ "url": TEST_SHORTENED_URL }))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let short_url = body.get("short_url").unwrap().as_str().unwrap();

    let result: String = connection.get(short_url).await.unwrap();
    let redis_key: ShortUrl = serde_json::from_str(result.as_str()).unwrap();

    assert_eq!(redis_key.long_url, TEST_SHORTENED_URL);
}
