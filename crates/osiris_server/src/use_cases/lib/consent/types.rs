//! What the provider sent back, and where the browser goes next.

pub struct Callback {
    pub state: Option<String>,
    pub code: Option<String>,
    /// The provider's own error, when the user refused or something failed there.
    pub error: Option<String>,
}

pub enum Outcome {
    /// To the client, with the connection's id and status.
    Redirect(String),
    /// No consent of ours: there is no client to send the browser to.
    Unknown,
}
