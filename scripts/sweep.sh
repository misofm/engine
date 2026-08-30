#!/usr/bin/env bash
# scripts/sweep.sh -- the hermetic check/test sweep. 101 explicit rows.
#
# Why this exists: the repo has 106 check-*/test-* scripts and ci.yml names 37 of them, so most
# gates had no committed runner at all -- they were reachable only by knowing they existed. Every
# row below is written out by name. There are deliberately no globs: a glob silently absorbs a new
# script (and silently drops a renamed one), which is how the coverage gap got here in the first
# place. Adding a script means adding its row.
#
# Scope: hermetic rows only. Excluded are the one-shot measurement runners (run-*-benchmark.sh and
# friends), which consume a single authorised measurement and write sealed artifacts under
# artifacts/issue*/ -- those are never swept. No row here touches artifacts/, needs a network
# fetch, a browser, or an audio device, and no row dirties the working tree (every mutation suite
# copies into its own mktemp -d first), so this script contains no restore or cleanup logic.
#
# Four check-*/test-* scripts are excluded; each is named at the bottom of this file with its
# reason. They are helpers, not entry points, and every one is already driven by an included row.
#
# Rows run cheapest-first so a policy break fails in seconds rather than after the build-bound
# tail. The sweep does not stop at the first red: it runs every row, prints a per-row summary, and
# exits nonzero if any row failed.
#
# Requires: bash, jq, rg, python3, node, cargo/rustc. On x86_64 the toolchain is pinned to
# +avx2,+fma (.cargo/config.toml) and miso-engine-lane will not build without it.
set -uo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }

log_dir=$(mktemp -d)
trap 'rm -rf -- "$log_dir"' EXIT

declare -a results=()
rows=0
failures=0
started=$(date +%s)

# Python rows run under `-I -B`, matching every python3 invocation elsewhere in scripts/. `-B`
# matters: a stray scripts/__pycache__ is exactly what check-effect-descriptor-v1.sh and
# check-effect-package-v1.sh fail on, so a sweep that wrote bytecode would break two later rows.
row() {
    local script=$1; shift
    local label=${script##*/} cmd=() rc start end
    case "$script" in
        *.py)  cmd=(python3 -I -B "$root/$script" "$@") ;;
        *.mjs) cmd=(node "$root/$script" "$@") ;;
        *)     cmd=(bash "$root/$script" "$@") ;;
    esac
    [[ -f "$root/$script" ]] || {
        printf 'sweep: row names a missing script: %s\n' "$script" >&2
        results+=("MISSING  ---  $label")
        rows=$((rows + 1)); failures=$((failures + 1))
        return
    }
    rows=$((rows + 1))
    start=$(date +%s)
    if "${cmd[@]}" >"$log_dir/$label.log" 2>&1; then rc=0; else rc=$?; fi
    end=$(date +%s)
    if [[ "$rc" == 0 ]]; then
        results+=("$(printf 'PASS  %4ss  %s' "$((end - start))" "$label")")
        printf '.'
    else
        results+=("$(printf 'FAIL  %4ss  %s (exit %s)' "$((end - start))" "$label" "$rc")")
        failures=$((failures + 1))
        printf 'F'
        {
            printf '\n----- %s failed (exit %s) -----\n' "$label" "$rc"
            tail -n 20 "$log_dir/$label.log"
            printf -- '----- end %s -----\n' "$label"
        } >&2
    fi
}

# ---- tier 1: instant policy and grep gates -------------------------------------------------
row scripts/check-artifact-evidence-leak.sh
row scripts/check-bench-policy.sh
row scripts/check-bench-preconditions.sh
row scripts/check-builtins-policy.sh
row scripts/check-command-kind-vocabulary.py
row scripts/check-command-reason-vocabulary.py
row scripts/check-conformance-boundaries.sh
row scripts/check-console-benchmark-fixture.sh
row scripts/check-intended-console-fixture.sh
row scripts/check-mono-console-fixture.sh
row scripts/check-effect-interchange-benchmark-108.sh
row scripts/check-effect-interchange-qualification.sh
row scripts/check-effect-runtime-fixtures.sh
row scripts/check-effect-runtime-policy.sh
row scripts/check-effect-state-migration-v1.sh
row scripts/check-env-vocabulary.sh
row scripts/check-fast-db-seal.sh
row scripts/check-unfused-seal.sh
row scripts/check-host-core-policy.sh
row scripts/check-lane-policy.sh
row scripts/check-math-policy.sh
row scripts/check-native-pcm-runner-portability-v1.sh
row scripts/check-native-pcm-runner-v1.sh
row scripts/check-protocol-control-policy.sh
row scripts/check-rack-benchmark-fixture.sh
row scripts/check-rack-policy.sh
row scripts/check-realtime-policy.sh
row scripts/check-session-policy.sh
row scripts/check-stem-store-v1.mjs --self-test
row scripts/check-workspace-policy.sh
row scripts/check-capi-qualification-v1.sh
row scripts/check-dsp-research.sh
row scripts/check-graph-determinism.sh
row scripts/check-graph-policy.sh
# #241/Fable F2: the canonical PCM vectors and manifest are generated contract bytes, so their
# generator check is an accounted sweep row rather than an implicit prerequisite of a Rust test.
row fixtures/stem-identity/v1/generate.py --check
# Argument-taking validators whose self-test is the standalone entry point. Their real runs are
# driven by the parent gates below; the self-test row keeps each one individually accounted for.
row scripts/check-builtins-listening-033.py --self-test
row scripts/check-builtins-listening-111.py --self-test
row scripts/check-parameter-metadata-v1.py --self-test
row scripts/check-abi-layout-v1.py --self-test
row scripts/check-sdk-generated.sh
row scripts/check-sdk-deletions.py --self-test
row scripts/check-session-map-shape.py --self-test
row scripts/check-step-vocabulary.py --self-test
row scripts/check-web-audioworklet-callgraph.py --self-test
row scripts/test-artifact-evidence-leak.sh
row scripts/test-bench-policy.sh
row scripts/test-builtins-fixtures.sh
row scripts/test-builtins-policy.sh
row scripts/test-effect-descriptor-capi.sh
row scripts/test-effect-interchange-benchmark-108-policy.sh
row scripts/test-effect-interchange-policy.sh
row scripts/test-effect-interchange-target-export-parser.sh
row scripts/test-effect-runtime-fixtures.sh
row scripts/test-env-vocabulary.sh
row scripts/test-graph-benchmark.sh
row scripts/test-host-core-policy.sh
row scripts/test-lane-policy.sh
row scripts/test-math-policy.sh
row scripts/test-native-pcm-runner-portability-v1-policy.sh
row scripts/test-native-pcm-runner-v1-policy.sh
row scripts/test-protocol-benchmark.sh
row scripts/test-protocol-control-policy.sh
row scripts/test-rack-policy.sh
row scripts/test-realtime-audit-hooks.sh
row scripts/test-realtime-policy.sh
row scripts/test-realtime-trace-validator.sh
row scripts/test-workspace-policy.sh

