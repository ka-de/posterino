// src/social/mod.rs
pub mod twitter;
pub mod mastodon;

#[async_trait::async_trait]
pub trait SocialClient {
    async fn post(&self, message: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn new() -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;
}
