//! The tokens minted for app installations, by auth config and installation, each until it
//! expires.

use osiris_common::secret::Secret;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, PoisonError},
};
use time::{Duration, OffsetDateTime};

type Key = (String, String);

#[derive(Default)]
pub struct InstallationTokenCache {
    tokens: Mutex<HashMap<Key, (Secret, OffsetDateTime)>>,
    /// One lock per installation, held while its token is minted, so calls that miss the cache at
    /// once mint one token between them.
    minting: Mutex<HashMap<Key, Arc<tokio::sync::Mutex<()>>>>,
}

impl InstallationTokenCache {
    /// The lock to hold while minting a token for `installation_id` of `auth_config_id`.
    pub fn minting(
        &self,
        auth_config_id: &str,
        installation_id: &str,
    ) -> Arc<tokio::sync::Mutex<()>> {
        self.minting
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .entry((auth_config_id.to_owned(), installation_id.to_owned()))
            .or_default()
            .clone()
    }

    /// Lets go of `lock`, the one [`Self::minting`] answered, once its mint is done: the entry is
    /// dropped when nothing else holds it, so ids that are never minted again leave nothing
    /// behind.
    pub fn done_minting(
        &self,
        auth_config_id: &str,
        installation_id: &str,
        lock: &Arc<tokio::sync::Mutex<()>>,
    ) {
        let mut minting = self.minting.lock().unwrap_or_else(PoisonError::into_inner);
        // Callers clone the lock under this same mutex, so two holders (the map and `lock`)
        // means no one else is waiting on it.
        if Arc::strong_count(lock) == 2 {
            minting.remove(&(auth_config_id.to_owned(), installation_id.to_owned()));
        }
    }

    /// How many installations' mint locks are held, for tests.
    #[cfg(test)]
    pub fn minting_locks(&self) -> usize {
        self.minting
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .len()
    }

    /// The token for `installation_id` of `auth_config_id`, if one is held that stays good past
    /// `now` plus `margin`.
    pub fn get(
        &self,
        auth_config_id: &str,
        installation_id: &str,
        now: OffsetDateTime,
        margin: Duration,
    ) -> Option<Secret> {
        let tokens = self.tokens.lock().unwrap_or_else(PoisonError::into_inner);
        tokens
            .get(&(auth_config_id.to_owned(), installation_id.to_owned()))
            .filter(|(_, expires_at)| expires_at.saturating_sub(margin) > now)
            .map(|(token, _)| token.clone())
    }

    /// Keeps `token` for `installation_id` of `auth_config_id` until `expires_at`.
    pub fn put(
        &self,
        auth_config_id: &str,
        installation_id: &str,
        token: Secret,
        expires_at: OffsetDateTime,
    ) {
        self.tokens
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(
                (auth_config_id.to_owned(), installation_id.to_owned()),
                (token, expires_at),
            );
    }

    /// Drops the token for `installation_id` of `auth_config_id`: the provider refused it.
    pub fn forget(&self, auth_config_id: &str, installation_id: &str) {
        self.tokens
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .remove(&(auth_config_id.to_owned(), installation_id.to_owned()));
    }

    /// Drops every token minted with `auth_config_id`'s credentials, which have changed.
    pub fn forget_auth_config(&self, auth_config_id: &str) {
        self.tokens
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .retain(|(config, _), _| config != auth_config_id);
    }
}
