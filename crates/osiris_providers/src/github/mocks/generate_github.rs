//! A GitHub provider whose every URL is the faked network's.

use crate::{ListLimits, ProviderUrls, github::Github};
use url::Url;
use wiremock::MockServer;

/// GitHub at `server`, with room for any list a test arranges.
pub fn generate_github(server: &MockServer) -> Github {
    generate_github_with(
        server,
        ListLimits {
            concurrency: 2,
            max_items: 5000,
            max_log_bytes: 262_144,
            max_scan_pages: 10,
        },
    )
}

/// GitHub at `server`, its lists going as far as `limits`.
#[allow(clippy::missing_panics_doc, clippy::expect_used, reason = "test setup")]
pub fn generate_github_with(server: &MockServer, limits: ListLimits) -> Github {
    let base = Url::parse(&server.uri()).expect("the faked network's URL");
    Github::new(
        &ProviderUrls {
            oauth_base: base.clone(),
            api_base: base,
        },
        limits,
    )
    .expect("the faked network's URLs")
}
