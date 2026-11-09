//! A repository and everything in it, read from and written to GitHub and put in our terms, with
//! every value placed in the path percent-encoded.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    AddRepositoryCollaborator, CompareCommits, CreateCommitComment, CreateOrUpdateFile, CreateRef,
    CreateRepository, DeleteFile, DeleteRef, DeleteRepository, ForkRepository, GetBranch,
    GetCommit, GetFileContents, GetRef, GetRepository, GetRepositoryPermissionForUser,
    GetRepositoryReadme, ListBranches, ListCommitComments, ListCommits, ListDirectoryContents,
    ListMatchingRefs, ListMyRepositories, ListOrganizationRepositories,
    ListRepositoryCollaborators, ListRepositoryContributors, ListRepositoryForks,
    ListRepositoryLanguages, ListRepositoryTags, ListRepositoryTopics, ListUserRepositories,
    MergeBranch, RemoveRepositoryCollaborator, RenameBranch, ReplaceRepositoryTopics,
    SyncForkBranchWithUpstream, UpdateRef, UpdateRepository,
    mocks::{
        generate_github_collaborator::generate_github_collaborator,
        generate_github_commit_comment::generate_github_commit_comment,
        generate_github_commit_comparison::generate_github_commit_comparison,
        generate_github_content::{
            generate_github_content_change, generate_github_content_entry,
            generate_github_file_content, generate_text,
        },
        generate_github_contributor::generate_github_contributor,
        generate_github_fork_sync::generate_github_fork_sync,
        generate_github_git_ref::generate_github_git_ref,
        generate_github_repository_invitation::generate_github_repository_invitation,
        generate_github_repository_permission::generate_github_repository_permission,
        generate_github_tag::generate_github_tag,
    },
};
use crate::github::mocks::generate_github_commit::generate_github_commit;
use crate::{
    ActingAs, ProviderError,
    github::{
        mocks::generate_github::generate_github,
        mocks::{
            MockGithubNetwork,
            generate_github_branch::generate_github_branch,
            generate_github_names::{
                generate_github_login, generate_github_repo, generate_github_token,
            },
            generate_github_repository::generate_github_repository,
            generate_sha::generate_sha,
        },
    },
};
use base64::{Engine, engine::general_purpose::STANDARD};
use fake::{Fake, faker::lorem::en::Sentence};
use rstest::rstest;
use serde_json::json;
use wiremock::MockServer;

fn http() -> reqwest::Client {
    reqwest::Client::new()
}

#[tokio::test]
async fn reads_a_repository_by_owner_and_name() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let answer = generate_github_repository(42);
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}"),
            &[],
            200,
            answer.clone(),
        )
        .await;
    let input = GetRepository {
        owner: owner.clone(),
        repo: repo.clone(),
    };

    let repository = generate_github(&server)
        .get_repository(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(repository.id, "42");
    assert_eq!(repository.full_name, answer["full_name"].as_str().unwrap());
    assert_eq!(
        repository.owner.login,
        answer["owner"]["login"].as_str().unwrap()
    );
    assert_eq!(repository.default_branch, "main");
    assert_eq!(repository.homepage.as_deref(), Some("https://example.com"));
    assert_eq!(repository.language.as_deref(), Some("Rust"));
    assert_eq!(repository.topics, ["osiris", "seed"]);
    assert_eq!(
        repository.license.as_ref().unwrap().spdx_id.as_deref(),
        Some("MIT")
    );
    assert!(repository.is_template && repository.has_discussions && !repository.has_projects);
    assert_eq!(
        (
            repository.stargazers_count,
            repository.forks_count,
            repository.open_issues_count,
            repository.size
        ),
        (12, 3, 4, 260)
    );
    assert_eq!(
        repository.pushed_at.as_deref(),
        Some("2024-02-02T00:00:00Z")
    );
    assert!(repository.permissions.as_ref().unwrap().admin);
    assert_eq!(repository.allow_squash_merge, None);
}

#[tokio::test]
async fn lists_a_page_of_branches_filtered_by_protection() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let branches = [generate_github_branch(true), generate_github_branch(true)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/branches"),
            &[("protected", "true"), ("per_page", "2"), ("page", "3")],
            200,
            serde_json::json!(branches),
        )
        .await;
    let input = ListBranches {
        owner: owner.clone(),
        repo: repo.clone(),
        protected: Some(true),
        per_page: Some(2),
        page: Some(3),
    };

    let listed = generate_github(&server)
        .list_branches(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].name, branches[0]["name"].as_str().unwrap());
    assert_eq!(
        listed[0].commit_sha,
        branches[0]["commit"]["sha"].as_str().unwrap()
    );
    assert!(listed[0].protected);
}

#[tokio::test]
async fn encodes_a_branch_name_with_a_slash_into_one_segment() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/branches/feature%2Flogin"),
            &[],
            200,
            generate_github_branch(false),
        )
        .await;
    let input = GetBranch {
        owner: owner.clone(),
        repo: repo.clone(),
        branch: "feature/login".into(),
    };

    let branch = generate_github(&server)
        .get_branch(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert!(!branch.protected);
}

