#!/usr/bin/env cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
shish-lib = "0.1"
---
use shish_lib::prelude::*;

fn main() -> Result<()> {
    Ok(())
}
