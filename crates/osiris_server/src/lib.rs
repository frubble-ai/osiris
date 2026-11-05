//! Osiris's server: the core, its HTTP layer and routes, and the processes that run them. The
//! binary (`main.rs`) only picks a process.

pub mod config;
pub mod core;
pub mod db;
pub mod http;
pub mod routes;
pub mod services;
