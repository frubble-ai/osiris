//! Writes osiris's `OpenAPI` document, generated from its routes, to `api-reference/openapi.json`
//! (or the path given). CI regenerates it and fails if the committed one differs.

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "api-reference/openapi.json".to_owned());
    let spec = osiris_server::services::api::spec().to_pretty_json()?;
    if let Some(directory) = std::path::Path::new(&path).parent() {
        std::fs::create_dir_all(directory)
            .with_context(|| format!("creating {}", directory.display()))?;
    }
    std::fs::write(&path, format!("{spec}\n")).with_context(|| format!("writing {path}"))?;
    Ok(())
}
