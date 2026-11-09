//! The search text `github.search_issues_and_pull_requests` sends as `q`: the caller's own search
//! text, followed by a GitHub qualifier for each filter given, as `OpenConnector` builds it.

use super::SearchIssuesAndPullRequests;
use crate::github::trimmed::trimmed;

/// The caller's search text (`query`, else `q`), then the qualifiers its filters stand for, joined
/// by spaces. A filter left out, empty or only spaces adds nothing.
pub fn search_query(input: &SearchIssuesAndPullRequests) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(text) = trimmed(input.query.as_deref()).or_else(|| trimmed(input.q.as_deref())) {
        parts.push(text.to_owned());
    }
    match (
        trimmed(input.owner.as_deref()),
        trimmed(input.repo.as_deref()),
    ) {
        (Some(owner), Some(repo)) => parts.push(format!("repo:{owner}/{repo}")),
        (_, Some(repo)) if repo.contains('/') => parts.push(format!("repo:{repo}")),
        (Some(owner), _) => parts.push(format!("user:{owner}")),
        _ => {}
    }
    if let Some(state) = trimmed(input.state.as_deref())
        && state != "all"
    {
        parts.push(format!("state:{state}"));
    }
    match input.kind.as_deref() {
        Some("issue") => parts.push("is:issue".to_owned()),
        Some("pr") => parts.push("is:pr".to_owned()),
        _ => {}
    }
    for (value, qualifier) in [
        (&input.label, "label"),
        (&input.author, "author"),
        (&input.assignee, "assignee"),
        (&input.mentions, "mentions"),
        (&input.language, "language"),
        (&input.base_branch, "base"),
        (&input.head_branch, "head"),
    ] {
        if let Some(value) = trimmed(value.as_deref()) {
            parts.push(format!("{qualifier}:{}", quoted(value)));
        }
    }
    match input.is_merged {
        Some(true) => parts.push("is:merged".to_owned()),
        Some(false) => parts.push("is:unmerged".to_owned()),
        None => {}
    }
    parts.join(" ").trim().to_owned()
}

/// `value` in double quotes, escaped as JSON, when it holds a space, so that GitHub reads it as
/// one value; as it is otherwise.
fn quoted(value: &str) -> String {
    if value.chars().any(char::is_whitespace) {
        serde_json::to_string(value).unwrap_or_else(|_| value.to_owned())
    } else {
        value.to_owned()
    }
}

#[cfg(test)]
mod tests;
