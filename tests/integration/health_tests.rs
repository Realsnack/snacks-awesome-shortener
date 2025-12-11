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
    let redis_service_ping = body.pointer("/services/redis/status").unwrap();
    let mongo_service_status = body.pointer("/services/mongo/status").unwrap();

    assert_eq!("HEALTHY", status);
    assert_eq!("HEALTHY", redis_service_ping);
    assert_eq!("HEALTHY", mongo_service_status);
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
    let redis_service_ping = body.pointer("/services/redis/status").unwrap();
    let mongo_service_status = body.pointer("/services/mongo/status").unwrap();

    assert_eq!("DEGRADED", status);
    assert_eq!("UNHEALTHY", redis_service_ping);
    assert_eq!("HEALTHY", mongo_service_status);
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
    let redis_service_ping = body.pointer("/services/redis/status").unwrap();
    let mongo_service_status = body.pointer("/services/mongo/status").unwrap();

    assert_eq!("DEGRADED", status);
    assert_eq!("HEALTHY", redis_service_ping);
    assert_eq!("UNHEALTHY", mongo_service_status);
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
    let redis_service_ping = body.pointer("/services/redis/status").unwrap();
    let mongo_service_status = body.pointer("/services/mongo/status").unwrap();

    assert_eq!("UNHEALTHY", status);
    assert_eq!("UNHEALTHY", redis_service_ping);
    assert_eq!("UNHEALTHY", mongo_service_status);
}
