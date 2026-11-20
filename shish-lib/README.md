# `sh`-ish 🤷‍♂️

This is an umbrella library that intends to make Cargo scripts slightly more ergonomic.

## Dependencies

This library depends on several crates:

- `anyhow` - for error handling.
- `clap` - for argument parsing.
- `duct` - for command invocation.

## Usage

The simplest way to use `sh`-ish is to import the prelude:

```rust
use shish_lib::prelude::*;
```

The prelude re-exports the most commonly used items from `sh`-ish's dependencies.

You can also import those dependencies (or items within) directly through their re-exports:

```rust
use shish_lib::anyhow;
use shish_lib::clap;
use shish_lib::duct;
```

## What are Cargo scripts?

See [RFC#3502](https://rust-lang.github.io/rfcs/3502-cargo-script.html)
