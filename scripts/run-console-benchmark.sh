#!/usr/bin/env bash
# Sole exactly-once Issue-149 console qualification timing entrypoint. Do not invoke its binary
# directly: the runner is what supplies the round marker and the host metadata, and a direct
# invocation produces a record whose provenance is a guess.
#
# One optional argument, `--phase2`, `--phase3` or `--issue163-phase0`, moves the artifact
# directory and changes nothing else. Phase 1's
# record is a consumed one-shot authority: it describes the tree that produced it, and the sealed
# fast dB tier (#144 item 5) deliberately moves the rendered bits it measured, so a phase-2 number
# belongs beside it rather than on top of it. Phase 3 gets its own directory for the opposite
# reason -- the multiband ramping split is class A and moves no rendered bit at all, so its console
# numbers are a *re-measurement* of the phase-2 tree and have to be readable as one rather than
# blended into it. `--issue163-phase0` writes to `artifacts/issue163-phase0` for the same reason
# again, and one further one: phase 0 changes what the subject *measures* -- five decomposition
# rows, a meters arm and an observation arm join the stream, and the run is admissible under
# preconditions the earlier records were never held to -- so its numbers are a new authority
# beside the issue-149 ones and not a continuation of them. Every directory keeps the same
# refusal-to-overwrite discipline, so none of them can be quietly re-run. `--issue163-phase3`
# writes to `artifacts/issue163-phase3` on the same terms as `--issue163-phase4` below: phase 3
# (bank interleave) is class A throughout, so every workload's output digest is the phase-1 digest
# to the bit, and the arm exists so that the interleaved tree's numbers describe the interleaved
# tree. `--issue163-phase4`
# writes to `artifacts/issue163-phase4` and is a *re-measurement* of the phase-1 subject: phase 4
# is class A throughout, so every workload's output digest is the phase-1 digest to the bit and
# the two records are directly comparable row by row. It gets its own directory anyway, for the
# reason every phase does -- a consumed one-shot describes the tree that produced it, and phase 4
# changes what the idle row costs without changing what any row computes.
#
# `--issue163-phase2` writes to `artifacts/issue163-phase2` and is the one arm in this list whose
# subject is **class B**. Phases 1, 3 and 4 are class A: every workload's output digest is the
# phase-1 digest to the bit, which is what lets their records be read row against row. Phase 2
# changes the numeric contract itself (fused multiply-add to unfused, owner ruling 2026-08-26), so
# every `output_sha256` in its record differs from every earlier record's by construction. That is
# not drift and it is not a defect: it is the change being measured, and
# `docs/rulings/unfused-multiply-add-audit.md` is the evidence that the new bits are the intended
# ones. The timing rows remain comparable -- the workloads, fixture, quantum and observation count
# are unchanged -- but the digest columns must not be compared, and a reader who diffs them will
# find every row different.
#
# `--round2-lim` and `--round2-lim-baseline` are the paired arms of the limiter's round-2
# effect-optimisation pass: the same rows with and without two class-A kernel changes (the uniform
# gain frame loop de-bookkeeped, and the detector history moved out of linear memory on the wasm
# target). Both are class A, so the two records must reproduce each other's `output_sha256`
# exactly on every row and every leg and differ only in time. The baseline arm is the base commit
# with this arm registration and nothing else.
#
# `--audit-chain-merge` and `--audit-chain-merge-baseline` are the paired arms of issue #202
# recommendation 2: the cross-rack cohort chain merge. `runtime::cohort_runs` took its merge
# candidates from the cohort planner's groups, which are pooled per `RackLocationV1` and do not
# exist at all for a builtin bank, so the 64-track intended strip ran three bank chains per cohort
# -- `{builtins, simd1, simd2}` -- and paid one planar/AoSoA round-trip for each. Candidacy is now
# taken from the lowered program's dataflow and proved lane by lane on it, so the whole strip fuses
# into one chain per cohort: 24 round-trips a block become 8. Class **A** -- every `output_sha256`
# must reproduce the baseline arm's exactly, on every row and every leg, and the two records differ
# only in time. The baseline arm is the base commit with this arm registration and nothing else.
# The one record expected to move a non-timing field is `console_placement`, whose two arms now
# report the *same* transposes per block: the retired layout's cross-rack pair fuses too, so which
# rack a slot was placed in no longer changes how many round-trips its cohort pays.
#
# `--strip2` and `--strip2-baseline` are the paired arms of the strip/overhead round's job 2: the
# fader and the pan matrix become strip-intrinsic banked chain slots. They were 128 individually
# dispatched per-track `Bound` ops at `L = f32` sitting *between* the cohorts' chains; they are now
# builtin banks at the cohort's lane order, so issue #202 rec 2's dataflow candidacy fuses
# `builtins -> EQ -> compressor -> limiter -> fader -> matrix` into ONE chain per cohort. The
# counters say so without ambiguity: the 64-track fixture goes from 32 bank slots to 48 with
# `chains` and `transposes` staying at one per cohort per block. Class **A** -- every
# `output_sha256` must reproduce the baseline arm's exactly, on every row and every leg, and the
# two records differ only in time.
#
# What is removed: 128 per-op passes, their buffers and most of the `execute_op` scaffolding for
# them, and the 64 `reduce_plane` stereo block copies out of the limiter's dedicated buffer -- the
# fader is now a chain *slot*, and a later slot's op is never executed at all. What is added:
# 63 arena buffers on this fixture, because a chain that spans two more stages holds its bank
# window over a longer op range. `dispatch_only` is the row that should move most; `console` is the
# row the round is for. The nine-track ragged row is expected to move the *other* way by a little:
# its one-track tail now banks its fader and matrix like any other cohort and pays one extra
# planar/AoSoA round-trip per block for them, which is reported rather than hidden.
#
# The baseline arm is the base commit with this arm registration and nothing else.
#
#
# `--strip3` and `--strip3-baseline` are the paired arms of the strip/overhead round's job 3: the
# route application and the master-bus accumulation fold into the cohort chain's own epilogue. The
# 64-track fixture paid, per block, 64 route ops -- a whole `mix2x2_block` pass over a buffer the
# chain had just scattered -- and then a 63-pass `sum_into_block` reduction over those 64 buffers.
# It now pays one pass: each lane's tile is routed where the transpose left it and goes straight
# into the master, the first contributor storing and the rest accumulating. The dead fan-in-zero
# fill under a bound source goes with it: a host source writes every word it is handed, so the
# `fill(0.0)` in front of it was two stereo blocks of dead stores per track per block.
#
# Class **A** -- every `output_sha256` must reproduce the baseline arm's exactly, on every row and
# every leg, and the two records differ only in time. That the epilogues sum in the reduction's own
# order is proved at bind on the lowered program, not assumed; a plan that cannot prove it renders
# the job-2 shape unchanged.
#
# The counter that says the fold fired is `bank_route_folds`, and it is a count for the same reason
# `bank_scatter_redirects` is: the optimisation is a thing *not done*, so it moves no rendered bit
# and no gate may rest on a timing difference. The 64-track fixture folds 64 lanes, the 128-track
# stretch folds 128, and the nine-track fixtures fold 9. `chains`, `slots` and `transposes` are
# unmoved throughout. `dispatch_only` is the row that should move most; the stretch row should move
# about twice the console row's absolute.
#
# The baseline arm is the base commit with this arm registration and nothing else.
#
# `--strip1` and `--strip1-baseline` are the paired arms of the strip/overhead round's job 1: the
# prepared-identity builtin-section elision. A builtin filter at a 0 Hz cutoff is *designed* as the
# arithmetic identity rather than branched around, so the two disabled SVF sections of a rack-free
# strip were executed as identities every block. A section whose six coefficient words and two
# retained state words are bit-pattern-equal to that identity in every lane of the bank is now
# decided elidable at bank construction, and a run of them is emitted as the single `add(+0.0)` the
# run composes to, at the run's position in the chain. Class **A** -- every `output_sha256` must
# reproduce the baseline arm's exactly, on every row and every leg, and the two records differ only
# in time. `sixty_four_track_dispatch_only` is the row that moves most; every other row's builtins
# carry a real design and elide nothing.
#
# The arm carries a second, smaller class-A change (candidate A1): the D7 sanitisation counter
# accumulates `1.0 & bad` rather than `select(bad, 1.0, 0.0)` at all four copies of the sanitise
# prologue. On a canonical mask those are the same bits, so this moves no digest either; it is
# worth about -0.10 to -0.14 us/block and it applies to *every* row, because every block is
# sanitised. So the expected motion of this pair is: `dispatch_only` to roughly 10.6 us, and
# `builtins_only`, `console` and `idle` each down by about 0.12 us -- a real, expected motion
# rather than noise. A row that does not move at all is as much a finding as one that moves too
# far. The baseline arm is the base commit with this arm registration and nothing else.
#
#
# `--strip4` is the strip/overhead round's job 4, and it is the one arm in this list that has **no
# baseline partner**. Job 4 adds no engine change at all: it is the measurement plane -- two
# overhead decomposition rows (`sixty_four_track_plumbing_only`, the route and the master reduction
# with no builtin prepared at all, and `sixty_four_track_gain_pan_only`, the identity sections with
# the fixture's real fader and pan) and three mono rows on a new checked-in fixture. Every existing
# row's `output_sha256` is unchanged from `strip3`, which is the arm this one is read against; the
# five new rows have no earlier number to be read against, and that is the point of capturing them.
# This capture is the post-strip-round baseline the sprint scoreboard quotes.
#
# `--mono2` is mono-collapse M2: the collapsed execution. Like `--strip4` it has **no baseline
# partner**, and for a stronger reason than "nothing changed" -- the baseline is *inside the run*.
# The `console_mono` record is a paired measurement whose two arms are the same fixture with the
# collapse taken and forced off, alternated observation by observation, so its delta is the
# mechanism's cost measured against itself on one machine at one moment. What the arm is read
# against externally is `strip4`: every non-mono row's `output_sha256` must be unchanged from it,
# because a session with no mono-source track never collapses and a change to its bits would mean
# the dispatch fired where it must not.
# `--issue-loop-eq-r1` writes to `artifacts/issue-loop-eq-r1` and is the effect-optimization loop's
# EQ round 1: parametric-EQ identity-section elision and the two-slot cohort chain (#181). Both are
# **class A** -- every workload's output digest is the #175 digest to the bit, on every row and every
# leg -- so its rows are read directly against `artifacts/issue175`, which is the standing authority
# for the intended strip. It gets its own directory for the reason every arm does: a consumed
# one-shot describes the tree that produced it, and this tree renders the same bits for less time.
# The one record that is *expected* to move a non-timing field is `console_placement`, whose
# `merged_chain_transposes_per_block` falls from 24 to 16; that count is derived from the realised
# bank count, and #175 wrote its equality specifically so that the day the graph layer took the
# saving, the equality would go red and say so.
#
# `--round2-lane` and `--round2-lane-baseline` are the paired arms of round 2's lane lowerings:
# `Lane::select` emitted as `blendv` instead of the three-instruction `bitselect`, and
# `Lane::max`/`Lane::min` emitted as the one instruction x86 and wasm each have with the D8 rule
# (`crates/miso-engine-lane/src/wide_impl.rs`). Both are **class A** and both are class A for a
# stronger reason than usual: they change emitted instructions only, so every workload's
# `output_sha256` must equal the baseline arm's on every row and every leg, and a single digit of
# difference is a defect rather than a re-pin. The two arms are one tree apart -- the baseline arm
# is captured with `wide_impl.rs` reverted to the base commit and nothing else changed -- so the
# rows are read against each other directly, and against `artifacts/issue175`, which remains the
# standing authority for the intended strip.
# `--round2-eqrack` and `--round2-eqrack-baseline` are the paired arms of rack/EQ round 2, and they
# are read against each other rather than against a standing directory: the baseline is captured at
# the merge base and the candidate on the same tree plus three changes -- the vectorised
# planar/AoSoA whole-bank transpose, the skipped bank-member dedication copy, and the EQ identity
# refresh batched out of the per-lane snap loop. All three are **class A**, so every `output_sha256`
# must be byte-identical between the two arms on every row, `console_automation` and the nine-track
# ragged row included; the ragged row matters twice over, because it is the fixture that exercises
# the partial-bank scalar transpose the tiled path deliberately does not replace. Unlike the EQ
# round-1 arm, `console_placement`'s `merged_chain_transposes_per_block` must *not* move: this round
# makes each transpose cheaper and does not remove one.
#
# # Admissibility (#144 item 13, #163 phase 0a)
#
# Everything from `check-bench-preconditions.sh` down to the warmup is a *precondition*, not a
# note. The runner refuses a measurement it cannot control and names which control it lacked. The
# escape hatch `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1` exists for machines where control is
# genuinely impossible, and it does not make the run look controlled: every record it writes
# carries `measurement_control: "uncontrolled"` and the validator refuses to let that record claim
# otherwise.
set -euo pipefail
phase_directory=issue149
if [[ "$#" == 1 ]]; then
    case "$1" in
        --phase2) phase_directory=issue149-phase2 ;;
        --phase3) phase_directory=issue149-phase3 ;;
        --issue163-phase0) phase_directory=issue163-phase0 ;;
        --issue163-phase1) phase_directory=issue163-phase1 ;;
        --issue163-phase2) phase_directory=issue163-phase2 ;;
        --issue163-phase3) phase_directory=issue163-phase3 ;;
        --issue163-phase4) phase_directory=issue163-phase4 ;;
        --issue175) phase_directory=issue175 ;;
        --issue182) phase_directory=issue182 ;;
        --issue-loop-eq-r1) phase_directory=issue-loop-eq-r1 ;;
        --compressor-round1) phase_directory=compressor-round1 ;;
        --compressor-round1-baseline) phase_directory=compressor-round1-baseline ;;
        --round1-composed) phase_directory=round1-composed ;;
        --issue184) phase_directory=issue184 ;;
        --round2-lane) phase_directory=round2-lane ;;
        --round2-lane-baseline) phase_directory=round2-lane-baseline ;;
        --round2-eqrack) phase_directory=round2-eqrack ;;
        --round2-eqrack-baseline) phase_directory=round2-eqrack-baseline ;;
        --round2-comp) phase_directory=round2-comp ;;
        --round2-comp-baseline) phase_directory=round2-comp-baseline ;;
        --round2-lim) phase_directory=round2-lim ;;
        --round2-lim-baseline) phase_directory=round2-lim-baseline ;;
        --round2-composed) phase_directory=round2-composed ;;
        --audit-chain-merge) phase_directory=audit-chain-merge ;;
        --audit-chain-merge-baseline) phase_directory=audit-chain-merge-baseline ;;
        --strip1) phase_directory=strip1 ;;
        --strip1-baseline) phase_directory=strip1-baseline ;;
        --strip2) phase_directory=strip2 ;;
        --strip2-baseline) phase_directory=strip2-baseline ;;
        --strip3) phase_directory=strip3 ;;
        --strip3-baseline) phase_directory=strip3-baseline ;;
        --strip4) phase_directory=strip4 ;;
        --mono2) phase_directory=mono2 ;;
        *) printf 'usage: %s [--phase2|--phase3|--issue163-phase0|--issue163-phase1|--issue163-phase2|--issue163-phase3|--issue163-phase4|--issue175|--issue182|--issue-loop-eq-r1|--compressor-round1|--compressor-round1-baseline|--round1-composed|--issue184|--round2-lane|--round2-lane-baseline|--round2-eqrack|--round2-eqrack-baseline|--round2-comp|--round2-comp-baseline|--round2-lim|--round2-lim-baseline|--round2-composed|--audit-chain-merge|--audit-chain-merge-baseline|--strip1|--strip1-baseline|--strip2|--strip2-baseline|--strip3|--strip3-baseline|--strip4|--mono2]\n' "$0" >&2; exit 2 ;;
    esac
