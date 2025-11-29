use snacks_awesome_shortener::config::Config;
use testcontainers::{
    ContainerAsync, GenericImage,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};
use tokio::task;

#[tokio::test]
async fn post_short_url_creates_entry() {
    let test_env = build_test_env(true, true).await;

    let url = format!("http://127.0.0.1:{}/short", test_env.app_port);

    let resp = reqwest::Client::new()
        .post(&url)
        .json(&serde_json::json!({ "url":"https://hltv.org" }))
        .send()
        .await
        .unwrap();

    let status = resp.status();
    let body: serde_json::Value = resp.json().await.unwrap();
    let short_url = body.get("short_url");
    let long_url = body.get("long_url");
    let expiration = body.get("expiration");

    assert_eq!(status, 200);
    assert!(short_url.is_some());
    assert!(long_url.is_some());
    assert!(expiration.is_some());
}

#[tokio::test]
async fn post_short_url_rejects_invalid_body() {
    let test_env = build_test_env(true, true).await;

    let url = format!("http://127.0.0.1:{}/short", test_env.app_port);

    let resp = reqwest::Client::new()
        .post(&url)
        .json(&serde_json::json!({ "surl":"https://hltv.org" }))
        .send()
        .await
        .unwrap();
    let status = resp.status();
    let body: serde_json::Value = resp.json().await.unwrap();

    assert_eq!(status, 400);
    assert_eq!(body.get("reason").unwrap(), "No 'url' in request body")
}

#[tokio::test]
async fn post_short_url_empty_body() {
    let test_env = build_test_env(true, true).await;

    let url = format!("http://127.0.0.1:{}/short", test_env.app_port);

    let resp = reqwest::Client::new()
        .post(&url)
        .send()
        .await
        .unwrap();
    let status = resp.status();
    let body: serde_json::Value = resp.json().await.unwrap();

    assert_eq!(status, 400);
    assert_eq!(body.get("reason").unwrap(), "No 'url' in request body")
}

#[tokio::test]
async fn get_short_url_object_non_existing() {
    let test_env = build_test_env(true, true).await;

    let url = format!("http://127.0.0.1:{}/short/should_not_exist", test_env.app_port);

    let resp = reqwest::get(&url).await.unwrap();
    let status = resp.status();

    assert_eq!(status, 404);
}

#[tokio::test]
async fn get_short_url_non_existing() {
    let test_env = build_test_env(true, true).await;

    let url = format!("http://127.0.0.1:{}/should_not_exist", test_env.app_port);

    let resp = reqwest::get(&url).await.unwrap();
    let status = resp.status();

    assert_eq!(status, 404);
}

async fn build_test_env(redis_enabled: bool, mongo_enabled: bool) -> TestEnv {
    let (redis_container, redis_url) = match redis_enabled {
        true => setup_redis().await,
        false => (None, "redis://0.0.0.0:0".to_string())
    };
    let (mongo_container, mongo_url) = match mongo_enabled {
        true => setup_mongo().await,
        false => (None, "mongodb://0.0.0.0:0".to_string())
    };

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", "127.0.0.1", "0"))
        .await
        .unwrap();

    let address = listener.local_addr().unwrap();
    let app_address = address.ip();
    let app_port = address.port();

    let app_config = Config::new(
        redis_url,
        mongo_url,
        app_address.to_string(),
        app_port.to_string(),
    );

    setup_app(app_config, listener).await;

    TestEnv { _redis_container: redis_container, _mongo_container: mongo_container,  app_port}
}

async fn setup_app(app_config: Config, listener: tokio::net::TcpListener) {
    let app = snacks_awesome_shortener::build_app(&app_config).await;

    task::spawn(async move {
        axum::serve(listener, app.into_make_service()).await.unwrap()
    });
}

async fn setup_redis() -> (Option<ContainerAsync<GenericImage>>, String) {
    let container = GenericImage::new("valkey/valkey", "latest")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .start()
        .await
        .unwrap();

    let redis_host = container.get_host().await.unwrap();
    let redis_port = container
        .get_host_port_ipv4(6379.tcp())
        .await
        .unwrap();
    let redis_url = format!("redis://{redis_host}:{redis_port}");

    (Some(container), redis_url)
}

async fn setup_mongo() -> (Option<ContainerAsync<GenericImage>>, String) {
    let container = GenericImage::new("mongo", "latest")
        .with_exposed_port(27017.tcp())
        .start()
        .await
        .unwrap();

    let mongo_host = container.get_host().await.unwrap();
    let mongo_port = container
        .get_host_port_ipv4(27017.tcp())
        .await
        .unwrap();
    let mongo_url = format!("mongodb://{mongo_host}:{mongo_port}");

    (Some(container), mongo_url)
}

struct TestEnv {
    _redis_container: Option<ContainerAsync<GenericImage>>,
    _mongo_container: Option<ContainerAsync<GenericImage>>,
    app_port: u16
}
