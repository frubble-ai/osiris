//! The spec alone, for `osiris-openapi`.

use super::v1::v1;

pub fn spec() -> utoipa::openapi::OpenApi {
    v1().split_for_parts().1
}
