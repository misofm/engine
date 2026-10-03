# #1213 *Address submix strips in browser live commands*: Sol verdict, attempt 1

- Reviewed: `git diff 047a503d6 e4a6269fe` (14 files, +1608/-117) and the follow-up
  `git diff e4a6269fe c13ac5e1d` (A1e headless gate, one MUTATIONS row, spec record; 3 files, +44/-2).
  Branch `codex/batch-submix-k2`, worktree `/home/bl/misofm/wt-submix-k2`.
- Binding: `AGENTS.md` (realtime rules, test-value rule, the acked-batch question) and
  `.github/ISSUE_SPECS/1213-address-submix-strips-in-browser-live-commands.md` with amendment A1
  (A1a-A1e) and its Attempt 1 record.
- How I ran it:
  - I did not modify the worktree, the branch or GitHub. `gh issue view 1213`: OPEN, and the title
    matches the spec's H1.
  - I exported `c13ac5e1d` to `/tmp/claude-1002/v1213/src`, copied `sdk/node_modules`, and used my
    own `CARGO_TARGET_DIR` and `TMPDIR`. I ran mutations in place and restored them byte for byte.
    Before writing this, `diff -r` of the touched trees against a fresh archive was IDENTICAL.
  - My probes are in `submix-verdicts/1213-attempt1-verifier-scratch.rs`, with instructions for
    running them.

## Verdict: FAIL

There is **one MAJOR** finding (MAJOR-1): this slice newly admits bus subscriptions through
`observe()`, and the shipped host's `observe()` then returns a `frameSlot` that breaks its
documented contract. There are four MINOR findings and four NITs.

The engine work itself is correct. I found no index-safety, atomicity, solo or allocation defect.
Every gate I ran passes, and every test-value claim I checked reproduces. Attempt 2 is small:
- fix the documentation (no wire or behaviour change);
- add one harness assertion;
- add three cheap test pins (MINOR-1 to MINOR-3).

## Gates (all run from `c13ac5e1d`, x86_64 AVX2)

| Gate | Result |
|---|---|
| `cargo test -p host-web --lib` | 139 passed, 0 failed, 1 ignored. The 10 new #1213 tests are green |
| `cargo test -p host-core -p host-web --all-targets` with section 7's feature set (the two touched crates) | every binary green; host-web unittests 142 passed |
| `build-web-audioworklet.sh --named-twin` | rc 0. Shipped module `7d6c0a8b...0f90b`, 2 695 834 B, which matches the record's ARTIFACT CHANGED line |
| `check-web-audioworklet.sh <A> <B>/...named.wasm` | rc 0 |
| `check-browser-expected-resources.py --artifacts` | rc 0 (32 red mutations) |
| `test-web-audioworklet.sh` | rc 0 |
| `check-sdk-generated.sh`, `check-sdk-types.sh` | rc 0 |
| `check-sdk-headless.sh` | rc 0. 342 pass, 0 fail, including A1a, D5a and A1e |
| `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -D warnings`; `cargo doc -D warnings` | rc 0 |
| `check-/test-` policy for host-core, realtime and workspace | all rc 0 |

- **Not run:**
  - the full section 7 workspace test command (disk; the diff touches only host-core's `solo.rs`
    and host-web, and workspace clippy compiled every caller of the renamed API);
  - `run-aarch64-tests.sh` (no arm64 host; it does not cover host-web anyway).

## Mutations

### Required by the brief

| Mutation | Result |
|---|---|
| (b) Restore #1207's GR skip: `if track == u32::MAX \|\| track as usize >= ready.tracks.len()` in the poll fold | **Red**: `a_bus_compressor_reports_its_gain_reduction_in_the_bus_word` and `a_bus_limiter_can_be_the_designated_master` (A1c holds) |
| (c) Tracks-only selected read: drop the `.or_else(... ready.submixes ...)` in `observation_selection_for_address` | **Red**: `a_bus_compressor_reports_its_gain_reduction_in_the_bus_word` |
| D3 core: kind 4 uses the old inline `muted \|\| (any_solo && !solo(track))` | **Red**: `a_bus_can_be_unmuted_while_a_track_is_soloed` |
| SDK D5a: the classifier bounds by `live_control_track_count` only (headless re-run) | **Red**: capability-evals 8, `'unknownTrack'` vs `'unknownEffect'` |

### My own

| Mutation | Result |
|---|---|
| Coalescing slot `ready.tracks.len() + strip` (a T-vs-N band mismatch) | **Red**: gates 2b, 3 and 7. `push`'s `slot - tracks` underflows (a debug panic at `lib.rs:1698`) |
| P1: kind 4 reads `let lane = 0_usize` (ignores a `Right` selector) | **Survives**. See MINOR-2 |
| P2: `prepared_queue_address`'s Eq base spelled `ready.tracks.len() * 3` | **Survives**. See MINOR-1 |
| P3: the admission's EQ owner marker `queue_slot: ready.tracks.len() * 3 + effect` | **Survives**. See MINOR-1 |
| Worklet D5a reverted to `message.trackIndex >= this.trackCount` | **Survives** the committed harness. See MINOR-3 |

