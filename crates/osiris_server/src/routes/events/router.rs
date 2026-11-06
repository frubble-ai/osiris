//! The `events` routes.

use super::trigger;
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new().routes(routes!(trigger::trigger_event))
}