elif [[ "$#" != 0 ]]; then
    printf 'usage: %s [--phase2|--phase3|--issue163-phase0|--issue163-phase1|--issue163-phase2|--issue163-phase3|--issue163-phase4|--issue175|--issue182|--issue-loop-eq-r1|--compressor-round1|--compressor-round1-baseline|--round1-composed|--issue184|--round2-lane|--round2-lane-baseline|--round2-eqrack|--round2-eqrack-baseline|--round2-comp|--round2-comp-baseline|--round2-lim|--round2-lim-baseline|--round2-composed|--audit-chain-merge|--audit-chain-merge-baseline|--strip1|--strip1-baseline|--strip2|--strip2-baseline|--strip3|--strip3-baseline|--strip4|--mono2]\n' "$0" >&2
    exit 2
fi
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
# shellcheck source=scripts/check-bench-preconditions.sh
source "$root/scripts/check-bench-preconditions.sh"

artifact_dir="$root/artifacts/$phase_directory"
raw="$artifact_dir/console-benchmark.raw.jsonl"
accepted="$artifact_dir/console-benchmark.accepted.jsonl"
stderr_log="$artifact_dir/console-benchmark.stderr.log"
disposition="$artifact_dir/console-benchmark.disposition.json"
# #184: the perf-counter evidence behind the records' cycle columns. One file per launch that was
# counted, kept beside the records for the same reason the stderr log is kept -- a derived column
# whose instrument left no trace is a claim, not a measurement.
core_clock_log="$artifact_dir/console-benchmark.core-clock.csv"
for path in "$raw" "$accepted" "$stderr_log" "$disposition" "$core_clock_log"; do
    [[ ! -e "$path" ]] || { printf 'refusing to overwrite console artifact: %s\n' "$path" >&2; exit 1; }
