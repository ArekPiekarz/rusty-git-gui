#!/usr/bin/env bash
# Script for building Rusty Git Gui with a profile tuned for maximal runtime performance.
# It can increase the compilation time by 50% in comparison to a normal release profile.

RUSTFLAGS="-Clink-arg=-fuse-ld=lld -Ctarget-cpu=native" cargo +nightly build --profile max-performance --features use_mimalloc -Z unstable-options && strip target/max-performance/rusty-git-gui
