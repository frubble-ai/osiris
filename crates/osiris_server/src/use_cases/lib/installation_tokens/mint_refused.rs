//! A mark on an error: the provider refused to mint an installation's token, so the failure is the
//! app's credentials (rotated, revoked, the app uninstalled), not what the call was about. A
//! caller that keeps working once the credentials are fixed, such as a trigger's check, reads it.

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct MintRefused;

impl fmt::Display for MintRefused {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("the provider refused to mint the installation's token")
    }
}
