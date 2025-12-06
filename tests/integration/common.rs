use testcontainers::{ContainerAsync, GenericImage};
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use tokio::task;
use snacks_awesome_shortener::config::Config;

pub const SHORT_ENDPOINT: &str = "short";
pub const BASE_URL: &str = "http://127.0.0.1";
pub const TEST_SHORTENED_URL: &str = "https://hltv.org";

pub async fn build_test_env(redis_enabled: bool, mongo_enabled: bool) -> TestEnv {
    let (redis_container, redis_url) = match redis_enabled {
        true => setup_redis().await,
        false => (None, "redis://0.0.0.0:12345".to_string())
    };
    let (mongo_container, mongo_url) = match mongo_enabled {
        true => setup_mongo().await,
        false => (None, "mongodb://0.0.0.0:12345?serverSelectionTimeoutMs=100".to_string())
    };

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", "127.0.0.1", "0"))
        .await
        .unwrap();

    let address = listener.local_addr().unwrap();
    let app_address = address.ip();
    let app_port = address.port();

    let app_config = Config::new(
        redis_url.clone(),
        mongo_url.clone(),
        app_address.to_string(),
        app_port.to_string(),
    );

    setup_app(app_config, listener).await;

    TestEnv { _redis_container: redis_container, redis_url, _mongo_container: mongo_container, mongo_url,  app_port}
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
    let redis_url = format!("redis://{}:{}", redis_host, redis_port);

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
    let mongo_url = format!("mongodb://{}:{}", mongo_host, mongo_port);

    (Some(container), mongo_url)
}

pub struct TestEnv {
    _redis_container: Option<ContainerAsync<GenericImage>>,
    pub redis_url: String,
    _mongo_container: Option<ContainerAsync<GenericImage>>,
    pub mongo_url: String,
    pub app_port: u16
}
