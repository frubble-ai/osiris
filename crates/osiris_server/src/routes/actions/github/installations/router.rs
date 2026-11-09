//! The installations action routes, every one registered.

use super::{list_installations, list_repositories};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(list_installations::github_list_installations))
        .routes(routes!(list_repositories::github_list_repositories))
}
