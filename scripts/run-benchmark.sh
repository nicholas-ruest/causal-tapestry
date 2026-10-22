#!/usr/bin/env bash
set -euo pipefail
export CARGO_INCREMENTAL=0
cargo run --locked --release -p ct-orchestrator --example benchmark

