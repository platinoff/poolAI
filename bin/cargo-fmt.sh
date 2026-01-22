#!/usr/bin/env bash
# cargo fmt --all. Run: bash bin/cargo-fmt.sh
set -e
cd "$(dirname "$0")/.."
cargo fmt --all
