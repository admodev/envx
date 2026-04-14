//! Implementations of each `envx` subcommand.
//!
//! Each module exposes a `run` (or `run_*`) function that is called from
//! `main` after argument parsing.  All subcommands share the [`crate::parser`]
//! module for `.env` file parsing.

pub mod audit;
pub mod diff;
pub mod encrypt;
