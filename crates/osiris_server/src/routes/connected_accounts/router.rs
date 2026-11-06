//! The `connected_accounts` routes.

use super::{create, delete, list, refresh, retrieve};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(
            create::create_connected_account,
            list::list_connected_accounts
        ))
        .routes(routes!(
            retrieve::retrieve_connected_account,
            delete::delete_connected_account
        ))
        .routes(routes!(refresh::refresh_connected_account))
}
