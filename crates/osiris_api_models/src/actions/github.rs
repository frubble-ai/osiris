//! GitHub's actions: their inputs, grouped as GitHub's API is, and the types they answer with,
//! which are the provider's own.

pub use osiris_providers::github::{
    activity, checks, installations, issues, pull_requests, releases, repositories, search,
    types::*, users, workflows,
};
