//! Which of the listed auth configs a connect goes through: the provider's only one, or an error
//! that says what to do when it has none or several.

use anyhow::bail;
use serde_json::Value;

pub fn choose_auth_config(provider: &str, configs: &Value) -> anyhow::Result<String> {
    let matching: Vec<String> = configs
        .get("data")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|config| config.get("provider").and_then(Value::as_str) == Some(provider))
        .filter_map(|config| config.get("id").and_then(Value::as_str).map(str::to_owned))
        .collect();
    match matching.as_slice() {
        [one] => Ok(one.clone()),
        [] => bail!(
            "no auth config for {provider}: `osiris auth-configs create --provider {provider} …`"
        ),
        _ => bail!(
            "several auth configs for {provider}: name one with --auth-config ({})",
            matching.join(", ")
        ),
    }
}

#[cfg(test)]
mod tests;
