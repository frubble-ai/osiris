//! What an idempotency key says about a request that carries it.

use serde_json::Value;

pub enum Begun {
    /// First time: run it, then finish it.
    New,
    /// Ran before: answer this again.
    Replay { status: u16, body: Value },
    /// The key was used for another request.
    Mismatch,
    /// The first request with this key is still running.
    InFlight,
}
