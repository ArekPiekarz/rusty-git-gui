#!/usr/bin/env bash
# Script for building Rusty Git Gui with a release profile.

cargo +nightly build --release && strip target/release/rusty-git-gui
