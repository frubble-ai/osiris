//! An auth config's OAuth client, as a provider needs it.

/// Decrypted, with osiris's own callback as the redirect.
pub struct OAuthClient<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub scopes: &'a [String],
    pub redirect_uri: &'a str,
}
