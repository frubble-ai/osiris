//! The scopes an action needs that a connection wasn't granted, checked before the provider is
//! called. A connection whose token carries no scopes at all (an app's user token, whose
//! reach is the app's installation permissions) is not checked here: the provider refuses what it
//! may not do.

use osiris_interfaces::ProviderCommon;

/// The scopes of `required` that `granted` (the provider's scope answer, comma- or space-separated)
/// doesn't cover, by the provider's own rules; none when `granted` is empty or absent.
pub fn missing_scopes<'a>(
    provider: &dyn ProviderCommon,
    granted: Option<&str>,
    required: &[&'a str],
) -> Vec<&'a str> {
    let granted: Vec<&str> = granted
        .unwrap_or_default()
        .split(|character: char| character == ',' || character.is_whitespace())
        .filter(|scope| !scope.is_empty())
        .collect();
    if granted.is_empty() {
        return Vec::new();
    }
    required
        .iter()
        .copied()
        .filter(|needed| {
            !granted
                .iter()
                .any(|scope| provider.scope_covers(scope, needed))
        })
        .collect()
}

#[cfg(test)]
mod tests;
