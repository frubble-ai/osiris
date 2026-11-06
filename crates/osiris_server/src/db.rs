//! The database connection, and the migration job. The pool opens its first connection at
//! startup, so a database that can't be reached, or whose TLS can't be established against
//! `DB_CA_PEM`, stops the process right away instead of failing the first request.

mod apply;
mod connect;
mod drop_owned;
mod migrate;

pub use connect::connect;
pub(crate) use drop_owned::drop_owned;
pub use migrate::migrate;

/// The migrations in `migrations/`, compiled in.
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

#[cfg(test)]
mod tests;
