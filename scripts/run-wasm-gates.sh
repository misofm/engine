#!/usr/bin/env bash
# Gate G5 (master plan #83 §3.6): the frozen cross-target corpus produces the same digests on this
# host and inside a WebAssembly module, and the `miso-engine-math` M3 and `miso-engine-effect-
# runtime` D1 pins replay under wasm.
#
# Three legs, one corpus (tools/miso-engine-wasm-gate-corpus):
#   native   -- run in this process at Scalar, Simd4 and Simd8.
#   wasm     -- the same crate built for wasm32-unknown-unknown without simd128 (backend scalar).
#   wasm+simd128 -- and with it (backend simd4), which is the only place the v128 software FMA of
#                   master plan §3.5 is actually executed.
#
# Every leg compares against pins generated from the scalar `Lane` oracle. A mismatch is never
# fixed by re-pinning: it means a target stopped agreeing with the oracle, which is the whole
# reason this gate exists (§10 fallback: compare lane by lane, do not re-pin from the wasm run).
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
readonly TARGET="wasm32-unknown-unknown"
readonly GUEST="miso_engine_wasm_gate_guest.wasm"

cd "$repository_root"

output_dir="${1:-target/ci/wasm-gates}"
mkdir -p "$output_dir"
evidence="$output_dir/wasm-gates.jsonl"
: >"$evidence"

# The host runner and the native leg. `--locked` everywhere: the pinned wasmtime is part of the
# gate, and a resolver that quietly moved it would change which modules validate.
cargo run --locked --release -q -p miso-engine-wasm-gates -- --native | tee -a "$evidence"

run_guest() {
    local name="$1"
    local feature="$2"
    local expected="$3"
    local target_directory="target/ci/wasm-gates-$name"

    CARGO_TARGET_DIR="$target_directory" RUSTFLAGS="-C target-feature=$feature" \
        cargo build --locked --release --target "$TARGET" -p miso-engine-wasm-gate-guest
    cargo run --locked --release -q -p miso-engine-wasm-gates -- \
        "$target_directory/$TARGET/release/$GUEST" --expect-backend "$expected" | tee -a "$evidence"
}

run_guest scalar -simd128 scalar
run_guest simd128 +simd128 simd4

printf 'wasm gates: ok (native + wasm scalar + wasm simd128), evidence in %s\n' "$evidence"
