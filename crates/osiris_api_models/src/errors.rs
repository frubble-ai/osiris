//! The error every endpoint answers with: `{ "error": { "code", "type", "message" } }`. `code` is
//! stable (`OS_xx`); `message` is for people.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorBody {
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorDetail {
    /// `OS_00` unauthorized, `OS_01` not found, `OS_02` invalid request, `OS_03` conflict,
    /// `OS_04` provider refused, `OS_05` provider unavailable, `OS_06` connection not active,
    /// `OS_07` unknown version, `OS_08` idempotency key reused, `OS_09` request in flight,
    /// `OS_10` body too large, `OS_11` missing scopes, `OS_99` internal. An open string: codes may
    /// be added.
    pub code: String,
    /// `unauthorized`, `not_found`, `invalid_request`, `conflict`, `provider_refused`,
    /// `provider_unavailable`, `connection_not_active`, `unknown_version`, `idempotency_mismatch`,
    /// `in_flight`, `too_large`, `missing_scopes`, `internal`.
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}
