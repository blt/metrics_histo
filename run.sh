#!/usr/bin/env bash

set -o errexit
set -o pipefail
set -o nounset
# set -o xtrace

RUSTFLAGS="-g" cargo build --release

valgrind --tool=massif ./target/release/metrics_histo
