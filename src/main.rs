//! Osiris: Frubble's integrations service. It holds organizations' OAuth connections to
//! providers (GitHub, Linear, Slack), keeps their tokens fresh, and runs reads with them for its
//! clients, over an HTTP API.

mod config;
mod db;
mod http;

use clap::{Parser, Subcommand};
use tracing_subscriber::{EnvFilter, fmt};

/// One binary, two commands: the service, and the migration its deploy runs once beforehand.
#[derive(Parser)]
#[command(name = "osiris", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Serve the HTTP API.
    Serve,
    /// Apply the pending database migrations, then exit.
    Migrate,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Locally, `.env` (git-ignored) fills in what the shell doesn't set; deployed, there is none.
    let _ = dotenvy::dotenv();
    init_tracing();
    match Cli::parse().command {
        Command::Serve => http::serve(config::Config::from_env()?).await,
        Command::Migrate => db::migrate(&config::DatabaseConfig::from_env()?).await,
    }
}

/// JSON logs for Cloud Logging when `LOG_FORMAT=json`, readable ones otherwise. `RUST_LOG` filters.
fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    if std::env::var("LOG_FORMAT").as_deref() == Ok("json") {
        fmt().json().with_env_filter(filter).init();
    } else {
        fmt().with_env_filter(filter).init();
    }
}