#[tokio::test]
async fn lists_the_users_repositories_sorted_and_filtered() {
    let token = generate_github_token();
    let server = MockServer::start().await;
    let repositories = [generate_github_repository(1), generate_github_repository(2)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/user/repos",
            &[
                ("visibility", "private"),
                ("sort", "pushed"),
                ("direction", "desc"),
                ("per_page", "2"),
                ("page", "1"),
            ],
            200,
            json!(repositories),
        )
        .await;
    let input = ListMyRepositories {
        visibility: Some("private".into()),
        sort: Some(" pushed ".into()),
        direction: Some("desc".into()),
        per_page: Some(2),
        page: Some(1),
    };

    let listed = generate_github(&server)
        .list_my_repositories(&http(), &token, &input, ActingAs::User)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[1].id, "2");
    assert_eq!(listed[0].url, repositories[0]["html_url"].as_str().unwrap());
}

#[tokio::test]
async fn lists_an_installation_s_repositories_on_an_installation_connection() {
    let token = generate_github_token();
    let server = MockServer::start().await;
    let repositories = [generate_github_repository(3)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/installation/repositories",
            &[("per_page", "2"), ("page", "1")],
            200,
            json!({ "total_count": 1, "repositories": repositories }),
        )
        .await;
    let input = ListMyRepositories {
        per_page: Some(2),
        page: Some(1),
        ..ListMyRepositories::default()
    };

    let listed = generate_github(&server)
        .list_my_repositories(&http(), &token, &input, ActingAs::Installation)
        .await
        .unwrap();

    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, "3");
}

#[tokio::test]
async fn leaves_out_a_blank_filter_on_an_installation_connection() {
    let server = MockServer::start().await;
    let (token, repository) = (generate_github_token(), generate_github_repository(3));
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/installation/repositories",
            &[],
            200,
            json!({ "total_count": 1, "repositories": [repository] }),
        )
        .await;
    let input = ListMyRepositories {
        visibility: Some("  ".into()),
        sort: Some(String::new()),
        ..ListMyRepositories::default()
    };

    let listed = generate_github(&server)
        .list_my_repositories(&http(), &token, &input, ActingAs::Installation)
        .await
        .unwrap();

    assert_eq!(listed.len(), 1);
}

#[rstest]
#[case::visibility(ListMyRepositories { visibility: Some("private".into()), ..ListMyRepositories::default() }, "visibility")]
#[case::sort(ListMyRepositories { sort: Some("pushed".into()), ..ListMyRepositories::default() }, "sort")]
#[case::direction(ListMyRepositories { direction: Some("asc".into()), ..ListMyRepositories::default() }, "direction")]
#[tokio::test]
async fn refuses_a_user_s_filter_on_an_installation_connection_without_asking_github(
    #[case] input: ListMyRepositories,
    #[case] named: &str,
) {
    let token = generate_github_token();
    // No answer is arranged: a request to GitHub would fail the call another way.
    let server = MockServer::start().await;

    let refused = generate_github(&server)
        .list_my_repositories(&http(), &token, &input, ActingAs::Installation)
        .await
        .unwrap_err();

    let ProviderError::Invalid { message, .. } = refused else {
        panic!("{refused}");
    };
    assert!(message.contains(named), "{message}");
}

