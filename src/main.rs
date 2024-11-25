// src/main.rs
use clap::Parser;
mod social;
mod config;

use social::twitter::TwitterClient;
use social::mastodon::MastodonClient;
use social::bluesky::BlueskyClient;
use social::SocialClient;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The message to post
    message: Vec<String>,

    /// The platform to post to (twitter, mastodon, bluesky)
    #[arg(short, long, default_value = "twitter")]
    platform: String,

    /// Post to all configured platforms
    #[arg(short, long)]
    all: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Join the message parts with spaces
    let joined_message = args.message.join(" ");
    
    // Replace the newline token with actual newlines
    let full_message = joined_message.replace("\\n", "\n");
    
    if args.all {
        // Try posting to all platforms
        let mut errors = Vec::new();

        // Try Twitter
        if let Err(e) = post_to_platform("twitter", &full_message).await {
            errors.push(format!("Twitter error: {e}"));
        }

        // Try Mastodon
        if let Err(e) = post_to_platform("mastodon", &full_message).await {
            errors.push(format!("Mastodon error: {e}"));
        }

        // Try Bluesky
        if let Err(e) = post_to_platform("bluesky", &full_message).await {
            errors.push(format!("Bluesky error: {e}"));
        }

        // If there were any errors, report them
        if !errors.is_empty() {
            return Err(errors.join("\n").into());
        }
    } else {
        // Original single platform behavior
        match args.platform.to_lowercase().as_str() {
            "twitter" => post_to_platform("twitter", &full_message).await?,
            "mastodon" => post_to_platform("mastodon", &full_message).await?,
            "bluesky" => post_to_platform("bluesky", &full_message).await?,
            _ => {
                return Err(format!(
                    "Unsupported platform: {}. Supported platforms are: twitter, mastodon, bluesky",
                    args.platform
                ).into());
            }
        }
    }

    Ok(())
}

async fn post_to_platform(platform: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    match platform {
        "twitter" => {
            let client = TwitterClient::new()?;
            client.post(message).await?;
        },
        "mastodon" => {
            let client = MastodonClient::new()?;
            client.post(message).await?;
        },
        "bluesky" => {
            let client = BlueskyClient::new()?;
            client.post(message).await?;
        },
        _ => return Err(format!("Unsupported platform: {platform}").into()),
    }
    Ok(())
}
