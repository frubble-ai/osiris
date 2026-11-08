//! The repository a trigger's config names, checked before anything is sent to GitHub.

/// Whether `owner` and `repo` can name a repository: letters, digits, `.`, `_` and `-`, at most
/// 100 each, and not `.` or `..`. GitHub's own account rules vary (an Enterprise Managed User's
/// login holds an `_`), so only what could change the path or can't be a name is refused; GitHub
/// answers for the rest.
pub fn repository_names(owner: &str, repo: &str) -> Result<(), String> {
    let owner_ok = (1..=100).contains(&owner.len())
        && owner != "."
        && owner != ".."
        && owner.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
        });
    if !owner_ok {
        return Err(format!(
            "config.owner {owner:?} is not a GitHub account name"
        ));
    }
    let repo_ok = (1..=100).contains(&repo.len())
        && repo != "."
        && repo != ".."
        && repo.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
        });
    if !repo_ok {
        return Err(format!(
            "config.repo {repo:?} is not a GitHub repository name"
        ));
    }
    Ok(())
}
