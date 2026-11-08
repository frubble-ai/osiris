//! GitHub's actions are checked against its own scope rules: a wider scope covers the ones it
//! nests, and a GitHub App's user token, which carries no scopes, is not checked.

use crate::use_cases::lib::shared::missing_scopes::missing_scopes;
use osiris_providers::{ListLimits, ProviderUrls, github::Github};
use rstest::rstest;
use url::Url;

fn github() -> Github {
    let base = Url::parse("https://github.test").unwrap();
    let urls = ProviderUrls {
        oauth_base: base.clone(),
        api_base: base,
    };
    Github::new(
        &urls,
        ListLimits {
            concurrency: 1,
            max_items: 10,
            max_log_bytes: 262_144,
        },
    )
    .unwrap()
}

#[rstest]
#[case::granted(Some("repo,workflow"), &["repo"], &[])]
#[case::missing(Some("repo"), &["repo", "workflow"], &["workflow"])]
#[case::repo_covers_its_parts(Some("repo"), &["repo:status"], &[])]
#[case::user_covers_its_reads(Some("user"), &["read:user", "user:email"], &[])]
#[case::an_app_token_with_no_scopes(Some(""), &["repo"], &[])]
fn names_the_scopes_a_github_connection_lacks(
    #[case] granted: Option<&str>,
    #[case] required: &[&str],
    #[case] missing: &[&str],
) {
    assert_eq!(missing_scopes(&github(), granted, required), missing);
}
