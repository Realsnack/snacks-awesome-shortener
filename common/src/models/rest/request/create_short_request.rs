use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateShortRequest {
    pub long_url: String,
    pub expiration: Option<i64>,
}

impl CreateShortRequest {
    #[must_use]
    pub const fn new(long_url: String, expiration: Option<i64>) -> Self {
        Self {
            long_url,
            expiration,
        }
    }
}
