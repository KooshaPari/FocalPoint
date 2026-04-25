#!/bin/bash
# Build script for motion pipeline: generate Rive + Lottie animations
# Justification: cargo/Rust invocation for platform-agnostic builds; <5 lines shell glue only.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "🎬 Building FocalPoint Motion Pipeline..."

# Generate Rive state machine
cargo run --quiet --bin rive-converter
echo ""

# Generate Lottie animations
cargo run --quiet --bin lottie-converter
echo ""

# Size gate check
echo "📏 Size Gate Verification:"
bash ./size-gate.sh

echo "✅ Motion pipeline build complete"
