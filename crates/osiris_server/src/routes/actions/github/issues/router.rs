//! The issues action routes, every one registered.

use super::{
    add_issue_assignees, add_issue_labels, clear_issue_labels, create_issue, create_issue_comment,
    create_issue_comment_reaction, create_issue_reaction, create_label, create_milestone,
    delete_issue_comment, delete_label, delete_milestone, get_issue, get_issue_comment, get_label,
    get_milestone, list_assignees, list_issue_comments, list_issue_events, list_issue_labels,
    list_issue_timeline_events, list_milestones, list_repository_issue_events,
    list_repository_issues, list_repository_labels, lock_issue, remove_issue_assignees,
    remove_issue_label, search_issues_and_pull_requests, set_issue_labels, unlock_issue,
    update_issue, update_issue_comment, update_label, update_milestone,
};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(add_issue_assignees::github_add_issue_assignees))
        .routes(routes!(add_issue_labels::github_add_issue_labels))
        .routes(routes!(clear_issue_labels::github_clear_issue_labels))
        .routes(routes!(create_issue::github_create_issue))
        .routes(routes!(create_issue_comment::github_create_issue_comment))
        .routes(routes!(
            create_issue_comment_reaction::github_create_issue_comment_reaction
        ))
        .routes(routes!(create_issue_reaction::github_create_issue_reaction))
        .routes(routes!(create_label::github_create_label))
        .routes(routes!(create_milestone::github_create_milestone))
        .routes(routes!(delete_issue_comment::github_delete_issue_comment))
        .routes(routes!(delete_label::github_delete_label))
        .routes(routes!(delete_milestone::github_delete_milestone))
        .routes(routes!(get_issue::github_get_issue))
        .routes(routes!(get_issue_comment::github_get_issue_comment))
        .routes(routes!(get_label::github_get_label))
        .routes(routes!(get_milestone::github_get_milestone))
        .routes(routes!(list_assignees::github_list_assignees))
        .routes(routes!(list_issue_comments::github_list_issue_comments))
        .routes(routes!(list_issue_events::github_list_issue_events))
        .routes(routes!(list_issue_labels::github_list_issue_labels))
        .routes(routes!(
            list_issue_timeline_events::github_list_issue_timeline_events
        ))
        .routes(routes!(list_milestones::github_list_milestones))
        .routes(routes!(
            list_repository_issue_events::github_list_repository_issue_events
        ))
        .routes(routes!(
            list_repository_issues::github_list_repository_issues
        ))
        .routes(routes!(
            list_repository_labels::github_list_repository_labels
        ))
        .routes(routes!(lock_issue::github_lock_issue))
        .routes(routes!(
            remove_issue_assignees::github_remove_issue_assignees
        ))
        .routes(routes!(remove_issue_label::github_remove_issue_label))
        .routes(routes!(
            search_issues_and_pull_requests::github_search_issues_and_pull_requests
        ))
        .routes(routes!(set_issue_labels::github_set_issue_labels))
        .routes(routes!(unlock_issue::github_unlock_issue))
        .routes(routes!(update_issue::github_update_issue))
        .routes(routes!(update_issue_comment::github_update_issue_comment))
        .routes(routes!(update_label::github_update_label))
        .routes(routes!(update_milestone::github_update_milestone))
}