done
[[ "$(uname -m)" == "x86_64" ]] || { printf 'Issue-149 qualification requires x86_64\n' >&2; exit 1; }
grep -qm1 -w avx2 /proc/cpuinfo || { printf 'Issue-149 qualification requires AVX2\n' >&2; exit 1; }
[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || {
    printf 'Issue-149 qualification requires a clean committed candidate\n' >&2
    exit 1
}

umask 077
mkdir -p "$artifact_dir"
set -o noclobber
: >"$stderr_log"

failed=1
failure_reason=unexpected_failure
workload_process_launches=0
warmup_launches=0
measured_rounds_completed=0
candidate_commit=
candidate_commit_sha256=
binary_sha256=
# Declared before the exit trap can fire so a failure *inside* the precondition block still writes
# a disposition that says what the run knew about its own admissibility at the point it failed.
measurement_control=unevaluated
cpu_affinity=unevaluated

artifact_identity() {
    local path=$1
    if [[ -e "$path" ]]; then
        printf '"%s" %s' "$(sha256sum "$path" | awk '{print $1}')" "$(wc -c <"$path")"
    else
        printf 'null 0'
    fi
}
write_disposition() {
    local status=$1 reason=$2 raw_identity accepted_identity stderr_identity
    raw_identity=$(artifact_identity "$raw")
    accepted_identity=$(artifact_identity "$accepted")
    stderr_identity=$(artifact_identity "$stderr_log")
    local raw_sha raw_bytes accepted_sha accepted_bytes stderr_sha stderr_bytes
    read -r raw_sha raw_bytes <<<"$raw_identity"
    read -r accepted_sha accepted_bytes <<<"$accepted_identity"
    read -r stderr_sha stderr_bytes <<<"$stderr_identity"
    local candidate_json=null candidate_sha_json=null binary_sha_json=null
    [[ -n "$candidate_commit" ]] && candidate_json="\"$candidate_commit\""
    [[ -n "$candidate_commit_sha256" ]] && candidate_sha_json="\"$candidate_commit_sha256\""
    [[ -n "$binary_sha256" ]] && binary_sha_json="\"$binary_sha256\""
    printf '{"schema_version":1,"issue":149,"status":"%s","reason":"%s","runner_invocations":1,"workload_process_launches":%s,"warmup_launches":%s,"measured_rounds_completed":%s,"measurement_control":"%s","cpu_affinity":"%s","candidate_commit":%s,"candidate_commit_sha256":%s,"binary_sha256":%s,"raw_sha256":%s,"raw_bytes":%s,"accepted_sha256":%s,"accepted_bytes":%s,"stderr_sha256":%s,"stderr_bytes":%s}\n' \
        "$status" "$reason" "$workload_process_launches" "$warmup_launches" \
        "$measured_rounds_completed" "$measurement_control" "$cpu_affinity" \
        "$candidate_json" "$candidate_sha_json" \
        "$binary_sha_json" "$raw_sha" "$raw_bytes" "$accepted_sha" "$accepted_bytes" \
        "$stderr_sha" "$stderr_bytes" >"$disposition"
}
on_exit() {
    local status=$?
    trap - EXIT
    if [[ "$failed" == 1 && ! -e "$disposition" ]]; then
        set +e
        write_disposition FAIL "$failure_reason"
    fi
    exit "$status"
}
on_signal() {
    failure_reason=interrupted
    exit 130
}
trap on_exit EXIT
trap on_signal INT TERM

failure_reason=fixture_failed
bash scripts/check-console-benchmark-fixture.sh >>"$stderr_log" 2>&1 || exit 1

failure_reason=candidate_identity_failed
candidate_commit=$(git rev-parse --verify HEAD 2>>"$stderr_log")
candidate_commit_sha256=$(printf '%s' "$candidate_commit" | sha256sum | awk '{print $1}')

# Freeze the release profile so the recorded build metadata describes the binary actually run.
export CARGO_PROFILE_RELEASE_OPT_LEVEL=3
export CARGO_PROFILE_RELEASE_LTO=false
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16
failure_reason=build_failed
cargo build --locked --release --quiet -p miso-engine-bench 2>>"$stderr_log"
binary="$root/target/release/miso_engine_bench"
[[ -x "$binary" ]] || { failure_reason=missing_binary; exit 1; }
failure_reason=binary_identity_failed
binary_sha256=$(sha256sum "$binary" | awk '{print $1}')

failure_reason=metadata_failed
cpu_model=$(awk -F: '/model name/ {gsub(/^ +/, "", $2); print $2; exit}' /proc/cpuinfo)
rust_version=$(rustc -V)
llvm_version=$(rustc -vV | awk -F: '/LLVM version/ {gsub(/^ +/, "", $2); print $2}')
target_triple=$(rustc -vV | awk -F: '/host/ {gsub(/^ +/, "", $2); print $2}')
target_features="runtime-avx2$(grep -qm1 ' fma ' /proc/cpuinfo && printf '%s' ',fma' || true);baseline"
governor_or_power_mode=unknown
if [[ -r /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor ]]; then
    governor_or_power_mode=$(< /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor)
fi

# ---------------------------------------------------------------------------------------------
# Preconditions. Each one refuses with a named reason, or is recorded as waived.
# ---------------------------------------------------------------------------------------------
failure_reason=precondition_failed
allow_uncontrolled=0
[[ "${MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED:-}" == 1 ]] && allow_uncontrolled=1
declare -a refusals=()
declare -a affinity=()
cpu_affinity=uncontrolled
note_affinity='affinity none'
note_sibling='smt not-checked'

# 1. Single-core affinity. Two tenants alternating on one core is the single largest source of
#    per-block variance on a loaded host, and it is the one the process itself can eliminate.
if command -v taskset >/dev/null 2>&1 && [[ -r /sys/devices/system/cpu/online ]] &&
    bench_cpu=$(bench_highest_cpu "$(< /sys/devices/system/cpu/online)") &&
    taskset -c "$bench_cpu" true >/dev/null 2>&1; then
    affinity=(taskset -c "$bench_cpu")
    cpu_affinity="$bench_cpu"
    note_affinity="affinity cpu $bench_cpu"
else
    refusals+=(affinity_unavailable)
    bench_cpu=
fi

# 2. Binary-mtime cooldown. A release build saturates every core; the package is hot and the
#    governor is ramped for tens of seconds afterwards. Wait out the remainder of the cooldown
#    before anything is timed, then refuse if the binary moved underneath us while we waited --
#    a second build racing this one would make the recorded sha256 describe a different program
#    than the one measured.
binary_mtime_before=$(stat -c %Y "$binary" 2>/dev/null) || {
    failure_reason=binary_mtime_unreadable
    exit 1
}
binary_age=$(( $(date +%s) - binary_mtime_before ))
cooldown_waited=0
if (( binary_age < MISO_ENGINE_BENCH_COOLDOWN_SECONDS )); then
    cooldown_waited=$(( MISO_ENGINE_BENCH_COOLDOWN_SECONDS - binary_age ))
    sleep "$cooldown_waited"
fi
binary_mtime_after=$(stat -c %Y "$binary" 2>/dev/null) || {
    failure_reason=binary_mtime_unreadable
    exit 1
}
[[ "$binary_mtime_before" == "$binary_mtime_after" ]] || {
    failure_reason=binary_rebuilt_during_cooldown
    exit 1
}

# 3. Load-average ceiling, read *after* the cooldown so it describes the machine that is about to
#    be measured rather than the machine that just finished compiling.
loadavg_text=$(< /proc/loadavg)
loadavg_one=$(bench_loadavg_one_minute "$loadavg_text") || loadavg_one=
if [[ -n "$loadavg_one" ]] &&
    bench_within_ceiling "$loadavg_one" "$MISO_ENGINE_BENCH_LOADAVG_CEILING"; then
    :
else
    refusals+=(loadavg_above_ceiling)
fi

# 4. SMT sibling quiet. Cheap where the topology is exported and skipped, not refused, where it is
#    not: a container that hides `/sys/devices/system/cpu/*/topology` is not evidence of a busy
#    sibling, and inventing a refusal from missing information is its own dishonesty. What the
#    check cannot establish it says it could not establish.
if [[ -n "$bench_cpu" ]]; then
    sibling_path="/sys/devices/system/cpu/cpu$bench_cpu/topology/thread_siblings_list"
    if [[ -r "$sibling_path" ]]; then
        siblings=$(bench_other_siblings "$bench_cpu" "$(< "$sibling_path")") || siblings=
        if [[ -z "$siblings" ]]; then
            note_sibling='smt none'
        else
            stat_before=$(< /proc/stat)
            sleep "$MISO_ENGINE_BENCH_SIBLING_SAMPLE_SECONDS"
            stat_after=$(< /proc/stat)
            note_sibling="smt siblings $siblings"
            for sibling in $siblings; do
                busy=$(bench_cpu_busy_percent "$stat_before" "$stat_after" "$sibling") || busy=
                if [[ -z "$busy" ]]; then
                    note_sibling="$note_sibling cpu$sibling=unreadable"
                    continue
                fi
                note_sibling="$note_sibling cpu$sibling=$busy%"
                bench_within_ceiling "$busy" "$MISO_ENGINE_BENCH_SIBLING_BUSY_CEILING" ||
                    refusals+=(smt_sibling_busy)
            done
        fi
    else
        note_sibling='smt topology-unavailable'
    fi
fi

if [[ "${#refusals[@]}" == 0 ]]; then
    measurement_control=controlled
    background_load_note="controlled; loadavg $loadavg_text; ceiling $MISO_ENGINE_BENCH_LOADAVG_CEILING; $note_affinity; $note_sibling; cooldown ${MISO_ENGINE_BENCH_COOLDOWN_SECONDS}s waited ${cooldown_waited}s"
elif [[ "$allow_uncontrolled" == 1 ]]; then
    measurement_control=uncontrolled
    background_load_note="uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived ${refusals[*]}; loadavg $loadavg_text; $note_affinity; $note_sibling; cooldown ${MISO_ENGINE_BENCH_COOLDOWN_SECONDS}s waited ${cooldown_waited}s"
else
    failure_reason="precondition_${refusals[0]}"
    printf 'refusing an uncontrolled measurement: %s\n' "${refusals[*]}" >&2
    printf 'loadavg %s; %s; %s\n' "$loadavg_text" "$note_affinity" "$note_sibling" >&2
    printf 'set MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 to record an uncontrolled run instead.\n' >&2
    exit 1
fi

# ---------------------------------------------------------------------------------------------
# The pinned core's clock (#184). A cycle column needs cycles, and wall time is not cycles.
# ---------------------------------------------------------------------------------------------
#
# `perf stat` counts `cycles` and `task-clock` over a launch, and their ratio is the clock the
# pinned core actually ran at while it was running this subject -- a hardware counter reading,
# taken under exactly the preconditions above, not a nameplate frequency and not `/proc/cpuinfo`.
# The warmup launch supplies it, because the subject has to have it in its environment *before* it
# builds a record and a `perf stat` result only exists after its workload has exited.
#
# The measured rounds are counted too, and their own ratio is checked against the exported figure.
# That is what makes using the warmup's number honest rather than convenient: if the core clocked
# differently while the numbers that are kept were being taken, the run refuses instead of
# publishing cycle columns derived from a clock that was not in force.
#
# A host with no usable counter is not a failure. It exports nothing, every record omits the whole
# column group, and the records validate exactly as the sealed ones under `artifacts/` do.
readonly MISO_ENGINE_BENCH_CORE_CLOCK_DRIFT_CEILING=0.03
core_clock_hz=
core_clock_source=
core_clock_available=0

# Cycles per second from one `perf stat -x,` CSV. The last complete pair in the file wins, so a
# file appended to by several launches reports the launch that wrote last. Empty when neither a
# `cycles` nor a `task-clock` row counted.
core_clock_from_csv() {
    awk -F, '
        $1 ~ /^[0-9]+([.][0-9]+)?$/ && $3 == "cycles" { cycles = $1 }
        $1 ~ /^[0-9]+([.][0-9]+)?$/ && $3 == "task-clock" { milliseconds = $1 }
        END { if (cycles > 0 && milliseconds > 0) printf "%.0f", cycles * 1000.0 / milliseconds }
    ' "$1"
}

# Refuse a run whose measured round did not clock like the warmup the records were told about.
core_clock_agrees() {
    local measured=$1
    awk -v exported="$core_clock_hz" -v measured="$measured" \
        -v ceiling="$MISO_ENGINE_BENCH_CORE_CLOCK_DRIFT_CEILING" \
        'BEGIN {
            drift = (measured - exported) / exported
            if (drift < 0) { drift = -drift }
            exit (drift <= ceiling) ? 0 : 1
        }'
}

