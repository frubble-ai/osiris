//! Prints an answer.

use serde_json::Value;

/// Print an answer as indented JSON.
pub fn print(value: &Value) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
