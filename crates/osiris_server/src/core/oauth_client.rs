//! An auth config as the OAuth client a provider needs, with osiris's own redirect.

use osiris_common::secret::ExposeSecret;
use osiris_domain::AuthConfig;
use osiris_interfaces::OAuthClient;

pub fn oauth_client<'a>(config: &'a AuthConfig, redirect_uri: &'a str) -> OAuthClient<'a> {
    OAuthClient {
        client_id: &config.client_id,
        client_secret: config.client_secret.expose_secret(),
        scopes: &config.scopes,
        redirect_uri,
    }
}
