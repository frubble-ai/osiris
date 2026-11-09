//! `github.rerequest_check_suite`: asks the app that made a check suite to run it again.

use crate::{
    ProviderError,
    github::{
        Github, github_id::GithubId, path_segment::path_segment, request::send,
        types::GithubAcknowledgement,
    },
};
use reqwest::Method;
use serde::{Deserialize, de::IgnoredAny};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RerequestCheckSuite {
    pub owner: String,
    pub repo: String,
    pub check_suite_id: GithubId,
}

impl RerequestCheckSuite {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["workflow"];
}

impl Github {
    /// `POST /repos/{owner}/{repo}/check-suites/{check_suite_id}/rerequest`, whose answer carries
    /// nothing of its own.
    pub async fn rerequest_check_suite(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &RerequestCheckSuite,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}/check-suites/{}/rerequest",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.check_suite_id.to_string())
        );
        let _: IgnoredAny =
            send::<_, ()>(self, http, token, Method::POST, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