#[tokio::test]
async fn creates_a_repository_sending_only_the_settings_given() {
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let answer = generate_github_repository(77);
    MockGithubNetwork::on(&server)
        .answer_with_body(
            "POST",
            "/user/repos",
            json!({
                "name": repo,
                "description": " a greeting ",
                "private": true,
                "auto_init": true,
                "license_template": "mit",
            }),
            201,
            answer.clone(),
        )
        .await;
    let input = CreateRepository {
        name: repo.clone(),
        description: Some(" a greeting ".into()),
        homepage: Some("  ".into()),
        private: Some(true),
        auto_init: Some(true),
        has_issues: None,
        has_projects: None,
        has_wiki: None,
        has_discussions: None,
        gitignore_template: None,
        license_template: Some("mit".into()),
    };

    let repository = generate_github(&server)
        .create_repository(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(repository.id, "77");
    assert_eq!(repository.full_name, answer["full_name"].as_str().unwrap());
}

#[tokio::test]
async fn deletes_a_repository_and_acknowledges_it() {
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer_empty("DELETE", &format!("/repos/octo%20cat/{repo}"), 204)
        .await;
    let input = DeleteRepository {
        owner: "octo cat".into(),
        repo: repo.clone(),
    };

    let done = generate_github(&server)
        .delete_repository(&http(), &token, &input)
        .await
        .unwrap();

    assert!(done.ok);
    assert_eq!(DeleteRepository::SCOPES, ["delete_repo"]);
}

#[tokio::test]
async fn updates_the_settings_of_a_repository() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer_with_body(
            "PATCH",
            &format!("/repos/{owner}/{repo}"),
            json!({ "name": "hi", "default_branch": "trunk", "archived": true }),
            200,
            generate_github_repository(5),
        )
        .await;
    let input = UpdateRepository {
        owner: owner.clone(),
        repo: repo.clone(),
        name: Some("hi".into()),
        description: None,
        homepage: None,
        private: None,
        visibility: None,
        default_branch: Some("trunk".into()),
        has_issues: None,
        has_projects: None,
        has_wiki: None,
        has_discussions: None,
        allow_squash_merge: None,
        allow_merge_commit: None,
        allow_rebase_merge: None,
        allow_auto_merge: None,
        delete_branch_on_merge: None,
        archived: Some(true),
    };

    let repository = generate_github(&server)
        .update_repository(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(repository.id, "5");
}

#[tokio::test]
async fn forks_a_repository_into_an_organization() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer_with_body(
            "POST",
            &format!("/repos/{owner}/{repo}/forks"),
            json!({ "organization": "acme", "default_branch_only": true }),
            202,
            generate_github_repository(9),
        )
        .await;
    let input = ForkRepository {
        owner: owner.clone(),
        repo: repo.clone(),
        organization: Some("acme".into()),
        name: None,
        default_branch_only: Some(true),
    };

    let fork = generate_github(&server)
        .fork_repository(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(fork.id, "9");
}

#[tokio::test]
async fn lists_the_forks_of_a_repository() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/forks"),
            &[("sort", "stargazers"), ("page", "2")],
            200,
            json!([generate_github_repository(3)]),
        )
        .await;
    let input = ListRepositoryForks {
        owner: owner.clone(),
        repo: repo.clone(),
        sort: Some("stargazers".into()),
        per_page: None,
        page: Some(2),
    };

    let forks = generate_github(&server)
        .list_repository_forks(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(forks[0].id, "3");
}

#[tokio::test]
async fn lists_an_organizations_repositories_by_type() {
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/orgs/acme%2Fx/repos",
            &[("type", "sources"), ("direction", "asc")],
            200,
            json!([generate_github_repository(11)]),
        )
        .await;
    let input = ListOrganizationRepositories {
        org: "acme/x".into(),
        repository_type: Some("sources".into()),
        sort: None,
        direction: Some("asc".into()),
        per_page: None,
        page: None,
    };

    let listed = generate_github(&server)
        .list_organization_repositories(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(listed[0].id, "11");
}

#[tokio::test]
async fn lists_a_users_repositories() {
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/users/mona/repos",
            &[("type", "owner"), ("sort", "updated"), ("per_page", "50")],
            200,
            json!([generate_github_repository(12)]),
        )
        .await;
    let input = ListUserRepositories {
        username: "mona".into(),
        repository_type: Some("owner".into()),
        sort: Some("updated".into()),
        direction: None,
        per_page: Some(50),
        page: None,
    };

    let listed = generate_github(&server)
        .list_user_repositories(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(listed[0].id, "12");
}

#[tokio::test]
async fn lists_commits_filtered_by_path_and_author() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let commits = [generate_github_commit(21), generate_github_commit(22)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/commits"),
            &[
                ("sha", "main"),
                ("path", "src/lib.rs"),
                ("author", "mona"),
                ("since", "2026-01-01T00:00:00Z"),
                ("per_page", "2"),
            ],
            200,
            json!(commits),
        )
        .await;
    let input = ListCommits {
        owner: owner.clone(),
        repo: repo.clone(),
        sha: Some("main".into()),
        path: Some("src/lib.rs".into()),
        author: Some("mona".into()),
        committer: None,
        since: Some("2026-01-01T00:00:00Z".into()),
        until: Some(String::new()),
        per_page: Some(2),
        page: None,
    };

    let listed = generate_github(&server)
        .list_commits(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].sha, commits[0]["sha"].as_str().unwrap());
    assert_eq!(
        listed[0].message,
        commits[0]["commit"]["message"].as_str().unwrap()
    );
    assert_eq!(listed[1].author.as_ref().unwrap().id, "22");
}

#[tokio::test]
async fn reads_a_commit_with_its_stats_and_files() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let answer = generate_github_commit(31);
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/commits/release%2F1.0"),
            &[],
            200,
            answer.clone(),
        )
        .await;
    let input = GetCommit {
        owner: owner.clone(),
        repo: repo.clone(),
        reference: "release/1.0".into(),
    };

    let commit = generate_github(&server)
        .get_commit(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(commit.url, answer["html_url"].as_str().unwrap());
    assert_eq!(
        commit.parents,
        [answer["parents"][0]["sha"].as_str().unwrap()]
    );
    assert_eq!(
        commit.stats.unwrap().total,
        answer["stats"]["total"].as_i64().unwrap()
    );
    let file = &commit.files.as_ref().unwrap()[0];
    assert_eq!(file.filename, "src/lib.rs");
    assert_eq!(file.previous_filename.as_deref(), Some("src/old.rs"));
    assert_eq!(file.sha.as_ref().map(String::len), Some(40));
    assert!(file.blob_url.as_deref().unwrap().ends_with("/src/lib.rs"));
    assert!(commit.committer_email.is_some());
    assert_eq!(
        commit.author_email.as_deref(),
        answer["commit"]["author"]["email"].as_str()
    );
}

