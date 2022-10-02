#!/bin/bash
set -e

cargo test
cargo test --examples
$(dirname "$0")/clippy-pedantic.sh
cargo doc --no-deps
cargo bench --no-run --profile dev

