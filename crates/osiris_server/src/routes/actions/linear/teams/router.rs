//! The teams action routes, every one registered.

use super::list_teams;
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new().routes(routes!(list_teams::linear_list_teams))
}
