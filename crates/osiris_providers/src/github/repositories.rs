//! A repository: its settings, branches, commits, references, contents, topics, collaborators
//! and commit comments.

mod contents_path;
mod git_ref_path;

pub mod add_repository_collaborator;
pub mod compare_commits;
pub mod create_commit_comment;
pub mod create_or_update_file;
pub mod create_ref;
pub mod create_repository;
pub mod delete_file;
pub mod delete_ref;
pub mod delete_repository;
pub mod fork_repository;
pub mod get_branch;
pub mod get_commit;
pub mod get_file_contents;
pub mod get_ref;
pub mod get_repository;
pub mod get_repository_permission_for_user;
pub mod get_repository_readme;
pub mod list_branches;
pub mod list_commit_comments;
pub mod list_commits;
pub mod list_directory_contents;
pub mod list_matching_refs;
pub mod list_my_repositories;
pub mod list_organization_repositories;
pub mod list_repository_collaborators;
pub mod list_repository_contributors;
pub mod list_repository_forks;
pub mod list_repository_languages;
pub mod list_repository_tags;
pub mod list_repository_topics;
pub mod list_user_repositories;
pub mod merge_branch;
pub mod remove_repository_collaborator;
pub mod rename_branch;
pub mod replace_repository_topics;
pub mod sync_fork_branch_with_upstream;
pub mod update_ref;
pub mod update_repository;

pub use add_repository_collaborator::AddRepositoryCollaborator;
pub use compare_commits::CompareCommits;
pub use create_commit_comment::CreateCommitComment;
pub use create_or_update_file::CreateOrUpdateFile;
pub use create_ref::CreateRef;
pub use create_repository::CreateRepository;
pub use delete_file::DeleteFile;
pub use delete_ref::DeleteRef;
pub use delete_repository::DeleteRepository;
pub use fork_repository::ForkRepository;
pub use get_branch::GetBranch;
pub use get_commit::GetCommit;
pub use get_file_contents::GetFileContents;
pub use get_ref::GetRef;
pub use get_repository::GetRepository;
pub use get_repository_permission_for_user::GetRepositoryPermissionForUser;
pub use get_repository_readme::GetRepositoryReadme;
pub use list_branches::ListBranches;
pub use list_commit_comments::ListCommitComments;
pub use list_commits::ListCommits;
pub use list_directory_contents::ListDirectoryContents;
pub use list_matching_refs::ListMatchingRefs;
pub use list_my_repositories::ListMyRepositories;
pub use list_organization_repositories::ListOrganizationRepositories;
pub use list_repository_collaborators::ListRepositoryCollaborators;
pub use list_repository_contributors::ListRepositoryContributors;
pub use list_repository_forks::ListRepositoryForks;
pub use list_repository_languages::ListRepositoryLanguages;
pub use list_repository_tags::ListRepositoryTags;
pub use list_repository_topics::ListRepositoryTopics;
pub use list_user_repositories::ListUserRepositories;
pub use merge_branch::MergeBranch;
pub use remove_repository_collaborator::RemoveRepositoryCollaborator;
pub use rename_branch::RenameBranch;
pub use replace_repository_topics::ReplaceRepositoryTopics;
pub use sync_fork_branch_with_upstream::SyncForkBranchWithUpstream;
pub use update_ref::UpdateRef;
pub use update_repository::UpdateRepository;

#[cfg(any(test, feature = "testing"))]
pub mod mocks;
#[cfg(test)]
mod tests;
