pub async fn handle_root_get() -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const NAME: &str = env!("CARGO_PKG_NAME");
    let hostname = std::env::var("HOSTNAME").unwrap();

    format!(
        r#"{{"application_name": "{}","version": "{}","hostname":"{}"}}"#,
        NAME,
        VERSION,
        hostname
    )
}
