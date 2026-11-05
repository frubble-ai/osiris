//! The `connections` subcommands.

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Connections {
    List(super::list::List),
    Get(super::get::Get),
    /// End the grant at the provider and forget the connection.
    Delete(super::delete::Delete),
    /// Refresh a connection's tokens now, replacing its scheduled refresh (admin key).
    Refresh(super::refresh::Refresh),
}

impl Connections {
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            Self::List(command) => command.run().await,
            Self::Get(command) => command.run().await,
            Self::Delete(command) => command.run().await,
            Self::Refresh(command) => command.run().await,
        }
    }
}
