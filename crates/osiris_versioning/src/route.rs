//! The route a request was sent to, as a version change sees it: its method and the route's
//! template, such as `GET /v1/connected-accounts/{id}`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Route<'a> {
    /// Upper case, such as `GET`.
    pub method: &'a str,
    /// The route's template, such as `/v1/connected-accounts/{id}`.
    pub path: &'a str,
}

impl Route<'_> {
    /// Whether this is `method` on `path`.
    pub fn is(&self, method: &str, path: &str) -> bool {
        self.method.eq_ignore_ascii_case(method) && self.path == path
    }
}
