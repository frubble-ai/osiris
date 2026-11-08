//! A comment on a line of the diff, given with a new review.

use super::DiffSide;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A comment on a line of the diff, given with a new review.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewCommentInput {
    /// The file's path in the repository.
    pub path: String,
    pub body: String,
    /// The line the comment is on, or the last line of a range of lines.
    pub line: Option<u64>,
    /// The side of the diff `line` is on.
    pub side: Option<DiffSide>,
    /// The first line of a range of lines the comment is on.
    pub start_line: Option<u64>,
    /// The side of the diff `start_line` is on.
    pub start_side: Option<DiffSide>,
}

/// A review comment as GitHub asks for it in a new review's body.
#[derive(Serialize)]
pub struct ReviewCommentBody<'a> {
    pub path: &'a str,
    pub body: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<DiffSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_side: Option<DiffSide>,
}

impl<'a> From<&'a ReviewCommentInput> for ReviewCommentBody<'a> {
    /// The comment as GitHub asks for it.
    fn from(input: &'a ReviewCommentInput) -> Self {
        Self {
            path: &input.path,
            body: &input.body,
            line: input.line,
            side: input.side,
            start_line: input.start_line,
            start_side: input.start_side,
        }
    }
}
