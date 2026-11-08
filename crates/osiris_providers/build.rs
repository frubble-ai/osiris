//! Lists the providers this crate implements: every folder in `src/` that holds a
//! `registration.rs`. The list is written to `registrations.rs` in the build's output folder, so
//! a provider registers itself from its own folder and no other file names it.

use std::{fmt::Write as _, fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = Path::new("src");
    println!("cargo::rerun-if-changed=src");
    let mut ids = Vec::new();
    for entry in fs::read_dir(source)? {
        let path = entry?.path();
        if path.join("registration.rs").is_file()
            && let Some(name) = path.file_name().and_then(|name| name.to_str())
        {
            ids.push(name.to_owned());
        }
    }
    ids.sort();
    let mut entries = String::new();
    for id in &ids {
        writeln!(entries, "    &crate::{id}::registration::REGISTRATION,")?;
    }
    let list = format!(
        "/// Every provider this crate implements, by the folders that register one.\n\
         pub static REGISTRATIONS: &[&Registration] = &[\n{entries}];\n"
    );
    let out = std::env::var("OUT_DIR")?;
    fs::write(Path::new(&out).join("registrations.rs"), list)?;
    Ok(())
}
