use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "envx", about = "A CLI tool for .env file management")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Show semantic diff between two env files
    Diff { file_a: PathBuf, file_b: PathBuf },
    /// Audit an env file against a schema
    Audit {
        #[arg(long)]
        schema: PathBuf,
        env_file: PathBuf,
    },
    /// Encrypt an env file with a passphrase
    Encrypt { file: PathBuf },
    /// Decrypt an age-encrypted env file
    Decrypt { file: PathBuf },
}
