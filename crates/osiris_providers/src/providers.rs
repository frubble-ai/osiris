//! The providers osiris implements, found by id, or by type for a provider's own actions.

use crate::{
    ListLimits, Provider, ProviderUrls,
    registration::{REGISTRATIONS, Registered, Registration},
};

/// Every registered provider, built at the URLs its settings give.
pub struct Providers {
    built: Vec<Registered>,
}

impl Providers {
    /// Builds every registered provider at the URLs `urls` answers for its id.
    pub fn new<E: From<url::ParseError>>(
        urls: impl Fn(&Registration) -> Result<ProviderUrls, E>,
        limits: ListLimits,
    ) -> Result<Self, E> {
        Self::of(REGISTRATIONS, urls, limits)
    }

    /// Builds the providers of `registrations` (a test adds its fake provider to them).
    pub fn of<E: From<url::ParseError>>(
        registrations: &[&Registration],
        urls: impl Fn(&Registration) -> Result<ProviderUrls, E>,
        limits: ListLimits,
    ) -> Result<Self, E> {
        let mut built = Vec::with_capacity(registrations.len());
        for registration in registrations {
            built.push((registration.build)(&urls(registration)?, limits)?);
        }
        Ok(Self { built })
    }

    /// The provider whose id is `id`.
    pub fn get(&self, id: &str) -> Option<&dyn Provider> {
        self.built
            .iter()
            .map(|registered| registered.provider.as_ref())
            .find(|provider| provider.id() == id)
    }

    /// The provider of type `P`, for its own actions.
    pub fn typed<P: Provider + 'static>(&self) -> Option<&P> {
        self.built
            .iter()
            .find_map(|registered| registered.typed.downcast_ref::<P>())
    }

    /// Every provider's id.
    pub fn ids(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.built.iter().map(|registered| registered.provider.id())
    }
}
