//! A list answered in batches, and how to ask for the next or the previous one.

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListParams {
    /// A batch's `next_cursor`: the batch after it.
    pub after: Option<String>,
    /// A batch's `previous_cursor`: the batch before it.
    pub before: Option<String>,
    /// How many items a batch holds, within the server's bounds.
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct List<T> {
    pub data: Vec<T>,
    /// Pass as `after` for the next batch; null on the last.
    pub next_cursor: Option<String>,
    /// Pass as `before` for the previous batch; null on the first.
    pub previous_cursor: Option<String>,
}
