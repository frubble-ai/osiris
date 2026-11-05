//! Osiris's own model: auth configs, connected accounts and their tokens, events, task outcomes, and the
//! errors its core answers with. Plain values: storage decrypts before handing them over.

pub mod auth_config;
pub mod connected_account;
pub mod connection_status;
pub mod errors;
pub mod event;
pub mod task_outcome;
pub mod token_set;

pub use auth_config::AuthConfig;
pub use connected_account::ConnectedAccount;
pub use connection_status::ConnectionStatus;
pub use errors::DomainError;
pub use event::{Event, EventType};
pub use task_outcome::TaskOutcome;
pub use token_set::TokenSet;
