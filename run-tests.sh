#!/usr/bin/env bash
# A script to run all tests and sum the results, because cargo doesn't have that last feature

RUST_BACKTRACE=full cargo test --all-features -Zfeatures=all -- --color=always | ./sum_cargo_tests.py
