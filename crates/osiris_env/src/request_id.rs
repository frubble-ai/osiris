//! Every request carries an `X-Request-Id`: the caller's, or a fresh UUID. It is echoed on the
//! response and recorded on the request's span, so one id follows a request through the logs.

use http::HeaderName;
use tower_http::request_id::{
    MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};

pub const HEADER: HeaderName = HeaderName::from_static("x-request-id");

pub fn set_layer() -> SetRequestIdLayer<MakeRequestUuid> {
    SetRequestIdLayer::new(HEADER, MakeRequestUuid)
}

pub fn propagate_layer() -> PropagateRequestIdLayer {
    PropagateRequestIdLayer::new(HEADER)
}

/// The id of a request, for its span.
pub fn of<B>(request: &http::Request<B>) -> String {
    request
        .extensions()
        .get::<RequestId>()
        .and_then(|id| id.header_value().to_str().ok())
        .unwrap_or("none")
        .to_owned()
}
