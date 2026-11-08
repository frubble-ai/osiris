//! A missing scope is named; a granted, a covering, or an unscoped token passes.

use super::missing_scopes;
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
        },
    )
    .unwrap()
}

#[rstest]
#[case::granted(Some("repo,workflow"), &["repo"], &[])]
#[case::spaces(Some("repo workflow"), &["workflow"], &[])]
#[case::missing(Some("repo"), &["repo", "workflow"], &["workflow"])]
#[case::covered_by_a_wider_scope(Some("repo"), &["repo:status"], &[])]
#[case::user_covers_its_reads(Some("user"), &["read:user", "user:email"], &[])]
#[case::no_scopes_recorded(None, &["repo"], &[])]
#[case::an_app_token_with_no_scopes(Some(""), &["repo"], &[])]
fn names_the_scopes_a_connection_lacks(
    #[case] granted: Option<&str>,
    #[case] required: &[&str],
    #[case] missing: &[&str],
) {
    assert_eq!(missing_scopes(&github(), granted, required), missing);
}
