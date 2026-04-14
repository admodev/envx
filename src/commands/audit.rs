//! Schema-based audit of a `.env` file.
//!
//! The schema is a plain-text file with one key name per line.  Lines that
//! begin with `#` or are blank are ignored.  Any key listed in the schema
//! that is missing from the env file, or present but empty, is flagged as an
//! error.  Keys in the env file that are *not* in the schema are flagged as
//! warnings.

use anyhow::{Context, Result};
use owo_colors::OwoColorize;
use std::path::Path;

/// Parse a schema file into a list of required key names.
fn parse_schema(path: &Path) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read schema {}", path.display()))?;

    Ok(content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect())
}

/// Audit `env_path` against the schema at `schema_path`.
///
/// Prints a report to stdout:
/// - `✗ MISSING  KEY` — required key absent from the env file
/// - `⚠ EMPTY    KEY` — required key present but has an empty value
/// - `⚠ EXTRA    KEY` — key in the env file not listed in the schema
///
/// Finishes with a summary line `N error(s), N warning(s)`.
///
/// # Exit behaviour
///
/// Calls `std::process::exit(1)` when there is at least one error (missing
/// or empty required key).
///
/// # Errors
///
/// Returns an error if either file cannot be read or parsed.
pub fn run(schema_path: &Path, env_path: &Path) -> Result<()> {
    let required = parse_schema(schema_path)?;
    let env = crate::parser::parse(env_path)?;

    let mut errors = 0u32;
    let mut warnings = 0u32;

    for key in &required {
        match env.get(key.as_str()) {
            None => {
                println!("{}", format!("✗ MISSING  {key}").red());
                errors += 1;
            }
            Some(v) if v.is_empty() => {
                println!("{}", format!("⚠ EMPTY    {key}").yellow());
                errors += 1;
            }
            Some(_) => {}
        }
    }

    for key in env.keys() {
        if !required.contains(key) {
            println!("{}", format!("⚠ EXTRA    {key} (not in schema)").yellow());
            warnings += 1;
        }
    }

    println!("{} error(s), {} warning(s)", errors, warnings);

    if errors > 0 {
        std::process::exit(1);
    }

    Ok(())
}
