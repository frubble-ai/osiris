//! Osiris: Frubble's integrations service. It holds users' OAuth connections to providers (GitHub,
//! Linear, Slack), keeps their tokens fresh and runs typed actions with them, over an HTTP API.
//!
//! One binary and one image. `OSIRIS_PROCESS` chooses what runs: `serve`, `worker` or `migrate`
//! (docs/architecture.md). Every setting comes from the environment.

use osiris_server::{
    config::{self, process::Process},
    db, services,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // A git-ignored `.env`, when there is one, fills in what the shell doesn't set. Deployed
    // there is none, and the variables come from the platform.
    let _ = dotenvy::dotenv();
    osiris_env::logger::init();
    let process = Process::from_env()?;
    let settings = config::load::load()?;
    tracing::info!(?process, commit = osiris_env::build::commit(), "starting");
    match process {
        Process::Serve => services::api::serve(config::ApiConfig::from_env(settings)?).await,
        Process::Worker => services::worker::serve(config::CoreConfig::from_env(settings)?).await,
        Process::Migrate => db::migrate(&config::DatabaseConfig::from_env(&settings)?).await,
    }
}
