//! GitHub's action routes, every one registered.

use super::{checks, installations, repositories, users, workflows};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(
            checks::create_commit_status::github_create_commit_status
        ))
        .routes(routes!(
            checks::get_commit_statuses::github_get_commit_statuses
        ))
        .routes(routes!(
            checks::list_check_runs_for_ref::github_list_check_runs_for_ref
        ))
        .routes(routes!(
            checks::rerequest_check_run::github_rerequest_check_run
        ))
        .routes(routes!(
            checks::rerequest_check_suite::github_rerequest_check_suite
        ))
        .routes(routes!(
            installations::list_installations::github_list_installations
        ))
        .routes(routes!(
            installations::list_repositories::github_list_repositories
        ))
        .routes(routes!(repositories::get_branch::github_get_branch))
        .routes(routes!(repositories::get_repository::github_get_repository))
        .routes(routes!(repositories::list_branches::github_list_branches))
        .routes(routes!(users::get_current_user::github_get_current_user))
        .routes(routes!(
            workflows::cancel_workflow_run::github_cancel_workflow_run
        ))
        .routes(routes!(
            workflows::disable_workflow::github_disable_workflow
        ))
        .routes(routes!(
            workflows::dispatch_workflow::github_dispatch_workflow
        ))
        .routes(routes!(
            workflows::download_workflow_artifact::github_download_workflow_artifact
        ))
        .routes(routes!(workflows::enable_workflow::github_enable_workflow))
        .routes(routes!(workflows::get_workflow::github_get_workflow))
        .routes(routes!(
            workflows::get_workflow_job_logs::github_get_workflow_job_logs
        ))
        .routes(routes!(
            workflows::get_workflow_run::github_get_workflow_run
        ))
        .routes(routes!(
            workflows::list_repository_workflows::github_list_repository_workflows
        ))
        .routes(routes!(
            workflows::list_workflow_run_artifacts::github_list_workflow_run_artifacts
        ))
        .routes(routes!(
            workflows::list_workflow_run_jobs::github_list_workflow_run_jobs
        ))
        .routes(routes!(
            workflows::list_workflow_runs::github_list_workflow_runs
        ))
        .routes(routes!(
            workflows::rerun_failed_jobs::github_rerun_failed_jobs
        ))
        .routes(routes!(workflows::rerun_workflow::github_rerun_workflow))
}
