// src/main.rs
use clap::Parser;
mod social;
mod config;

use social::twitter::TwitterClient;
use social::SocialClient;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The message to post
    message: String,

    /// The platform to post to (twitter, mastodon, bluesky)
    #[arg(short, long, default_value = "twitter")]
    platform: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    match args.platform.as_str() {
        "twitter" => {
            let client = TwitterClient::new()?;
            client.post(&args.message).await?;
        },
        // Future platforms will be added here
        _ => println!("Unsupported platform: {}", args.platform),
    }

    Ok(())
}
