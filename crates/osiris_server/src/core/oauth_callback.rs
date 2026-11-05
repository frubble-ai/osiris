//! Where the provider sends the browser after its consent: the code becomes tokens, and the
//! browser goes on to the client's `callback_url` with the connection's id and status.

pub mod activate_consent;
pub mod back_to;
pub mod consent_error;
pub mod consent_failed;
pub mod disable_consent;
pub mod exchange;
pub mod finish_consent;
pub mod first_refresh;
pub mod types;

pub use types::{Callback, Outcome};
