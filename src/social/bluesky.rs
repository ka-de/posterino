use reqwest::Client;
use serde_json::json;
use crate::config::Config;
use super::SocialClient;
use regex::Regex;

pub struct BlueskyClient {
    identifier: String,
    password: String,
    instance_url: String,
    client: Client,
}

#[derive(Debug)]
struct UrlFacet {
    start: usize,
    end: usize,
    url: String,
}

impl BlueskyClient {
    fn detect_urls(text: &str) -> Vec<UrlFacet> {
        let url_regex = Regex::new(r"https?://[^\s]+").unwrap();
        let mut facets = Vec::new();
        
        for mat in url_regex.find_iter(text) {
            facets.push(UrlFacet {
                start: mat.start(),
                end: mat.end(),
                url: mat.as_str().to_string(),
            });
        }
        
        facets
    }
}

#[async_trait::async_trait]
impl SocialClient for BlueskyClient {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::load("bluesky")?;
        
        Ok(BlueskyClient {
            identifier: config.get("identifier")?,
            password: config.get("password")?,
            instance_url: config.get("instance_url")?,
            client: Client::new(),
        })
    }

    async fn post(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        // First, authenticate to get session
        let auth_response = self.client
            .post(format!("{}/xrpc/com.atproto.server.createSession", self.instance_url))
            .json(&json!({
                "identifier": self.identifier,
                "password": self.password
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let access_jwt = auth_response["accessJwt"].as_str()
            .ok_or("Failed to get access token")?;

        // Detect URLs and create facets
        let url_facets = Self::detect_urls(message);
        let facets = url_facets.iter().map(|f| {
            json!({
                "index": {
                    "byteStart": f.start,
                    "byteEnd": f.end
                },
                "features": [{
                    "$type": "app.bsky.richtext.facet#link",
                    "uri": f.url
                }]
            })
        }).collect::<Vec<serde_json::Value>>();

        // Create the post
        let response = self.client
            .post(format!("{}/xrpc/com.atproto.repo.createRecord", self.instance_url))
            .header("Authorization", format!("Bearer {access_jwt}"))
            .json(&json!({
                "repo": self.identifier,
                "collection": "app.bsky.feed.post",
                "record": {
                    "$type": "app.bsky.feed.post",
                    "text": message,
                    "createdAt": chrono::Utc::now().to_rfc3339(),
                    "facets": facets
                }
            }))
            .send()
            .await?;

        if response.status().is_success() {
            println!("Post created successfully on Bluesky!");
            println!("Response: {}", response.text().await?);
        } else {
            println!("Failed to post to Bluesky: {}", response.text().await?);
        }

        Ok(())
    }
} 