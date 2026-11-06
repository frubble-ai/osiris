//! The database handle (`Db`, on the context as `ctx.db`) and what its operations share: the
//! tenant whose data key encrypts a value, the rows as the database answers them, and the data
//! keys. The operations themselves are in `lib/<domain>/`.

pub mod data_keys;
pub mod errors;
pub mod handle;
pub mod ping;
pub mod rows;
pub mod tenant;

pub use handle::Db;
pub use tenant::Tenant;
