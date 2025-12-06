use axum::extract::Path;

pub async fn handle_root_get() -> String {
    String::from("Hello, world!")
}

pub async fn handle_greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}