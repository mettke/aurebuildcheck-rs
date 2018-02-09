#!/bin/sh
cargo build --release
# Stripping removes a lot of debug symbols and therefore
# decreases the binary size. Unfortunately it also removes
# the ability to use panic
strip target/release/aurebuildcheck-rs
