#!/usr/bin/env bash
# Script for building Rusty Git Gui with a release profile and options tuned for better runtime performance and size.

RUSTFLAGS="-Clink-arg=-fuse-ld=lld -Ctarget-cpu=native" cargo build --release && strip target/release/rusty-git-gui
