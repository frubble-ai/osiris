//! Releases and their assets, read from and written to GitHub and put in our terms, with every
//! value placed in the path percent-encoded.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    CreateRelease, DeleteRelease, DeleteReleaseAsset, GenerateReleaseNotes, GetLatestRelease,
    GetRelease, GetReleaseAsset, GetReleaseByTag, ListReleaseAssets, ListReleases, UpdateRelease,
    mocks::{
        MockGithubReleaseRequests, generate_github_release::generate_github_release,
        generate_github_release_asset::generate_github_release_asset,
    },
};
use crate::github::github_id::GithubId;
use crate::{
    ProviderError,
    github::mocks::{
        MockGithubNetwork,
        generate_github::generate_github,
        generate_github_names::{
            generate_github_login, generate_github_repo, generate_github_token,
        },
    },
};
use fake::{Fake, faker::lorem::en::Sentence};
use rstest::rstest;
use serde_json::{Value, json};
use wiremock::MockServer;

#[tokio::test]
async fn lists_a_page_of_releases() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let releases = [
        generate_github_release(11, "v2.0.0"),
        generate_github_release(10, "v1.0.0"),
    ];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/releases"),
            &[("per_page", "2"), ("page", "1")],
            200,
            json!(releases),
        )
        .await;
    let input = ListReleases {
        owner: owner.clone(),
        repo: repo.clone(),
        per_page: Some(2),
        page: Some(1),
    };

    let listed = generate_github(&server)
        .list_releases(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, "11");
    assert_eq!(listed[0].tag_name, "v2.0.0");
    assert_eq!(listed[0].url, releases[0]["html_url"].as_str().unwrap());
    assert_eq!(
        listed[0].author.as_ref().unwrap().login,
        releases[0]["author"]["login"].as_str().unwrap()
    );
}

#[tokio::test]
async fn creates_a_release_sending_only_the_fields_given() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let notes: String = Sentence(3..8).fake();
    MockGithubReleaseRequests::on(&server)
        .answer_with_body(
            "POST",
            &format!("/repos/{owner}/{repo}/releases"),
            json!({
                "tag_name": "v1.0.0",
                "body": notes,
                "prerelease": true,
                "make_latest": "false",
            }),
            201,
            generate_github_release(12, "v1.0.0"),
        )
        .await;
    let input = CreateRelease {
        owner: owner.clone(),
        repo: repo.clone(),
        tag_name: "v1.0.0".into(),
        target_commitish: None,
        name: None,
        body: Some(notes),
        draft: None,
        prerelease: Some(true),
        generate_release_notes: None,
        make_latest: Some("false".into()),
    };

    let release = generate_github(&server)
        .create_release(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(release.id, "12");
    assert_eq!(release.tag_name, "v1.0.0");
}

#[tokio::test]
async fn creates_a_release_with_its_target_trimmed_a_blank_latest_left_out_and_its_name_as_given() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubReleaseRequests::on(&server)
        .answer_with_body(
            "POST",
            &format!("/repos/{owner}/{repo}/releases"),
            json!({
                "tag_name": "v1.0.0",
                "target_commitish": "main",
                "name": "  Spring  ",
            }),
            201,
            generate_github_release(13, "v1.0.0"),
        )
        .await;
    let input = CreateRelease {
        owner: owner.clone(),
        repo: repo.clone(),
        tag_name: "v1.0.0".into(),
        target_commitish: Some(" main ".into()),
        name: Some("  Spring  ".into()),
        body: None,
        draft: None,
        prerelease: None,
        generate_release_notes: None,
        make_latest: Some("   ".into()),
    };

    let release = generate_github(&server)
        .create_release(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(release.id, "13");
}

#[tokio::test]
async fn reads_a_release_by_its_id() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let mut answer = generate_github_release(13, "v1.2.0");
    answer["assets"] = json!([generate_github_release_asset(21)]);
    answer["reactions"] = json!({"total_count": 2, "+1": 1, "-1": 0, "laugh": 0, "confused": 0, "heart": 1, "hooray": 0, "rocket": 0, "eyes": 0});
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/releases/13"),
            &[],
            200,
            answer.clone(),
        )
        .await;
    let input = GetRelease {
        owner: owner.clone(),
        repo: repo.clone(),
        release_id: GithubId(13),
    };

    let release = generate_github(&server)
        .get_release(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(release.id, "13");
    assert_eq!(release.name.as_deref(), answer["name"].as_str());
    assert_eq!(
        release.published_at.as_deref(),
        Some("2026-11-01T10:05:00Z")
    );
    assert_eq!(
        Some(release.upload_url.as_str()),
        answer["upload_url"].as_str()
    );
    assert_eq!(release.assets.len(), 1);
    assert_eq!(release.assets[0].id, "21");
    let reactions = release.reactions.unwrap();
    assert_eq!(
        (reactions.total_count, reactions.plus_one, reactions.heart),
        (2, 1, 1)
    );
    assert_eq!(release.updated_at.as_deref(), Some("2026-11-01T10:06:00Z"));
    assert!(!release.immutable);
}

