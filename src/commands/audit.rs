use anyhow::{Context, Result};
use owo_colors::OwoColorize;
use std::path::Path;

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
