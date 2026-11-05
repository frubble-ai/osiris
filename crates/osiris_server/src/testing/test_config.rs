//! The configuration fixture: the code's default settings, and values for everything read from
//! the environment. The database values only satisfy the type: a test runs on the pool its
//! `#[sqlx::test]` gives it.

use crate::config::{Config, DatabaseConfig, Settings, TasksConfig};
use osiris_common::secret::secret;
use osiris_pubsub::PubSubConfig;

pub const PUBLIC_URL: &str = "http://osiris.test";

pub fn build_test_config() -> Config {
    let settings = Settings::default();
    Config {
        database: DatabaseConfig {
            host: "localhost".into(),
            port: 5432,
            name: "osiris_test".into(),
            user: "osiris".into(),
            password: secret("osiris"),
            ca_pem: String::new(),
            max_connections: settings.database.max_connections,
            connect_timeout: settings.database.connect_timeout(),
        },
        encryption_key: secret("a-test-secret-that-is-long-enough-for-osiris"),
        public_url: PUBLIC_URL.into(),
        events: PubSubConfig {
            project: "osiris-test".into(),
            events_topic: "osiris-events".into(),
            emulator_host: None,
        },
        tasks: TasksConfig::Local {
            handler_url: "http://worker.test/tasks".into(),
            token: secret(super::WORKER_TOKEN),
        },
        settings,
    }
}
