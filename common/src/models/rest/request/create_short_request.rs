use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateShortRequest {
    pub long_url: String,
    pub expiration: Option<i64>,
}

impl CreateShortRequest {
    pub fn new(long_url: String, expiration: Option<i64>) -> CreateShortRequest {
        CreateShortRequest {
            long_url,
            expiration,
        }
    }
}
