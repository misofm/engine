#!/usr/bin/env bash
# Issue #163 phase 2 step 1 wasm console preflight: everything that can fail without launching the
# workload.
#
# AGENTS.md requires benchmark infrastructure to preflight arguments, schema, output persistence,
# shell exit semantics and overwrite refusal *before* the timed workload runs, so that a runner
# defect cannot consume the one authorised measurement. Nothing here is timed, and nothing here
# instantiates the guest for anything but a shape check.
#
# `--strip2` and `--strip2-baseline` are the wasm half of the strip round's job 2, the banked
# fader and pan matrix. Same class-A statement as the native pair and the same expected shape of
# motion, larger here because the four-lane guest pays proportionally more per dispatched op: the
# round removes 128 of them per block on the 64-track fixture. Every leg's `output_sha256` must
# reproduce the baseline arm's exactly, and `digest_identity` must stay `all_legs_identical`.
#
# `--strip1` and `--strip1-baseline` are the wasm half of the strip round's job 1, the
# prepared-identity builtin-section elision; see the runner's header.
set -euo pipefail
[[ "$#" -le 1 ]] || { printf 'usage: %s [--after|--issue175|--issue182|--issue-loop-eq-r1|--compressor-round1|--compressor-round1-baseline|--round1-composed|--issue183|--round2-lane|--round2-lane-baseline|--round2-eqrack|--round2-eqrack-baseline|--round2-comp|--round2-comp-baseline|--round2-lim|--round2-lim-baseline|--round2-composed|--audit-chain-merge|--audit-chain-merge-baseline|--strip1|--strip1-baseline|--strip2|--strip2-baseline|--strip3|--strip3-baseline|--strip4|--mono2|--mono3|--mono3-baseline]
' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

fail() { printf 'wasm console preflight failure: %s\n' "$1" >&2; exit 1; }

# Mirrors `run-wasm-console-benchmark.sh`'s arms: default is the pre-change browser baseline,
# `--after` is the same rows on the unfused tree (issue #163 phase 2), and `--issue175` is the
# wasm half of the intended-placement family, whose row set the standing fixture changed, and
# `--issue-loop-eq-r1` is the wasm half of the effect-optimization loop's EQ round 1 (identity-
# section elision plus the two-slot cohort chain), which is class A against #175 on every leg.
# `--issue182` is the same strip re-measured after the limiter's effect-optimisation round, which
# is class A and must therefore reproduce every `output_sha256` of the #175 arm exactly.
# `--round2-lim` and `--round2-lim-baseline` are the paired arms of the limiter's round-2
# effect-optimisation pass, on the same terms: two class-A kernel changes, so the two records must
# reproduce each other's `output_sha256` exactly on every row and every leg.
# `--compressor-round1` and `--compressor-round1-baseline` are the paired arms of the compressor's
# effect-optimisation round: the same rows with and without two class-A kernel changes, so they
# must reproduce each other's digests exactly and differ only in time.
# `--issue183` is the paired W4/W8 arm: two guest modules, the same source at two values of the
# `miso_wasm_simd8` build-time cfg, timed inside one observation so the width ratio is a paired
# statistic. Its extra preflight checks are the ones that keep a mislabelled module out of the
# record -- the host must refuse the four-lane module in the eight-lane slot and the reverse.
#
# `--round2-lane` and `--round2-lane-baseline` are the same paired shape for round 2's lane
# lowerings, whose wasm half is `f32x4.pmax`/`f32x4.pmin` for `Lane::max`/`Lane::min`.
# `--round2-eqrack` and `--round2-eqrack-baseline` are the paired arms of the rack/EQ round 2: the
# vectorised planar/AoSoA transpose, the skipped bank-member dedication copy and the batched EQ
# identity refresh. All three are class A, so the two arms must reproduce each other's digests on
# every row and every leg and differ only in time.
# `--round2-comp` and `--round2-comp-baseline` are the same pairing for the compressor's round 2,
# the staged idle body and the pre-gathered detector taps, which is class A on the same terms.
# `--mono3` and `--mono3-baseline` are the wasm halves of the M3 / #210-phase-3 pair; see the
# native runner's header.
arm=baseline
case "${1:-}" in
    --after) arm=after; shift ;;
    --issue175) arm=issue175; shift ;;
    --issue182) arm=issue182; shift ;;
    --issue-loop-eq-r1) arm=issue-loop-eq-r1; shift ;;
    --compressor-round1) arm=compressor-round1; shift ;;
    --compressor-round1-baseline) arm=compressor-round1-baseline; shift ;;
    --round1-composed) arm=round1-composed; shift ;;
    --issue183) arm=issue183; shift ;;
    --round2-lane) arm=round2-lane; shift ;;
    --round2-lane-baseline) arm=round2-lane-baseline; shift ;;
    --round2-eqrack) arm=round2-eqrack; shift ;;
    --round2-eqrack-baseline) arm=round2-eqrack-baseline; shift ;;
    --round2-comp) arm=round2-comp; shift ;;
    --round2-comp-baseline) arm=round2-comp-baseline; shift ;;
    --round2-lim) arm=round2-lim; shift ;;
    --round2-lim-baseline) arm=round2-lim-baseline; shift ;;
    --round2-composed) arm=round2-composed; shift ;;
    --audit-chain-merge) arm=audit-chain-merge; shift ;;
    --audit-chain-merge-baseline) arm=audit-chain-merge-baseline; shift ;;
    --strip1) arm=strip1; shift ;;
    --strip1-baseline) arm=strip1-baseline; shift ;;
    --strip2) arm=strip2; shift ;;
    --strip2-baseline) arm=strip2-baseline; shift ;;
    --strip3) arm=strip3; shift ;;
    --strip3-baseline) arm=strip3-baseline; shift ;;
    --strip4) arm=strip4; shift ;;
    --mono2) arm=mono2; shift ;;
    --mono3) arm=mono3; shift ;;
    --mono3-baseline) arm=mono3-baseline; shift ;;
