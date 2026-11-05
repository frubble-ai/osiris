//! Calling osiris with the profile's keys: the version header always sent, error answers turned
//! into their code and message.

pub mod api_client;
pub mod error_from_answer;
pub mod print;

pub use api_client::{Client, Key};
pub use print::print;
