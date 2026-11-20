use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

/// Command line tool to manage Cargo scripts (RFC 3502) that use shish-lib
#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    subcommand: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new script with shish-lib boilerplate
    New { path: PathBuf },
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.subcommand {
        Command::New { path } => new(path)?,
    };
    Ok(())
}

fn new(path: PathBuf) -> Result<()> {
    let mut script = create_script(&path)?;
    write_boilerplate(&mut script)?;
    Ok(())
}

fn create_script(path: impl AsRef<Path>) -> Result<File> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o777)
        .open(path)
        .context("failed to create script file")
}

fn write_boilerplate(file: &mut File) -> Result<()> {
    const BOILERPLATE: &str = include_str!("../boilerplate.rs");

    file.write_all(BOILERPLATE.as_bytes())
        .context("failed to write boilerplate to script file")?;
    Ok(())
}
