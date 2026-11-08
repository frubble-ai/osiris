//! The action routes: every provider's router, merged.

use crate::http::state::ApiState;
use utoipa_axum::router::OpenApiRouter;

include!(concat!(env!("OUT_DIR"), "/provider_action_routers.rs"));

pub fn router() -> OpenApiRouter<ApiState> {
    provider_routers()
        .into_iter()
        .fold(OpenApiRouter::new(), OpenApiRouter::merge)
}