#[tokio::test]
async fn compares_two_commits() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let answer = generate_github_commit_comparison(3);
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/compare/main...mona%3Afeature"),
            &[("per_page", "10")],
            200,
            answer.clone(),
        )
        .await;
    let input = CompareCommits {
        owner: owner.clone(),
        repo: repo.clone(),
        basehead: "main...mona:feature".into(),
        per_page: Some(10),
        page: None,
    };

    let comparison = generate_github(&server)
        .compare_commits(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(comparison.ahead_by, 3);
    assert_eq!(comparison.commits.len(), 3);
    assert_eq!(
        comparison.commits[2].sha,
        answer["commits"][2]["sha"].as_str().unwrap()
    );
    assert_eq!(comparison.files[0].filename, "README.md");
}

#[tokio::test]
async fn merges_a_branch_and_answers_the_merge_commit() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let answer = generate_github_commit(41);
    MockGithubNetwork::on(&server)
        .answer_with_body(
            "POST",
            &format!("/repos/{owner}/{repo}/merges"),
            json!({ "base": "main", "head": "feature", "commit_message": "Merge it" }),
            201,
            answer.clone(),
        )
        .await;
    let input = MergeBranch {
        owner: owner.clone(),
        repo: repo.clone(),
        base: "main".into(),
        head: "feature".into(),
        commit_message: Some("Merge it".into()),
    };

    let merge = generate_github(&server)
        .merge_branch(&http(), &token, &input)
        .await
        .unwrap();

    assert!(merge.merged);
    assert_eq!(merge.sha.as_deref(), answer["sha"].as_str());
    assert_eq!(merge.message, answer["commit"]["message"].as_str().unwrap());
}

#[tokio::test]
async fn a_merge_with_nothing_to_merge_answers_not_merged() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer_empty("POST", &format!("/repos/{owner}/{repo}/merges"), 204)
        .await;
    let input = MergeBranch {
        owner: owner.clone(),
        repo: repo.clone(),
        base: "main".into(),
        head: "main".into(),
        commit_message: None,
    };

    let merge = generate_github(&server)
        .merge_branch(&http(), &token, &input)
        .await
        .unwrap();

    assert!(!merge.merged);
    assert_eq!(merge.sha, None);
}

#[tokio::test]
async fn renames_a_branch() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let answer = generate_github_branch(false);
    MockGithubNetwork::on(&server)
        .answer_with_body(
            "POST",
            &format!("/repos/{owner}/{repo}/branches/feature%2Fold/rename"),
            json!({ "new_name": "feature/new" }),
            201,
            answer.clone(),
        )
        .await;
    let input = RenameBranch {
        owner: owner.clone(),
        repo: repo.clone(),
        branch: "feature/old".into(),
        new_name: "feature/new".into(),
    };

    let branch = generate_github(&server)
        .rename_branch(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(branch.name, answer["name"].as_str().unwrap());
}

#[tokio::test]
async fn syncs_a_fork_branch_with_its_upstream() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let answer = generate_github_fork_sync("main");
    MockGithubNetwork::on(&server)
        .answer_with_body(
            "POST",
            &format!("/repos/{owner}/{repo}/merge-upstream"),
            json!({ "branch": "main" }),
            200,
            answer.clone(),
        )
        .await;
    let input = SyncForkBranchWithUpstream {
        owner: owner.clone(),
        repo: repo.clone(),
        branch: "main".into(),
    };

    let sync = generate_github(&server)
        .sync_fork_branch_with_upstream(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(sync.merge_type.as_deref(), Some("fast-forward"));
    assert_eq!(sync.base_branch.as_deref(), answer["base_branch"].as_str());
    assert_eq!(sync.message, answer["message"].as_str().unwrap());
}

#[tokio::test]
async fn creates_a_reference() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let sha = generate_sha();
    let answer = generate_github_git_ref("refs/heads/feature");
    MockGithubNetwork::on(&server)
        .answer_with_body(
            "POST",
            &format!("/repos/{owner}/{repo}/git/refs"),
            json!({ "ref": "refs/heads/feature", "sha": sha }),
            201,
            answer.clone(),
        )
        .await;
    let input = CreateRef {
        owner: owner.clone(),
        repo: repo.clone(),
        reference: "refs/heads/feature".into(),
        sha,
    };

    let reference = generate_github(&server)
        .create_ref(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(reference.ref_name, "refs/heads/feature");
    assert_eq!(reference.sha, answer["object"]["sha"].as_str().unwrap());
    assert_eq!(reference.object_type, "commit");
}