esac
[[ "$#" == 0 ]] || fail "usage: $0 [--after|--issue175|--issue182|--issue-loop-eq-r1|--compressor-round1|--compressor-round1-baseline|--round1-composed|--issue183|--round2-lane|--round2-lane-baseline|--round2-eqrack|--round2-eqrack-baseline|--round2-comp|--round2-comp-baseline|--round2-lim|--round2-lim-baseline|--round2-composed|--audit-chain-merge|--audit-chain-merge-baseline|--strip1|--strip1-baseline|--strip2|--strip2-baseline|--strip3|--strip3-baseline|--strip4|--mono2|--mono3|--mono3-baseline]"

if [[ "$arm" == after ]]; then
    artifact_dir="$root/artifacts/issue163-phase2"
elif [[ "$arm" == issue175 ]]; then
    artifact_dir="$root/artifacts/issue175"
elif [[ "$arm" == issue182 ]]; then
    artifact_dir="$root/artifacts/issue182"
elif [[ "$arm" == issue-loop-eq-r1 ]]; then
    artifact_dir="$root/artifacts/issue-loop-eq-r1"
elif [[ "$arm" == compressor-round1 ]]; then
    artifact_dir="$root/artifacts/compressor-round1"
elif [[ "$arm" == compressor-round1-baseline ]]; then
    artifact_dir="$root/artifacts/compressor-round1-baseline"
elif [[ "$arm" == round1-composed ]]; then
    artifact_dir="$root/artifacts/round1-composed"
elif [[ "$arm" == issue183 ]]; then
    artifact_dir="$root/artifacts/issue183"
elif [[ "$arm" == round2-lane ]]; then
    artifact_dir="$root/artifacts/round2-lane"
elif [[ "$arm" == round2-lane-baseline ]]; then
    artifact_dir="$root/artifacts/round2-lane-baseline"
elif [[ "$arm" == round2-eqrack ]]; then
    artifact_dir="$root/artifacts/round2-eqrack"
elif [[ "$arm" == round2-eqrack-baseline ]]; then
    artifact_dir="$root/artifacts/round2-eqrack-baseline"
elif [[ "$arm" == round2-comp ]]; then
    artifact_dir="$root/artifacts/round2-comp"
elif [[ "$arm" == round2-comp-baseline ]]; then
    artifact_dir="$root/artifacts/round2-comp-baseline"
elif [[ "$arm" == round2-lim ]]; then
    artifact_dir="$root/artifacts/round2-lim"
elif [[ "$arm" == round2-lim-baseline ]]; then
    artifact_dir="$root/artifacts/round2-lim-baseline"
elif [[ "$arm" == round2-composed ]]; then
    artifact_dir="$root/artifacts/round2-composed"
elif [[ "$arm" == audit-chain-merge ]]; then
    artifact_dir="$root/artifacts/audit-chain-merge"
elif [[ "$arm" == audit-chain-merge-baseline ]]; then
    artifact_dir="$root/artifacts/audit-chain-merge-baseline"
elif [[ "$arm" == strip1 ]]; then
    artifact_dir="$root/artifacts/strip1"
elif [[ "$arm" == strip1-baseline ]]; then
    artifact_dir="$root/artifacts/strip1-baseline"
elif [[ "$arm" == strip2 ]]; then
    artifact_dir="$root/artifacts/strip2"
elif [[ "$arm" == strip2-baseline ]]; then
    artifact_dir="$root/artifacts/strip2-baseline"
