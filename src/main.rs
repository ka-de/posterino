// src/main.rs
use clap::Parser;
mod social;
mod config;

use social::twitter::TwitterClient;
use social::mastodon::MastodonClient;
use social::SocialClient;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The message to post
    message: Vec<String>,

    /// The platform to post to (twitter, mastodon, bluesky)
    #[arg(short, long, default_value = "twitter")]
    platform: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Join the message parts with spaces
    let joined_message = args.message.join(" ");
    
    // Replace the newline token with actual newlines
    let full_message = joined_message.replace("\\n", "\n");
    
    match args.platform.to_lowercase().as_str() {
        "twitter" => {
            let client = TwitterClient::new()?;
            client.post(&full_message).await?;
        },
        "mastodon" => {
            let client = MastodonClient::new()?;
            client.post(&full_message).await?;
        },
        _ => {
            return Err(format!("Unsupported platform: {}. Supported platforms are: twitter, mastodon", args.platform).into());
        }
    }

    Ok(())
}
