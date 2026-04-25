#!/bin/bash
# Size gate: enforce animation file limits
# Lottie: ≤30KB each; Rive: ≤50KB each
# Fails CI if exceeded; non-zero exit on violation.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOTTIE_DIR="$SCRIPT_DIR/lottie"
RIVE_DIR="$SCRIPT_DIR/rive"
LOTTIE_MAX=30720  # 30KB in bytes
RIVE_MAX=51200    # 50KB in bytes

failed=0

# Check Lottie files
if [ -d "$LOTTIE_DIR" ]; then
    echo "Lottie animations (≤30KB per file):"
    for file in "$LOTTIE_DIR"/*.json; do
        if [ -f "$file" ]; then
            size=$(wc -c < "$file")
            filename=$(basename "$file")
            if [ "$size" -gt "$LOTTIE_MAX" ]; then
                echo "  ❌ $filename: $(( size / 1024 ))KB (exceeds 30KB)"
                failed=1
            else
                echo "  ✅ $filename: $(( size / 1024 ))KB"
            fi
        fi
    done
fi

echo ""

# Check Rive files
if [ -d "$RIVE_DIR" ]; then
    echo "Rive state machines (≤50KB per file):"
    for file in "$RIVE_DIR"/*.json; do
        if [ -f "$file" ]; then
            size=$(wc -c < "$file")
            filename=$(basename "$file")
            if [ "$size" -gt "$RIVE_MAX" ]; then
                echo "  ❌ $filename: $(( size / 1024 ))KB (exceeds 50KB)"
                failed=1
            else
                echo "  ✅ $filename: $(( size / 1024 ))KB"
            fi
        fi
    done
fi

if [ "$failed" -eq 1 ]; then
    echo ""
    echo "❌ Size gate FAILED: animations exceed budget"
    exit 1
fi

echo ""
echo "✅ All animations within size budget"
