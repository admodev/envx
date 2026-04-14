//! Semantic diff between two `.env` files.
//!
//! Values for keys that match common sensitive patterns (`SECRET`, `KEY`,
//! `TOKEN`, `PASSWORD`, `PASS`, `PWD`) are redacted with bullet characters so
//! secrets are never echoed to the terminal.

use crate::parser;
use anyhow::Result;
use owo_colors::OwoColorize;
use std::path::Path;

/// Key substrings that trigger value redaction.
const SENSITIVE_PATTERNS: &[&str] = &["SECRET", "KEY", "TOKEN", "PASSWORD", "PASS", "PWD"];

fn is_sensitive(key: &str) -> bool {
    let upper = key.to_uppercase();
    SENSITIVE_PATTERNS.iter().any(|p| upper.contains(p))
}

fn redact(value: &str) -> String {
    "•".repeat(value.len().max(7))
}

/// Compare `file_a` and `file_b` and print a coloured semantic diff.
///
/// Keys present in both files with equal values are shown dimmed.  Changed
/// values are shown as `- old` / `+ new` lines (or redacted for sensitive
/// keys).  Keys only in one file are shown as removed (`-`) or added (`+`).
///
/// # Exit behaviour
///
/// Calls `std::process::exit(1)` when any difference is found so the
/// function can be used as a CI gate.
///
/// # Errors
///
/// Returns an error if either file cannot be parsed.
pub fn run(file_a: &Path, file_b: &Path) -> Result<()> {
    let map_a = parser::parse(file_a)?;
    let map_b = parser::parse(file_b)?;

    println!("{}", format!("--- {}", file_a.display()).bold());
    println!("{}", format!("+++ {}", file_b.display()).bold());

    let mut differences = false;

    // Collect all keys preserving order: A first, then new keys in B
    let mut all_keys: Vec<&str> = map_a.keys().map(String::as_str).collect();
    for k in map_b.keys() {
        if !map_a.contains_key(k.as_str()) {
            all_keys.push(k.as_str());
        }
    }

    for key in all_keys {
        match (map_a.get(key), map_b.get(key)) {
            (Some(va), Some(vb)) if va == vb => {
                println!("  {}", format!("{key}={va}").dimmed());
            }
            (Some(va), Some(vb)) => {
                differences = true;
                if is_sensitive(key) {
                    println!(
                        "{}",
                        format!("~ {key}={} → {}", redact(va), redact(vb)).yellow()
                    );
                } else {
                    println!("{}", format!("- {key}={va}").red());
                    println!("{}", format!("+ {key}={vb}").green());
                }
            }
            (Some(va), None) => {
                differences = true;
                println!("{}", format!("- {key}={va}").red());
            }
            (None, Some(vb)) => {
                differences = true;
                println!("{}", format!("+ {key}={vb}").green());
            }
            (None, None) => unreachable!(),
        }
    }

    if differences {
        std::process::exit(1);
    }

    Ok(())
}
