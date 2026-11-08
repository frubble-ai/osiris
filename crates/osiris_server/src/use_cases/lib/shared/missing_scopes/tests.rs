//! A missing scope is named; a granted, a covering, or an unscoped token passes.

use super::missing_scopes;
use osiris_interfaces::ProviderCommon;
use rstest::rstest;
use url::Url;

/// A provider whose `admin` scope covers every `admin:` one, as a provider with nesting scopes
/// says.
struct Nesting(Url);

impl ProviderCommon for Nesting {
    fn id(&self) -> &'static str {
        "nesting"
    }

    fn base_url(&self) -> &Url {
        &self.0
    }

    fn scope_covers(&self, granted: &str, required: &str) -> bool {
        granted == required
            || required
                .strip_prefix(granted)
                .is_some_and(|rest| rest.starts_with(':'))
    }
}

fn nesting() -> Nesting {
    Nesting(Url::parse("https://nesting.test").unwrap())
}

#[rstest]
#[case::granted(Some("read,write"), &["read"], &[])]
#[case::spaces(Some("read write"), &["write"], &[])]
#[case::missing(Some("read"), &["read", "write"], &["write"])]
#[case::covered_by_a_wider_scope(Some("admin"), &["admin:users", "admin:teams"], &[])]
#[case::a_narrower_scope_covers_nothing_wider(Some("admin:users"), &["admin"], &["admin"])]
#[case::no_scopes_recorded(None, &["read"], &[])]
#[case::a_token_with_no_scopes(Some(""), &["read"], &[])]
fn names_the_scopes_a_connection_lacks(
    #[case] granted: Option<&str>,
    #[case] required: &[&str],
    #[case] missing: &[&str],
) {
    assert_eq!(missing_scopes(&nesting(), granted, required), missing);
}
