use crate::integration::common;
use crate::integration::common::{BASE_URL, SHORT_ENDPOINT, TEST_SHORTENED_URL};
use common::build_test_env;
use reqwest::redirect;

#[tokio::test]
async fn post_short_url_creates_entry() {
    let test_env = build_test_env(true, true).await;

    let resp = reqwest::Client::new()
        .post(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, SHORT_ENDPOINT
        ))
        .json(&serde_json::json!({ "url": TEST_SHORTENED_URL }))
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
    assert_eq!(long_url.unwrap(), TEST_SHORTENED_URL);
    assert!(expiration.is_some());
}

#[tokio::test]
async fn post_short_url_rejects_invalid_body() {
    let test_env = build_test_env(true, true).await;

    let resp = reqwest::Client::new()
        .post(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, SHORT_ENDPOINT
        ))
        .json(&serde_json::json!({ "surl": TEST_SHORTENED_URL }))
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

    let resp = reqwest::Client::new()
        .post(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, SHORT_ENDPOINT
        ))
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

    let resp = reqwest::Client::new()
        .get(format!(
            "{}:{}/{}/{}",
            BASE_URL, test_env.app_port, SHORT_ENDPOINT, "should_not_exist"
        ))
        .send()
        .await
        .unwrap();
    let status = resp.status();

    assert_eq!(status, 404);
}

#[tokio::test]
async fn get_short_url_non_existing() {
    let test_env = build_test_env(true, true).await;

    let resp = reqwest::Client::new()
        .get(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, "should_not_exist"
        ))
        .send()
        .await
        .unwrap();
    let status = resp.status();

    assert_eq!(status, 404);
}

#[tokio::test]
async fn get_short_url_redirect() {
    let test_env = build_test_env(true, true).await;

    let resp = reqwest::Client::new()
        .post(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, SHORT_ENDPOINT
        ))
        .json(&serde_json::json!({ "url": TEST_SHORTENED_URL }))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let short_url = body.get("short_url").unwrap().as_str().unwrap();

    let client = reqwest::Client::builder()
        .redirect(redirect::Policy::none())
        .build()
        .unwrap();

    let resp = client
        .get(format!("{}:{}/{}", BASE_URL, test_env.app_port, short_url))
        .send()
        .await
        .unwrap();
    let status = resp.status();

    assert_eq!(status, 307);
}

#[tokio::test]
async fn get_short_url_object() {
    let test_env = build_test_env(true, true).await;

    let resp = reqwest::Client::new()
        .post(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, SHORT_ENDPOINT
        ))
        .json(&serde_json::json!({ "url": TEST_SHORTENED_URL }))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let short_url = body.get("short_url").unwrap().as_str().unwrap();

    let client = reqwest::Client::builder()
        .redirect(redirect::Policy::none())
        .build()
        .unwrap();

    let resp = client
        .get(format!("{}:{}/{}/{}", BASE_URL, test_env.app_port, SHORT_ENDPOINT, short_url))
        .send()
        .await
        .unwrap();
    let status = resp.status();
    let body: serde_json::Value = resp.json().await.unwrap();
    let long_url = body.get("long_url");

    assert_eq!(status, 200);
    assert!(long_url.is_some());
    assert_eq!(long_url.unwrap(), TEST_SHORTENED_URL);
}

#[tokio::test]
async fn post_short_url_creates_entry_unavailable_redis() {
    let test_env = build_test_env(false, true).await;

    let resp = reqwest::Client::new()
        .post(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, SHORT_ENDPOINT
        ))
        .json(&serde_json::json!({ "url": TEST_SHORTENED_URL }))
        .send()
        .await;

    let resp = match resp {
        Ok(resp) => resp,
        Err(e) => {
            panic!("Failed to get response from API: {}", e);
        }
    };

    let status = resp.status();

    assert_eq!(status, 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    let short_url = body.get("short_url");
    let long_url = body.get("long_url");
    let expiration = body.get("expiration");

    assert_eq!(status, 200);
    assert!(short_url.is_some());
    assert_eq!(short_url.unwrap().to_string().len(), 8);
    assert!(long_url.is_some());
    assert_eq!(long_url.unwrap(), TEST_SHORTENED_URL);
    assert!(expiration.is_some());
}

