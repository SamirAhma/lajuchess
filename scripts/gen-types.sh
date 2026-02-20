#!/usr/bin/env bash
# Regenerate TypeScript types from Rust structs/enums
# Run this whenever you change types in src/game.rs or src/main.rs
set -e
typeshare ./src --lang=typescript --output-file=frontend/src/lib/types.ts
echo "✅ Types regenerated → frontend/src/lib/types.ts"
