//! Lists every provider's action router: each folder in `src/routes/actions/` that holds a
//! `router.rs`. The list is written to `provider_action_routers.rs` in the build's output folder,
//! so a provider's routes are found from its own folder and no other file names it.

use std::{fmt::Write as _, fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = Path::new("src/routes/actions");
    println!("cargo::rerun-if-changed=src/routes/actions");
    let mut ids = Vec::new();
    for entry in fs::read_dir(source)? {
        let path = entry?.path();
        if path.join("router.rs").is_file()
            && let Some(name) = path.file_name().and_then(|name| name.to_str())
        {
            ids.push(name.to_owned());
        }
    }
    ids.sort();
    let mut routers = String::new();
    for id in &ids {
        writeln!(routers, "        crate::routes::actions::{id}::router(),")?;
    }
    let count = ids.len();
    let list = format!(
        "/// Every provider's action router, by the folders that hold one.\n\
         fn provider_routers() -> [OpenApiRouter<ApiState>; {count}] {{\n    [\n{routers}    ]\n}}\n"
    );
    let out = std::env::var("OUT_DIR")?;
    fs::write(Path::new(&out).join("provider_action_routers.rs"), list)?;
    Ok(())
}
