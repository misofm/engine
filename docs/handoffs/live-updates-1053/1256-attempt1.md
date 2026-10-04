Verdict: PASS

# #1256 attempt 1: adversarial verdict (Sol)

- Commit under review: `f02d09b24` (parent `a2a417c9a`), branch `codex/1053-live-updates`.
- Reviewed from the export `/tmp/claude-1002/v1256-a1` (deleted after review). Builds used
  `CARGO_TARGET_DIR=/tmp/claude-1002/vtarget-1053` (kept, as the rules say). Logs:
  `/tmp/claude-1002/v1256-a1-logs/`.
- I did not edit, build in or check out `/home/bl/misofm/wt-1053`.

## Summary

The slice does what D1-D5 ask, and what umbrella D4, D5, D7 and D8 need from it.

- **D1.** `prepare_runtime` (`crates/capi/src/runtime/compile.rs:445-456`) calls
  `prepare_host_runtime_with_live_lanes` with depth 16 (`LIVE_QUEUE_DEPTH`, `:36`) and
  `FADER_AND_MATRIX`. Both this entry and the lanes-free `prepare_host_runtime` prepare with
  `between_render_calls: false`, so delivery stays `Concurrent`. No fused pass is lost: the C ABI
  never had one.
- **D2.** Only `strip_controls` (as a boxed slice) and `track_count` are kept. The strip ID list
  and the empty vectors drop inside `prepare_runtime`, on the control thread. `ProviderEpoch`
  carries the `StripLanes` in `::current`, in `::candidate` and in the structural arm.
- **D3.** `host_core::strip_control_table_bytes` is a pure layout mirror. It is added to
  `epoch_rows`, and so to `epoch_retained`, `active_retained` and `largest`.
  `prepared_capi_resources` counts `normalized_model().strips()`, which covers tracks and submixes.
  Because `epoch_retained` carries the strip row, `validate_replacement_peak` (and #1257's D8
  admission) charge the prospective table.
- **D4.** The oracle replays the same call and keeps the same disposal. It has five owners, with
  `strips` dropped first, and the completeness and owner assertions include it. The double charge
  is recorded in the attempt record and was not removed, as the spec asks.
- **D5.** The docs paragraph is present and accurate.

**Realtime.**
- The render thread never drops a plan or a provider epoch. The producers live in `SessionState`
  on the control side.
- A reclaimed plan is dropped before its provider, in `synchronize_plan_epochs` on the control
  thread. Session and plan destruction are host control-thread calls.
