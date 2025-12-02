use reqwest::redirect;
use crate::integration::common;
use common::build_test_env;

#[tokio::test]
async fn post_short_url_creates_entry() {
    let test_env = build_test_env(true, true).await;
    let post_url = "https://hltv.org";

    let url = format!("http://127.0.0.1:{}/short", test_env.app_port);

    let resp = reqwest::Client::new()
        .post(&url)
        .json(&serde_json::json!({ "url": post_url }))
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
    assert_eq!(short_url.unwrap().to_string().len(), 8);
    assert!(long_url.is_some());
    assert_eq!(long_url.unwrap(), post_url);
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

    let resp = reqwest::Client::new().post(&url).send().await.unwrap();
    let status = resp.status();
    let body: serde_json::Value = resp.json().await.unwrap();

    assert_eq!(status, 400);
    assert_eq!(body.get("reason").unwrap(), "No 'url' in request body")
}

#[tokio::test]
async fn get_short_url_object_non_existing() {
    let test_env = build_test_env(true, true).await;

    let url = format!(
        "http://127.0.0.1:{}/short/should_not_exist",
        test_env.app_port
    );
    let resp = reqwest::Client::new().get(&url).send().await.unwrap();
    let status = resp.status();

    assert_eq!(status, 404);
}

#[tokio::test]
async fn get_short_url_non_existing() {
    let test_env = build_test_env(true, true).await;

    let url = format!("http://127.0.0.1:{}/should_not_exist", test_env.app_port);

    let resp = reqwest::Client::new().get(&url).send().await.unwrap();
    let status = resp.status();

    assert_eq!(status, 404);
}

#[tokio::test]
async fn get_short_url_redirect() {
    let test_env = build_test_env(true, true).await;

    let resp = reqwest::Client::new()
        .post(format!("http://127.0.0.1:{}/short", test_env.app_port))
        .json(&serde_json::json!({ "url": "https://hltv.org" }))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let short_url = body.get("short_url").unwrap().as_str().unwrap();

    let url = format!("http://127.0.0.1:{}/{}", test_env.app_port, short_url);

    let client = reqwest::Client::builder()
        .redirect(redirect::Policy::none())
        .build()
        .unwrap();

    let resp = client.get(&url).send().await.unwrap();
    let status = resp.status();

    assert_eq!(status, 307);
}

#[tokio::test]
async fn get_short_url_object() {
    let test_env = build_test_env(true, true).await;
    let post_url = "https://hltv.org";

    let resp = reqwest::Client::new()
        .post(format!("http://127.0.0.1:{}/short", test_env.app_port))
        .json(&serde_json::json!({ "url": post_url }))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let short_url = body.get("short_url").unwrap().as_str().unwrap();

    let url = format!("http://127.0.0.1:{}/short/{}", test_env.app_port, short_url);

    let client = reqwest::Client::builder()
        .redirect(redirect::Policy::none())
        .build()
        .unwrap();

    let resp = client.get(&url).send().await.unwrap();
    let status = resp.status();
    let body: serde_json::Value = resp.json().await.unwrap();
    let long_url = body.get("long_url");

    assert_eq!(status, 200);
    assert!(long_url.is_some());
    assert_eq!(long_url.unwrap(), post_url);
}
