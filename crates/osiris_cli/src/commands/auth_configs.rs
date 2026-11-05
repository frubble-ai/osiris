//! `osiris auth-configs …`: the providers' apps.

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum AuthConfigs {
    /// Register a provider's app.
    Create(create::Create),
    List(list::List),
    Get(get::Get),
    /// Change an app's client, secret or scopes.
    Update(update::Update),
    /// Delete an app no connection uses.
    Delete(delete::Delete),
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