- The rings are `Arc`s whose `Ring` drop only drops plain-data records.
- `audit capi`: 100,000 calls, 0 allocations, 0 deallocations, 0 locks, 0 syscalls.
- Its `pcm_digest` `ff6cdcb96cdcdad5` matches the pre-slice records (#1206, #1240, #1242 and
  #1243 verdicts). So the C entry point's rendered bits did not move on the nine-track EQ fixture.

**Acked-batch question.** Not applicable: this slice adds no admission and pushes nothing.

**Paths.** Every changed path is authorized. The builtins-compiler change is documentation only.
`tools/bench/src/console.rs` changes one sentence.

**Budget raise.** I accept the 19,200 -> 31,424 raise of `builtin_processor_payload_bytes` and
`builtin_retained_payload_bytes`.
- It is a ceiling, not an exactness assertion.
- The move is structural: two rings, a producer entry and a seal row per strip.
- The value is 28,521 + 10 %, rounded up to 64, the same policy as the earlier 19,200.
- The reason is in the `REFERENCE_BUDGETS` doc (`resource_lifecycle.rs:883-888`).
- No exactness assertion was loosened.

## Findings

### BLOCKER
None.

### MAJOR
None.

### MINOR

1. **No exact oracle runs a session with submix strips or routes into a submix.** So two
   plausible defects survive every capi test, and the attempt record misstates M3 and M4.
   - Where: `crates/capi/tests/resource_lifecycle.rs:780-785` (the oracle's fixture list), and
     `crates/capi/src/runtime/tests.rs:2141` (gate 1's sessions).
   - None of the oracle's three fixtures, and none of gate 1's sessions, has a submix.
     `two_track_three_submix_session` runs only through admission tests.
   - **O3 (mine).** I made `prepared_capi_resources` count `normalized_model().tracks` instead of
     `.strips()`. That under-charges every submix producer against D3's "tracks and submixes
     alike". The whole `cargo test -p capi` stays green.
   - **M3 (re-run).** `routes: true` stays green everywhere, for the same reason: no capi fixture
     has a route lane to attach.
   - **The record is wrong about M3 and M4.** It says they "show only as larger resource rows,
     which the budget test bounds". That is wrong both ways:
     - M4 (`effects: true`), my re-run, is caught exactly by
       `capi_retained_bytes_charge_every_byte_the_compile_retains` (left 387,119, right 258,135),
       by `tiny_control_frame_still_accounts_three_provider_counters_exactly` and by the budget
       test. The oracle's `host_half` spells `FADER_AND_MATRIX` independently, so any lane capi
       attaches beyond it is uncharged compile bytes.
     - M3 moves no row on any capi fixture, so no budget bounds it.
   - **My probe (temporary, reverted).** I added
     `fixtures/session/v1/console-sixty-four-track-sends.json` (64 tracks, 10 submixes, 202
     routes) to `observe_compile`, with routes and effects raised to 10,000, the graph, effect
     state, effect scratch, builtin and named-allocation caps raised to 4e9 and the capi cap to
     4e8. Then
     `assert_capi_retained_bytes_are_complete` and `assert_host_owners_are_charged`:
     - **Head:** green (capi 1,507,644 observed). The implementation is exact on submixes.
     - **O3:** red (left 1,507,644, right 1,506,236: ten 136-byte producers plus 48 ID bytes).
     - **M3:** red (left 1,672,922, right 1,507,644).
   - By inspection, `track_count: strip_controls.len()` also survives gate 1. Every gate-1 fixture
     has `strips.len() == tracks.len()`.
   - **Fix:**
     - Add a submix-routed session to the `capi_retained_bytes_charge_every_byte_the_compile_retains`
       loop: either that fixture with raised caps, or a routed variant of
       `two_track_three_submix_session`.
     - Optionally run gate 1 once on a submix session, so the tracks-then-submixes order and
       `track_count` are checked.
     - Correct the M3/M4 sentence in the attempt record.
   - It is MINOR, not MAJOR, because the code is correct today (the probe is exact). What is
     missing is regression cover. Earlier verdicts in this batch rated equivalent gaps MINOR
     (#1254 MINOR-1, #1255 MINOR-1). Apply it before #1257 lands: #1257 will push through these
     producers by strip index.

### NIT

1. **`largest` overstates the strip row** (`compile.rs:208`).
   - `epoch_rows` feeds `largest`, but the strip row is the slice plus every `track_id`. Those are
     separate allocations, so the largest single allocation is the slice alone.
   - The error is in the safe direction.
   - Fix: feed `Layout::array::<TrackControlProducer>(n)` to `largest`, or comment that it is
     conservative.
2. **`#[allow(dead_code)]`** (`control.rs:13`, `:28`).
   - Prefer `#[cfg_attr(not(test), expect(dead_code, reason = "#1257 pushes through these"))]`.
   - It then fails under `-D warnings` once #1257 reads the fields, so the allow cannot outlive
     its reason.
   - A plain `expect` would not work: the tests read the fields, and the expectation would go
     unfulfilled under `cfg(test)`.
3. **Wording** (`control.rs:9-11`).
   - "they outlive whichever of the plan and the epoch drops last" should say they outlive
     whichever drops first, and the last owner frees them.
4. **The double charge is not visible at the charge site** (`compile.rs:155-156`).
   - The comment says the rings are builtins' rows. It does not say that builtins'
     `add_vector_layout::<TrackControlProducer>` also charges the producer vector, which is D4's
     recorded double charge.
   - One clause there would stop a later reader from "fixing" one side blindly.

## Test value (one sentence per new or changed test)

- **`c_abi_plans_with_live_lanes_render_like_lanes_free_plans`** (gate 1): red if capi's live-lane
  preparation changes a rendered bit against the lanes-free host-core plan.
  - My M5 re-run (a `Mute` record pushed at preparation) went red at "parity 1 tracks at
    44100 Hz: block 0".
  - The existing `direct_and_c_render_match_*` compares capi with capi and cannot see it. The
    barrier test catches only a full mute.
  - It is also red if capi attaches the input lane: my M1 re-run went red at "no strip carries an
    input lane".
- **The `resource_lifecycle` oracle change** (`host_half`, `HostOwners.strips`, the five-owner
  sum, `strip_table_charge`, the double-live prospective strip term): red if capi keeps a
  producer table it does not charge exactly, or keeps any lane beyond fader/matrix on a fixture
  that has one.
  - My M2 re-run (producers dropped) went red: left 256,884, right 258,135.
  - My M1 re-run went red: left 266,559, right 258,135.
  - My M4 re-run went red.
  - Before this change no test charged a strip producer at all.
  - It does not cover submixes (MINOR 1).

## Mutations I ran

Each was applied in the export, run, and restored from a saved copy. The tree was diffed
pristine afterwards.

| # | Mutation | Result |
|---|---|---|
| M1 (re-run) | `strip_input: true` | red: gate 1 (`no strip carries an input lane`); the oracle, the tiny-frame oracle and budgets also red |
| M2 (mine, oracle side) | `controls` replaced by an empty slice | red: the oracle and the tiny-frame oracle |
| M3 (re-run) | `routes: true` | **green** in all 47 capi tests; red only with my submix probe (MINOR 1) |
| M4 (re-run) | `effects: true` | gate 1 green; the oracle, the tiny-frame oracle and budgets red |
| M5 (re-run) | `Mute` record pushed on strip 0 at preparation | red: gate 1 at block 0 (and the barrier test) |
| O3 (mine) | charge counts `tracks`, not `strips()` | **green** in all capi tests; red only with my submix probe (MINOR 1) |

## Gates I re-ran (export of `f02d09b24`, x86-64-v3)

| Gate | Result |
|---|---|
| 1, 2, 3a: `cargo test --locked -p capi` | pass: 36 lib (including gate 1) + 11 `resource_lifecycle` |
| 3b: the workspace test command, verbatim | pass: 117 result lines, 1,315 passed, 0 failed, 10 ignored |
| 4: `cargo build --locked --release -p audit -p bench -p capi -p session-validator` | pass |
| 4: `./target/release/audit capi` | 100,000 calls; allocations 0, deallocations 0, locks 0, syscalls 0, total_violations 0; `pcm_digest ff6cdcb96cdcdad5` (unchanged from pre-slice records) |
| 4: `check-capi-abi.sh` and `--self-test` | pass (x86_64, shared and static) |
| 4: `check-scalar-oracle-absent.py --native target/release/libcapi.so` | pass |
| 4: `cargo test --locked --release -p audit -p bench -p console-workload` | pass: 113 passed, 0 failed |
| 5: `cargo fmt --all -- --check` | pass |
| 5: host-core, realtime and workspace policy check + self-test | pass |
| batch: `check-ci-path-routing.py` | pass |
| Compiler warnings across every build above | none |

**Not re-run.**
- The workspace test build took the disk from 18G to 5.1G free. The rules say not to build below
  10G, so I did not run:
  - `cargo clippy ... -D warnings`;
  - `cargo doc` with `-D warnings`;
  - `check-cross-targets.sh` (the iOS `memset_pattern16` rule);
  - the worklet chain.
- The implementer records all four as passing. I found nothing that makes me doubt them:
  - The builds have no warnings.
  - The host-core addition is a pure function that host-web does not call.
  - The builtins-compiler change is documentation only.
- AArch64 NEON (`run-aarch64-tests.sh debug`) is CI-only, and I did not run it.

## Other checks

- **tail_kind and the report.** Gate 1 asserts latency, tail kind and tail samples equal to the
  lanes-free plan's.
  - It runs at four rates × {1, 10} tracks, on both the parity session (infinite tail) and the
    bare session (asserted `TAIL_FINITE` first).
  - Only an input lane could make the tail infinite, and the C ABI does not attach one.
- **Resource rows.** The capi row after the slice is 258,135, as recorded. The arithmetic checks:
  - The strip table is 9 × 136 + 27 = 1,251.
  - `ProviderEpoch` grows by 24 bytes in each of three charged slots: 72.
  - The O1 and O2 values in the record agree with my M2 output.
- **No new refusal class.** With these lanes, live-control preparation refuses only on resource
  caps (`builtin.prepared.control_set` is an internal seal). So the only new C ABI refusals are
  callers whose exact builtin or capi caps now fall short, which D5's docs paragraph tells them.
- **The duplicated test constant.** `LIVE_QUEUE_DEPTH` is restated in `resource_lifecycle.rs`.
  It acts as an independent pin: a capi depth change turns the oracle red rather than drifting
  silently.
