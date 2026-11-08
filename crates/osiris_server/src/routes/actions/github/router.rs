//! GitHub's action routes, every one registered.

use super::{installations, repositories, users};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(
            installations::list_installations::github_list_installations
        ))
        .routes(routes!(
            installations::list_repositories::github_list_repositories
        ))
        .routes(routes!(repositories::get_branch::github_get_branch))
        .routes(routes!(repositories::get_repository::github_get_repository))
        .routes(routes!(repositories::list_branches::github_list_branches))
        .routes(routes!(users::get_current_user::github_get_current_user))
}
