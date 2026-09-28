#!/usr/bin/env bash
# Gate G5 (master plan #83 §3.6): the frozen cross-target corpus produces the same digests on this
# host and inside a WebAssembly module, and the `math` M3 and `effect-runtime` D1 pins replay
# under wasm.
#
# Three legs, one corpus (tools/wasm-gate-corpus):
#   native   -- run in this process at Scalar, Simd4 and Simd8.
#   wasm     -- the same crate built for wasm32-unknown-unknown without simd128 (backend scalar).
#   wasm+simd128 -- and with it (backend simd4), which is the only place the v128 software FMA of
#                   master plan §3.5 is actually executed.
#
# After the legs, `check_v8_spill` (issues #1000, #1009) holds the shipped AudioWorklet module's EQ
# cascade loops to V8's register allocation under the pinned Node; it needs that Node on PATH, and
# only it does. `--without-v8-spill` leaves it out. CI passes that flag in `wasm-guests`, because
# `artifact-gates` runs the same gate on the downloaded, pin-verified artifact: the bytes that ship,
# with no second fat-LTO build.
#
# Every leg compares against pins generated from the scalar `Lane` oracle. A mismatch is never
# fixed by re-pinning: it means a target stopped agreeing with the oracle, which is the whole
# reason this gate exists (§10 fallback: compare lane by lane, do not re-pin from the wasm run).
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
readonly TARGET="wasm32-unknown-unknown"
readonly GUEST="wasm_gate_guest.wasm"

cd "$repository_root"

v8_spill=1
if [[ ${1-} == --without-v8-spill ]]; then
    v8_spill=0
    shift
fi
output_dir="${1:-target/ci/wasm-gates}"
mkdir -p "$output_dir"
evidence="$output_dir/wasm-gates.jsonl"
: >"$evidence"

# The host runner and the native leg. `--locked` everywhere: the pinned wasmtime is part of the
# gate, and a resolver that quietly moved it would change which modules validate.
cargo run --locked --release -q -p wasm-gates -- --native | tee -a "$evidence"

