// src/config.rs
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::io;

#[derive(Deserialize)]
pub struct Config {
    data: std::collections::HashMap<String, String>,
}

impl Config {
    pub fn load(_platform: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        
        // Check if config file exists
        if !config_path.exists() {
            // Create config directory if it doesn't exist
            if let Some(config_dir) = config_path.parent() {
                fs::create_dir_all(config_dir)?;
            }
            
            // Create example config file
            let example_config = r#"[data]
consumer_key = "your_twitter_consumer_key"
consumer_secret = "your_twitter_consumer_secret"
access_token = "your_twitter_access_token"
access_token_secret = "your_twitter_access_token_secret"
"#;
            
            fs::write(&config_path, example_config)?;
            
            return Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Config file not found. An example config has been created at: {}\nPlease fill in your Twitter API credentials.",
                    config_path.display()
                )
            )));
        }

        let config_str = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }

    pub fn get(&self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.data
            .get(key)
            .cloned()
            .ok_or_else(|| format!("Config key '{}' not found", key).into())
    }

    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| "Could not find config directory")?;
        path.push("posterino");
        path.push("config.toml");
        Ok(path)
    }
}
