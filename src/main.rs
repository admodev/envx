mod cli;
mod commands;
mod crypto;
mod parser;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use owo_colors::OwoColorize;

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Diff { file_a, file_b } => commands::diff::run(&file_a, &file_b),
        Command::Audit { schema, env_file } => commands::audit::run(&schema, &env_file),
        Command::Encrypt { file } => commands::encrypt::run_encrypt(&file),
        Command::Decrypt { file } => commands::encrypt::run_decrypt(&file),
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}
