//! Finishing a consent: what the provider sent back, where the browser goes next, and how the
//! connection ends up (active, disabled or failed).

pub mod activate_consent;
pub mod back_to;
pub mod consent_error_reason;
pub mod consent_failed;
pub mod disable_consent;
pub mod first_refresh;
pub mod types;

pub use types::{Callback, Outcome};
