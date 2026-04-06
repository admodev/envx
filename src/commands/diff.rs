use crate::parser;
use anyhow::Result;
use owo_colors::OwoColorize;
use std::path::Path;

const SENSITIVE_PATTERNS: &[&str] = &["SECRET", "KEY", "TOKEN", "PASSWORD", "PASS", "PWD"];

fn is_sensitive(key: &str) -> bool {
    let upper = key.to_uppercase();
    SENSITIVE_PATTERNS.iter().any(|p| upper.contains(p))
}

fn redact(value: &str) -> String {
    "•".repeat(value.len().max(7))
}

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