#[tokio::test]
async fn get_short_url_object_unavailable_redis() {
    let test_env = build_test_env(false, true).await;

    let resp = reqwest::Client::new()
        .post(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, SHORT_ENDPOINT
        ))
        .json(&serde_json::json!({ "url": TEST_SHORTENED_URL }))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let short_url = body.get("short_url").unwrap().as_str().unwrap();

    let client = reqwest::Client::builder()
        .redirect(redirect::Policy::none())
        .build()
        .unwrap();

    let resp = client
        .get(format!(
            "{}:{}/{}/{}",
            BASE_URL, test_env.app_port, SHORT_ENDPOINT, short_url
        ))
        .send()
        .await
        .unwrap();
    let status = resp.status();
    let body: serde_json::Value = resp.json().await.unwrap();
    let long_url = body.get("long_url");

    assert_eq!(status, 200);
    assert!(long_url.is_some());
    assert_eq!(long_url.unwrap(), TEST_SHORTENED_URL);
}

#[tokio::test]
async fn post_short_url_creates_entry_unavailable_mongo() {
    let test_env = build_test_env(true, false).await;

    let resp = reqwest::Client::new()
        .post(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, SHORT_ENDPOINT
        ))
        .json(&serde_json::json!({ "url": TEST_SHORTENED_URL }))
        .send()
        .await;

    let resp = match resp {
        Ok(resp) => resp,
        Err(e) => {
            panic!("Failed to get response from API: {}", e);
        }
    };

    let status = resp.status();

    assert_eq!(status, 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    let short_url = body.get("short_url");
    let long_url = body.get("long_url");
    let expiration = body.get("expiration");

    assert_eq!(status, 200);
    assert!(short_url.is_some());
    assert_eq!(short_url.unwrap().to_string().len(), 8);
    assert!(long_url.is_some());
    assert_eq!(long_url.unwrap(), TEST_SHORTENED_URL);
    assert!(expiration.is_some());
}

#[tokio::test]
async fn get_short_url_object_unavailable_mongo() {
    let test_env = build_test_env(true, false).await;

    let resp = reqwest::Client::new()
        .post(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, SHORT_ENDPOINT
        ))
        .json(&serde_json::json!({ "url": TEST_SHORTENED_URL }))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    let short_url = body.get("short_url").unwrap().as_str().unwrap();

    let client = reqwest::Client::builder()
        .redirect(redirect::Policy::none())
        .build()
        .unwrap();

    let resp = client
        .get(format!(
            "{}:{}/{}/{}",
            BASE_URL, test_env.app_port, SHORT_ENDPOINT, short_url
        ))
        .send()
        .await
        .unwrap();
    let status = resp.status();
    let body: serde_json::Value = resp.json().await.unwrap();
    let long_url = body.get("long_url");

    assert_eq!(status, 200);
    assert!(long_url.is_some());
    assert_eq!(long_url.unwrap(), TEST_SHORTENED_URL);
}

#[tokio::test]
async fn post_short_url_unavailable_redis_and_mongo() {
    let test_env = build_test_env(false, false).await;

    let resp = reqwest::Client::new()
        .post(format!(
            "{}:{}/{}",
            BASE_URL, test_env.app_port, SHORT_ENDPOINT
        ))
        .json(&serde_json::json!({ "url": TEST_SHORTENED_URL }))
        .send()
        .await;

    let resp = match resp {
        Ok(resp) => resp,
        Err(e) => {
            panic!("Failed to get response from API: {}", e);
        }
    };

    let status = resp.status();

    assert_eq!(status, 500);
}

#[tokio::test]
async fn get_short_url_object_unavailable_redis_and_mongo() {
    let test_env = build_test_env(false, false).await;

    let resp = reqwest::Client::new()
        .get(format!(
            "{}:{}/{}/{}",
            BASE_URL, test_env.app_port, SHORT_ENDPOINT, "random_url"
        ))
        .send()
        .await
        .unwrap();

    let status = resp.status();

    assert_eq!(status, 404);
}
