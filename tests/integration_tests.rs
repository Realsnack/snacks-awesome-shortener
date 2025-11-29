use snacks_awesome_shortener::config::Config;
use testcontainers::{
    ContainerAsync, GenericImage,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};
use tokio::task;
use snacks_awesome_shortener::setup_logging;

#[tokio::test]
async fn test_post_short_should_succeed() {
    let test_env = build_test_env().await;

    let url = format!("http://127.0.0.1:{}/short", test_env.app_port);
    println!("Calling url: {}", url);

    let resp = reqwest::Client::new()
        .post(&url)
        .json(&serde_json::json!({ "url":"https://hltv.org" }))
        .send()
        .await
        .unwrap();

    println!("{:?}", resp);

    assert_eq!(resp.status(), 200);
}

async fn build_test_env() -> TestEnv {
    setup_logging();
    let redis_container = setup_redis().await;
    let mongo_container = setup_mongo().await;

    let redis_host = redis_container.get_host().await.unwrap();
    let redis_port = redis_container
        .get_host_port_ipv4(6379.tcp())
        .await
        .unwrap();
    let mongo_host = mongo_container.get_host().await.unwrap();
    let mongo_port = mongo_container
        .get_host_port_ipv4(27017.tcp())
        .await
        .unwrap();

    let redis_url = format!("redis://{redis_host}:{redis_port}");
    let mongo_url = format!("mongodb://{mongo_host}:{mongo_port}");

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
    println!("Config: {:?}", app_config);

    setup_app(app_config, listener).await;

    TestEnv { _redis_container: redis_container, _mongo_container: mongo_container,  app_port}
}

async fn setup_app(app_config: Config, listener: tokio::net::TcpListener) {
    let app = snacks_awesome_shortener::build_app(&app_config).await;

    task::spawn(async move {
        axum::serve(listener, app.into_make_service()).await.unwrap()
    });
}

async fn setup_redis() -> ContainerAsync<GenericImage> {
    GenericImage::new("valkey/valkey", "latest")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .start()
        .await
        .unwrap()
}

async fn setup_mongo() -> ContainerAsync<GenericImage> {
    GenericImage::new("mongo", "latest")
        .with_exposed_port(27017.tcp())
        //.with_wait_for(WaitFor::Duration { length: core::time::Duration::from_secs(5), })
        .start()
        .await
        .unwrap()
}

struct TestEnv {
    _redis_container: ContainerAsync<GenericImage>,
    _mongo_container: ContainerAsync<GenericImage>,
    app_port: u16
}
