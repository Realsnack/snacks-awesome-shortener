use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ShortRequest {
    pub url: String,
    pub expiration: Option<usize>
}