elif [[ "$arm" == strip3 ]]; then
    artifact_dir="$root/artifacts/strip3"
elif [[ "$arm" == strip3-baseline ]]; then
    artifact_dir="$root/artifacts/strip3-baseline"
elif [[ "$arm" == strip4 ]]; then
    artifact_dir="$root/artifacts/strip4"
elif [[ "$arm" == mono2 ]]; then
    artifact_dir="$root/artifacts/mono2"
elif [[ "$arm" == mono3 ]]; then
    artifact_dir="$root/artifacts/mono3"
elif [[ "$arm" == mono3-baseline ]]; then
    artifact_dir="$root/artifacts/mono3-baseline"
else
    artifact_dir="$root/artifacts/issue163-phase2-wasm-baseline"
fi
for name in wasm-console-benchmark.raw.jsonl wasm-console-benchmark.accepted.jsonl \
    wasm-console-benchmark.stderr.log wasm-console-benchmark.disposition.json; do
    path="$artifact_dir/$name"
    [[ ! -e "$path" && ! -L "$path" ]] || fail "phase-2 wasm artifact already exists: $path"
done

for tool in awk cmp cp git jq sha256sum wc; do
    command -v "$tool" >/dev/null 2>&1 || fail "required tool is unavailable: $tool"
done

bash scripts/check-console-benchmark-fixture.sh >/dev/null || fail 'fixture check failed'
bash scripts/check-console-fixtures.sh >/dev/null || fail 'console fixture check failed'
bash scripts/test-wasm-console-benchmark.sh >/dev/null || fail 'validator mutation suite failed'
# The admissibility predicates the run is about to be refused by. A precondition whose own
# self-test is red would refuse or admit for the wrong reason, and the run is one-shot.
bash scripts/check-bench-preconditions.sh >/dev/null || fail 'bench precondition self-test failed'
bash scripts/check-bench-policy.sh >/dev/null || fail 'bench policy failed'

# The workspace/all-features form is what CI runs, and it is the form that matters: Cargo unifies
# features across the packages one invocation selects, so a single-package clippy resolves a
# different feature set than the shipped resolution.
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings >/dev/null 2>&1 ||
    fail 'workspace clippy failed'

guest_target=target/ci/issue163-phase2-guest
CARGO_TARGET_DIR="$guest_target" RUSTFLAGS="-C target-feature=+simd128" \
    cargo build --locked --release --quiet --target wasm32-unknown-unknown \
    -p wasm-console-guest || fail 'guest build failed'
guest="$guest_target/wasm32-unknown-unknown/release/wasm_console_guest.wasm"
[[ -f "$guest" ]] || fail 'guest module is missing'

cargo build --locked --release --quiet -p wasm-console || fail 'release build failed'
binary="$root/target/release/wasm_console"
[[ -x "$binary" ]] || fail 'release binary is missing'

# The host refuses to run without its runner's round marker. Proving that here means a direct
# invocation cannot quietly produce an unprovenanced record later.
if MISO_ENGINE_BENCH_ROUND= "$binary" "$guest" >/dev/null 2>&1; then
    fail 'the wasm console host accepted an empty round marker'
fi
if MISO_ENGINE_BENCH_ROUND=1 "$binary" >/dev/null 2>&1; then
    fail 'the wasm console host accepted a missing guest module argument'
fi
if MISO_ENGINE_BENCH_ROUND=1 "$binary" "$guest" extra-argument >/dev/null 2>&1; then
    fail 'the wasm console host accepted a surplus argument'
fi
if MISO_ENGINE_BENCH_ROUND=1 "$binary" /nonexistent.wasm >/dev/null 2>&1; then
    fail 'the wasm console host accepted a guest module that does not exist'
fi
# A guest built without `+simd128` must be refused rather than timed and reported as a simd128
# number. Built into its own target directory so the measured artifact above is not disturbed.
scalar_target=target/ci/issue163-phase2-guest-scalar
CARGO_TARGET_DIR="$scalar_target" cargo build --locked --release --quiet \
    --target wasm32-unknown-unknown -p wasm-console-guest ||
    fail 'scalar guest build failed'
scalar_guest="$scalar_target/wasm32-unknown-unknown/release/wasm_console_guest.wasm"
if MISO_ENGINE_BENCH_ROUND=1 "$binary" "$scalar_guest" >/dev/null 2>&1; then
    fail 'the wasm console host accepted a guest built without simd128'
fi

