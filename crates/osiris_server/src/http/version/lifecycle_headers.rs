//! A superseded release's answers say so (RFC 9745 and RFC 8594): `Deprecation` is the day the
//! next release came out, and `Sunset` that day plus `[api] support_days`, after which the release
//! may be removed. The current release's answers carry neither.

use super::superseded::superseded;
use axum::http::{HeaderMap, HeaderValue};
use osiris_api_models::version::Release;
use time::{Duration, format_description::FormatItem, macros::format_description};

/// An HTTP date (RFC 9110), such as `Sun, 06 Nov 1994 08:49:37 GMT`.
const HTTP_DATE: &[FormatItem<'_>] = format_description!(
    "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT"
);

/// Adds `Deprecation` and `Sunset` to `headers` when a release after `release` is out.
pub fn lifecycle_headers(
    releases: &[Release],
    release: &Release,
    support: Duration,
    headers: &mut HeaderMap,
) {
    let Some((since, sunset)) = superseded(releases, release, support) else {
        return;
    };
    if let Ok(value) = HeaderValue::from_str(&format!("@{}", since.unix_timestamp())) {
        headers.insert("deprecation", value);
    }
    if let Some(value) = sunset
        .format(HTTP_DATE)
        .ok()
        .and_then(|text| HeaderValue::from_str(&text).ok())
    {
        headers.insert("sunset", value);
    }
}
