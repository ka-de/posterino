// src/social/twitter.rs
use oauth1::Token;
use reqwest::Client;
use serde_json::json;
use crate::config::Config;
use super::SocialClient;

pub struct TwitterClient {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: String,
    client: Client,
}

#[async_trait::async_trait]
impl SocialClient for TwitterClient {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::load("twitter")?;
        
        Ok(TwitterClient {
            consumer_key: config.get("consumer_key")?,
            consumer_secret: config.get("consumer_secret")?,
            access_token: config.get("access_token")?,
            access_token_secret: config.get("access_token_secret")?,
            client: Client::new(),
        })
    }

    async fn post(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Create consumer token
        let consumer = Token::new(
            self.consumer_key.clone(),
            self.consumer_secret.clone(),
        );

        // Create access token
        let access = Token::new(
            self.access_token.clone(),
            self.access_token_secret.clone(),
        );

        let auth_header = oauth1::authorize(
            "POST",
            "https://api.twitter.com/2/tweets",
            &consumer,
            Some(&access),
            None,
        );

        let response = self.client
            .post("https://api.twitter.com/2/tweets")
            .header("Authorization", auth_header)
            .json(&json!({
                "text": message
            }))
            .send()
            .await?;

        if response.status().is_success() {
            println!("Tweet posted successfully!");
            println!("Response: {}", response.text().await?);
        } else {
            println!("Failed to post tweet: {}", response.text().await?);
        }

        Ok(())
    }
}
