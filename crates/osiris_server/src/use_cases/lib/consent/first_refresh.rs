//! Decides, at connect, whether the new connection needs a refresh task, from whether its
//! provider issues refresh tokens and whether this consent returned one.

/// What a finished consent needs before the connection can be activated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirstRefresh {
    /// The provider issues refresh tokens and sent one: schedule the first refresh, then activate.
    Schedule,
    /// The provider issues no refresh tokens (its tokens never expire): activate with nothing
    /// scheduled.
    NotNeeded,
    /// The provider issues refresh tokens but sent none (for example, the app has expiring tokens
    /// or token rotation turned off): the connection can't be kept fresh, so it is not activated.
    MissingRefreshToken,
}

pub fn first_refresh(issues_refresh_tokens: bool, got_refresh_token: bool) -> FirstRefresh {
    match (issues_refresh_tokens, got_refresh_token) {
        // A refresh token is always scheduled, even from a provider that says it issues none.
        (_, true) => FirstRefresh::Schedule,
        (true, false) => FirstRefresh::MissingRefreshToken,
        (false, false) => FirstRefresh::NotNeeded,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_a_refresh_token_only_from_a_provider_that_issues_them() {
        assert_eq!(first_refresh(true, true), FirstRefresh::Schedule);
        assert_eq!(
            first_refresh(true, false),
            FirstRefresh::MissingRefreshToken
        );
        assert_eq!(first_refresh(false, false), FirstRefresh::NotNeeded);
        assert_eq!(first_refresh(false, true), FirstRefresh::Schedule);
    }
}
