//! Every command, one module each.

pub mod actions;
pub mod auth_configs;
pub mod connect;
pub mod connections;
pub mod listen;
pub mod login;
pub mod trigger;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "osiris", version, about = "Osiris's developer CLI")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Save where osiris is and the keys to call it with.
    Login(login::Login),
    /// The providers' apps (admin key).
    #[command(subcommand, name = "auth-configs")]
    AuthConfigs(auth_configs::AuthConfigs),
    /// Connect a provider: starts the consent and opens it in your browser.
    Connect(connect::Connect),
    /// Connections (API key).
    #[command(subcommand)]
    Connections(connections::Connections),
    /// Actions with a connection (API key).
    #[command(subcommand)]
    Actions(actions::Actions),
    /// Receive osiris's events and forward each to a local endpoint, as Pub/Sub pushes them.
    Listen(listen::Listen),
    /// Send a test event for a connection (admin key).
    Trigger(trigger::Trigger),
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<()> {
        match self.command {
            Command::Login(command) => command.run(),
            Command::AuthConfigs(command) => command.run().await,
            Command::Connect(command) => command.run().await,
            Command::Connections(command) => command.run().await,
            Command::Actions(command) => command.run().await,
            Command::Listen(command) => command.run().await,
            Command::Trigger(command) => command.run().await,
        }
    }
}
