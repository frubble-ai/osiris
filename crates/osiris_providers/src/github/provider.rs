//! GitHub's provider: its OAuth definition, its URLs, and how it ends a grant.

use super::{
    app::AppCredentials,
    triggers::{
        ON_REPO_EVENT, RepoEventConfig, WATCH_PULL_REQUEST, WatchPullRequestConfig,
        receive_delivery::receive_delivery,
    },
};
use crate::{
    InstallationToken, ListLimits, OAuthClient, Provider, ProviderCommon, ProviderError,
    ProviderUrls, TriggerCheck, TriggerReceipt,
};
use async_trait::async_trait;
use osiris_interfaces::oauth::{ClientAuth, OAuth2Definition};
use reqwest::header::HeaderMap;
use serde_json::Value;
use url::Url;

pub struct Github {
    pub(super) oauth: OAuth2Definition,
    pub(super) api_base: Url,
    pub(super) limits: ListLimits,
}

impl Github {
    pub fn new(urls: &ProviderUrls, limits: ListLimits) -> Result<Self, url::ParseError> {
        Ok(Self {
            oauth: OAuth2Definition {
                authorization_url: urls.oauth_base.join("/login/oauth/authorize")?,
                token_url: urls.oauth_base.join("/login/oauth/access_token")?,
                refresh_token_url: None,
                scope_separator: " ",
                client_auth: ClientAuth::Post,
                pkce: false,
                authorization_params: &[],
                // GitHub Apps issue refresh tokens when expiring user tokens are on (the default).
                issues_refresh_tokens: true,
            },
            api_base: urls.api_base.clone(),
            limits,
        })
    }
}

impl ProviderCommon for Github {
    fn id(&self) -> &'static str {
        "github"
    }

    fn base_url(&self) -> &Url {
        &self.api_base
    }

    /// GitHub's scopes nest: `repo` covers `repo:status` and the other `repo:` scopes, `user`
    /// covers `read:user` and `user:email`, and `write:` and `admin:` cover the `read:` of the
    /// same thing (GitHub's scope docs).
    fn scope_covers(&self, granted: &str, required: &str) -> bool {
        if granted == required {
            return true;
        }
        match (granted, required) {
            ("repo", needed) => needed.starts_with("repo:") || needed == "public_repo",
            ("user", "read:user" | "user:email" | "user:follow") => true,
            (wider, needed) => {
                let Some(thing) = needed.strip_prefix("read:") else {
                    return needed
                        .strip_prefix("write:")
                        .is_some_and(|thing| wider == format!("admin:{thing}"));
                };
                wider == format!("write:{thing}") || wider == format!("admin:{thing}")
            }
        }
    }
}

#[async_trait]
impl Provider for Github {
    fn oauth(&self) -> &OAuth2Definition {
        &self.oauth
    }

    async fn revoke(
        &self,
        http: &reqwest::Client,
        client: &OAuthClient<'_>,
        access_token: &str,
    ) -> Result<(), ProviderError> {
        super::revoke::revoke(self, http, client, access_token).await
    }

    /// A GitHub app may carry its App credentials, to act as its installations: `app_id` in
    /// `settings` and `private_key` in `secret_settings`, both or neither.
    fn validate_settings(
        &self,
        settings: &Value,
        secret_settings: Option<&Value>,
    ) -> Result<(), String> {
        let empty = |value: &Value| value.as_object().is_none_or(serde_json::Map::is_empty);
        if empty(settings) && secret_settings.is_none_or(empty) {
            return Ok(());
        }
        AppCredentials::read(settings, secret_settings).map(|_| ())
    }

    async fn installation_token(
        &self,
        http: &reqwest::Client,
        settings: &Value,
        secret_settings: &Value,
        installation_id: &str,
    ) -> Result<InstallationToken, ProviderError> {
        let credentials = AppCredentials::read(settings, Some(secret_settings))
            .map_err(|reason| ProviderError::refused("github", reason))?;
        self.mint_installation_token(http, &credentials, installation_id)
            .await
    }

    fn validate_trigger(&self, kind: &str, config: &Value) -> Result<(), String> {
        match kind {
            ON_REPO_EVENT => RepoEventConfig::read(config).map(|_| ()),
            WATCH_PULL_REQUEST => WatchPullRequestConfig::read(config).map(|_| ()),
            _ => Err(format!(
                "github has no trigger {kind:?}; it has {ON_REPO_EVENT} and {WATCH_PULL_REQUEST}"
            )),
        }
    }

    fn trigger_checks_on_schedule(&self, kind: &str) -> bool {
        kind == WATCH_PULL_REQUEST
    }

    async fn subscribe_trigger(
        &self,
        http: &reqwest::Client,
        token: &str,
        kind: &str,
        config: &Value,
        callback_url: &str,
        secret: &str,
    ) -> Result<Option<String>, ProviderError> {
        let refused = |reason: String| ProviderError::refused("github", reason);
        let (owner, repo, events, insecure_ssl) = match kind {
            ON_REPO_EVENT => {
                let config = RepoEventConfig::read(config).map_err(refused)?;
                (
                    config.owner,
                    config.repo,
                    config.events,
                    config.insecure_ssl,
                )
            }
            WATCH_PULL_REQUEST => {
                let config = WatchPullRequestConfig::read(config).map_err(refused)?;
                (
                    config.owner,
                    config.repo,
                    vec!["pull_request".to_owned()],
                    false,
                )
            }
            _ => return Err(refused(format!("github has no trigger {kind:?}"))),
        };
        self.create_hook(
            http,
            token,
            &owner,
            &repo,
            callback_url,
            secret,
            &events,
            insecure_ssl,
        )
        .await
        .map(Some)
    }

    async fn unsubscribe_trigger(
        &self,
        http: &reqwest::Client,
        token: &str,
        _kind: &str,
        config: &Value,
        subscription_id: &str,
    ) -> Result<(), ProviderError> {
        let field = |name: &str| config.get(name).and_then(Value::as_str).unwrap_or_default();
        let (owner, repo) = (field("owner"), field("repo"));
        self.delete_hook(http, token, owner, repo, subscription_id)
            .await
    }

    fn receive_trigger(
        &self,
        kind: &str,
        config: &Value,
        headers: &HeaderMap,
        body: &[u8],
        secret: &str,
    ) -> TriggerReceipt {
        receive_delivery(kind, config, headers, body, secret)
    }

    async fn check_trigger(
        &self,
        http: &reqwest::Client,
        token: &str,
        kind: &str,
        config: &Value,
        checkpoint: Option<&Value>,
    ) -> Result<TriggerCheck, ProviderError> {
        if kind != WATCH_PULL_REQUEST {
            return Err(ProviderError::refused(
                "github",
                format!("{kind} isn't checked on a schedule"),
            ));
        }
        let config = WatchPullRequestConfig::read(config)
            .map_err(|reason| ProviderError::refused("github", reason))?;
        self.check_pull_request(http, token, &config, checkpoint)
            .await
    }
}
