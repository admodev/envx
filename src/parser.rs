use anyhow::{bail, Context, Result};
use indexmap::IndexMap;
use std::path::Path;

pub fn parse(path: &Path) -> Result<IndexMap<String, String>> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;

    let mut map = IndexMap::new();

    for (lineno, line) in content.lines().enumerate() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let eq = line.find('=').with_context(|| {
            format!(
                "{}:{}: malformed line (expected KEY=VALUE): {:?}",
                path.display(),
                lineno + 1,
                line
            )
        })?;

        let key = line[..eq].trim().to_string();
        let raw_val = line[eq + 1..].trim();

        if key.is_empty() {
            bail!(
                "{}:{}: empty key in line: {:?}",
                path.display(),
                lineno + 1,
                line
            );
        }

        let value = strip_quotes(raw_val);
        map.insert(key, value.to_string());
    }

    Ok(map)
}

fn strip_quotes(s: &str) -> &str {
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        &s[1..s.len() - 1]
    } else {
        s
    }
}
