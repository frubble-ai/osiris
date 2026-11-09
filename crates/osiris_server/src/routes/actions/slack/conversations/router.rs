//! The conversations action routes, every one registered.

use super::list_channels;
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new().routes(routes!(list_channels::slack_list_channels))
}
