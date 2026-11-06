//! `[pagination]`: how long a list's pages are.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct PaginationSettings {
    /// A page's length when the request names none.
    pub default_limit: i64,
    /// The longest page a request may ask for.
    pub max_limit: i64,
}

impl PaginationSettings {
    /// The page length for a request's `limit`: its own, within bounds, or the default.
    pub fn limit(&self, asked: Option<i64>) -> i64 {
        match asked {
            Some(limit) if limit > 0 => limit.min(self.max_limit),
            _ => self.default_limit,
        }
    }
}

impl Default for PaginationSettings {
    fn default() -> Self {
        Self {
            default_limit: 20,
            max_limit: 100,
        }
    }
}

#[cfg(test)]
mod tests;
