//! Osiris's server: the use cases, the HTTP layer and routes, and the processes that run them. The
//! binary (`main.rs`) only picks a process.

pub mod config;
pub mod db;
pub mod http;
pub mod routes;
pub mod services;
#[cfg(test)]
pub mod testing;
pub mod use_cases;