core_clock_probe=$(mktemp)
if command -v perf >/dev/null 2>&1 &&
    perf stat -x, -e cycles,task-clock -o "$core_clock_probe" -- true >/dev/null 2>&1 &&
    [[ -n "$(core_clock_from_csv "$core_clock_probe")" ]]; then
    core_clock_available=1
fi
rm -f "$core_clock_probe"

run_round() {
    local round=$1 counted=${2:-}
    local -a counter=()
    if [[ -n "$counted" ]]; then
        counter=(perf stat -x, -e cycles,task-clock --append -o "$core_clock_log" --)
    fi
    workload_process_launches=$((workload_process_launches + 1))
    MISO_ENGINE_BENCH_ROUND="$round" \
    MISO_ENGINE_BENCH_CORE_CLOCK_HZ="$core_clock_hz" \
    MISO_ENGINE_BENCH_CORE_CLOCK_SOURCE="$core_clock_source" \
    MISO_ENGINE_BENCH_CANDIDATE_COMMIT="$candidate_commit" \
    MISO_ENGINE_BENCH_CPU_MODEL="$cpu_model" \
    MISO_ENGINE_BENCH_RUST_VERSION="$rust_version" \
    MISO_ENGINE_BENCH_LLVM_VERSION="$llvm_version" \
    MISO_ENGINE_BENCH_TARGET_TRIPLE="$target_triple" \
    MISO_ENGINE_BENCH_TARGET_FEATURES="$target_features" \
    MISO_ENGINE_BENCH_PROFILE=release \
    MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE="$background_load_note" \
    MISO_ENGINE_BENCH_MEASUREMENT_CONTROL="$measurement_control" \
    MISO_ENGINE_BENCH_CPU_AFFINITY="$cpu_affinity" \
    MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE="$governor_or_power_mode" \
    "${counter[@]}" "${affinity[@]}" "$binary" console
}