#[tokio::test]
async fn reads_a_reference_keeping_its_slashes_and_encoding_its_parts() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/git/ref/heads/feature/a%20b"),
            &[],
            200,
            generate_github_git_ref("refs/heads/feature/a b"),
        )
        .await;
    let input = GetRef {
        owner: owner.clone(),
        repo: repo.clone(),
        reference: "heads/feature/a b".into(),
    };

    let reference = generate_github(&server)
        .get_ref(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(reference.ref_name, "refs/heads/feature/a b");
}

#[tokio::test]
async fn lists_the_references_matching_a_prefix() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/git/matching-refs/heads/feat"),
            &[],
            200,
            json!([
                generate_github_git_ref("refs/heads/feat-a"),
                generate_github_git_ref("refs/heads/feat-b"),
            ]),
        )
        .await;
    let input = ListMatchingRefs {
        owner: owner.clone(),
        repo: repo.clone(),
        reference: "heads/feat".into(),
    };

    let references = generate_github(&server)
        .list_matching_refs(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(references.len(), 2);
    assert_eq!(references[1].ref_name, "refs/heads/feat-b");
}

#[tokio::test]
async fn moves_a_reference_by_force() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let sha = generate_sha();
    MockGithubNetwork::on(&server)
        .answer_with_body(
            "PATCH",
            &format!("/repos/{owner}/{repo}/git/refs/tags/v1"),
            json!({ "sha": sha, "force": true }),
            200,
            generate_github_git_ref("refs/tags/v1"),
        )
        .await;
    let input = UpdateRef {
        owner: owner.clone(),
        repo: repo.clone(),
        reference: "tags/v1".into(),
        sha,
        force: Some(true),
    };

    let reference = generate_github(&server)
        .update_ref(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(reference.ref_name, "refs/tags/v1");
}

#[tokio::test]
async fn deletes_a_reference_and_acknowledges_it() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer_empty(
            "DELETE",
            &format!("/repos/{owner}/{repo}/git/refs/heads/old"),
            204,
        )
        .await;
    let input = DeleteRef {
        owner: owner.clone(),
        repo: repo.clone(),
        reference: "heads/old".into(),
    };

    let done = generate_github(&server)
        .delete_ref(&http(), &token, &input)
        .await
        .unwrap();

    assert!(done.ok);
}

#[rstest]
#[case::get("get")]
#[case::update("update")]
#[case::delete("delete")]
#[tokio::test]
async fn refuses_a_reference_that_is_not_a_branch_or_a_tag_without_asking_github(
    #[case] action: &str,
) {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .expect_nothing_at(&format!("/repos/{owner}/{repo}/git/ref/pull/1/head"))
        .await;
    let github = generate_github(&server);
    let (owner, repo, reference) = (owner.clone(), repo.clone(), "pull/1/head");

    let refused = match action {
        "get" => github
            .get_ref(
                &http(),
                &token,
                &GetRef {
                    owner,
                    repo,
                    reference: reference.into(),
                },
            )
            .await
            .map(|_| ()),
        "update" => github
            .update_ref(
                &http(),
                &token,
                &UpdateRef {
                    owner,
                    repo,
                    reference: reference.into(),
                    sha: generate_sha(),
                    force: None,
                },
            )
            .await
            .map(|_| ()),
        _ => github
            .delete_ref(
                &http(),
                &token,
                &DeleteRef {
                    owner,
                    repo,
                    reference: reference.into(),
                },
            )
            .await
            .map(|_| ()),
    };

    assert!(matches!(refused, Err(ProviderError::Invalid { .. })));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[rstest]
#[case::root(None, "contents")]
#[case::empty(Some(""), "contents")]
#[case::slashes_trimmed(Some("/src/lib/"), "contents/src/lib")]
#[case::parts_encoded(Some("my docs/a#b"), "contents/my%20docs/a%23b")]
#[tokio::test]
async fn lists_the_entries_of_a_directory(#[case] path: Option<&str>, #[case] route: &str) {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let entries = [
        generate_github_content_entry("src/lib.rs", "file"),
        generate_github_content_entry("src/bin", "dir"),
    ];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/{route}"),
            &[("ref", "v1")],
            200,
            json!(entries),
        )
        .await;
    let input = ListDirectoryContents {
        owner: owner.clone(),
        repo: repo.clone(),
        path: path.map(str::to_owned),
        reference: Some("v1".into()),
    };

    let listed = generate_github(&server)
        .list_directory_contents(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].entry_type, "file");
    assert_eq!(listed[1].entry_type, "dir");
    assert_eq!(listed[0].sha, entries[0]["sha"].as_str().unwrap());
    assert_eq!(listed[1].download_url, None);
}

