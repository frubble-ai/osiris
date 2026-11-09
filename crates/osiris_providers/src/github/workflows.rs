//! GitHub Actions: workflows, their runs, jobs, logs and artifacts.

pub mod cancel_workflow_run;
pub mod disable_workflow;
pub mod dispatch_workflow;
pub mod download_workflow_artifact;
pub mod enable_workflow;
mod follow_download;
pub mod get_workflow;
pub mod get_workflow_job_logs;
pub mod get_workflow_run;
pub mod list_repository_workflows;
pub mod list_workflow_run_artifacts;
pub mod list_workflow_run_jobs;
pub mod list_workflow_runs;
#[cfg(any(test, feature = "testing"))]
pub mod mocks;
pub mod rerun_failed_jobs;
pub mod rerun_workflow;

pub use cancel_workflow_run::CancelWorkflowRun;
pub use disable_workflow::DisableWorkflow;
pub use dispatch_workflow::DispatchWorkflow;
pub use download_workflow_artifact::DownloadWorkflowArtifact;
pub use enable_workflow::EnableWorkflow;
pub use get_workflow::GetWorkflow;
pub use get_workflow_job_logs::GetWorkflowJobLogs;
pub use get_workflow_run::GetWorkflowRun;
pub use list_repository_workflows::ListRepositoryWorkflows;
pub use list_workflow_run_artifacts::ListWorkflowRunArtifacts;
pub use list_workflow_run_jobs::ListWorkflowRunJobs;
pub use list_workflow_runs::ListWorkflowRuns;
pub use rerun_failed_jobs::RerunFailedJobs;
pub use rerun_workflow::RerunWorkflow;

#[cfg(test)]
mod tests;
