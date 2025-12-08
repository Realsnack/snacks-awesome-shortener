use crate::integration::common::{BASE_URL, HEALTH_ENDPOINT, build_test_env};

#[tokio::test]
async fn get_health_services_healthy() {
    let test_env = build_test_env(true, true).await;

    let resp = reqwest::Client::new()
        .get(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, HEALTH_ENDPOINT
        ))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let status = body.get("status").unwrap();
    let redis_service_ping = body.pointer("/services/redis/ping").unwrap();
    let mongo_service_ping = body.pointer("/services/mongo/ping").unwrap();

    assert_eq!("UP", status);
    assert_eq!("PONG", redis_service_ping);
    assert!(mongo_service_ping.to_string().contains("ok"));
}

#[tokio::test]
async fn get_health_redis_unavailable() {
    let test_env = build_test_env(false, true).await;

    let resp = reqwest::Client::new()
        .get(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, HEALTH_ENDPOINT
        ))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let status = body.get("status").unwrap();
    let redis_service_ping = body.pointer("/services/redis/ping").unwrap();
    let mongo_service_ping = body.pointer("/services/mongo/ping").unwrap();

    assert_eq!("UP", status);
    assert!(redis_service_ping.to_string().contains("No redis"));
    assert!(mongo_service_ping.to_string().contains("ok"));
}

#[tokio::test]
async fn get_health_mongo_unavailable() {
    let test_env = build_test_env(true, false).await;

    let resp = reqwest::Client::new()
        .get(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, HEALTH_ENDPOINT
        ))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let status = body.get("status").unwrap();
    let redis_service_ping = body.pointer("/services/redis/ping").unwrap();
    let mongo_service_ping = body.pointer("/services/mongo/ping").unwrap();

    assert_eq!("UP", status);
    assert_eq!("PONG", redis_service_ping);
    assert!(mongo_service_ping.to_string().contains("Received mongo error"));
}

#[tokio::test]
async fn get_health_redis_and_mongo_unavailable() {
    let test_env = build_test_env(false, false).await;

    let resp = reqwest::Client::new()
        .get(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, HEALTH_ENDPOINT
        ))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let status = body.get("status").unwrap();
    let redis_service_ping = body.pointer("/services/redis/ping").unwrap();
    let mongo_service_ping = body.pointer("/services/mongo/ping").unwrap();

    // TODO: Service shouldn't be UP at this point
    assert_eq!("UP", status);
    assert!(redis_service_ping.to_string().contains("No redis"));
    assert!(mongo_service_ping.to_string().contains("Received mongo error"));
}
