//! The `triggers` routes.

use super::{create, delete, list, retrieve};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(create::create_trigger, list::list_triggers))
        .routes(routes!(retrieve::retrieve_trigger, delete::delete_trigger))
}
