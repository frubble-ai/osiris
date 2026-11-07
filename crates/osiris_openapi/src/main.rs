//! Writes osiris's `OpenAPI` documents, generated from its routes, under `api-reference/` (or the
//! directory given): `openapi.json` at the current release, and `versions/<release>.json` for every
//! release osiris serves, each taken back through the changes made since. CI regenerates them and
//! fails if the committed ones differ.

use anyhow::Context;
use osiris_api_models::version::RELEASES;
use osiris_server::services::api::{spec, versioned_spec};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let root = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "api-reference".to_owned());
    let root = Path::new(&root);
    write(&root.join("openapi.json"), &spec())?;
    for release in RELEASES {
        write(
            &root.join("versions").join(format!("{}.json", release.name)),
            &versioned_spec(RELEASES, release),
        )?;
    }
    Ok(())
}

fn write(path: &Path, spec: &serde_json::Value) -> anyhow::Result<()> {
    if let Some(directory) = path.parent() {
        std::fs::create_dir_all(directory)
            .with_context(|| format!("creating {}", directory.display()))?;
    }
    let text = serde_json::to_string_pretty(spec)?;
    std::fs::write(path, format!("{text}\n")).with_context(|| format!("writing {}", path.display()))
}
