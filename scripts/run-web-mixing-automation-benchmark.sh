#!/usr/bin/env bash
# Issue #1003: the browser arm of `console_mixing_automation`, on the shipped `host_web.wasm`.
#
# The console benchmark's native row renders the mono console in-process. This arm renders the
# same session, with the same eight controls, through the product's browser path: the host_web
# module built with the delivery recipe, booted and driven under Node's V8 through
# `prepared-control.js` and `miso_engine_web_v1_command_submit` exactly as the SDK drives it, with
# `miso_engine_web_v1_render` alone inside the clock (`scripts/web-mixing-automation-benchmark.mjs`
# says what else it does and why).
#
# Three subcommands, so that nothing is built while anything is timed:
#
#   prepare WORKDIR        Untimed. WORKDIR must be an empty directory. Builds host_web.wasm with
#                          the flags of `scripts/build-web-audioworklet.sh` into WORKDIR, and
#                          writes the native row's resolved control table beside it
#                          (`cargo run --example mixing_automation_controls`).
#   preflight WORKDIR      Untimed. The seven-arm premises (restated == quiet, every automated
#                          effect moves bits) on the prepared module.
#   run WORKDIR --step N   The one timed invocation: the premises, then the three arms alternated
#                          per observation. Writes `artifacts/steps/N/web-mixing-automation.jsonl`
#                          and refuses to overwrite it. Requires unmodified tracked files, pins the
#                          process to the highest online CPU, and refuses a loaded host unless
#                          `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1`, in which case the record says
#                          `uncontrolled`, as the console runner's do.
#
# The module is built here rather than taken from `build-web-audioworklet.sh` because that script
# refuses to hand over an artifact whose digest is not the committed pin, and a batch branch repins
# only at its boundary. The record states the digest and whether it matches the pin, so a record
# taken on a repinned tree proves the recipe is the delivery one.
set -euo pipefail

usage() {
    printf 'usage: %s prepare WORKDIR | preflight WORKDIR | run WORKDIR --step NAME\n' "$0" >&2
    exit 2
}
[[ "$#" -ge 2 ]] || usage
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
cd "$root"
command=$1
workdir=$2
harness="$root/scripts/web-mixing-automation-benchmark.mjs"

case "$command" in
    prepare)
        [[ "$#" == 2 ]] || usage
        [[ -d "$workdir" && ! -L "$workdir" ]] || { printf 'WORKDIR must be an existing directory\n' >&2; exit 2; }
        [[ -z "$(find "$workdir" -mindepth 1 -maxdepth 1 -print -quit)" ]] ||
            { printf 'WORKDIR must be empty; refusing overwrite\n' >&2; exit 2; }
        workdir=$(cd "$workdir" && pwd -P)
        cargo_home=${CARGO_HOME:-$HOME/.cargo}
        # The delivery recipe, flag for flag: `simd128`, stripped debug information, and the two
        # path remaps that make the digest a function of the source alone.
        CARGO_TARGET_DIR="$workdir/target" \
        RUSTFLAGS="-C target-feature=+simd128 -C strip=debuginfo --remap-path-prefix=$cargo_home=/cargo --remap-path-prefix=$root=/repo" \
            cargo build --locked --release --target wasm32-unknown-unknown -p host-web
        cp -- "$workdir/target/wasm32-unknown-unknown/release/host_web.wasm" "$workdir/host_web.wasm"
        cargo run --locked --release --quiet -p console-workload \
            --example mixing_automation_controls >"$workdir/controls.json"
        jq -e '.controls | length == 8' "$workdir/controls.json" >/dev/null
        observed=$(sha256sum "$workdir/host_web.wasm" | awk '{print $1}')
        pinned=$(tr -d '\n' <hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256)
        printf 'host_web.wasm %s (pin %s: %s)\n' "$observed" "$pinned" \
            "$([[ "$observed" == "$pinned" ]] && printf match || printf 'differs; this tree is not repinned')"
        ;;
    preflight)
        [[ "$#" == 2 ]] || usage
        node --no-liftoff "$harness" preflight "$workdir/host_web.wasm" "$workdir/controls.json"
        ;;
    run)
        [[ "$#" == 4 && "$3" == --step ]] || usage
        [[ "$4" =~ ^[a-z0-9][a-z0-9-]{0,63}$ ]] || { printf 'invalid --step name: %s\n' "$4" >&2; exit 2; }
        artifact_dir="$root/artifacts/steps/$4"
        record="$artifact_dir/web-mixing-automation.jsonl"
        stderr_log="$artifact_dir/web-mixing-automation.stderr.log"
        for path in "$record" "$stderr_log"; do
            [[ ! -e "$path" && ! -L "$path" ]] ||
                { printf 'refusing to overwrite web artifact: %s\n' "$path" >&2; exit 1; }
        done
        # Tracked files only: every input the run reads -- this script, the harness,
        # `prepared-control.js`, the fixture, the ABI layout -- is tracked, and the console runner's
        # own step artifacts may already sit untracked in the directory this run writes to.
        [[ -z "$(git status --porcelain=v1 --untracked-files=no)" ]] ||
            { printf 'the browser arm requires a clean committed candidate\n' >&2; exit 1; }
        [[ -f "$workdir/host_web.wasm" && -f "$workdir/controls.json" ]] ||
            { printf 'run prepare first\n' >&2; exit 1; }
        source "$root/scripts/check-bench-preconditions.sh"
        cpu=$(bench_highest_cpu "$(< /sys/devices/system/cpu/online)")
        loadavg_text=$(< /proc/loadavg)
        loadavg_one=$(bench_loadavg_one_minute "$loadavg_text")
        if bench_within_ceiling "$loadavg_one" "$MISO_ENGINE_BENCH_LOADAVG_CEILING"; then
            control="controlled; loadavg $loadavg_text; ceiling $MISO_ENGINE_BENCH_LOADAVG_CEILING; affinity cpu $cpu"
        elif [[ "${MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED:-}" == 1 ]]; then
            control="uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived loadavg_above_ceiling; loadavg $loadavg_text; affinity cpu $cpu"
        else
            printf 'refusing an uncontrolled measurement: loadavg %s\n' "$loadavg_text" >&2
            exit 1
        fi
        umask 077
        mkdir -p "$artifact_dir"
        set -o noclobber
        raw=$(mktemp)
        trap 'rm -f -- "$raw"' EXIT
        taskset -c "$cpu" node --no-liftoff "$harness" run "$workdir/host_web.wasm" \
            "$workdir/controls.json" >|"$raw" 2>"$stderr_log"
        # The premises the harness asserted in-run, restated on the record it printed: a record
        # that says otherwise was not produced by this harness.
        jq -e '
            .record == "web_mixing_automation" and .observations == 1000 and
            .arms == ["quiet","restated","automated"] and (.controls | length) == 8 and
            .quiet_output_sha256 == .restated_output_sha256 and
            .restated_output_sha256 != .automated_output_sha256 and
            (.preflight_output_sha256 | [.automated_eq_only, .automated_compressor_only,
                .automated_limiter_only] | all(. != $restated)) and
            .records_admitted.restated == 8000 and .records_admitted.automated == 8000 and
            .descriptive_only == true' \
            --arg restated "$(jq -r .restated_output_sha256 "$raw")" "$raw" >/dev/null
        jq -c --arg commit "$(git rev-parse --verify HEAD)" --arg control "$control" \
            --arg cpu "$cpu" \
            '. + {candidate_commit: $commit, measurement_control: $control, cpu_affinity: $cpu}' \
            "$raw" >"$record"
        printf '%s\n' "$record"
        ;;
    *) usage ;;
esac
