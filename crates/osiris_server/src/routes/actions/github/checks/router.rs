//! The checks action routes, every one registered.

use super::{
    create_commit_status, get_commit_statuses, list_check_runs_for_ref, rerequest_check_run,
    rerequest_check_suite,
};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(create_commit_status::github_create_commit_status))
        .routes(routes!(get_commit_statuses::github_get_commit_statuses))
        .routes(routes!(
            list_check_runs_for_ref::github_list_check_runs_for_ref
        ))
        .routes(routes!(rerequest_check_run::github_rerequest_check_run))
        .routes(routes!(rerequest_check_suite::github_rerequest_check_suite))
}
