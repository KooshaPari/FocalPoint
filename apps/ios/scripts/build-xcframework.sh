#!/usr/bin/env bash
# Rust-vs-shell: wraps 3× cargo build + lipo + xcodebuild -create-xcframework into
# one reproducible step. A Rust reimplementation would be a 300-line subprocess
# orchestrator for negligible gain. Per Phenotype scripting policy.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
export PATH="$HOME/.cargo/bin:$PATH"
# Cargo.lock pulls clap_lex 1.1.0 (edition2024); workspace pin 1.82 cannot parse it.
CARGO="${CARGO:-cargo +1.93.1}"
cd "$ROOT"
for T in aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios; do
    $CARGO build --release -p focus-ffi --target "$T"
done
SIM_UNIV="$ROOT/target/ios-sim-universal/release"
mkdir -p "$SIM_UNIV"
lipo -create \
    "$ROOT/target/aarch64-apple-ios-sim/release/libfocus_ffi.a" \
    "$ROOT/target/x86_64-apple-ios/release/libfocus_ffi.a" \
    -output "$SIM_UNIV/libfocus_ffi.a"
# Regenerate header in include dir (source of truth for xcframework).
bash "$ROOT/crates/focus-ffi/scripts/ios-bindings.sh"
cp "$ROOT/apps/ios/FocalPoint/Sources/FocalPointCore/focus_ffiFFI.h" "$ROOT/crates/focus-ffi/include/"
OUT="$ROOT/apps/ios/FocalPoint/Frameworks/FocusFFI.xcframework"
rm -rf "$OUT"
mkdir -p "$(dirname "$OUT")"
xcodebuild -create-xcframework \
    -library "$ROOT/target/aarch64-apple-ios/release/libfocus_ffi.a" \
        -headers "$ROOT/crates/focus-ffi/include" \
    -library "$SIM_UNIV/libfocus_ffi.a" \
        -headers "$ROOT/crates/focus-ffi/include" \
    -output "$OUT"
echo "Built: $OUT"
