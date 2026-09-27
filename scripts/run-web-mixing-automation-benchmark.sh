#!/usr/bin/env bash
# Issue #1003: the browser arm of `console_mixing_automation`, on the shipped `host_web.wasm`.
#
# The console benchmark's native row renders the mono console in-process. This arm renders the
# same session, with the same eight controls, through the product's browser path: the host_web
# module built with the delivery recipe, booted and driven under Node's V8 through
# `prepared-control.js` and `miso_engine_web_v1_command_submit` exactly as the SDK drives it, with
# `miso_engine_web_v1_render` alone inside the clock (`scripts/web-mixing-automation-benchmark.mjs`
# says what else it does and why, and how its input differs from the native row's).
#
# Three subcommands, so that nothing is built while anything is timed:
#
#   prepare WORKDIR        Untimed. WORKDIR must be an empty directory, and the tracked files
#                          unmodified. Builds host_web.wasm with the flags of
#                          `scripts/build-web-audioworklet.sh` into WORKDIR, writes the native
#                          row's resolved control table beside it (`cargo run --example
#                          mixing_automation_controls`), and records the commit both were built at,
#                          with their digests, in `provenance.json` (#1011).
#   preflight WORKDIR      Untimed. The seven-arm premises (restated == quiet, every automated
#                          effect moves bits) on the prepared module.
#   run WORKDIR --step N   The one timed invocation. It refuses a module or a control table that
#                          was not prepared at HEAD, or that changed after `prepare` recorded it.
#                          Then it launches the harness three times, as the console runner does: one
#                          warmup, whose record is discarded, and two measured rounds (#1011). The
#                          two records must pass `scripts/web-mixing-automation-validator.jq`,
#                          which requires them to agree on every digest, before either is written
#                          to `artifacts/steps/N/web-mixing-automation.jsonl`; it refuses to
#                          overwrite that file. Requires unmodified tracked files, pins every launch
#                          to the highest online CPU, and refuses a loaded host unless
#                          `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1`, in which case the records say
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

# Tracked files only: every input a build or a run reads -- the sources, this script, the harness,
# `prepared-control.js`, the fixture, the ABI layout -- is tracked, and the console runner's own step
# artifacts may already sit untracked in the directory a run writes to.
require_clean_tree() {
    [[ -z "$(git status --porcelain=v1 --untracked-files=no)" ]] ||
        { printf '%s requires unmodified tracked files\n' "$1" >&2; exit 1; }
}
digest() { sha256sum "$1" | awk '{print $1}'; }

case "$command" in
    prepare)
        [[ "$#" == 2 ]] || usage
        [[ -d "$workdir" && ! -L "$workdir" ]] || { printf 'WORKDIR must be an existing directory\n' >&2; exit 2; }
        [[ -z "$(find "$workdir" -mindepth 1 -maxdepth 1 -print -quit)" ]] ||
            { printf 'WORKDIR must be empty; refusing overwrite\n' >&2; exit 2; }
        workdir=$(cd "$workdir" && pwd -P)
        # The commit is what `run` holds the module to, so it has to describe what is built.
        require_clean_tree prepare
        commit=$(git rev-parse --verify HEAD)
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
        # Nothing moved under the build: the recorded commit is still the tree that was built.
        require_clean_tree prepare
        [[ "$(git rev-parse --verify HEAD)" == "$commit" ]] ||
            { printf 'HEAD moved during prepare; refusing to record a commit\n' >&2; exit 1; }
        jq -n --arg commit "$commit" --arg module "$(digest "$workdir/host_web.wasm")" \
            --arg controls "$(digest "$workdir/controls.json")" \
            '{commit: $commit, module_sha256: $module, controls_sha256: $controls}' \
            >"$workdir/provenance.json"
        observed=$(digest "$workdir/host_web.wasm")
        pinned=$(tr -d '\n' <hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256)
        printf 'host_web.wasm %s at %s (pin %s: %s)\n' "$observed" "$commit" "$pinned" \
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
        require_clean_tree 'the browser arm'
        provenance="$workdir/provenance.json"
        [[ -f "$workdir/host_web.wasm" && -f "$workdir/controls.json" && -f "$provenance" ]] ||
            { printf 'run prepare first\n' >&2; exit 1; }
        # The module and the control table belong to the commit the rounds run at, and are the
        # bytes `prepare` recorded. Either mismatch would publish one commit's module under another
        # commit's name.
        commit=$(git rev-parse --verify HEAD)
        prepared_commit=$(jq -r '.commit' "$provenance")
        [[ "$prepared_commit" == "$commit" ]] ||
            { printf 'the module was prepared at %s, not at HEAD %s; run prepare again\n' \
                "$prepared_commit" "$commit" >&2; exit 1; }
        [[ "$(digest "$workdir/host_web.wasm")" == "$(jq -r '.module_sha256' "$provenance")" ]] ||
            { printf 'host_web.wasm changed after prepare recorded it\n' >&2; exit 1; }
        [[ "$(digest "$workdir/controls.json")" == "$(jq -r '.controls_sha256' "$provenance")" ]] ||
            { printf 'controls.json changed after prepare recorded it\n' >&2; exit 1; }
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
        raw=$(mktemp -d)
        trap 'rm -rf -- "$raw"' EXIT
        : >"$stderr_log"
        # One process per round, as the console runner launches its binary: the warmup's record is
        # discarded, and its in-run assertions still have to pass.
        for round in warmup 1 2; do
            taskset -c "$cpu" node --no-liftoff "$harness" run "$workdir/host_web.wasm" \
                "$workdir/controls.json" "$round" >"$raw/$round.json" 2>>"$stderr_log"
        done
        for round in 1 2; do
            jq -c --arg commit "$commit" --arg prepared "$prepared_commit" \
                --arg control "$control" --arg cpu "$cpu" \
                '. + {candidate_commit: $commit, prepared_commit: $prepared,
                      measurement_control: $control, cpu_affinity: $cpu}' \
                "$raw/$round.json" >>"$raw/rounds.jsonl"
        done
        jq -s -e -L "$root/scripts" -f "$root/scripts/web-mixing-automation-validator.jq" \
            "$raw/rounds.jsonl" >/dev/null ||
            { printf 'the browser rounds failed their validator\n' >&2; exit 1; }
        set -o noclobber
        cp -- "$raw/rounds.jsonl" "$record"
        printf '%s\n' "$record"
        ;;
    *) usage ;;
esac
