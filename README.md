# Posterino 🚀

A powerful, command-line tool written in Rust for cross-posting to multiple social media platforms. Currently supports Twitter, Mastodon and Bluesky with an extensible architecture for adding more platforms.

## Features

- 📝 Post to multiple social media platforms from the command line
- 🔄 Support for Twitter/X, Mastodon, and Bluesky
- ⚡ Asynchronous operations using Tokio
- 🔐 Secure configuration management
- 🎯 Platform-specific authentication handling
- 📦 Easy to extend for additional platforms

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Building from Source

```bash
git clone https://github.com/yourusername/posterino.git
cd posterino
cargo build --release
```

The binary will be available at `target/release/posterino`

### Installing via Cargo on Windows with build-std

```bash
cargo install -Z build-std --target x86_64-pc-windows-msvc --path .
```

## Usage

### Basic Command Structure

```bash
posterino [OPTIONS] <MESSAGE>
```

### Post to Twitter (default)

```bash
posterino "Hello, World!"
```

### Post to Mastodon

```bash
posterino "Hello, World!" --platform mastodon
```

### Post to Bluesky

```bash
posterino "Hello, World!" --platform bluesky
```

### Post multi-line message

```bash
posterino "First line\nSecond line"
```

### Post to All Platforms

```bash
posterino "Hello, everyone!" --all
```

## Configuration

Before using Posterino, you need to set up your configuration file. The first run will create a template configuration file at:

- Linux/macOS: `~/.config/posterino/config.toml`
- Windows: `%APPDATA%\posterino\config.toml`

### Configuration Format

The configuration file is in TOML format. Here's an example configuration:

```toml
[twitter]
consumer_key = "your_twitter_consumer_key"
consumer_secret = "your_twitter_consumer_secret"
access_token = "your_twitter_access_token"
access_token_secret = "your_twitter_access_token_secret"

[mastodon]
access_token = "your_mastodon_access_token"
instance_url = "https://mastodon.social"

[bluesky]
identifier = "your.handle@bsky.social"
password = "your_app_password"
instance_url = "https://bsky.social"
```

### Getting API Keys

#### Twitter/X

1. Go to the [Twitter Developer Portal](https://developer.twitter.com/en/portal/dashboard)
2. Create a new app
3. Generate consumer keys and access tokens
4. Add them to your config file

#### Mastodon

1. Go to your Mastodon instance's settings
2. Navigate to Development > New Application
3. Generate access token
4. Add the token and instance URL to your config file

#### Bluesky

1. Go to your Bluesky account settings
2. Create an App Password (do not use your main account password)
3. Note your handle (identifier) and the app password
4. Add them to your config file along with the instance URL

## Architecture

Posterino is built with extensibility in mind, using Rust's trait system for platform implementations:

- `SocialClient` trait defines the interface for all platform implementations
- Platform-specific clients (Twitter, Mastodon) implement this trait
- Configuration management is centralized and type-safe
- Async/await for efficient network operations

## Development

### Project Structure

### Adding a New Platform

1. Create a new file in `src/social/` for your platform
2. Implement the `SocialClient` trait for your platform
3. Add configuration structure in `config.rs`
4. Update the platform matching in `main.rs`

### Running Tests

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [clap](https://github.com/clap-rs/clap) for CLI argument parsing
- Uses [tokio](https://tokio.rs/) for async runtime
- Uses [reqwest](https://github.com/seanmonstar/reqwest) for HTTP requests
- Uses [oauth1](https://github.com/dgrijalva/oauth1) for Twitter authentication
