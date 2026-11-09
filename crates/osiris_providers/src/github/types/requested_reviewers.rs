//! Who a pull request's review is requested from.

use super::{GithubTeam, GithubUser};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Who a pull request's review is requested from.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubRequestedReviewers {
    pub users: Vec<GithubUser>,
    pub teams: Vec<GithubTeam>,
}
