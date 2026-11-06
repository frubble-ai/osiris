//! `osiris`: the developer CLI. Manage the providers' apps, connect a provider in your browser,
//! inspect and refresh connections, run actions, and receive or trigger events locally.

#![allow(clippy::print_stdout, reason = "a CLI answers on stdout")]

mod client;
mod commands;
mod profile;

use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // The keys `login` saves can come from the repository's `.env`, as for the server.
    dotenvy::dotenv().ok();
    commands::Cli::parse().run().await
}
