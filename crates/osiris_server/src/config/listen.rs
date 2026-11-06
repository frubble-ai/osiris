//! Where a process listens: every interface (`0.0.0.0`) on `PORT`, which Cloud Run sets and which
//! is required everywhere, locally too.

use super::env_vars::required;
use anyhow::Context;

pub fn listen_address() -> anyhow::Result<String> {
    let port = required("PORT")?;
    let port: u16 = port
        .trim()
        .parse()
        .with_context(|| format!("PORT must be a port number, not {port:?}"))?;
    Ok(format!("0.0.0.0:{port}"))
}
