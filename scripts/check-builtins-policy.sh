#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

paths=(crates/miso-engine-builtins crates/miso-engine-builtins-compiler)
rg --fixed-strings 'unsafe' "${paths[@]}" && exit 1 || true
rg --fixed-strings 'MAX_TRACKS' "${paths[@]}" && exit 1 || true
rg --fixed-strings 'miso-engine-builtins' Cargo.toml crates/miso-engine-builtins/Cargo.toml crates/miso-engine-builtins-compiler/Cargo.toml >/dev/null
rg --fixed-strings 'miso_engine_builtins' crates/miso-engine-builtins/Cargo.toml crates/miso-engine-builtins-compiler/Cargo.toml >/dev/null

cargo metadata --no-deps --format-version 1 >/dev/null
