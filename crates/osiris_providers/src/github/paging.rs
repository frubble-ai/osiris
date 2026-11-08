//! The page a list action asks GitHub for, as GitHub names it: `per_page` (at most 100) and
//! `page`, from 1. Left out, GitHub's own defaults apply.

/// The query a list sends for `per_page` and `page`, those given.
pub fn paging(per_page: Option<u32>, page: Option<u32>) -> Vec<(&'static str, String)> {
    let mut query = Vec::new();
    if let Some(per_page) = per_page {
        query.push(("per_page", per_page.to_string()));
    }
    if let Some(page) = page {
        query.push(("page", page.to_string()));
    }
    query
}
