//! The workflows action routes, every one registered.

use super::{
    cancel_workflow_run, disable_workflow, dispatch_workflow, download_workflow_artifact,
    enable_workflow, get_workflow, get_workflow_job_logs, get_workflow_run,
    list_repository_workflows, list_workflow_run_artifacts, list_workflow_run_jobs,
    list_workflow_runs, rerun_failed_jobs, rerun_workflow,
};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(cancel_workflow_run::github_cancel_workflow_run))
        .routes(routes!(disable_workflow::github_disable_workflow))
        .routes(routes!(dispatch_workflow::github_dispatch_workflow))
        .routes(routes!(
            download_workflow_artifact::github_download_workflow_artifact
        ))
        .routes(routes!(enable_workflow::github_enable_workflow))
        .routes(routes!(get_workflow::github_get_workflow))
        .routes(routes!(get_workflow_job_logs::github_get_workflow_job_logs))
        .routes(routes!(get_workflow_run::github_get_workflow_run))
        .routes(routes!(
            list_repository_workflows::github_list_repository_workflows
        ))
        .routes(routes!(
            list_workflow_run_artifacts::github_list_workflow_run_artifacts
        ))
        .routes(routes!(
            list_workflow_run_jobs::github_list_workflow_run_jobs
        ))
        .routes(routes!(list_workflow_runs::github_list_workflow_runs))
        .routes(routes!(rerun_failed_jobs::github_rerun_failed_jobs))
        .routes(routes!(rerun_workflow::github_rerun_workflow))
}
