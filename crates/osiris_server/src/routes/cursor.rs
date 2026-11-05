//! A list request's cursor, its length bounded by `[pagination]`.

use crate::http::state::ApiState;
use osiris_storage::Cursor;

pub fn cursor(
    state: &ApiState,
    after: Option<String>,
    before: Option<String>,
    limit: Option<i64>,
) -> Cursor {
    Cursor {
        after,
        before,
        limit: state.config.settings.pagination.limit(limit),
    }
}
