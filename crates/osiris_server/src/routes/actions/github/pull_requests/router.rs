//! The pull requests action routes, every one registered.

use super::{
    check_pull_request_merged, create_pull_request, create_pull_request_review,
    create_pull_request_review_comment, delete_pending_pull_request_review,
    delete_pull_request_review_comment, dismiss_pull_request_review, get_pull_request,
    get_pull_request_review, list_pull_request_commits, list_pull_request_files,
    list_pull_request_requested_reviewers, list_pull_request_review_comments,
    list_pull_request_reviews, list_pull_requests, list_pull_requests_associated_with_commit,
    merge_pull_request, remove_pull_request_reviewers, reply_pull_request_review_comment,
    request_pull_request_reviewers, submit_pull_request_review, update_pull_request,
    update_pull_request_branch, update_pull_request_review_comment,
};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(check_pull_request_merged::github_check_pull_request_merged))
        .routes(routes!(create_pull_request::github_create_pull_request))
        .routes(routes!(create_pull_request_review::github_create_pull_request_review))
        .routes(routes!(create_pull_request_review_comment::github_create_pull_request_review_comment))
        .routes(routes!(delete_pending_pull_request_review::github_delete_pending_pull_request_review))
        .routes(routes!(delete_pull_request_review_comment::github_delete_pull_request_review_comment))
        .routes(routes!(dismiss_pull_request_review::github_dismiss_pull_request_review))
        .routes(routes!(get_pull_request::github_get_pull_request))
        .routes(routes!(get_pull_request_review::github_get_pull_request_review))
        .routes(routes!(list_pull_request_commits::github_list_pull_request_commits))
        .routes(routes!(list_pull_request_files::github_list_pull_request_files))
        .routes(routes!(list_pull_request_requested_reviewers::github_list_pull_request_requested_reviewers))
        .routes(routes!(list_pull_request_review_comments::github_list_pull_request_review_comments))
        .routes(routes!(list_pull_request_reviews::github_list_pull_request_reviews))
        .routes(routes!(list_pull_requests::github_list_pull_requests))
        .routes(routes!(list_pull_requests_associated_with_commit::github_list_pull_requests_associated_with_commit))
        .routes(routes!(merge_pull_request::github_merge_pull_request))
        .routes(routes!(remove_pull_request_reviewers::github_remove_pull_request_reviewers))
        .routes(routes!(reply_pull_request_review_comment::github_reply_pull_request_review_comment))
        .routes(routes!(request_pull_request_reviewers::github_request_pull_request_reviewers))
        .routes(routes!(submit_pull_request_review::github_submit_pull_request_review))
        .routes(routes!(update_pull_request::github_update_pull_request))
        .routes(routes!(update_pull_request_branch::github_update_pull_request_branch))
        .routes(routes!(update_pull_request_review_comment::github_update_pull_request_review_comment))
}
