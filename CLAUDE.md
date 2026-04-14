# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build                        # debug build
cargo build --release              # release build (binary: target/release/envx)
cargo test                         # run all tests
cargo test <test_name>             # run single test
cargo clippy -- -D warnings        # lint
cargo fmt                          # format
```

Run the tool:
```bash
cargo run -- diff .env.example .env
cargo run -- audit --schema schema.env .env
cargo run -- encrypt .env
cargo run -- decrypt .env.age
```

## Architecture

Single-binary CLI (`envx`) dispatched via `main.rs` → `cli::Command` enum → `commands::{diff,audit,encrypt}`.

**Module layout:**
- `cli.rs` — clap `Cli`/`Command` structs (all subcommand arg parsing lives here)
- `parser.rs` — shared `.env` parser; returns `IndexMap<String, String>` (insertion order preserved); strips `"` / `'` quotes, skips comments and blank lines
- `crypto.rs` — thin wrappers around `age` passphrase encrypt/decrypt
- `commands/diff.rs` — semantic diff; redacts values whose keys match `SENSITIVE_PATTERNS`; exits 1 when differences found
- `commands/audit.rs` — validates env against a schema file (plain text, one key per line, `#` comments); exits 1 on missing/empty required keys
- `commands/encrypt.rs` — prompts passphrase twice, writes `<file>.age`; decrypt strips `.age` suffix

**Key design decisions:**
- `diff` exits 1 (not 0) when files differ — intentional for CI/scripting use
- `audit` schema is just a flat list of key names, not a typed schema
- Encryption is passphrase-only (`age` passphrase mode); no public-key recipients
- `parser` is shared across all commands; new commands should use it rather than rolling their own
