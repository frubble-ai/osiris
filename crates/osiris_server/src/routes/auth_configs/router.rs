//! The `auth_configs` routes.

use super::{create, delete, list, retrieve, update};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(create::create_auth_config, list::list_auth_configs))
        .routes(routes!(
            retrieve::retrieve_auth_config,
            update::update_auth_config,
            delete::delete_auth_config
        ))
}