run_guest() {
    local name="$1"
    local feature="$2"
    local expected="$3"
    local target_directory="target/ci/wasm-gates-$name"

    CARGO_TARGET_DIR="$target_directory" RUSTFLAGS="-C target-feature=$feature" \
        cargo build --locked --release --target "$TARGET" -p wasm-gate-guest
    cargo run --locked --release -q -p wasm-gates -- \
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
            subject = /true_peak_limiter/ && !/10HotChannel/ && !/7History/
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

# Issue #949 gate 5: the `f64` lane vocabulary must lower to `f64x2` vector opcodes under simd128.
#
# `lane::Widen` is written portably -- `wide` has no f32-to-f64 conversion and `core::arch` is
# confined to two files -- so the packed `f64x2.promote_low_f32x4` is a codegen outcome of the
# release profile's fat LTO, not a spelling. Correctness never depends on it (a scalarised widen is
# the same conversion per lane, with the same bits, and the f64_lane_mismatches count above holds
# both guest legs to that), but the reason the vocabulary exists is to stop doing this arithmetic
# one lane at a time. So the census is taken over the body of each exported probe, and each must
# contain `f64x2.promote_low_f32x4`, `f64x2.mul` and `f64x2.add`, and none of the scalar
# `f64.promote_f32`, `f64.mul` or `f64.add`:
#
#   miso_gate_f64_lane_probe     the energy shape `e = e.add(w.mul(w))` at Simd4 (issue #949);
#   miso_gate_meter_block_probe  the real `lane::kernels::builtins::meter_block::<Simd4>`, the
#                                kernel of the graph's banked full meter pass on the browser's
#                                four-lane banks (issue #950, amendment 4). Its body must also
#                                have no scalar `f32.add`, `f32.gt` or `f32.abs`: the sanitize,
#                                peak and counts stay `f32x4` too.
#
# The graph's own pass is reached through `call_indirect` in the AudioWorklet artifact, so no other
# required gate reads its instructions; the second probe is the pin on the kernel it runs. The
# simd128 leg only: without simd128, `wide`'s `f64x2` is an array and scalar code is the correct
# lowering.
census_f64_probe() {
    local module="$1" probe="$2"
    wasm-objdump -d "$module" | awk -v probe="<$probe>:" '
        /^[0-9a-f]+ func\[[0-9]+\] </ {
            inside = index($0, probe) > 0
            if (inside) seen++
            next
        }
        inside && /f64x2\.promote_low_f32x4/ { promote++ }
        inside && /f64x2\.mul/ { vector_mul++ }
        inside && /f64x2\.add/ { vector_add++ }
        inside && /f64\.promote_f32/ { scalar_promote++ }
        inside && /f64\.mul/ { scalar_mul++ }
        inside && /f64\.add/ { scalar_add++ }
        inside && /[^x]f32\.(add|gt|abs)/ { scalar_f32++ }
        inside && /f32x4\.abs/ { vector_abs++ }
        inside && /f32x4\.add/ { vector_f32_add++ }
        END {
            printf "%d %d %d %d %d %d %d %d %d %d\n", seen, promote, vector_mul, vector_add,
                scalar_promote, scalar_mul, scalar_add, scalar_f32, vector_abs, vector_f32_add
        }
    '
}

check_f64_lane_lowering() {
    local module="$1" probe census seen promote vector_mul vector_add scalar_promote scalar_mul
    local scalar_add scalar_f32 vector_abs vector_f32_add summary
    for probe in miso_gate_f64_lane_probe miso_gate_meter_block_probe; do
        census="$(census_f64_probe "$module" "$probe")"
        read -r seen promote vector_mul vector_add scalar_promote scalar_mul scalar_add scalar_f32 \
            vector_abs vector_f32_add <<<"$census"
        summary="f64x2.promote_low_f32x4=$promote f64x2.mul=$vector_mul f64x2.add=$vector_add"
        summary="$summary f64.promote_f32=$scalar_promote f64.mul=$scalar_mul f64.add=$scalar_add"
        summary="$summary f32.{add,gt,abs}=$scalar_f32 f32x4.abs=$vector_abs f32x4.add=$vector_f32_add"
        [[ "$seen" == 1 ]] || {
            printf 'wasm gates: %s not found exactly once in %s (found %s)\n' \
                "$probe" "$module" "$seen" >&2
            return 1
        }
        if ((promote == 0 || vector_mul == 0 || vector_add == 0 ||
            scalar_promote != 0 || scalar_mul != 0 || scalar_add != 0)); then
            printf 'wasm gates: %s is not vectorised on the simd128 leg: %s\n' \
                "$probe" "$summary" >&2
            printf 'wasm gates: see crates/lane/src/f64_lane.rs, "Lowering"\n' >&2
            return 1
        fi
        if [[ "$probe" == miso_gate_meter_block_probe ]] &&
            ((scalar_f32 != 0 || vector_abs == 0 || vector_f32_add == 0)); then
            printf 'wasm gates: %s left f32 lane arithmetic scalar on the simd128 leg: %s\n' \
                "$probe" "$summary" >&2
            return 1
        fi
        printf 'wasm gates: %s census (simd128): %s\n' "$probe" "$summary"
    done
}

# Issues #1000 and #1009: V8's register allocation of the parametric EQ's stationary cascade loops
# in the shipped AudioWorklet module. `check-web-audioworklet-v8-spill.py` has the rule and what it
# does and does not prove; it times nothing. The module comes from `build-web-audioworklet.sh
# --module-only`, so the cargo line has one home and these are the bytes that ship at this commit.
# That mode does not hold the module to the digest pin: a batch repins once, at its boundary, and
# the loops are a property of the source whether or not the pin has caught up. The pins are checked
# first, so a Node other than the pinned one fails before the build.
check_v8_spill() {
    local module_dir="target/ci/wasm-gates-web" started finished
    python3 -B scripts/check-web-audioworklet-v8-spill.py --check-toolchain
    python3 -B scripts/check-web-audioworklet-v8-spill.py --self-test
    rm -rf -- "$module_dir"
    mkdir -p "$module_dir"
    bash scripts/build-web-audioworklet.sh --module-only "$module_dir"
    started="$EPOCHREALTIME"
    python3 -B scripts/check-web-audioworklet-v8-spill.py \
        "$module_dir/miso-engine-v1-audio-worklet.simd128.wasm"
    finished="$EPOCHREALTIME"
    awk -v a="$started" -v b="$finished" \
        'BEGIN { printf "wasm gates: V8 spill gate ran in %.1f s (build excluded)\n", b - a }'
}

command -v wasm-objdump >/dev/null 2>&1 || {
    printf 'wasm gates: wasm-objdump is required for the detector-residency and f64 lane pins\n' >&2
    exit 1
}

run_guest scalar -simd128 scalar
run_guest simd128 +simd128 simd4

for leg in scalar simd128; do
    check_detector_residency "target/ci/wasm-gates-$leg/$TARGET/release/$GUEST" "$leg"
done
printf 'wasm gates: detector history resident in locals on both guest legs\n'
check_f64_lane_lowering "target/ci/wasm-gates-simd128/$TARGET/release/$GUEST"
legs="native + wasm scalar + wasm simd128"
if ((v8_spill)); then
    check_v8_spill
    legs+=" + V8 EQ loops"
else
    legs+="; V8 EQ loops left out (--without-v8-spill)"
fi

printf 'wasm gates: ok (%s), evidence in %s\n' "$legs" "$evidence"
