#[derive(Debug)]
pub struct ShortUrl {
    short_url: String,
    long_url: String,
    expiration: usize,
}

impl ShortUrl {
    pub fn new(short_url: String, long_url: String, expiration: usize) -> ShortUrl{
        ShortUrl {
            short_url,
            long_url,
            expiration
        }
    }
}