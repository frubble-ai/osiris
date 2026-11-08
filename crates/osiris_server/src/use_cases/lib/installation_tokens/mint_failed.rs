//! A mark on an error: an installation's token couldn't be minted with its app's credentials
//! (the provider refused them, or they are missing or can't sign), so the failure is the app's
//! credentials, not what the call was about. A caller that keeps working once the credentials
//! are fixed, such as a trigger's check, reads it.

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct MintFailed;

impl fmt::Display for MintFailed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .write_str("the installation's token couldn't be minted with its app's credentials")
    }
}
