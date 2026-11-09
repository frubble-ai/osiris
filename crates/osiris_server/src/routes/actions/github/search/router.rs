//! The search action routes, every one registered.

use super::{
    search_code, search_commits, search_labels, search_repositories, search_topics, search_users,
};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(search_code::github_search_code))
        .routes(routes!(search_commits::github_search_commits))
        .routes(routes!(search_labels::github_search_labels))
        .routes(routes!(search_repositories::github_search_repositories))
        .routes(routes!(search_topics::github_search_topics))
        .routes(routes!(search_users::github_search_users))
}
