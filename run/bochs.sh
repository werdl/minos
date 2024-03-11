#!/bin/sh

set -e

dir=$(dirname "$0")

# Clean up lock files that Bochs creates
rm -f "$dir/../target/x86_64-minos/release/bootimage-minos.bin.lock"
rm -f "$dir/../disk.img.lock"

# Run Bochs (type "continue" if debuger is active)
cd "$dir" && bochs -qf "bochs.rc"