# One untimed warmup, then exactly the two frozen measured rounds. Raw stdout is append-only after
# its exclusive creation; failures preserve every byte emitted by the failed process.
failure_reason=warmup_failed
counted=
if (( core_clock_available == 1 )); then
    counted=counted
fi
run_round warmup "$counted" >/dev/null 2>>"$stderr_log" || exit 1
warmup_launches=1
if [[ -n "$counted" ]]; then
    failure_reason=core_clock_unreadable
    core_clock_hz=$(core_clock_from_csv "$core_clock_log")
    [[ -n "$core_clock_hz" ]] || exit 1
    core_clock_source="perf stat cycles/task-clock over the warmup launch, cpu $cpu_affinity"
fi
failure_reason=round_1_failed
run_round 1 "$counted" >"$raw" 2>>"$stderr_log" || exit 1
measured_rounds_completed=1
if [[ -n "$counted" ]]; then
    failure_reason=precondition_core_clock_drift
    core_clock_agrees "$(core_clock_from_csv "$core_clock_log")" || {
        printf 'refusing cycle columns taken under a clock that moved: exported %s Hz\n' \
            "$core_clock_hz" >&2
        exit 1
    }
fi
failure_reason=round_2_failed
run_round 2 "$counted" >>"$raw" 2>>"$stderr_log" || exit 1
measured_rounds_completed=2
if [[ -n "$counted" ]]; then
    failure_reason=precondition_core_clock_drift
    core_clock_agrees "$(core_clock_from_csv "$core_clock_log")" || {
        printf 'refusing cycle columns taken under a clock that moved: exported %s Hz\n' \
            "$core_clock_hz" >&2
        exit 1
    }
fi
failure_reason=record_count
[[ "$(wc -l <"$raw")" == 46 ]] || exit 1
failure_reason=validation_failed
jq -s -e -L scripts -f scripts/console-benchmark-validator.jq "$raw" >/dev/null || exit 1
failure_reason=accepted_promotion_failed
: >"$accepted"
cp -- "$raw" "$accepted"
cmp -s -- "$raw" "$accepted" || exit 1
write_disposition PASS complete
failed=0
trap - EXIT INT TERM
printf '%s\n' "$accepted"
