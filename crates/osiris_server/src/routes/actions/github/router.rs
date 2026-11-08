//! GitHub's action routes, every one registered.

<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
use super::{installations, issues, repositories, users};
=======
use super::{installations, pull_requests, repositories, users};
>>>>>>> port/github-pull_requests
=======
use super::{checks, installations, repositories, users, workflows};
>>>>>>> port/github-checks_workflows
=======
use super::{activity, installations, releases, repositories, search, users};
>>>>>>> port/github-releases_activity_search
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
        .routes(routes!(installations::list_installations::github_list_installations))
        .routes(routes!(installations::list_repositories::github_list_repositories))
        .routes(routes!(repositories::add_repository_collaborator::github_add_repository_collaborator))
        .routes(routes!(repositories::compare_commits::github_compare_commits))
        .routes(routes!(repositories::create_commit_comment::github_create_commit_comment))
        .routes(routes!(repositories::create_or_update_file::github_create_or_update_file))
        .routes(routes!(repositories::create_ref::github_create_ref))
        .routes(routes!(repositories::create_repository::github_create_repository))
        .routes(routes!(repositories::delete_file::github_delete_file))
        .routes(routes!(repositories::delete_ref::github_delete_ref))
        .routes(routes!(repositories::delete_repository::github_delete_repository))
        .routes(routes!(repositories::fork_repository::github_fork_repository))
=======
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
        .routes(routes!(
            issues::add_issue_assignees::github_add_issue_assignees
        ))
        .routes(routes!(issues::add_issue_labels::github_add_issue_labels))
        .routes(routes!(
            issues::clear_issue_labels::github_clear_issue_labels
        ))
        .routes(routes!(issues::create_issue::github_create_issue))
        .routes(routes!(
            issues::create_issue_comment::github_create_issue_comment
        ))
        .routes(routes!(
            issues::create_issue_comment_reaction::github_create_issue_comment_reaction
        ))
        .routes(routes!(
            issues::create_issue_reaction::github_create_issue_reaction
        ))
        .routes(routes!(issues::create_label::github_create_label))
        .routes(routes!(issues::create_milestone::github_create_milestone))
        .routes(routes!(
            issues::delete_issue_comment::github_delete_issue_comment
        ))
        .routes(routes!(issues::delete_label::github_delete_label))
        .routes(routes!(issues::delete_milestone::github_delete_milestone))
        .routes(routes!(issues::get_issue::github_get_issue))
        .routes(routes!(issues::get_issue_comment::github_get_issue_comment))
        .routes(routes!(issues::get_label::github_get_label))
        .routes(routes!(issues::get_milestone::github_get_milestone))
        .routes(routes!(issues::list_assignees::github_list_assignees))
        .routes(routes!(
            issues::list_issue_comments::github_list_issue_comments
        ))
        .routes(routes!(issues::list_issue_events::github_list_issue_events))
        .routes(routes!(issues::list_issue_labels::github_list_issue_labels))
        .routes(routes!(
            issues::list_issue_timeline_events::github_list_issue_timeline_events
        ))
        .routes(routes!(issues::list_milestones::github_list_milestones))
        .routes(routes!(
            issues::list_repository_issue_events::github_list_repository_issue_events
        ))
        .routes(routes!(
            issues::list_repository_issues::github_list_repository_issues
        ))
        .routes(routes!(
            issues::list_repository_labels::github_list_repository_labels
        ))
        .routes(routes!(issues::lock_issue::github_lock_issue))
        .routes(routes!(
            issues::remove_issue_assignees::github_remove_issue_assignees
        ))
        .routes(routes!(
            issues::remove_issue_label::github_remove_issue_label
        ))
        .routes(routes!(
            issues::search_issues_and_pull_requests::github_search_issues_and_pull_requests
        ))
        .routes(routes!(issues::set_issue_labels::github_set_issue_labels))
        .routes(routes!(issues::unlock_issue::github_unlock_issue))
        .routes(routes!(issues::update_issue::github_update_issue))
        .routes(routes!(
            issues::update_issue_comment::github_update_issue_comment
        ))
        .routes(routes!(issues::update_label::github_update_label))
        .routes(routes!(issues::update_milestone::github_update_milestone))
>>>>>>> port/github-issues
=======
        .routes(routes!(installations::list_installations::github_list_installations))
        .routes(routes!(installations::list_repositories::github_list_repositories))
        .routes(routes!(pull_requests::check_pull_request_merged::github_check_pull_request_merged))
        .routes(routes!(pull_requests::create_pull_request::github_create_pull_request))
        .routes(routes!(pull_requests::create_pull_request_review::github_create_pull_request_review))
        .routes(routes!(pull_requests::create_pull_request_review_comment::github_create_pull_request_review_comment))
        .routes(routes!(pull_requests::delete_pending_pull_request_review::github_delete_pending_pull_request_review))
        .routes(routes!(pull_requests::delete_pull_request_review_comment::github_delete_pull_request_review_comment))
        .routes(routes!(pull_requests::dismiss_pull_request_review::github_dismiss_pull_request_review))
        .routes(routes!(pull_requests::get_pull_request::github_get_pull_request))
        .routes(routes!(pull_requests::get_pull_request_review::github_get_pull_request_review))
        .routes(routes!(pull_requests::list_pull_request_commits::github_list_pull_request_commits))
        .routes(routes!(pull_requests::list_pull_request_files::github_list_pull_request_files))
        .routes(routes!(pull_requests::list_pull_request_requested_reviewers::github_list_pull_request_requested_reviewers))
        .routes(routes!(pull_requests::list_pull_request_review_comments::github_list_pull_request_review_comments))
        .routes(routes!(pull_requests::list_pull_request_reviews::github_list_pull_request_reviews))
        .routes(routes!(pull_requests::list_pull_requests::github_list_pull_requests))
        .routes(routes!(pull_requests::list_pull_requests_associated_with_commit::github_list_pull_requests_associated_with_commit))
        .routes(routes!(pull_requests::merge_pull_request::github_merge_pull_request))
        .routes(routes!(pull_requests::remove_pull_request_reviewers::github_remove_pull_request_reviewers))
        .routes(routes!(pull_requests::reply_pull_request_review_comment::github_reply_pull_request_review_comment))
        .routes(routes!(pull_requests::request_pull_request_reviewers::github_request_pull_request_reviewers))
        .routes(routes!(pull_requests::submit_pull_request_review::github_submit_pull_request_review))
        .routes(routes!(pull_requests::update_pull_request::github_update_pull_request))
        .routes(routes!(pull_requests::update_pull_request_branch::github_update_pull_request_branch))
        .routes(routes!(pull_requests::update_pull_request_review_comment::github_update_pull_request_review_comment))
