//! The query parameters a command was given: the ones left out are not sent.

pub fn present_params<const N: usize>(
    params: [(&'static str, Option<String>); N],
) -> Vec<(&'static str, String)> {
    params
        .into_iter()
        .filter_map(|(name, value)| value.map(|value| (name, value)))
        .collect()
}

#[cfg(test)]
mod tests;
