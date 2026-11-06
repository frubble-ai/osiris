//! A task scheduler backed by Google Cloud Tasks, using Google's official client
//! (`google-cloud-tasks-v2`) and the service account's Application Default Credentials.
//!
//! Each osiris task becomes a Cloud Tasks HTTP task with the same name, on a single queue. When it
//! is due, Cloud Tasks posts the task as JSON to the worker's `/tasks` with an OIDC token. Cloud
//! Run checks that token before the request reaches the worker. When the worker does not answer
//! 2xx, the queue's retry policy decides when the task runs again. The worker also checks that token
//! itself (`OidcAuthenticator`), so it never relies on the platform alone.

pub mod config;
#[cfg(feature = "testing")]
pub mod mocks;
pub mod oidc;
pub mod outcome;
pub mod scheduler;

pub use config::CloudTasksConfig;
pub use oidc::{OidcAuthenticator, OidcConfig};
pub use scheduler::CloudTasksScheduler;
