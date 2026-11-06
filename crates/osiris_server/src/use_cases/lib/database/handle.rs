//! The database the use cases run on, built once at boot: the pool, the master key its data keys
//! are wrapped with, and the data keys already read. Secrets go in encrypted and come out
//! decrypted, so no use case handles an encrypted value. Its operations are in the domain folders
//! of `lib/`, one per file.

use osiris_common::crypto::{Cipher, MasterKey};
use sqlx::PgPool;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::Mutex as AsyncMutex;

#[derive(Clone)]
pub struct Db {
    pub(crate) pool: PgPool,
    pub(crate) master: MasterKey,
    /// Data keys already read, by tenant.
    pub(crate) keys: Arc<Mutex<HashMap<String, Cipher>>>,
    /// How long a refresh waits for another refresh's row lock before giving up.
    pub(crate) lock_timeout: Duration,
    /// One in-process flight per connection: waiters do not take a pool connection until they can
    /// run. Cross-instance contention still uses the row lock and `lock_timeout`.
    pub(crate) refresh_flights: Arc<Mutex<HashMap<String, Arc<AsyncMutex<()>>>>>,
}

impl Db {
    pub fn new(pool: PgPool, master: MasterKey, lock_timeout: Duration) -> Self {
        Self {
            pool,
            master,
            keys: Arc::default(),
            lock_timeout,
            refresh_flights: Arc::default(),
        }
    }
}