#[tokio::test]
async fn reads_the_latest_release() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/releases/latest"),
            &[],
            200,
            generate_github_release(14, "v3.0.0"),
        )
        .await;
    let input = GetLatestRelease {
        owner: owner.clone(),
        repo: repo.clone(),
    };

    let release = generate_github(&server)
        .get_latest_release(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(release.tag_name, "v3.0.0");
    assert!(!release.draft);
}

#[rstest]
#[case::plain("v1.0.0", "releases/tags/v1.0.0")]
#[case::with_a_slash("release/1.0", "releases/tags/release%2F1.0")]
#[tokio::test]
async fn reads_a_release_by_its_tag_encoded_into_one_segment(
    #[case] tag: &str,
    #[case] route: &str,
) {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/{route}"),
            &[],
            200,
            generate_github_release(15, tag),
        )
        .await;
    let input = GetReleaseByTag {
        owner: owner.clone(),
        repo: repo.clone(),
        tag: tag.into(),
    };

    let release = generate_github(&server)
        .get_release_by_tag(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(release.tag_name, tag);
}

#[tokio::test]
async fn lists_a_page_of_a_releases_assets() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let assets = [generate_github_release_asset(21)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/releases/16/assets"),
            &[("per_page", "50")],
            200,
            json!(assets),
        )
        .await;
    let input = ListReleaseAssets {
        owner: owner.clone(),
        repo: repo.clone(),
        release_id: GithubId(16),
        per_page: Some(50),
        page: None,
    };

    let listed = generate_github(&server)
        .list_release_assets(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, "21");
    assert_eq!(
        listed[0].download_url,
        assets[0]["browser_download_url"].as_str().unwrap()
    );
    assert_eq!(listed[0].size, assets[0]["size"].as_i64().unwrap());
}

#[tokio::test]
async fn updates_a_release_sending_only_the_fields_given() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubReleaseRequests::on(&server)
        .answer_with_body(
            "PATCH",
            &format!("/repos/{owner}/{repo}/releases/17"),
            json!({ "name": "Spring", "draft": false }),
            200,
            generate_github_release(17, "v1.0.0"),
        )
        .await;
    let input = UpdateRelease {
        owner: owner.clone(),
        repo: repo.clone(),
        release_id: GithubId(17),
        tag_name: None,
        target_commitish: None,
        name: Some("Spring".into()),
        body: None,
        draft: Some(false),
        prerelease: None,
        make_latest: None,
    };

    let release = generate_github(&server)
        .update_release(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(release.id, "17");
}

#[tokio::test]
async fn generates_release_notes_for_a_tag() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let body: String = Sentence(3..8).fake();
    MockGithubReleaseRequests::on(&server)
        .answer_with_body(
            "POST",
            &format!("/repos/{owner}/{repo}/releases/generate-notes"),
            json!({ "tag_name": "v2.0.0", "previous_tag_name": "v1.0.0" }),
            200,
            json!({ "name": "v2.0.0", "body": body }),
        )
        .await;
    let input = GenerateReleaseNotes {
        owner: owner.clone(),
        repo: repo.clone(),
        tag_name: "v2.0.0".into(),
        target_commitish: None,
        previous_tag_name: Some("v1.0.0".into()),
        configuration_file_path: None,
    };

    let notes = generate_github(&server)
        .generate_release_notes(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(notes.name, "v2.0.0");
    assert_eq!(notes.body, body);
}

#[tokio::test]
async fn reads_a_release_asset_by_its_id() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    let answer = generate_github_release_asset(22);
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/releases/assets/22"),
            &[],
            200,
            answer.clone(),
        )
        .await;
    let input = GetReleaseAsset {
        owner: owner.clone(),
        repo: repo.clone(),
        asset_id: GithubId(22),
    };

    let asset = generate_github(&server)
        .get_release_asset(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(asset.id, "22");
    assert_eq!(asset.name, answer["name"].as_str().unwrap());
    assert_eq!(asset.label, None);
    assert_eq!(asset.state, "uploaded");
    assert_eq!(asset.digest.as_deref(), answer["digest"].as_str());
}

#[rstest]
#[case::release("releases/18", false)]
#[case::asset("releases/assets/18", true)]
#[tokio::test]
async fn deletes_a_release_or_an_asset_and_acknowledges_it(
    #[case] route: &str,
    #[case] asset: bool,
) {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "DELETE",
            &format!("/repos/{owner}/{repo}/{route}"),
            &[],
            204,
            Value::Null,
        )
        .await;
    let github = generate_github(&server);
    let http = reqwest::Client::new();

    let acknowledged = if asset {
        let input = DeleteReleaseAsset {
            owner: owner.clone(),
            repo: repo.clone(),
            asset_id: GithubId(18),
        };
        github.delete_release_asset(&http, &token, &input).await
    } else {
        let input = DeleteRelease {
            owner: owner.clone(),
            repo: repo.clone(),
            release_id: GithubId(18),
        };
        github.delete_release(&http, &token, &input).await
    };

    assert!(acknowledged.unwrap().ok);
}

#[tokio::test]
async fn a_missing_release_is_a_refusal() {
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/releases/latest"),
            &[],
            404,
            json!({ "message": "Not Found" }),
        )
        .await;
    let input = GetLatestRelease {
        owner: owner.clone(),
        repo: repo.clone(),
    };

    let error = generate_github(&server)
        .get_latest_release(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap_err();

    assert!(matches!(error, ProviderError::Refused { .. }));
}
