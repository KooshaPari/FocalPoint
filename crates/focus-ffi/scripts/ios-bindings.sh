#!/usr/bin/env bash
# Rust-vs-shell: this is a 3-command wrapper (cargo build + cargo run + cp-by-flag);
# writing a dedicated Rust binary to orchestrate other cargo invocations would
# be strictly worse than a 5-line bash glue. Per Phenotype scripting policy.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cargo build --manifest-path "$HERE/../Cargo.toml" --release -p focus-ffi
cargo run --manifest-path "$HERE/../Cargo.toml" --release -p focus-ffi --bin uniffi-bindgen -- \
    generate "$HERE/../src/focus_ffi.udl" --language swift \
    --out-dir "$HERE/../../../apps/ios/FocalPoint/Sources/FocalPointCore/"
