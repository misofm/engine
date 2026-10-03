# #1242 attempt 2 verdict: Apply VCA offsets and mutes at preparation

**Verdict: PASS.** There is no BLOCKER and no MAJOR. MAJOR-1 is fixed for every lane combination I
could construct, and MINOR-1 (the submix seeding) is closed. Attempts 1 and 2, judged together,
meet the spec and umbrella rule 5 ("mute wins"). A session without VCAs stages exactly the records
it staged at `4ce7767fc`. There is one MINOR, to fix before close: two plausible defects in the new
split arm pass every committed test. Two verified one-hunk test patches close it. There is also one
optional NIT.

- **Implementation:** `d5dce7b05` on parent `4ce7767fc` (#1243), branch `codex/batch-vca`.
  Attempt 1 is `250b72e94` (verdict `1242-attempt1.md`).
- **Review copies:** `git archive` exports of `d5dce7b05` (gates), a second copy of it (probes and
  mutations) and `4ce7767fc` (the no-VCA comparison), all under `/tmp/claude-1002/v1242b/`, deleted
  after review. I never touched the worktree, which the #1244 implementer is editing.
- **Host:** x86-64-v3 AVX2, 32 cores.
- **Probes:** `1242-attempt2-verifier-scratch.rs`, next to this file.

## The fix

When a `Both` kind 4's effective lanes differ, the `COMMAND_MUTE` arm stages one `Left` and one
`Right` record, each with its own lane's effective mute, and it calls `record_emitted` once per
lane. Otherwise it stages the single record it staged before, from the covered lane (`Right` reads
lane 1, while `Left` and `Both` read lane 0).

- Only a VCA can make the lanes differ. After `set_user_mute(Both)` the user mute is equal on both
  lanes, and the solo term is per strip. So the split is unreachable without a VCA, and the
  rewritten comment says exactly that.
- **Budget.** Each wire record still lowers to at most 2 entries, so `command_decoded`'s
  `2 * MAXIMUM_COMMAND_RECORDS + 2 * strip_count + route_count` still holds.
- **Room and refusal.** `command_wanted` counts both records, so the room check stays
  all-or-nothing. Both records carry the wire `index`, so a refusal still names the command.
- **Follow-ramp lookup.** It matches `Left` against `changed[0]` and `Right` against `changed[1]`.
  The split records therefore ramp a following send exactly as the reference's two wire records
  do.
- **Scope.** The C ABI has no live mute producer: the only producer of `TrackFaderRecord::Mute`
  outside compilers and tests is host-web. Rule 6 is still read only at preparation (#1053 has not
  landed), so the defect had no C ABI twin.

## Gates re-run on `d5dce7b05` (all exit 0)

- **test-debug-a** (the spec's gate 10 `cargo test` command, `--no-fail-fast`): 1,267 passed,
  0 failed and 9 ignored, over 114 binaries. This matches the attempt record.
- **host-web and host-core** with test support: 374 passed, 0 failed and 3 ignored, over 29
  binaries. The host-web lib alone is 169 passed and 1 ignored.
- **Hygiene:** `cargo fmt --all -- --check`, clippy `--workspace --all-targets --all-features -D
  warnings`, and `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`.
- **Policy pairs:** the session, builtins, graph, host-core, realtime and workspace checks and their
  self-tests. `test-workspace-policy.sh` prints the same two "directed fault unexpectedly passed"
  lines (`mutant-population` and `isa-build-late`) on the `4ce7767fc` export, and exits 0 on both.
- **Cross-targets:** `check-cross-targets.sh` reports PASS. host-core is still at its 4-call
  `memset_pattern16` ceiling, and there is no new row.
- **Worklet chain:**
  - a fresh `build-web-audioworklet.sh --named-twin B A` (shipped module `e9d195fe…`);
  - `check-web-audioworklet.sh A B/…named.wasm`, which includes the trap audit of the
    `command_submit` closure that holds the new arm;
  - `check-browser-expected-resources.py --artifacts A`, with no re-pin;
  - `test-web-audioworklet.sh A`;
  - `check-sdk-headless.sh A`: 357 pass, 0 fail.
- **`run-aarch64-tests.sh debug`:** waits for CI's `aarch64-debug` at the batch push, because this
  host is x86-64.
- **Gate 8:** not re-run, and the skip is justified. The only code change is host-web's kind 4
  arm, and no gate 8 tool drives live admission:
  - `audit`, `bench` and `console-workload` do not depend on host-web.
  - `session-validator` depends on host-web, but never submits commands.
  - `parameter-metadata`'s `submit_commands` tests are in test-debug-a, which is green.

## MAJOR-1: fixed for every lane combination

- **P1** (`a_both_lane_unmute_keeps_a_one_lane_vca_mute`): green.
- **P2** (`a_browser_vca_renders_as_its_effective_faders_under_solo_and_mute`): green at 48 seeds
  (as committed), and at **2,000 seeds** with the loop bound read from an env var.
- **My extended differential** (`sol_probe_p2_extended_with_ramps`): **1,000 seeds**, green.
  - **Setup:**
    - up to 6 VCAs over 4 levels, biased toward one-lane mutes, with random own faders and mutes;
    - batches of 1 to 6 records mixing solo toggles and kind 4 on every channel and every strip;
    - **non-zero smoothing** drawn from {0, 1, 5, 64, 129, 256};
    - 10 batches per seed, with 3 compared blocks after each batch so that every ramp is
      rendered.
  - **Checks after every batch:**
    - the live host's emitted mute equals its effective mute on every strip and lane;
    - the effective mute, the emitted mute and every follow mirror equal the written-directly
      reference's;
    - every live send's `source_lane_muted` equals its source strip's effective mute.
  - **Reach:** 990 split `Both` commands (693 of them ramped), 609 batches mixing a solo and a split
    kind 4, 3,785 single-lane un-mutes of a VCA-muted lane, and 2,916 un-mutes of a VCA-muted
    submix.
- **My exhaustive probe** (`sol_probe_exhaustive_lane_combinations`): **2,304 cases**, all green.
  - **Dimensions:**
    - the VCA mute (L, R, both or none);
    - the member, either track `drums` (its follow send `drums-verb`) or submix `room`, to which I
      added a following bus-to-bus send `room-verb`;
    - the member's own mute (4 combinations);
    - solo (none, the member's own track, or another track);
    - the kind 4 channel (0, 1 or 2) and value;
    - smoothing 0 or 64;
    - the solo in its own batch or in the kind 4's batch.
  - **Sequence:** each case runs solo on, then the kind 4, then solo off together with a `Both`
    kind 4 of the opposite value. After every batch it runs the full mirror check and renders 2
    compared blocks.
  - **End check:** every VCA-muted lane is still effective-muted and emitted-muted.
- **My deterministic mixed batches** (`sol_probe_mixed_batch_split_and_single_lanes`): one
  submission carries a solo on the member, a split `Both` un-mute on `drums`, a split `Both`
  un-mute on submix `room` and a second solo. Later batches add single-lane commands on the
  VCA-muted lane and the open lane, then un-solo, at smoothing 64, 7 and 200. Green against the
  written-directly reference at every block.
- **Rollback** (`sol_probe_split_at_queue_depth_one`): at queue depth 1, a split is refused whole
  with `RESULT_BACKPRESSURE`, and the user and emitted mirrors are left as they were.

## No change without a VCA (proved against `4ce7767fc`)

I instrumented both exports identically. A `#[cfg(test)]` thread-local logs every staged entry
(queue slot, wire index, the full `TrackFaderRecord` or `RouteControlRecord`) and the whole
`command_wanted` vector after the follow pass. Both exports then ran the same probe:

- **Session and commands:** #1224's follow session with `vcas: []` and random own faders and mutes,
  then 14 random batches of 1 to 6 solo or kind 4 records on every channel, at smoothing
  {0, 1, 3, 64, 128, 300}.
- **Recorded per seed:** the FNV hash of every rendered block's bits, the final emitted and
  effective mirrors, the follow mirrors and every result code.
- **Result:** at 400 seeds the two outputs are **byte-identical** (`cmp`, 1,631,297 bytes):
  - 17,980 staged records, among them 5,394 `Both` mutes and 2,496 follow records;
  - all 5,600 batches admitted.

## MINOR-1 (attempt 1): closed

The submix seeding is defended. My M2 (submixes seeded `vca_mute: [false; 2]`) is red in the
committed P2, as the attempt record says.

## NIT skips: justified

- **NIT-1 (no `try_reserve`).** `compile_ready` calls `prepare_host_runtime_*`, which runs the
  builtins compiler's composition (seal and lowering) infallibly and at the same size. It does so
  before the seeding line, so a fallible call there would guard nothing new.
- **NIT-2 (four composition passes).** Fixing it needs a `CompiledSession` field outside the
  authorized paths, and the verdict asked not to restructure it.

## Deviation accepted

The three `hosts/host-web/MUTATIONS.md` rows fall outside the authorized paths. That file is
host-web's existing mutation ledger, the K3 follow-ups append to it the same way, and each row
matches a mutation I reproduced.

## MINOR-1 (this verdict): two plausible defects in the split arm pass every committed test

Both committed tests run at smoothing 0, and neither counts the records a non-split `Both` stages.

- **M3: the split records drop the command's window** (`smoothing_samples: 0`).
  - Effect: an un-mute that splits hard-switches the open lane and its follow send, an audible
    click.
  - Every committed test stays green. My extended differential, exhaustive probe and mixed-batch
    probe are red.
- **M6: every `Both` command splits** (the `&& left != right` guard is dropped).
  - Effect: every both-lanes mute in every session, VCA or not, stages two records. That doubles
    fader-queue use, and at `live_control_command_queue_records = 1` it refuses every
    both-lanes mute with backpressure.
  - The render does not change, so every committed test and every probe of mine stays green. Only
    the record comparison above sees it, and that is PR evidence, not a test.

**Fix, verified in the scratch copy** (the diff is at the top of the scratch file):

1. **P2: draw smoothing.** Use `let smoothing = [0_u32, 64][draw.below(2) as usize];` for each
   record, solos included, on both hosts. Update the doc comment's "all at smoothing 0".
   - With it, P2 is green at 48 seeds and at 1,500 seeds (43 s), and red on M3, M1, M2 and M4.
   - The written-directly reference receives the same records, so a ramped re-mute's `-0.0` is
     identical on both hosts. The smoothing-0 caveat of gate 5 does not apply here.
2. **P1: count the records.** Stage a second `channel` 2 un-mute on `bass` (strip 0, no VCA) in the
   same submission. Then assert that `follow_fader_room` dropped by 2 on `drums` and by 1 on
   `bass` before the render. P1 stays green and turns red on M6.

## NIT (optional)

The split stages two records into one fader queue, so a both-lanes kind 4 on a strip with a
one-lane VCA mute needs two slots. At queue depth 1 it is always refused with backpressure. It
rolls back cleanly, and a caller can send `Left` and `Right` separately. This matches the existing
two-record lowerings (a per-lane effect parameter on `channel` 2, and the solo pass's per-strip
pair), but nothing documents it for kind 4. One sentence in `BUILTINS_AND_METERING_V1.md`'s "VCA
groups" paragraph would cover it.

For information: on the VCA-muted lane, the split re-stages `muted: true`. That follows the
established rule that "an explicit kind 4 always stages a record". It is also what the
written-directly reference stages, so it is not a finding.

## Test value (one sentence each)

- `a_both_lane_unmute_keeps_a_one_lane_vca_mute`: turns red if a `Both` kind 4 stages one record
  from one lane's effective mute (M1, M7) or mirrors a split lane wrongly (M5).
- `a_browser_vca_renders_as_its_effective_faders_under_solo_and_mute`: turns red if a `Both` kind 4
  stages one lane's value for both (M1, M7), drops one of the two split records (M4), or seeds a
  submix's VCA mute as open (M2).

## My mutations (applied to the scratch copy, full host-web lib suite run, reverted)

| # | Mutation | Committed tests red | My probes |
|---|---|---|---|
| M1 | attempt-1 arm restored (one record from the first covered lane) | P1, P2 | red |
| M2 | submixes seeded `vca_mute: [false; 2]` | P2 | red |
| M3 | split records staged with `smoothing_samples: 0` | **none** | red (extended, exhaustive, mixed) |
| M4 | split stages both records but `produced = 1` (the `Right` record is never pushed) | P2 | red |
| M5 | split mirrors `record_emitted(track, lane, !muted)` | P1 | red |
| M6 | every `Both` command splits (guard `left != right` dropped) | **none** | green (only the record log sees it) |
| M7 | split `Right` record carries the left lane's value | P1, P2 | red |

My instrumentation itself (it allocates in admission) turned all six of host-web's
admission-allocation tests red. That shows those tests guard this function too.
