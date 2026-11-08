//! GitHub's action routes, every one registered.

use super::{installations, pull_requests, repositories, users};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
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
        .routes(routes!(repositories::get_branch::github_get_branch))
        .routes(routes!(repositories::get_repository::github_get_repository))
        .routes(routes!(repositories::list_branches::github_list_branches))
        .routes(routes!(users::get_current_user::github_get_current_user))
}