#[tokio::test]
async fn refuses_to_list_a_file_as_a_directory() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/contents/README.md"),
            &[],
            200,
            generate_github_file_content("README.md", b"hi"),
        )
        .await;
    let input = ListDirectoryContents {
        owner: owner.clone(),
        repo: repo.clone(),
        path: Some("README.md".into()),
        reference: None,
    };

    let refused = generate_github(&server)
        .list_directory_contents(&http(), &token, &input)
        .await;

    assert!(matches!(refused, Err(ProviderError::Invalid { .. })));
}

#[tokio::test]
async fn reads_a_text_file_decoded() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let text = generate_text().repeat(5);
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/contents/docs/guide%201.md"),
            &[("ref", "main")],
            200,
            generate_github_file_content("docs/guide 1.md", text.as_bytes()),
        )
        .await;
    let input = GetFileContents {
        owner: owner.clone(),
        repo: repo.clone(),
        path: "docs/guide 1.md".into(),
        reference: Some("main".into()),
    };

    let file = generate_github(&server)
        .get_file_contents(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(file.path, "docs/guide 1.md");
    assert_eq!(file.content_base64, STANDARD.encode(text.as_bytes()));
    assert_eq!(file.decoded_content.as_deref(), Some(text.as_str()));
}

#[tokio::test]
async fn reads_a_binary_file_without_decoding_it() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let bytes = [0xff_u8, 0xfe, 0x00, 0x80];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/contents/logo.png"),
            &[],
            200,
            generate_github_file_content("logo.png", &bytes),
        )
        .await;
    let input = GetFileContents {
        owner: owner.clone(),
        repo: repo.clone(),
        path: "logo.png".into(),
        reference: None,
    };

    let file = generate_github(&server)
        .get_file_contents(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(file.content_base64, STANDARD.encode(bytes));
    assert_eq!(file.decoded_content, None);
}

#[rstest]
#[case::a_directory(json!([generate_github_content_entry("src/lib.rs", "file")]))]
#[case::a_symlink(generate_github_content_entry("src", "symlink"))]
#[tokio::test]
async fn refuses_to_read_what_is_not_a_regular_file(#[case] answer: serde_json::Value) {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/contents/src"),
            &[],
            200,
            answer,
        )
        .await;
    let input = GetFileContents {
        owner: owner.clone(),
        repo: repo.clone(),
        path: "src".into(),
        reference: None,
    };

    let refused = generate_github(&server)
        .get_file_contents(&http(), &token, &input)
        .await;

    assert!(matches!(refused, Err(ProviderError::Invalid { .. })));
}

#[tokio::test]
async fn reads_the_readme_decoded() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let text = generate_text();
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/readme"),
            &[("ref", "dev")],
            200,
            generate_github_file_content("README.md", text.as_bytes()),
        )
        .await;
    let input = GetRepositoryReadme {
        owner: owner.clone(),
        repo: repo.clone(),
        reference: Some("dev".into()),
    };

    let readme = generate_github(&server)
        .get_repository_readme(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(readme.name, "README.md");
    assert_eq!(readme.decoded_content.as_deref(), Some(text.as_str()));
}

