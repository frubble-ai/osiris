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
    osiris_env::logger::init();
    let process = Process::from_env()?;
    let settings = config::load::load()?;
    tracing::info!(?process, commit = osiris_env::build::commit(), "starting");
    match process {
        Process::Serve => services::api::serve(config::ApiConfig::from_env(settings)?).await,
        Process::Worker => services::worker::serve(config::Config::from_env(settings)?).await,
        Process::Migrate => {
            // The reset workflow passes this for one execution. The job's command stays
            // the binary; Cloud Run replaces the arguments only for that execution.
            let reset = std::env::args().any(|arg| arg == "--reset");
            db::migrate(&config::DatabaseConfig::from_env(&settings)?, reset).await
        }
    }
}
