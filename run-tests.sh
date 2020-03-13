#!/usr/bin/env bash
# A script to run all tests and sum the results, because cargo doesn't have that last feature

RUST_BACKTRACE=full cargo test --workspace --all-features -Zfeatures=all | ./sum_cargo_tests.py
