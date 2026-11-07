//! The spec's path template a concrete path was sent to: the one matching it segment by segment
//! with the most literal segments, so `/v1/auth-configs/{id}` loses to a literal route of the same
//! length.

use serde_json::Value;

/// The path template of `spec` that `path` was sent to, if any.
pub fn template_of<'a>(spec: &'a Value, path: &str) -> Option<&'a str> {
    let segments: Vec<&str> = path.split('/').collect();
    spec.get("paths")?
        .as_object()?
        .keys()
        .filter_map(|template| {
            let parts: Vec<&str> = template.split('/').collect();
            if parts.len() != segments.len() {
                return None;
            }
            let mut literal = 0;
            for (part, segment) in parts.iter().zip(&segments) {
                if part.starts_with('{') && part.ends_with('}') {
                    continue;
                }
                if part != segment {
                    return None;
                }
                literal += 1;
            }
            Some((literal, template.as_str()))
        })
        .max_by_key(|(literal, _)| *literal)
        .map(|(_, template)| template)
}
