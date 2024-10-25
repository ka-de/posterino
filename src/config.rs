// src/config.rs
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::io;

#[derive(Deserialize)]
pub struct Config {
    twitter: Option<TwitterConfig>,
    mastodon: Option<MastodonConfig>,
}

#[derive(Deserialize)]
struct TwitterConfig {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: String,
}

#[derive(Deserialize)]
struct MastodonConfig {
    access_token: String,
    instance_url: String,
}

impl Config {
    pub fn load(platform: &str) -> Result<PlatformConfig, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        
        if !config_path.exists() {
            if let Some(config_dir) = config_path.parent() {
                fs::create_dir_all(config_dir)?;
            }
            
            let example_config = r#"[twitter]
consumer_key = "your_twitter_consumer_key"
consumer_secret = "your_twitter_consumer_secret"
access_token = "your_twitter_access_token"
access_token_secret = "your_twitter_access_token_secret"

[mastodon]
access_token = "your_mastodon_access_token"
instance_url = "https://mastodon.social"
"#;
            
            fs::write(&config_path, example_config)?;
            
            return Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Config file not found. An example config has been created at: {}\nPlease fill in your credentials.",
                    config_path.display()
                )
            )));
        }

        let config_str = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config_str)?;
        
        match platform {
            "twitter" => {
                if let Some(twitter_config) = config.twitter {
                    Ok(PlatformConfig::Twitter(twitter_config))
                } else {
                    Err("Twitter configuration not found in config file".into())
                }
            },
            "mastodon" => {
                if let Some(mastodon_config) = config.mastodon {
                    Ok(PlatformConfig::Mastodon(mastodon_config))
                } else {
                    Err("Mastodon configuration not found in config file".into())
                }
            },
            _ => Err(format!("Unsupported platform: {}", platform).into())
        }
    }

    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| "Could not find config directory")?;
        path.push("posterino");
        path.push("config.toml");
        Ok(path)
    }
}

pub enum PlatformConfig {
    Twitter(TwitterConfig),
    Mastodon(MastodonConfig),
}

impl PlatformConfig {
    pub fn get(&self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            PlatformConfig::Twitter(config) => {
                match key {
                    "consumer_key" => Ok(config.consumer_key.clone()),
                    "consumer_secret" => Ok(config.consumer_secret.clone()),
                    "access_token" => Ok(config.access_token.clone()),
                    "access_token_secret" => Ok(config.access_token_secret.clone()),
                    _ => Err(format!("Unknown Twitter config key: {}", key).into())
                }
            },
            PlatformConfig::Mastodon(config) => {
                match key {
                    "access_token" => Ok(config.access_token.clone()),
                    "instance_url" => Ok(config.instance_url.clone()),
                    _ => Err(format!("Unknown Mastodon config key: {}", key).into())
                }
            }
        }
    }
}