## What I verified holds

- **Index safety.**
  - Every band now spells `N = T + S`:
    - `queue_available`, `push` and `preflight_effect`;
    - `builtin_input_slot` and both branches of `prepared_queue_address`;
    - the generic guard;
    - the three builtin bands, the mute and solo slots and the coalescing slot;
    - the input-filter owner and both EQ owner markers;
    - companion validation, the push pass and the commit pass;
    - `queue_count` and `command_staging_count`.
  - Every per-strip table is built tracks-then-submixes from one strip order: `controls`
    (`strip_controls`), `input_filter_shadows`, the solo seeds, `rack_effects`, `effect_base` and
    `observation_present`.
  - No T-sized table remains that a strip index can reach:
    - `copy_input_filter_config` and `copy_eq_target_config` now resolve a bus;
    - `copy_live_control_track_id` stays track-only, which is correct: it is the track-ID query.
  - Out-of-range accesses all use `get`/`get_mut`, so they refuse rather than panic. The only
    unchecked arithmetic (`push`'s `slot - tracks`) is fed only by consistently computed slots.
  - Boundaries: index `T` and `T + S - 1` are exercised (gates 1-7, and gate 2c on both buses).
    `T + S` is refused by the same `>=` guard. That boundary is pinned at `T` by the existing
    `S = 0` refusal tests, and at `T + S` for the selected read, the boot word and the SDK
    classifier.
  - JS and SDK: the worklet has no array indexed by `trackIndex`. The SDK observation map already
    bounds by `strips.length`. The SDK and worklet classifiers both use `T + S`. The one JS
    mismatch is `frameSlot` (MAJOR-1).
- **Atomicity (the acked-batch question).** Every lowered record, every coalesced solo record and
  every prepared-owner target count lands in `command_wanted` before the room pass. The room pass
  checks every slot with the strip-aware `queue_available` before any push. A refusal rolls back
  the solo transaction, the input shadows and the EQ owners. No ack can precede a drop. Gates 4a
  and 4b pin this, and I confirmed both test-value claims by reading the code.
- **Solo and mute.**
  - host-web no longer composes the effective mute itself. The only remaining copies are in
    comments; kind 4 reads `ready.solo.effective_mute`.
  - Submixes are seeded `solo_safe: true` from their own fader mutes.
  - Kind 9 at a bus refuses `notSoloable`/`RESULT_INVALID_ARGUMENT` at its own wire index, before
    any state moves (gate 2c behind a valid record; the A1e headless test end to end).
  - Under solo, returns and buses stay audible (gate 2b, including the proof that `a` reaches the
    output only through them), and a bus can be unmuted (gate 3).
- **Gate 7.** Allocation-free admission and render on a bus batch: 0 allocations and 0 frees.
  The fold is in `poll_meters`, outside the measured closure. It is covered by #1209's existing
  `bus_meters_render_and_poll_without_allocating`.
- **`S = 0`.** `strip_count == tracks.len()`, so every slot is unchanged. No existing test was
  edited beyond the two A1d renames.

## Findings

### MAJOR-1: the shipped host's `observe()` returns a bus binding whose `frameSlot` points past `trackGrDb`

This slice is what makes `observe({ trackIndex: T + j })` admissible; before it, the record was
refused `unknownTrack`.

- **The defect.**
  - `miso-engine-v1-audio-worklet-host.js:1351` sets `frameSlot: subscription.trackIndex`, under
    the comment "one gain-reduction slot per track, so the slot is the track".
  - The shipped contract (`miso-engine-v1-audio-worklet-host.d.ts:672-673` and its SDK mirror) is
    "Index into `MisoMeterFrame.trackGrDb` this tap folds into".
  - For a bus, `frameSlot` is `T + j`, but `trackGrDb.length` is `T`. The bus's reading is at
    `submixGrDb[j]`.
- **Proof.** Scratch probe in the worklet-host harness, 2 tracks and 2 submixes: the bus
  subscription's binding is `{"trackIndex":2,...,"frameSlot":2}`. An app that follows the
  contract reads `trackGrDb[2] === undefined` for the bus.
- **Why MAJOR.** It is a wrong result on a public, documented surface, reachable today through
  the path this slice opened.
- **Fix** (doc and test only, P17 spellings kept):
  - Redefine `frameSlot` in the `.d.ts` and its byte mirror as the **strip index**: it indexes
    `trackGrDb` when it is below `trackCount`, and `submixGrDb[frameSlot - trackCount]`
    otherwise. Fix the `:1350` comment to match.
  - Pin it with one harness assertion: `observe()` at `trackIndex = trackIds.length` returns
    `frameSlot === trackIds.length`, and the doc'd mapping lands on `submixGrDb[0]`.
  - Fix the neighbouring `trackIndex` docs in the same pass, because agents read them as the
    addressing contract:
    - `MisoCommand.trackIndex` (`.d.ts:336`): "Index into the canonical track order";
    - `MisoSessionMap.tracks` (`:392`): "`trackIndex` indexes this";
    - `MisoObservationSubscription.trackIndex` (`:634`);
    - the `sessionMap()` doc (`:886`);
    - `MisoCommandReason.UnknownTrack` (`:289`): "not a track of the compiled session".

### MINOR-1: the prepared-EQ owner spellings have no `S > 0` test, and gate 1's test-value claim overreaches

- **What survives.** P2 and P3 above survive the whole committed suite. Gate 1 drives a
  compressor, so the prepared-owner EQ path never runs in a session with submixes. That path
  covers five D1 spellings: `prepared_queue_address` Eq, the admission marker, companion
  validation, the push pass and the commit pass.
- **Consequences.**
  - Gate 1's "red if ... any band spelling still uses the track count" is not true for those
    spellings.
  - This is the most common bus insert: a live EQ on a bus (and on any track in a session with
    buses).
- **Current code is correct.** My scratch probe `verifier_scratch_prepared_eq_on_a_bus_equals_the_track`:
  - setup: the gate 1 pair with a `miso.parametric-eq` insert; `stage_prepared_eq_parameter` and
    `submit_prepared_commands(1, 104)` at index `T` and at `0`;
  - result: green, bit-identical;
  - it is red under P2 and under P3.
- **Fix.** Commit that probe as a gate 1 arm.

### MINOR-2: kind 4's new lane derivation is unpinned for a `Right` selector

- **What changed.** `let lane = usize::from(matches!(lanes, BuiltinLaneSelector::Right))` is new
  logic: before D3 the effective value did not depend on the lane.
- **What survives.** P1 (`lane = 0`) survives every committed test. Under P1, a right-only mute
  with no solo engaged stages `Mute { Right, muted: false }`: an acknowledged mute that never
  mutes.
- **Why gate 1 cannot catch it.** Gate 1's paired design cannot see a lane bug common to tracks
  and buses. It also sends only a left mute and a both-lane unmute.
- **Fix.** Commit `verifier_scratch_right_lane_mute_on_a_bus_equals_booted_right_mute`:
  - setup: a live kind 4, channel 1, then channel 0, at the bus, against a host booted with that
    lane muted;
  - result: green, and red under P1.

### MINOR-3: the worklet D5a classifier is untested, but the harness can drive it

- **The record's claim is wrong.** It says the classifier has "no harness path". In fact,
  `testProcessor()`'s `makeProcessor()` builds the real processor over fake exports with 2 tracks
  and 1 submix, and `receiveEqTargetConfig` can be called directly.
- **Probe** (about 15 lines, in the scratch file):
  - setup: stub `eq_target_config_copy` to return 1;
  - expected: index 1 gives reason 4, index 2 (the bus) gives 4, the bus with rack 0 gives 3,
    and index 3 gives 2;
  - result: green on `c13ac5e1d`, red under the revert, while the committed harness stays rc 0.
- **Is the gap acceptable?** No, not for a D5a deliverable: the fix costs one block. The code
  itself is correct (`submixIds` is set in the same construction path as `trackCount`, so
  `.length` cannot throw).

### MINOR-4: Rust docs still call the index word a track index

- `WebObservationSelection.track_index` (`lib.rs:692`) and `WebObservationResult.track_index`
  (`:716`) say "Canonical normalized track index". A1a made the first of these a strip index.
- `admit_commands`' doc (`:4303-4306`) still says "two *more* per track" and
  `2 * track_count`.

### NITs

1. Gate 7's doc claims it turns red "if ... the bus fold allocates", but `poll_meters` is outside
   the measured closure. Either include `poll_meters()` in `measured` or drop the clause (#1209's
   test covers the fold).
2. The name `a_bus_record_behind_a_bad_track_record_is_never_pushed` reads backwards: the bus
   record is at index 0, *ahead of* the bad one. Suggest `..._ahead_of_a_bad_track_record_...`.
3. **Test growth (about 1,220 lines): justified, verbose.**
   - Why it is not ceremony: each of the 10 tests maps to a spec gate and has a recorded,
     reproducible red mutation (I re-ran four). None pins bytes, digests or prose. The new
     helpers (splitmix per-lane feeds, the route-ID-ordered two-rounding bus sum, the paired
     builders) are what VERIFY-2 M13 requires.
   - Why it is verbose: about 300 lines are rustfmt expansions of 10-argument `stage_command`
     calls, such as gate 1's 13-tuple table. A small typed command builder would remove them.
     That is optional.
4. Record accuracy: besides MINOR-3's wrong "no harness path", gate 1's test-value sentence
   overclaims (MINOR-1). Correct both in the attempt 2 record.

## What attempt 2 must contain

1. **MAJOR-1:** the `frameSlot` and `trackIndex` docs in the `.d.ts` and its byte mirror, the host
   JS comment, and one `observe()` bus-binding assertion in `test-web-audioworklet.mjs`.
2. **MINOR-1 to MINOR-3:** commit the three probes from the scratch file, as tests with their
   mutation rows (P2/P3, P1, the worklet revert).
3. **MINOR-4 and the NITs:** optional, but cheap.

No engine logic needs to change. With docs and tests only, the shipped Wasm module's bytes should
not move from `7d6c0a8b...0f90b`.
