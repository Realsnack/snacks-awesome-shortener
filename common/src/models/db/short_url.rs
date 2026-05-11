use chrono::NaiveDateTime;

pub struct ShortUrl {
    pub short_url: String,
    pub long_url: String,
    pub expiration: usize,
    pub created: Option<NaiveDateTime>,
    pub last_used: Option<NaiveDateTime>,
    pub use_counter: usize,
    pub is_expired: bool,
}

impl ShortUrl {
    pub fn new(
        short_url: String,
        long_url: String,
        expiration: usize,
        created: Option<NaiveDateTime>,
        last_used: Option<NaiveDateTime>,
        use_counter: usize,
        is_expired: bool,
    ) -> ShortUrl {
        ShortUrl {
            short_url,
            long_url,
            expiration,
            created,
            last_used,
            use_counter,
            is_expired,
        }
    }
}