# ---- tier 2: seconds; mutation suites and audit probes -------------------------------------
row scripts/test-builtins-graph-audit-probes.sh
row scripts/test-fast-db-seal.sh
row scripts/test-unfused-seal.sh
row scripts/test-console-benchmark.sh
row scripts/test-capi-abi.sh
row scripts/test-effect-runtime-policy.sh
row scripts/test-rack-benchmark.sh
row scripts/test-builtins-audit-probes.sh
row scripts/check-realtime-audit-leak.sh
row scripts/test-capi-qualification-v1-policy.sh
row scripts/test-effect-interchange-reference-runner.sh
row scripts/test-builtins-benchmark.sh
row scripts/check-capi-abi.sh
row scripts/check-builtins-listening.sh
row scripts/test-realtime-audit-leak.sh
row scripts/test-web-audioworklet.sh
row scripts/test-wasm-kernel-timing.sh
row scripts/test-wasm-console-benchmark.sh
row scripts/test-effect-interchange-benchmark.sh

# ---- tier 3: build-bound, ten seconds and up -----------------------------------------------
row scripts/check-wasm-realtime-atomics.sh
row scripts/check-effect-descriptor-v1.sh
row scripts/check-parametric-eq-targets.sh
row scripts/check-builtins-fixtures.sh
row scripts/check-protocol-benchmark-wasm-parity.sh
row scripts/check-effect-package-v1.sh
row scripts/check-protocol-wasm-parity.sh --self-test
row scripts/check-browser-expected-resources.py
row scripts/check-builtins-targets.sh

# ---- tier 4: heavy cross-target builds -----------------------------------------------------
row scripts/check-effect-interchange-targets.sh
row scripts/test-native-vectorization-report.sh
row scripts/check-effect-contract.sh
row scripts/check-flac-decoder.sh
row scripts/check-web-audioworklet.sh
row scripts/check-sdk-headless.sh

# ---- deliberately not swept ----------------------------------------------------------------
# run-wasm-kernel-timing.sh, run-console-benchmark.sh, run-wasm-console-benchmark.sh and the other
#   run-*.sh entry points -- one-shot measurement runners. Each consumes a single authorised
#   measurement and refuses to overwrite the artifacts it writes, so a sweep row would either burn
#   the measurement or fail on the second sweep. Their hermetic halves are swept:
#   check-bench-preconditions.sh, test-console-benchmark.sh, test-wasm-kernel-timing.sh and
#   test-wasm-console-benchmark.sh are all rows above.
# preflight-console-benchmark.sh, preflight-wasm-console-benchmark.sh and the other preflight-*.sh
#   entry points -- the halves of those runners that can fail without launching a workload. They
#   build release binaries and a wasm guest and run a workspace clippy, so they are minutes rather
#   than seconds, and every gate they call is already an independent row above. They are run
#   immediately before the one-shot they protect, which is the only moment their answer means
#   anything.
# check-capi-object-symbols-v1.py -- pure helper, no self-test: exits 2 unless handed three argv
#   paths. Exercised for real by check-capi-qualification-v1.sh, which is a row above.
# check-capi-qualification-evidence-v1.py -- pure helper, no self-test: exits 2 unless handed a
#   root plus committed|preserved. Exercised by check-capi-qualification-v1.sh, a row above.
# test-web-audioworklet.mjs -- a real standalone runner, but test-web-audioworklet.sh (a row
#   above) already executes it; a row here would double-run the same assertions.
# check-sdk-types.sh -- needs `sdk/node_modules` (TypeScript), and installing it needs the network.
#   Every row here is hermetic by construction, which is the one property the sweep exists to have,
#   so the SDK's static half sits beside the qualification suite as a runnable-but-unswept gate.
#   The SDK's BEHAVIOURAL evals need no node_modules at all -- they run under Node's native type
#   stripping -- and are swept as check-sdk-headless.sh.

finished=$(date +%s)
printf '\n\n'
for line in "${results[@]}"; do printf '  %s\n' "$line"; done
printf '\nsweep: %s rows, %s passed, %s failed, %ss\n' \
    "$rows" "$((rows - failures))" "$failures" "$((finished - started))"
[[ "$failures" == 0 ]] || exit 1
