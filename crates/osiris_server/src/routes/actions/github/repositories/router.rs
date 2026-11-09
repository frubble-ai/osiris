//! The repositories action routes, every one registered.

use super::{
    add_repository_collaborator, compare_commits, create_commit_comment, create_or_update_file,
    create_ref, create_repository, delete_file, delete_ref, delete_repository, fork_repository,
    get_branch, get_commit, get_file_contents, get_ref, get_repository,
    get_repository_permission_for_user, get_repository_readme, list_branches, list_commit_comments,
    list_commits, list_directory_contents, list_matching_refs, list_my_repositories,
    list_organization_repositories, list_repository_collaborators, list_repository_contributors,
    list_repository_forks, list_repository_languages, list_repository_tags, list_repository_topics,
    list_user_repositories, merge_branch, remove_repository_collaborator, rename_branch,
    replace_repository_topics, sync_fork_branch_with_upstream, update_ref, update_repository,
};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(
            add_repository_collaborator::github_add_repository_collaborator
        ))
        .routes(routes!(compare_commits::github_compare_commits))
        .routes(routes!(create_commit_comment::github_create_commit_comment))
        .routes(routes!(create_or_update_file::github_create_or_update_file))
        .routes(routes!(create_ref::github_create_ref))
        .routes(routes!(create_repository::github_create_repository))
        .routes(routes!(delete_file::github_delete_file))
        .routes(routes!(delete_ref::github_delete_ref))
        .routes(routes!(delete_repository::github_delete_repository))
        .routes(routes!(fork_repository::github_fork_repository))
        .routes(routes!(get_branch::github_get_branch))
        .routes(routes!(get_commit::github_get_commit))
        .routes(routes!(get_file_contents::github_get_file_contents))
        .routes(routes!(get_ref::github_get_ref))
        .routes(routes!(get_repository::github_get_repository))
        .routes(routes!(
            get_repository_permission_for_user::github_get_repository_permission_for_user
        ))
        .routes(routes!(get_repository_readme::github_get_repository_readme))
        .routes(routes!(list_branches::github_list_branches))
        .routes(routes!(list_commit_comments::github_list_commit_comments))
        .routes(routes!(list_commits::github_list_commits))
        .routes(routes!(
            list_directory_contents::github_list_directory_contents
        ))
        .routes(routes!(list_matching_refs::github_list_matching_refs))
        .routes(routes!(list_my_repositories::github_list_my_repositories))
        .routes(routes!(
            list_organization_repositories::github_list_organization_repositories
        ))
        .routes(routes!(
            list_repository_collaborators::github_list_repository_collaborators
        ))
        .routes(routes!(
            list_repository_contributors::github_list_repository_contributors
        ))
        .routes(routes!(list_repository_forks::github_list_repository_forks))
        .routes(routes!(
            list_repository_languages::github_list_repository_languages
        ))
        .routes(routes!(list_repository_tags::github_list_repository_tags))
        .routes(routes!(
            list_repository_topics::github_list_repository_topics
        ))
        .routes(routes!(
            list_user_repositories::github_list_user_repositories
        ))
        .routes(routes!(merge_branch::github_merge_branch))
        .routes(routes!(
            remove_repository_collaborator::github_remove_repository_collaborator
        ))
        .routes(routes!(rename_branch::github_rename_branch))
        .routes(routes!(
            replace_repository_topics::github_replace_repository_topics
        ))
        .routes(routes!(
            sync_fork_branch_with_upstream::github_sync_fork_branch_with_upstream
        ))
        .routes(routes!(update_ref::github_update_ref))
        .routes(routes!(update_repository::github_update_repository))
}
