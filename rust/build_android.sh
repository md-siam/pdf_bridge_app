#!/usr/bin/env bash
set -euo pipefail

RUST_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$RUST_DIR/.." && pwd)"
JNI_LIBS_DIR="$PROJECT_DIR/android/app/src/main/jniLibs"

mkdir -p "$JNI_LIBS_DIR"

cargo ndk \
  --target arm64-v8a \
  --target armeabi-v7a \
  --target x86_64 \
  --target x86 \
  -o "$JNI_LIBS_DIR" \
  build --release

echo "Rust Android libraries copied to: $JNI_LIBS_DIR"