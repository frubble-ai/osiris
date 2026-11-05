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
mod tests {
    #![allow(
        clippy::unwrap_used,
        reason = "tests: a failure here is the test failing"
    )]

    use super::*;
    use fake::{Fake, faker::lorem::en::Word};
    use serde_json::json;

    fn config(provider: &str) -> (String, Value) {
        let id = format!("ac_{}", Word().fake::<String>());
        (id.clone(), json!({ "id": id, "provider": provider }))
    }

    #[test]
    fn chooses_the_provider_s_only_app() {
        let (github, github_config) = config("github");
        let (_, linear_config) = config("linear");
        let configs = json!({ "data": [linear_config, github_config] });

        assert_eq!(choose_auth_config("github", &configs).unwrap(), github);
    }

    #[test]
    fn says_how_to_register_one_when_the_provider_has_none() {
        let (_, linear_config) = config("linear");
        let error = choose_auth_config("github", &json!({ "data": [linear_config] })).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("osiris auth-configs create --provider github")
        );
    }

    #[test]
    fn asks_to_name_one_when_the_provider_has_several() {
        let (first, first_config) = config("github");
        let (second, second_config) = config("github");
        let error = choose_auth_config("github", &json!({ "data": [first_config, second_config] }))
            .unwrap_err();

        assert!(error.to_string().contains("--auth-config"));
        assert!(error.to_string().contains(&first) && error.to_string().contains(&second));
    }
}
