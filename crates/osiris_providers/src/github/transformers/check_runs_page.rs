//! A page of a commit's check runs, as GitHub wraps it.

use super::check_run::CheckRunAnswer;
use serde::Deserialize;

/// A page of a commit's check runs, as GitHub wraps it.
#[derive(Deserialize)]
pub struct CheckRunsPage {
    #[serde(default)]
    pub check_runs: Vec<CheckRunAnswer>,
}
