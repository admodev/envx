//! # envx
//!
//! A CLI tool for `.env` file management: semantic diff, schema audit, and
//! passphrase-based encryption.
//!
//! ## Subcommands
//!
//! | Command | Description |
//! |---------|-------------|
//! | [`commands::diff`] | Semantic diff between two `.env` files |
//! | [`commands::audit`] | Validate a `.env` file against a schema |
//! | [`commands::encrypt`] | Encrypt / decrypt `.env` files with [age] |
//!
//! ## Quick start
//!
//! ```bash
//! # Compare two env files
//! envx diff .env.example .env
//!
//! # Audit env file against a schema (one key per line)
//! envx audit --schema schema.env .env
//!
//! # Encrypt
//! envx encrypt .env          # writes .env.age
//! envx decrypt .env.age      # restores .env
//! ```
//!
//! ## Exit codes
//!
//! - `0` — success / no differences
//! - `1` — differences found, missing/empty required keys, or any error
//!
//! [age]: https://age-encryption.org

pub mod cli;
pub mod commands;
pub mod crypto;
pub mod parser;
