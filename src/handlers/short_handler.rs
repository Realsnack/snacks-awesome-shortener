use tide::{Response, StatusCode};
use crate::state::AppState;
use serde_json::json;
use tide::log::info;
use crate::models::short_url::ShortUrl;

pub async fn handle_short_redirect(req: tide::Request<AppState>) -> tide::Result {
    let short_url = req.param("short")?.to_string();
    info!("Searching for short_url: '{}'", short_url);
    let service = &req.state().shorts_service;

    let response = match service.get_long_url(short_url).await {
        Some(long_url) => {
            let short_url_object: ShortUrl = serde_json::from_str(&long_url)?;
            let mut res = tide::Response::new(StatusCode::Found);
            res.insert_header("Location", short_url_object.long_url);
            res
        },
        None => {
            info!("short_url not found in redis");
            Response::new(StatusCode::NotFound)
        }
    };

    Ok(response)
}

pub async fn handle_short_get(req: tide::Request<AppState>) -> tide::Result {
    let short_url = req.param("short")?.to_string();

    let service = &req.state().shorts_service;
    let response = match service.get_long_url(short_url).await {
        Some(short_url) => {
            Response::builder(StatusCode::Ok)
                .body(short_url)
                .build()
        },
        None => {
            info!("short_url not found in redis");
            Response::new(StatusCode::NotFound)
        }
    };

    Ok(response)
}

pub async fn handle_short_post(mut req: tide::Request<AppState>) -> tide::Result {
    let body: serde_json::Value = req.body_json().await?;
    let long_url = body["url"].as_str().unwrap_or_default();

    let service = &req.state().shorts_service;
    let short_url = service.generate_short_url(long_url.into()).await;

    Ok(Response::builder(StatusCode::Ok)
        .body(json!(short_url))
        .build())
}
