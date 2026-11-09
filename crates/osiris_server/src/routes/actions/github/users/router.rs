//! The users action routes, every one registered.

use super::{get_current_user, get_user};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(get_current_user::github_get_current_user))
        .routes(routes!(get_user::github_get_user))
}
