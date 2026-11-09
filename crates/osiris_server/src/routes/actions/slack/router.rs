//! Slack's action routes: every group's router, merged.

use super::conversations;
use crate::http::state::ApiState;
use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new().merge(conversations::router())
}
