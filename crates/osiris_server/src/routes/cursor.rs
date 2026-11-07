//! A list request's cursor, its length bounded by `[pagination]`. A cursor is an item's id, so one
//! with a character no id has (a NUL, a space, anything but ASCII letters, digits and `_`) is
//! refused as `OS_02` before it reaches the database, which can't compare some of them at all. An
//! empty cursor (`?after=`) is no cursor.

use crate::http::{error::ApiError, state::ApiState};
use osiris_domain::Cursor;

pub fn cursor(
    state: &ApiState,
    after: Option<String>,
    before: Option<String>,
    limit: Option<i64>,
) -> Result<Cursor, ApiError> {
    let after = after.filter(|given| !given.is_empty());
    let before = before.filter(|given| !given.is_empty());
    for given in [&after, &before].into_iter().flatten() {
        if !given
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
        {
            return Err(ApiError::Invalid(
                "the cursor names a record that no longer exists".into(),
            ));
        }
    }
    Ok(Cursor {
        after,
        before,
        limit: state.config.settings.pagination.limit(limit),
    })
}