#[rstest]
#[case::text(Some("hello world"), None, "aGVsbG8gd29ybGQ=")]
#[case::base64_unwrapped(Some("ignored"), Some("aGVs\r\nbG8="), "aGVsbG8=")]
#[case::empty(None, None, "")]
#[tokio::test]
async fn writes_a_file_with_its_content_base64_encoded(
    #[case] content: Option<&str>,
    #[case] content_base64: Option<&str>,
    #[case] sent: &str,
) {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let message: String = Sentence(3..6).fake();
    let answer = generate_github_content_change(Some("docs/a b.md"));
    MockGithubNetwork::on(&server)
        .answer_with_body(
            "PUT",
            &format!("/repos/{owner}/{repo}/contents/docs/a%20b.md"),
            json!({ "message": message, "content": sent, "branch": "dev" }),
            201,
            answer.clone(),
        )
        .await;
    let input = CreateOrUpdateFile {
        owner: owner.clone(),
        repo: repo.clone(),
        path: "docs/a b.md".into(),
        message: message.clone(),
        content: content.map(str::to_owned),
        content_base64: content_base64.map(str::to_owned),
        sha: Some(" ".into()),
        branch: Some("dev".into()),
    };

    let change = generate_github(&server)
        .create_or_update_file(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(change.content.unwrap().path, "docs/a b.md");
    assert_eq!(change.commit.sha, answer["commit"]["sha"].as_str().unwrap());
    assert_eq!(
        change.commit.url,
        answer["commit"]["html_url"].as_str().unwrap()
    );
}

#[tokio::test]
async fn deletes_a_file_in_a_commit() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let sha = generate_sha();
    let answer = generate_github_content_change(None);
    MockGithubNetwork::on(&server)
        .answer_with_body(
            "DELETE",
            &format!("/repos/{owner}/{repo}/contents/old.txt"),
            json!({ "message": "Remove it", "sha": sha }),
            200,
            answer.clone(),
        )
        .await;
    let input = DeleteFile {
        owner: owner.clone(),
        repo: repo.clone(),
        path: "old.txt".into(),
        message: "Remove it".into(),
        sha,
        branch: None,
    };

    let change = generate_github(&server)
        .delete_file(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(change.content, None);
    assert_eq!(change.commit.sha, answer["commit"]["sha"].as_str().unwrap());
}

#[rstest]
#[case::up_to_the_user("../../../../user")]
#[case::up_to_the_repository("..")]
#[case::current_folder("./notes/../../hooks")]
#[tokio::test]
async fn refuses_a_file_path_that_climbs_out_of_the_repository_without_asking_github(
    #[case] path: &str,
) {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    // No answer is arranged: a request reaching GitHub would fail the test another way.
    let server = MockServer::start().await;
    let github = generate_github(&server);
    let read = GetFileContents {
        owner: owner.clone(),
        repo: repo.clone(),
        path: path.into(),
        reference: None,
    };
    let delete = DeleteFile {
        owner: owner.clone(),
        repo: repo.clone(),
        path: path.into(),
        message: "Remove it".into(),
        sha: generate_sha(),
        branch: None,
    };

    let read = github.get_file_contents(&http(), &token, &read).await;
    let deleted = github.delete_file(&http(), &token, &delete).await;

    assert!(
        matches!(read, Err(ProviderError::Invalid { .. })),
        "{read:?}"
    );
    assert!(
        matches!(deleted, Err(ProviderError::Invalid { .. })),
        "{deleted:?}"
    );
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn lists_the_tags_of_a_repository() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let tags = [generate_github_tag(), generate_github_tag()];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/tags"),
            &[("per_page", "2")],
            200,
            json!(tags),
        )
        .await;
    let input = ListRepositoryTags {
        owner: owner.clone(),
        repo: repo.clone(),
        per_page: Some(2),
        page: None,
    };

    let listed = generate_github(&server)
        .list_repository_tags(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(listed[1].name, tags[1]["name"].as_str().unwrap());
    assert_eq!(
        listed[1].commit_sha,
        tags[1]["commit"]["sha"].as_str().unwrap()
    );
}

#[tokio::test]
async fn lists_the_languages_of_a_repository_most_used_first() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let rust: i64 = (10_000..20_000).fake();
    let shell: i64 = (1..1000).fake();
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/languages"),
            &[],
            200,
            json!({ "Shell": shell, "Rust": rust }),
        )
        .await;
    let input = ListRepositoryLanguages {
        owner: owner.clone(),
        repo: repo.clone(),
    };

    let languages = generate_github(&server)
        .list_repository_languages(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(languages.len(), 2);
    assert_eq!(
        (languages[0].name.as_str(), languages[0].bytes),
        ("Rust", rust)
    );
    assert_eq!(
        (languages[1].name.as_str(), languages[1].bytes),
        ("Shell", shell)
    );
}

#[tokio::test]
async fn lists_the_contributors_of_a_repository_with_anonymous_ones() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let contributors = [generate_github_contributor(51)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/contributors"),
            &[("anon", "true"), ("page", "1")],
            200,
            json!(contributors),
        )
        .await;
    let input = ListRepositoryContributors {
        owner: owner.clone(),
        repo: repo.clone(),
        anon: Some(true),
        per_page: None,
        page: Some(1),
    };

    let listed = generate_github(&server)
        .list_repository_contributors(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(listed[0].id, "51");
    assert_eq!(
        listed[0].contributions,
        contributors[0]["contributions"].as_i64().unwrap()
    );
}

#[tokio::test]
async fn an_empty_repository_has_no_contributors() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer_empty("GET", &format!("/repos/{owner}/{repo}/contributors"), 204)
        .await;
    let input = ListRepositoryContributors {
        owner: owner.clone(),
        repo: repo.clone(),
        anon: None,
        per_page: None,
        page: None,
    };

    let listed = generate_github(&server)
        .list_repository_contributors(&http(), &token, &input)
        .await
        .unwrap();

    assert!(listed.is_empty());
}

#[tokio::test]
async fn lists_the_topics_of_a_repository() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/topics"),
            &[("per_page", "5")],
            200,
            json!({ "names": ["rust", "api"] }),
        )
        .await;
    let input = ListRepositoryTopics {
        owner: owner.clone(),
        repo: repo.clone(),
        per_page: Some(5),
        page: None,
    };

    let topics = generate_github(&server)
        .list_repository_topics(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(topics.names, ["rust", "api"]);
}

#[tokio::test]
async fn replaces_the_topics_of_a_repository() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer_with_body(
            "PUT",
            &format!("/repos/{owner}/{repo}/topics"),
            json!({ "names": ["cli"] }),
            200,
            json!({ "names": ["cli"] }),
        )
        .await;
    let input = ReplaceRepositoryTopics {
        owner: owner.clone(),
        repo: repo.clone(),
        names: vec!["cli".into()],
    };

    let topics = generate_github(&server)
        .replace_repository_topics(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(topics.names, ["cli"]);
}

