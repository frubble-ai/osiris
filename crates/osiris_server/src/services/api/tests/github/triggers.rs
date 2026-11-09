//! Triggers through the API: created at GitHub as a repository webhook delivering to osiris, each
//! signed delivery published as a `trigger.fired` event, an unsigned one refused as if the trigger
//! didn't exist, a watched pull request checked on its deliveries and on schedule, and the webhook
//! removed when the trigger or its connection is deleted.

mod checking;
mod creating;
mod deleting;
mod delivering;
mod support;
