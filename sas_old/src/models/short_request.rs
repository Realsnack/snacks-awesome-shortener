use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ShortRequest {
    pub url: String,
    pub expiration: Option<usize>
}