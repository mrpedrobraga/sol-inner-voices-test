#![allow(unused)]
use clap::{Parser, Subcommand};
use miette::Diagnostic;
use std::path::PathBuf;
use thiserror::Error;

// Usage: `sol <arguments>`
fn main() -> Result<(), CliError> {
    let args = CliArgs::parse();

    match args.command {
        PrimaryAction::Init { path } => {
            let dir = path.unwrap_or(std::env::current_dir()?);
            workspace::init(dir)?;
        }
        PrimaryAction::Build { path, watch } => {
            let dir = path.unwrap_or(std::env::current_dir()?);
            workspace::build(dir)?;
        }
    }

    Ok(())
}

#[derive(Debug, clap::Parser)]
#[command(
    name = "sol",
    about = "Sol parser, compiler and package manager.",
    version
)]
struct CliArgs {
    #[command(subcommand)]
    command: PrimaryAction,
}

#[derive(Debug, Subcommand)]
enum PrimaryAction {
    Init {
        // If unspecified, use CWD.
        path: Option<PathBuf>,
    },
    Build {
        // Override of the workspace path to be built.
        // Defaults to the CWD.
        path: Option<PathBuf>,

        #[arg(short, long)]
        watch: bool,
    },
}