#[tokio::test]
async fn lists_the_collaborators_of_a_repository() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/collaborators"),
            &[("affiliation", "direct"), ("permission", "push")],
            200,
            json!([generate_github_collaborator(61)]),
        )
        .await;
    let input = ListRepositoryCollaborators {
        owner: owner.clone(),
        repo: repo.clone(),
        affiliation: Some("direct".into()),
        permission: Some("push".into()),
        per_page: None,
        page: None,
    };

    let listed = generate_github(&server)
        .list_repository_collaborators(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(listed[0].id, "61");
    assert!(listed[0].permissions.as_ref().unwrap().push);
    assert_eq!(listed[0].role_name.as_deref(), Some("write"));
}

#[tokio::test]
async fn invites_a_collaborator_who_must_accept() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer_with_body(
            "PUT",
            &format!("/repos/{owner}/{repo}/collaborators/mona%2Bx"),
            json!({ "permission": "maintain" }),
            201,
            generate_github_repository_invitation(71, 72, "maintain"),
        )
        .await;
    let input = AddRepositoryCollaborator {
        owner: owner.clone(),
        repo: repo.clone(),
        username: "mona+x".into(),
        permission: Some("maintain".into()),
    };

    let added = generate_github(&server)
        .add_repository_collaborator(&http(), &token, &input)
        .await
        .unwrap();

    assert!(added.invited);
    let invitation = added.invitation.unwrap();
    assert_eq!(invitation.id, "71");
    assert_eq!(invitation.permissions, "maintain");
    assert_eq!(invitation.invitee.unwrap().id, "72");
}

#[tokio::test]
async fn adds_a_collaborator_at_once_without_an_invitation() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer_empty(
            "PUT",
            &format!("/repos/{owner}/{repo}/collaborators/mona"),
            204,
        )
        .await;
    let input = AddRepositoryCollaborator {
        owner: owner.clone(),
        repo: repo.clone(),
        username: "mona".into(),
        permission: None,
    };

    let added = generate_github(&server)
        .add_repository_collaborator(&http(), &token, &input)
        .await
        .unwrap();

    assert!(!added.invited);
    assert_eq!(added.invitation, None);
}

#[tokio::test]
async fn removes_a_collaborator_and_acknowledges_it() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer_empty(
            "DELETE",
            &format!("/repos/{owner}/{repo}/collaborators/mona"),
            204,
        )
        .await;
    let input = RemoveRepositoryCollaborator {
        owner: owner.clone(),
        repo: repo.clone(),
        username: "mona".into(),
    };

    let done = generate_github(&server)
        .remove_repository_collaborator(&http(), &token, &input)
        .await
        .unwrap();

    assert!(done.ok);
}

#[tokio::test]
async fn reads_an_accounts_permission_in_a_repository() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/collaborators/mona/permission"),
            &[],
            200,
            generate_github_repository_permission(81, "admin"),
        )
        .await;
    let input = GetRepositoryPermissionForUser {
        owner: owner.clone(),
        repo: repo.clone(),
        username: "mona".into(),
    };

    let permission = generate_github(&server)
        .get_repository_permission_for_user(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(permission.permission, "admin");
    assert_eq!(permission.user.unwrap().id, "81");
}

#[tokio::test]
async fn comments_on_a_line_of_a_commit() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let sha = generate_sha();
    let answer = generate_github_commit_comment(91, 92, "src/lib.rs", 4);
    MockGithubNetwork::on(&server)
        .answer_with_body(
            "POST",
            &format!("/repos/{owner}/{repo}/commits/{sha}/comments"),
            json!({ "body": "Nice", "path": "src/lib.rs", "position": 4 }),
            201,
            answer.clone(),
        )
        .await;
    let input = CreateCommitComment {
        owner: owner.clone(),
        repo: repo.clone(),
        commit_sha: sha,
        body: "Nice".into(),
        path: Some("src/lib.rs".into()),
        position: Some(4),
    };

    let comment = generate_github(&server)
        .create_commit_comment(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(comment.id, "91");
    assert_eq!(comment.position, Some(4));
    assert_eq!(comment.url, answer["html_url"].as_str().unwrap());
    assert_eq!(comment.user.unwrap().id, "92");
}

#[tokio::test]
async fn lists_the_comments_on_a_commit() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let sha = generate_sha();
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/commits/{sha}/comments"),
            &[("per_page", "30"), ("page", "2")],
            200,
            json!([generate_github_commit_comment(93, 94, "README.md", 1)]),
        )
        .await;
    let input = ListCommitComments {
        owner: owner.clone(),
        repo: repo.clone(),
        commit_sha: sha,
        per_page: Some(30),
        page: Some(2),
    };

    let listed = generate_github(&server)
        .list_commit_comments(&http(), &token, &input)
        .await
        .unwrap();

    assert_eq!(listed[0].id, "93");
    assert_eq!(listed[0].path.as_deref(), Some("README.md"));
}
