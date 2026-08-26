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

# Round 2 R2: the limiter's twelve-word detector history must live in wasm *locals*.
#
# LLVM idiom-recognises the shift of a twelve-word array as a block move, and the guest then
# re-reads through linear memory every tap it has just copied -- a store-to-load round trip per
# tap per frame, on a kernel that is latency-bound. `History<L>` is twelve named fields precisely
# so that there is no such idiom left to recognise, and this pin is what keeps it that way.
#
# What it refuses is a `memory.copy` whose size is a whole history (`HISTORY_WORDS * sizeof(L)`)
# or the eleven words a shift actually moves, inside a function the limiter owns. Those two sizes
# are the signature of the regression and of nothing else: the other copies a limiter block makes
# -- the `BankProcessReport` at `process_bank`'s exit, a state payload buffer -- are other sizes.
# The sizes are derived, not guessed: one lane is 4 bytes at `Lane = f32`, 16 at `Simd4` and 32 at
# the wasm `Simd8` (two v128 halves), and `HISTORY_WORDS` is 12.
#
# `HotChannel::load` and `History::load`/`store` are excluded by name, and only they. Those are
# the once-per-block gather and scatter of the whole hot state; moving twelve words as a unit
# there is the intended shape, and whether the backend emits it as a block move is a decision
# about one copy per block rather than one per frame. Deleting the exclusion is how you check the
# pin is still wired to something: with it gone, the scalar and Simd8 legs go red on those.
readonly HISTORY_SHIFT_SIZES="44 48 176 192 352 384"

check_detector_residency() {
    local module="$1" name="$2" found
    found="$(wasm-objdump -d "$module" | awk -v sizes="$HISTORY_SHIFT_SIZES" '
        BEGIN { split(sizes, list, " "); for (i in list) forbidden[list[i]] = 1 }
        /^[0-9a-f]+ func\[[0-9]+\] </ {
            subject = /miso_engine_true_peak_limiter/ && !/10HotChannel/ && !/7History/
            fn = $0
            size = ""
        }
        subject && /i32\.const/ { size = $NF }
        subject && /memory\.copy/ && (size in forbidden) { print size, fn }
    ')"
    [[ -z "$found" ]] || {
        printf 'wasm gates: the detector history is shifted through linear memory (%s leg)\n%s\n' \
            "$name" "$found" >&2
        return 1
    }
}

command -v wasm-objdump >/dev/null 2>&1 || {
    printf 'wasm gates: wasm-objdump is required for the detector-residency pin\n' >&2
    exit 1
}

run_guest scalar -simd128 scalar
run_guest simd128 +simd128 simd4

for leg in scalar simd128; do
    check_detector_residency "target/ci/wasm-gates-$leg/$TARGET/release/$GUEST" "$leg"
done
printf 'wasm gates: detector history resident in locals on both guest legs\n'

printf 'wasm gates: ok (native + wasm scalar + wasm simd128), evidence in %s\n' "$evidence"
