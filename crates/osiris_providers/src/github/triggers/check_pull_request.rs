//! A watched pull request checked: its state now, and whether it changed since the last check.
//! Its `version` is the SHA-256 of what is watched (title, state, draft, merged, the head and
//! base commits, when it was updated), so the same state is the same version. The first check
//! only records the state, as `OpenConnector`'s watch starts from the pull request as it is.

use super::WatchPullRequestConfig;
use crate::{
    ProviderError, TriggerCheck,
    github::{Github, pull_requests::GetPullRequest},
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

impl Github {
    /// The watched pull request's check, against `checkpoint` (`{version, sequence}`) from the last.
    pub async fn check_pull_request(
        &self,
        http: &reqwest::Client,
        token: &str,
        config: &WatchPullRequestConfig,
        checkpoint: Option<&Value>,
    ) -> Result<TriggerCheck, ProviderError> {
        let current = self
            .get_pull_request(
                http,
                token,
                &GetPullRequest {
                    owner: config.owner.clone(),
                    repo: config.repo.clone(),
                    pull_number: config.number,
                },
            )
            .await?;
        let pull_request = json!({
            "id": current.id,
            "number": current.number,
            "title": current.title,
            "url": current.url,
            "state": current.state,
            "draft": current.draft,
            "merged": current.merged,
            "updated_at": current.updated_at,
            "head_sha": current.head.sha,
            "base_sha": current.base.sha,
        });
        // serde_json writes an object's keys in order, so the same state is the same bytes.
        let digest = Sha256::digest(pull_request.to_string().as_bytes());
        let version = hex::encode(digest);
        let last_version = checkpoint
            .and_then(|seen| seen.get("version"))
            .and_then(Value::as_str);
        let sequence = checkpoint
            .and_then(|seen| seen.get("sequence"))
            .and_then(Value::as_u64)
            .unwrap_or(0);
        let Some(last_version) = last_version else {
            return Ok(TriggerCheck {
                checkpoint: json!({ "version": version, "sequence": 0 }),
                fired: None,
            });
        };
        if last_version == version {
            return Ok(TriggerCheck {
                checkpoint: json!({ "version": version, "sequence": sequence }),
                fired: None,
            });
        }
        let sequence = sequence + 1;
        Ok(TriggerCheck {
            checkpoint: json!({ "version": version, "sequence": sequence }),
            fired: Some((
                format!("{sequence}-{version}"),
                json!({ "pull_request": pull_request, "version": version }),
            )),
        })
    }
}
