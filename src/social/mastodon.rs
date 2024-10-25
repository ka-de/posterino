// src/social/mastodon.rs
use reqwest::Client;
use serde_json::json;
use crate::config::Config;
use super::SocialClient;

pub struct MastodonClient {
    access_token: String,
    instance_url: String,
    client: Client,
}

#[async_trait::async_trait]
impl SocialClient for MastodonClient {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::load("mastodon")?;
        
        Ok(MastodonClient {
            access_token: config.get("access_token")?,
            instance_url: config.get("instance_url")?,
            client: Client::new(),
        })
    }

    async fn post(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = format!("{}/api/v1/statuses", self.instance_url.trim_end_matches('/'));
        
        let response = self.client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&json!({
                "status": message,
                "visibility": "public"
            }))
            .send()
            .await?;

        if response.status().is_success() {
            println!("Status posted successfully to Mastodon!");
            println!("Response: {}", response.text().await?);
        } else {
            println!("Failed to post to Mastodon: {}", response.text().await?);
        }

        Ok(())
    }
}
