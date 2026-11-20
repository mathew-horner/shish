//! Shell-ish's prelude.

// Common anyhow error-handling utilities.
//
// Coincidentally, this is all items that anyhow exports except Chain, which
// I don't imagine is used enough in scripts to warrant inclusion in the
// prelude.
pub use anyhow::{Context, Error, Ok, Result, anyhow, bail, ensure};

// Command-line argument parsing through clap.
//
// We push the derive API for simplicity's sake. Scripts should be fairly
// trivial to define argument parsing behavior for.
pub use clap::{Parser, Subcommand, ValueEnum};

// duct::cmd is what provides shell-like command execution. It's way easier to
// use than std::process::Command.
pub use duct::cmd;
