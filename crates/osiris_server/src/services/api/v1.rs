//! Every `/v1` route, with the spec they describe.

use super::doc::ApiDoc;
use crate::{http::state::ApiState, routes};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

pub fn v1() -> OpenApiRouter<ApiState> {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(routes::auth_configs::router())
        .merge(routes::connected_accounts::router())
        .merge(routes::actions::router())
        .merge(routes::events::router())
}
