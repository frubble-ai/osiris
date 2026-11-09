//! GitHub's action routes: every group's router, merged.

use super::{
    activity, checks, installations, issues, pull_requests, releases, repositories, search, users,
    workflows,
};
use crate::http::state::ApiState;
use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .merge(activity::router())
        .merge(checks::router())
        .merge(installations::router())
        .merge(issues::router())
        .merge(pull_requests::router())
        .merge(releases::router())
        .merge(repositories::router())
        .merge(search::router())
        .merge(users::router())
        .merge(workflows::router())
}
