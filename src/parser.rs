//! Shared `.env` file parser.
//!
//! Parses a `.env` file into an [`IndexMap`] that preserves insertion order.
//! Both single-quoted and double-quoted values are unquoted automatically.
//! Blank lines and lines beginning with `#` are skipped.
//!
//! [`IndexMap`]: indexmap::IndexMap

use anyhow::{bail, Context, Result};
use indexmap::IndexMap;
use std::path::Path;

/// Parse a `.env` file at `path` into an ordered key→value map.
///
/// # Errors
///
/// Returns an error if:
/// - the file cannot be read,
/// - a non-blank, non-comment line has no `=` separator, or
/// - a key is empty.
///
/// # Example
///
/// Given the file:
/// ```text
/// # comment
/// DATABASE_URL=postgres://localhost/mydb
/// SECRET_KEY="s3cr3t"
/// ```
///
/// ```no_run
/// use std::path::Path;
/// use envx_secure::parser;
///
/// let map = parser::parse(Path::new(".env")).unwrap();
/// assert_eq!(map["DATABASE_URL"], "postgres://localhost/mydb");
/// assert_eq!(map["SECRET_KEY"], "s3cr3t");
/// ```
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

/// Strip a single layer of matching `"…"` or `'…'` quotes from `s`.
fn strip_quotes(s: &str) -> &str {
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        &s[1..s.len() - 1]
    } else {
        s
    }
}
