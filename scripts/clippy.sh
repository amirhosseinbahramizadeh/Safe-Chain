#!/usr/bin/env bash
set -e
cargo clippy --workspace --all-targets --all-features -- -D warnings
