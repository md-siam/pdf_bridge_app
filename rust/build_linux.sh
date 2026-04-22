#!/usr/bin/env bash
set -euo pipefail

CRATE_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$CRATE_DIR/.." && pwd)"
RUNNER_LIB_DIR="$PROJECT_DIR/linux/runner/lib"
STATIC_LIB_DIR="$PROJECT_DIR/linux/runner/rust_static"

mkdir -p "$RUNNER_LIB_DIR"
mkdir -p "$STATIC_LIB_DIR"

echo "Building Rust library for Linux..."
cd "$CRATE_DIR"
cargo build --release

echo "Copying shared libraries (.so) to $RUNNER_LIB_DIR ..."
find "$CRATE_DIR/target/release" -maxdepth 1 -name "*.so" -exec cp -f {} "$RUNNER_LIB_DIR" \;

echo "Copying static libraries (.a) to $STATIC_LIB_DIR ..."
find "$CRATE_DIR/target/release" -maxdepth 1 -name "*.a" -exec cp -f {} "$STATIC_LIB_DIR" \;

echo "Done."
echo
echo "Shared libraries copied to:"
find "$RUNNER_LIB_DIR" -maxdepth 1 -name "*.so" || true
echo
echo "Static libraries copied to:"
find "$STATIC_LIB_DIR" -maxdepth 1 -name "*.a" || true