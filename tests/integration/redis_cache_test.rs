use redis::{AsyncCommands, RedisResult};
use crate::integration::common::build_test_env;

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
        .post(format!("http://127.0.0.1:{}/short", test_env.app_port))
        .json(&serde_json::json!({ "url": "https://hltv.org" }))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let short_url = body.get("short_url").unwrap().as_str().unwrap();

    let result: RedisResult<String> = connection.get(short_url).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn post_short_redis_unavailable() {}