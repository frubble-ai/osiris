//! The search text `github.search_issues_and_pull_requests` sends as `q`: the caller's own search
//! text, followed by a GitHub qualifier for each filter given, as `OpenConnector` builds it.

use super::SearchIssuesAndPullRequests;

/// The caller's search text (`query`, else `q`), then the qualifiers its filters stand for, joined
/// by spaces. A filter left out, empty or only spaces adds nothing.
pub fn search_query(input: &SearchIssuesAndPullRequests) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(text) = given(input.query.as_ref()).or_else(|| given(input.q.as_ref())) {
        parts.push(text.to_owned());
    }
    match (given(input.owner.as_ref()), given(input.repo.as_ref())) {
        (Some(owner), Some(repo)) => parts.push(format!("repo:{owner}/{repo}")),
        (_, Some(repo)) if repo.contains('/') => parts.push(format!("repo:{repo}")),
        (Some(owner), _) => parts.push(format!("user:{owner}")),
        _ => {}
    }
    if let Some(state) = given(input.state.as_ref())
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
        if let Some(value) = given(value.as_ref()) {
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

/// `value` without its surrounding spaces, or nothing when that leaves it empty.
fn given(value: Option<&String>) -> Option<&str> {
    value
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
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