# The paired arm's second guest, and the two refusals that keep the record honest about which
# module produced which leg. Both modules export the same names, so a swap would otherwise be
# invisible: what separates them is the backend constant each one reports.
guest_simd8_sha256=null
if [[ "$arm" == issue183 ]]; then
    simd8_target=target/ci/issue183-guest-simd8
    CARGO_TARGET_DIR="$simd8_target" \
        RUSTFLAGS="-C target-feature=+simd128 --cfg miso_wasm_simd8" \
        cargo build --locked --release --quiet --target wasm32-unknown-unknown \
        -p wasm-console-guest || fail 'eight-lane guest build failed'
    guest_simd8="$simd8_target/wasm32-unknown-unknown/release/wasm_console_guest.wasm"
    [[ -f "$guest_simd8" ]] || fail 'eight-lane guest module is missing'
    [[ "$(sha256sum "$guest_simd8" | awk '{print $1}')" != \
       "$(sha256sum "$guest" | awk '{print $1}')" ]] ||
        fail 'the eight-lane guest hashed the same as the four-lane one'
    if MISO_ENGINE_BENCH_ROUND=1 "$binary" "$guest" "$guest" >/dev/null 2>&1; then
        fail 'the wasm console host accepted the four-lane guest in the eight-lane slot'
    fi
    if MISO_ENGINE_BENCH_ROUND=1 "$binary" "$guest_simd8" >/dev/null 2>&1; then
        fail 'the wasm console host accepted the eight-lane guest in the four-lane slot'
    fi
    if MISO_ENGINE_BENCH_ROUND=1 "$binary" "$guest" /nonexistent.wasm >/dev/null 2>&1; then
        fail 'the wasm console host accepted an eight-lane guest module that does not exist'
    fi
    guest_simd8_sha256="\"$(sha256sum "$guest_simd8" | awk '{print $1}')\""
fi

candidate_commit=$(git rev-parse --verify HEAD)
jq -n -S \
    --arg commit "$candidate_commit" \
    --arg commit_sha256 "$(printf '%s' "$candidate_commit" | sha256sum | awk '{print $1}')" \
    --arg binary_sha256 "$(sha256sum "$binary" | awk '{print $1}')" \
    --arg guest_sha256 "$(sha256sum "$guest" | awk '{print $1}')" \
    --argjson guest_simd8_sha256 "$guest_simd8_sha256" \
    --arg host_sha256 "$(sha256sum tools/wasm-console/src/main.rs | awk '{print $1}')" \
    --arg guest_source_sha256 "$(sha256sum tools/wasm-console-guest/src/lib.rs | awk '{print $1}')" \
    --arg subject_sha256 "$(sha256sum tools/console-workload/src/lib.rs | awk '{print $1}')" \
    --arg fixture_sha256 "$(sha256sum fixtures/session/v1/console-sixty-four-track.toml | awk '{print $1}')" \
    --arg standing_fixture_sha256 "$(sha256sum fixtures/session/v1/console-sixty-four-track-intended.toml | awk '{print $1}')" \
    --arg fixture_generator_sha256 "$(sha256sum scripts/derive-intended-console-fixture.py | awk '{print $1}')" \
    --arg mono_fixture_sha256 "$(sha256sum fixtures/session/v1/console-sixty-four-track-mono.toml | awk '{print $1}')" \
    --arg mono_fixture_generator_sha256 "$(sha256sum scripts/derive-mono-console-fixture.py | awk '{print $1}')" \
    --arg runner_sha256 "$(sha256sum scripts/run-wasm-console-benchmark.sh | awk '{print $1}')" \
    --arg validator_sha256 "$(sha256sum scripts/wasm-console-benchmark-validator.jq | awk '{print $1}')" \
    --arg preconditions_sha256 "$(sha256sum scripts/check-bench-preconditions.sh | awk '{print $1}')" \
    '{schema_version: 1, issue: 163, phase: "2-step1",
      kind: "wasm_console_benchmark_preflight",
      workload_launches: 0, warmup_rounds: 1, measured_rounds: 2, records_required: 32,
      candidate_commit: $commit, candidate_commit_sha256: $commit_sha256,
      binary_sha256: $binary_sha256, guest_module_sha256: $guest_sha256,
      guest_simd8_module_sha256: $guest_simd8_sha256,
      host_source_sha256: $host_sha256, guest_source_sha256: $guest_source_sha256,
      subject_source_sha256: $subject_sha256, fixture_sha256: $fixture_sha256,
      standing_fixture_sha256: $standing_fixture_sha256,
      fixture_generator_sha256: $fixture_generator_sha256,
      mono_fixture_sha256: $mono_fixture_sha256,
      mono_fixture_generator_sha256: $mono_fixture_generator_sha256,
      runner_sha256: $runner_sha256, validator_sha256: $validator_sha256,
      preconditions_sha256: $preconditions_sha256}'

printf 'wasm console benchmark preflight: PASS (workload launches 0)\n' >&2
