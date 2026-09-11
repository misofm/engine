# Repair metering timing fixture final-chunk admission

## Approved execution scope — 2026-09-11

Astra XHIGH approves the minimum slice at base `e51011a7`. Astra LOW implements;
Astra XHIGH reviews the source and freezes the fixture/validator before timing.
Only `hosts/host-web/src/tests.rs` and this spec are implementation paths.
Share finite-source feeding between the timing caller and a two-block non-timed
probe; derive the boundary marker from `block + 1 == source_blocks`. Preserve
unrelated `feed_and_render` callers. Reject an early marker, accept an ordinary
chunk and render to drain the ring, reject an unmarked final chunk, then accept
and render the correctly marked final chunk on the same host. Run that probe
from a focused regression and the existing preflight before timed helpers.

Keep all frozen workload constants, serializer/persistence, output overwrite
refusal and payload/window comparisons. Root checkpoints exact green paths and
pushes promptly. No timed workload until independent freeze of pushed source,
exact command/environment/output and validator. Exactly one new invocation with
one warmup per five modes and two measured rounds; no retries or tuning. If that
runner fails, preserve evidence and stop/rescope under the tighter tooling rule.
No production changes, new framework, compiler captures or artifact repinning.
Reviewed-head PR/main CI, upstream evidence, verified closure and clean worktree
removal remain required. At most two active issues (#213 and #522).

## Problem and evidence

The sole descriptive timing invocation for #519/#520 at frozen `11cb3c2e` failed during poll-mode warmup before measured rounds. `timed_poll_mode` prepares exactly 8192 quanta of source content, but its shared `feed_and_render` helper submits every chunk with `end_of_region=false`. The final chunk ends exactly at the prepared region boundary, so `crates/host-core/src/source.rs:169` correctly returns `EndOfRegionMismatch`; the test fails at `hosts/host-web/src/tests.rs:1748` with result 1 instead of 0.

The failed log and JSONL contain the header only; no timing figures are available. The failed log and header are preserved in `docs/evidence/metering-519-520/timing-failed.log` and `timing-failed.jsonl`. Do not retry that invocation or claim a measured speedup. #519/#520's independently tested work avoidance and numerical contracts are not weakened by this tooling failure.

## Smallest useful correction

Correct the timing fixture's final source chunk to mark the actual region boundary, without changing engine admission semantics or general render behavior. Add a small non-timed fixture admission check for an ordinary chunk and the final chunk; the latter must be accepted with its end marker and rejected without it. Extend preflight to exercise this finite fixture boundary setup without launching the timed workload.

Keep the existing frozen 9-stream/128-frame/32-block-period/256-window comparison, shared JSONL serializer, output creation/overwrite refusal, immediate record persistence, and peak/poll payload checks. No generic benchmark framework, artifact promotion subsystem, DSP changes, new performance target or gate weakening.

## Acceptance and one new invocation

- Focused non-timed boundary regression and serialization/persistence preflight pass before any timed work.
- Reviewer freezes the repaired fixture and validator. Execute one new successor-owned invocation, exactly one warmup per mode and two measured rounds, with no retries or tuning.
- Preserve each measured mode immediately; verify comparable peak hashes, poll hashes and emitted window counts before reporting results.
- Report compiler/target, exact source revision, workload, raw records, units and limitations. Values describe isolated accumulator/Rust poll work, including clock/test-counter overhead; they do not establish end-to-end browser callback performance.
- If the repaired runner fails, retain raw evidence and stop under the bounded tooling-attempt rule. Do not block already qualified engine capabilities or weaken their correctness gates.


## Attempt 1 fixture correction checkpoint

The timing poll feeder now uses a finite-source helper that derives the end marker
from `block + 1 == source_blocks`. The two-block probe uses this same helper for
accepted chunks on one host: it first rejects an early marker, accepts/renders the
ordinary chunk to drain the one-quantum ring, rejects an unmarked final chunk, then
accepts/renders the marked final chunk. A focused normal test and the existing
preflight both call the probe before any timed helper is reached. General
`feed_and_render` callers and the frozen timing constants, serialization,
persistence, overwrite refusal and payload/window comparisons remain unchanged.

No timed invocation belongs to this checkpoint. Non-timed evidence and independent
source/validator freeze precede the sole new timing invocation.

## Attempt 1 reviewed one-shot result

Astra XHIGH approved source/preflight/command/validator freeze at pushed clean
`6a4e45c93d87378f73be529bc3a2ed3c847c74ef`. Root ran the approved invocation
exactly once; exit zero, one test passed. Independent post-run review PASS checks
all 11 records, exact keys/types/order, 8192 accumulator blocks, paired payload
hashes, and 256 windows in every poll row. No timed retry or tuning occurred.

Compiler: rustc 1.97.1 (8bab26f4f68e0e26f0bb7960be334d5b520ea452), native
`x86_64-unknown-linux-gnu`, release, repository `+avx2,+fma` flags. Exact command:

```sh
CARGO_TARGET_DIR=/tmp/issue522-attempt1-target \
MISO_ENGINE_METER_TIMING_MODE=run \
MISO_ENGINE_METER_TIMING_OUTPUT=/tmp/issue522-timing-qaqo72bn/timing.jsonl \
cargo test --locked --release -p host-web --lib -- --ignored --exact tests::selective_meter_and_readiness_descriptive_timing
```

Run directory was `/home/bl/misofm/engine-meter-timing-final-chunk`. The recorder
required the exact clean revision and fresh output/sentinel/log paths. Full argv,
PATH, inherited build environment, source/runner/freeze hashes, exit and raw log
are retained at `/tmp/issue522-timing-qaqo72bn`; independent review records are
`xhigh-freeze.json` and `xhigh-post-run.json`. Non-timed release boundary regression
and serialization/persistence/overwrite preflight each passed one test before the
freeze (`/tmp/issue522-attempt1-preflight`).

Measured totals for the frozen 8192 blocks, in milliseconds:

| Mode | Round 0 | Round 1 |
| --- | ---: | ---: |
| Accumulator disabled | 0.094645 | 0.090446 |
| Sample peak | 17.835792 | 17.863036 |
| Full metrics | 41.247948 | 41.261625 |
| Legacy scan poll | 0.469334 | 0.457442 |
| Readiness poll | 0.266380 | 0.266463 |

These are isolated accumulator/Rust poll totals, including clock/test-counter
overhead; they do not establish end-to-end browser callback performance or a new
release budget. Both rounds preserve peak/full payload identity and legacy/ready
poll payload identity; track-pop attempts are 73728 versus 2304, with zero effect
scans. The workload remains nine streams, 128 frames, period 32, 256 windows,
one warmup per mode and two measured rounds. Earlier #519/#520 failed evidence
is unchanged and no result is attributed to that failed invocation.

Raw persisted output (1664 bytes; SHA-256
`5307dc0a16cb827fb60127c71ab5abacf34fe39e3873773216fc1d02cd13f186`):

```jsonl
{"kind":"header","label":"isolated metering ns for 9 streams; poll-only callback ns","measured_rounds":2,"period_blocks":32,"quantum":128,"schema":1,"warmups":1,"windows":256}
{"blocks":8192,"elapsed_ns":94645,"kind":"accumulator","mode":"disabled","payload_hash":14695981039346656037,"round":0,"schema":1}
{"blocks":8192,"elapsed_ns":17835792,"kind":"accumulator","mode":"peak","payload_hash":4028661150835562277,"round":0,"schema":1}
{"blocks":8192,"elapsed_ns":41247948,"kind":"accumulator","mode":"full","payload_hash":4028661150835562277,"round":0,"schema":1}
{"effect_scans":0,"elapsed_ns":469334,"emitted_windows":256,"kind":"poll","mode":"legacy_scan","payload_hash":8199157996960894757,"round":0,"schema":1,"track_pop_attempts":73728}
{"effect_scans":0,"elapsed_ns":266380,"emitted_windows":256,"kind":"poll","mode":"readiness","payload_hash":8199157996960894757,"round":0,"schema":1,"track_pop_attempts":2304}
{"blocks":8192,"elapsed_ns":90446,"kind":"accumulator","mode":"disabled","payload_hash":14695981039346656037,"round":1,"schema":1}
{"blocks":8192,"elapsed_ns":17863036,"kind":"accumulator","mode":"peak","payload_hash":4028661150835562277,"round":1,"schema":1}
{"blocks":8192,"elapsed_ns":41261625,"kind":"accumulator","mode":"full","payload_hash":4028661150835562277,"round":1,"schema":1}
{"effect_scans":0,"elapsed_ns":457442,"emitted_windows":256,"kind":"poll","mode":"legacy_scan","payload_hash":8199157996960894757,"round":1,"schema":1,"track_pop_attempts":73728}
{"effect_scans":0,"elapsed_ns":266463,"emitted_windows":256,"kind":"poll","mode":"readiness","payload_hash":8199157996960894757,"round":1,"schema":1,"track_pop_attempts":2304}
```

Integration review finds main `49b59e5a` changes only #213 corpus/spec paths;
the fixture correction applies independently. Timings belong specifically to
`6a4e45c9`. Final exact-head review and required PR/main CI remain delivery gates.
