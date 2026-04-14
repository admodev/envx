//! Command-line interface definitions.
//!
//! All argument parsing is handled by [`clap`] via the derive API.
//! Add new subcommands here by extending [`Command`] and wiring them up in
//! `main.rs`.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Top-level CLI entry point.
#[derive(Parser)]
#[command(name = "envx", about = "A CLI tool for .env file management")]
pub struct Cli {
    /// The subcommand to run.
    #[command(subcommand)]
    pub command: Command,
}

/// Available subcommands.
#[derive(Subcommand)]
pub enum Command {
    /// Show a semantic diff between two `.env` files.
    ///
    /// Values for keys that match common sensitive patterns (`SECRET`, `KEY`,
    /// `TOKEN`, `PASSWORD`, `PASS`, `PWD`) are redacted in the output.
    /// Exits with code `1` when any difference is found.
    Diff {
        /// Reference env file (shown as `---`).
        file_a: PathBuf,
        /// Target env file (shown as `+++`).
        file_b: PathBuf,
    },

    /// Validate a `.env` file against a schema.
    ///
    /// The schema is a plain-text file with one key name per line; lines
    /// starting with `#` and blank lines are ignored.  Missing or empty
    /// required keys cause an exit code of `1`.
    Audit {
        /// Path to the schema file.
        #[arg(long)]
        schema: PathBuf,
        /// Path to the `.env` file to audit.
        env_file: PathBuf,
    },

    /// Encrypt a `.env` file with a passphrase using [age].
    ///
    /// Prompts for a passphrase twice (confirmation).  Writes the ciphertext
    /// to `<file>.age` next to the original.
    ///
    /// [age]: https://age-encryption.org
    Encrypt {
        /// Path to the plaintext `.env` file to encrypt.
        file: PathBuf,
    },

    /// Decrypt an age-encrypted `.env` file.
    ///
    /// The input file must have an `.age` extension.  The plaintext is written
    /// to the path obtained by stripping the `.age` suffix.
    Decrypt {
        /// Path to the `.age` encrypted file.
        file: PathBuf,
    },
}
