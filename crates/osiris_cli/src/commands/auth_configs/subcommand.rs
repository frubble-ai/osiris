//! The `auth-configs` subcommands.

use clap::Subcommand;

#[derive(Subcommand)]
pub enum AuthConfigs {
    /// Register a provider's app.
    Create(super::create::Create),
    List(super::list::List),
    Get(super::get::Get),
    /// Change an app's client, secret or scopes.
    Update(super::update::Update),
    /// Delete an app no connection uses.
    Delete(super::delete::Delete),
}

impl AuthConfigs {
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            Self::Create(command) => command.run().await,
            Self::List(command) => command.run().await,
            Self::Get(command) => command.run().await,
            Self::Update(command) => command.run().await,
            Self::Delete(command) => command.run().await,
        }
    }
}