>>>>>>> port/github-pull_requests
=======
        .routes(routes!(activity::check_repository_starred::github_check_repository_starred))
        .routes(routes!(activity::list_authenticated_user_events::github_list_authenticated_user_events))
        .routes(routes!(activity::list_authenticated_user_received_events::github_list_authenticated_user_received_events))
        .routes(routes!(activity::list_my_starred_repositories::github_list_my_starred_repositories))
        .routes(routes!(activity::list_public_events::github_list_public_events))
        .routes(routes!(activity::list_repository_events::github_list_repository_events))
        .routes(routes!(activity::list_repository_stargazers::github_list_repository_stargazers))
        .routes(routes!(activity::list_repository_watchers::github_list_repository_watchers))
        .routes(routes!(activity::list_user_public_events::github_list_user_public_events))
        .routes(routes!(activity::list_user_received_public_events::github_list_user_received_public_events))
        .routes(routes!(activity::star_repository::github_star_repository))
        .routes(routes!(activity::unstar_repository::github_unstar_repository))
        .routes(routes!(installations::list_installations::github_list_installations))
        .routes(routes!(installations::list_repositories::github_list_repositories))
        .routes(routes!(releases::create_release::github_create_release))
        .routes(routes!(releases::delete_release::github_delete_release))
        .routes(routes!(releases::delete_release_asset::github_delete_release_asset))
        .routes(routes!(releases::generate_release_notes::github_generate_release_notes))
        .routes(routes!(releases::get_latest_release::github_get_latest_release))
        .routes(routes!(releases::get_release::github_get_release))
        .routes(routes!(releases::get_release_asset::github_get_release_asset))
        .routes(routes!(releases::get_release_by_tag::github_get_release_by_tag))
        .routes(routes!(releases::list_release_assets::github_list_release_assets))
        .routes(routes!(releases::list_releases::github_list_releases))
        .routes(routes!(releases::update_release::github_update_release))
>>>>>>> port/github-releases_activity_search
        .routes(routes!(repositories::get_branch::github_get_branch))
        .routes(routes!(repositories::get_commit::github_get_commit))
        .routes(routes!(repositories::get_file_contents::github_get_file_contents))
        .routes(routes!(repositories::get_ref::github_get_ref))
        .routes(routes!(repositories::get_repository::github_get_repository))
        .routes(routes!(repositories::get_repository_permission_for_user::github_get_repository_permission_for_user))
        .routes(routes!(repositories::get_repository_readme::github_get_repository_readme))
        .routes(routes!(repositories::list_branches::github_list_branches))
<<<<<<< HEAD
        .routes(routes!(repositories::list_commit_comments::github_list_commit_comments))
        .routes(routes!(repositories::list_commits::github_list_commits))
        .routes(routes!(repositories::list_directory_contents::github_list_directory_contents))
        .routes(routes!(repositories::list_matching_refs::github_list_matching_refs))
        .routes(routes!(repositories::list_my_repositories::github_list_my_repositories))
        .routes(routes!(repositories::list_organization_repositories::github_list_organization_repositories))
        .routes(routes!(repositories::list_repository_collaborators::github_list_repository_collaborators))
        .routes(routes!(repositories::list_repository_contributors::github_list_repository_contributors))
        .routes(routes!(repositories::list_repository_forks::github_list_repository_forks))
        .routes(routes!(repositories::list_repository_languages::github_list_repository_languages))
        .routes(routes!(repositories::list_repository_tags::github_list_repository_tags))
        .routes(routes!(repositories::list_repository_topics::github_list_repository_topics))
        .routes(routes!(repositories::list_user_repositories::github_list_user_repositories))
        .routes(routes!(repositories::merge_branch::github_merge_branch))
        .routes(routes!(repositories::remove_repository_collaborator::github_remove_repository_collaborator))
        .routes(routes!(repositories::rename_branch::github_rename_branch))
        .routes(routes!(repositories::replace_repository_topics::github_replace_repository_topics))
        .routes(routes!(repositories::sync_fork_branch_with_upstream::github_sync_fork_branch_with_upstream))
        .routes(routes!(repositories::update_ref::github_update_ref))
        .routes(routes!(repositories::update_repository::github_update_repository))
=======
        .routes(routes!(search::search_code::github_search_code))
        .routes(routes!(search::search_commits::github_search_commits))
        .routes(routes!(search::search_labels::github_search_labels))
        .routes(routes!(search::search_repositories::github_search_repositories))
        .routes(routes!(search::search_topics::github_search_topics))
        .routes(routes!(search::search_users::github_search_users))
>>>>>>> port/github-releases_activity_search
        .routes(routes!(users::get_current_user::github_get_current_user))
<<<<<<< HEAD
        .routes(routes!(users::get_user::github_get_user))
=======
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
>>>>>>> port/github-checks_workflows
}
