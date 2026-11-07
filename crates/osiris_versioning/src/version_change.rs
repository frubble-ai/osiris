//! One breaking change, as the release that made it describes it: how a request written for the
//! release before is brought up to it, and how its answers, its events and its spec are taken back
//! to the release before. The code only knows the current model; older releases exist only as
//! these changes, applied one after another (Stripe's and Intercom's version change modules).
//!
//! A change only touches what it is about and leaves every other route, status and event as it is.

use super::Route;
use serde_json::Value;

pub trait VersionChange: Sync {
    /// What changed, as clients read it: a test requires these words in the release's changelog
    /// (`docs/changelog/<god>.md`), so the changelog lists every breaking change.
    fn description(&self) -> &'static str;

    /// The `OpenAPI` document at this release, turned into the one of the release before.
    fn spec(&self, _spec: &mut Value) {}

    /// A request's JSON body written for the release before, brought up to this release.
    fn request(&self, _route: &Route<'_>, _body: &mut Value) {}

    /// A request's query written for the release before, brought up to this release.
    fn query(&self, _route: &Route<'_>, _query: &mut Vec<(String, String)>) {}

    /// An answer's JSON body at this release, taken back to the release before.
    fn response(&self, _route: &Route<'_>, _status: u16, _body: &mut Value) {}

    /// An event's JSON at this release, taken back to the release before.
    fn event(&self, _event: &mut Value) {}
}
