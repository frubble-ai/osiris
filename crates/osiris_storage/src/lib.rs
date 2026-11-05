//! Osiris's database: one module per table. Secrets go in encrypted and come out decrypted, so
//! no code outside this crate handles encrypted values. Tokens are always saved in the same
//! statement as the refresh task scheduled for them.

pub mod auth_configs;
pub mod connected_accounts;
pub mod data_keys;
pub mod errors;
pub mod idempotency;
pub mod ping;
pub mod rows;
pub mod task_outcomes;
pub mod tenant;

pub use errors::StorageError;
pub use osiris_domain::{Cursor, Page};
pub use tenant::Tenant;

use osiris_common::crypto::{Cipher, MasterKey};
use sqlx::PgPool;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// The database, and the master key its data keys are wrapped with.
#[derive(Clone)]
pub struct Store {
    pub pool: PgPool,
    master: MasterKey,
    /// Data keys already read, by tenant.
    keys: Arc<Mutex<HashMap<String, Cipher>>>,
    /// How long a refresh waits for another refresh's row lock before giving up.
    lock_timeout: std::time::Duration,
}

/// The migrations in `migrations/`, compiled in.
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

impl Store {
    pub fn new(pool: PgPool, master: MasterKey, lock_timeout: std::time::Duration) -> Self {
        Self {
            pool,
            master,
            keys: Arc::default(),
            lock_timeout,
        }
    }
}
