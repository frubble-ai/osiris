//! What the API adds to the configuration: its two keys.

use super::{Config, Settings, env_vars::secret};
use osiris_common::secret::{ExposeSecret, Secret};

#[derive(Clone)]
pub struct ApiConfig {
    pub config: Config,
    /// What a client sends to use connections and actions.
    pub api_key: Secret,
    /// What an operator sends to register providers' apps, refresh connections and trigger events; never given to a client.
    pub admin_key: Secret,
}

impl ApiConfig {
    pub fn from_env(settings: Settings) -> anyhow::Result<Self> {
        let (api_key, admin_key) = (secret("OSIRIS_API_KEY")?, secret("OSIRIS_ADMIN_KEY")?);
        ensure_distinct_keys(&api_key, &admin_key)?;
        Ok(Self {
            config: Config::from_env(settings)?,
            api_key,
            admin_key,
        })
    }
}

/// The two keys must differ: with the same value, the API key would pass every admin check.
pub fn ensure_distinct_keys(api_key: &Secret, admin_key: &Secret) -> anyhow::Result<()> {
    anyhow::ensure!(
        api_key.expose_secret() != admin_key.expose_secret(),
        "OSIRIS_ADMIN_KEY must differ from OSIRIS_API_KEY: the API key would pass every admin check"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use osiris_common::secret::secret;

    #[test]
    fn refuses_an_admin_key_equal_to_the_api_key() {
        let key = secret("a-key-that-is-long-enough-0123456789");
        let error = ensure_distinct_keys(&key, &key.clone()).unwrap_err();
        assert!(error.to_string().contains("OSIRIS_ADMIN_KEY"));
        assert!(ensure_distinct_keys(&key, &secret("another-key-that-is-long-enough-01")).is_ok());
    }
}
