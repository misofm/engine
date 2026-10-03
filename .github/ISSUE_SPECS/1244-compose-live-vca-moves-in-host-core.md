# Compose live VCA moves in host-core

Slice V5 of *VCA groups* (#1239). It is in the VCA batch, after *Apply VCA offsets and mutes at
preparation* (#1242). It is the host-core half of a live VCA, as *Produce live send records from
host-core* (#1221) was for live sends: a control-plane state that every host can own, here with
unit tests only. The browser wires it into admission in *Ride VCA groups live in the browser*
(#1245), and the C ABI in *Deliver value-only VCA edits to the running C ABI plan* (#1247).

The design record cited below (`DESIGN`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

host-core can recompute, live, what a VCA move or a member's own fader move means for every member
lane, exactly as preparation would compute it from the edited session:

- the member fader dB the render plane must now be told, only for lanes whose effective value
  changed;
- each strip's new VCA mute term, fed into the one strip-mute owner, whose existing delta then
  emits only the lanes whose effective mute changed, and whose effective mute the existing
  follow-mute pass reads;
- all of it under a transaction shadow, so a refused submission leaves no trace.

No render code and no host wiring change here.

## Context (verified on `8c6268967`)

- **After #1242** (`crates/session`, re-exported):
  - `session::vca_effective_db(member_db, offsets_db) -> f32`: the one composition (sum in `f64`,
    member first then each offset in the order given, clamp `[-144, 24]` in `f64`, round once to
    `f32`; no offset returns `member_db` bit for bit);
  - `SessionModel::vca_reach(&self) -> Vec<Vec<usize>>`: per strip in `strips()` order, the
    indices into `vcas` of every reaching VCA, each once, sorted by VCA ID;
  - `SessionModel::effective_strip_faders(&self) -> Vec<EffectiveStripFader { db, mute, vca_mute }>`;
  - `host_core::StripMuteSeed.vca_mute` and `LiveControlSoloState`'s per-strip `vca_mute`, composed
    as `user_mute || vca_mute || (any_solo && !solo_safe && !solo)`, with no setter yet.
- **The strip-mute owner** (`crates/host-core/src/solo.rs`, after #1242): shadow, `commit` and
  `rollback` (`:303-333` at `8c6268967`), `record_emitted` (`:270-281`) and `strip_delta`
  (`:284-301`), which yields at most two records, one `Both` when both lanes change to one value.
  "Never emit a redundant record" (`:46-60`) is a correctness rule: a retarget of a settled lane
  re-enters the ramp kernel and can turn an exact `+0.0` into `-0.0`.
- **The follow composition** (`crates/host-core/src/live_route_state.rs:232-258`,
  `LiveRouteMuteFollow::delta`) reads any `effective_mute(strip, lane)` function, so once the strip
  state composes `vca_mute`, a VCA mute flows to following sends with no change there.
- **The record a member gets** is `builtins_compiler::TrackFaderRecord::FaderDb { lanes, db,
  smoothing_samples }` (`crates/builtins-compiler/src/lib.rs:128-150`); a fader record is
  downstream of the mono-collapse seam, so a per-lane record does not retire a collapse (compare
  `TrackInputRecord`'s note, `:159-174`).
- **The mirror precedent** is `LiveRouteState` (`live_route_state.rs:67-230`): arrays allocated at
  preparation with `try_reserve_exact`, an `empty()` for a plan without routes, `update` taking the
  shadow on the first mutation, `commit`, `rollback`.
- **Indices.** A strip index is the `strips()` position of the normalized model (tracks, then
  submixes), the index every live-control queue, the solo state and a command record use. A VCA
  index is the position in the normalized model's `vcas`, which is canonical VCA-ID order.
- **Allocation counters**: `bench_support::alloc::{assert_installed, current_thread_counters,
  current_thread_delta_since}`, as `crates/host-core/tests/live_routes.rs:16`, `:978-990` use them.
- **The browser's callgraph rule** (`scripts/check-web-audioworklet-callgraph.py`, run by
  `check-web-audioworklet.sh` over `miso_engine_web_v1_command_submit --allocation-only`) refuses
  any reachable function whose name contains `free` (#1222 deviation 1).

## Decisions frozen for this slice

- **D1. `LiveVcaState`** (a new `vca` module, re-exported from `lib.rs`):

  ```rust
  pub type LiveVcaFaderDelta = [Option<(BuiltinLaneSelector, f32)>; 2];
  pub struct LiveVcaState { /* see below */ }
  impl LiveVcaState {
      /// From a normalized model. A model with no VCA returns `Self::empty()` and allocates nothing.
      pub fn try_new(model: &SessionModel) -> Result<Self, TryReserveError>;
      pub fn empty() -> Self;
      pub fn vca_count(&self) -> usize;
      /// Whether any VCA reaches this strip.
      pub fn reaches(&self, strip: usize) -> bool;
      /// The strips this VCA reaches, ascending (the inverse of the reach table, built once).
      pub fn reached_by(&self, vca: usize) -> &[usize];
      /// Strips some VCA reaches (the browser sizes its staging by it, #1245).
      pub fn reached_strip_count(&self) -> usize;
      pub fn set_vca_db(&mut self, vca: usize, lanes: BuiltinLaneSelector, db: f32) -> bool;
      pub fn set_vca_mute(&mut self, vca: usize, lanes: BuiltinLaneSelector, muted: bool) -> bool;
      /// A reached strip's own fader value; `false`, changing nothing, for a strip no VCA reaches.
      pub fn set_member_db(&mut self, strip: usize, lanes: BuiltinLaneSelector, db: f32) -> bool;
      /// `vca_effective_db(own, reach offsets in ascending VCA-ID order)` of a reached strip.
      pub fn effective_db(&self, strip: usize, lane: usize) -> f32;
      pub fn vca_mute(&self, strip: usize) -> [bool; 2];
      /// The fader records a reached strip still owes: lanes whose `effective_db` differs (`!=`)
      /// from what the render plane was last told; one `Both` when both change to one value.
      pub fn fader_delta(&self, strip: usize) -> LiveVcaFaderDelta;
      pub fn record_emitted_db(&mut self, strip: usize, lanes: BuiltinLaneSelector, db: f32);
      pub fn transaction_open(&self) -> bool;
      pub fn commit(&mut self);
      pub fn rollback(&mut self);
      /// Every byte `try_new` reserved (tables, mirrors and shadow); 0 for `empty()`. A host
      /// charges it as retained bridge or C ABI state (#1245, #1247).
      pub fn retained_bytes(&self) -> u64;
      /// The largest single allocation among them; 0 for `empty()`.
      pub fn largest_allocation_bytes(&self) -> u64;
  }
  ```

  - It keeps, per VCA, `db: [f32; 2]` and `mute: [bool; 2]`; per strip, the own `db: [f32; 2]`
    and the emitted effective `db: [f32; 2]` (seeded from `effective_strip_faders()`, which is what
    the prepared fader bakes); and the reach as flattened, preparation-time tables built from
    `vca_reach()`: per strip, its VCAs (offsets read in that order, so the sum order is the session
    helper's), and per VCA, its strips (`reached_by`).
  - It never stores an effective value as the member's own: a VCA that clamps a member and is then
    restored returns the member to its own balance.
  - Every array, the shadow included, is reserved in `try_new`; nothing afterwards allocates.
  - It never calls `session::vca_effective_db`'s arithmetic by another spelling.
- **D2. The strip-mute owner's setter.** `LiveControlSoloState::set_vca_mute(strip, lanes: [bool; 2])
  -> bool` sets the strip's VCA mute term; `false`, changing nothing, for an unknown strip. The
  owner gains a `vca_mute_shadow`, reserved in `try_new` (`solo.rs:142-162`), copied in `shadow()`
  and restored in `rollback()` (`:303-333`) with the other arrays, so the term is transactional like
  the user mute. Nothing else in `effective_mute` or `strip_delta` changes.
- **D3. The intended use** (wired by #1245 and #1247): a VCA dB move calls `set_vca_db`, then each
  reached strip's `fader_delta` is staged and `record_emitted_db`'d; a VCA mute move calls
  `set_vca_mute`, then, for each strip in `reached_by(vca)`, the strip-mute owner's `set_vca_mute`
  with `vca_mute(strip)`; the owner's `strip_delta` and the follow pass then emit the changes; a member's own fader move
  calls `set_member_db`, then stages `effective_db`.

## Deliverables

1. D1 in `crates/host-core/src/vca.rs` (or the name the crate's style prefers), re-exported.
2. D2 in `crates/host-core/src/solo.rs`.
3. Unit and integration tests (gates 1-6), and `crates/host-core/tests/MUTATIONS.md` rows for each
   red mutation.

## Authorized paths

- `crates/host-core/src/{lib.rs,solo.rs}` and one new module
- `crates/host-core/tests/` (one new test file) and `crates/host-core/tests/MUTATIONS.md`
- this spec

## Non-goals

- No host wiring: no command kind, no admission, no C ABI path (#1245, #1247).
- No change to `LiveRouteState` or `LiveRouteMuteFollow`.
- No VCA solo, no send trim and no automation.

## Hazards

- **One composition.** A second spelling of the sum, its order or its clamp would let a live move
  land on other bits than a fresh plan (gate 1).
- **Redundant records move bits.** `fader_delta` must yield nothing for an unchanged lane, including
  a member clamped before and after the move.
- **Names.** No method reachable from the browser's command submission may contain `free` (the
  callgraph rule above).
- **Allocation.** The setters and deltas run on the browser's audio thread at admission.
- **The iOS memset rule.** `check-cross-targets.sh` (`:121-139`) counts `bl _memset_pattern16` per
  product crate against `scripts/lib/aarch64-known-defects.py`; host-core's ceiling is 4. New code
  stores no splatted non-zero constant to memory (fill with a loop or from a seeded array).

## Objective gates

1. **A live recompute equals preparation.** New host-core test: random VCA forests (depth up to 4,
   up to 16 members, overlapping memberships, diamonds, tracks and submixes as members) and random
   sequences of up to 64 edits (`set_vca_db`, `set_vca_mute`, `set_member_db` on random lanes,
   values across the whole domain including clamping), 32 seeds. After every edit, for every
   reached strip and lane, `effective_db` equals, bit for bit, `effective_strip_faders()` of a
   session model the test edits the same way, and `vca_mute` equals its `vca_mute`.
   The test also asserts that `reached_by(v)` is exactly the strips whose `vca_reach()` contains `v`.
   *Test value: it turns red if live composition differs from preparation in order, reach, clamp
   or lane, if a member's own value is overwritten by an effective one, or if the inverse table
   misses a nested member.*
2. **Never a redundant record.** After `record_emitted_db` of every delta, `fader_delta` is empty
   for every strip; a VCA move that changes no effective value (every member clamped at -144 dB
   before and after, or a VCA with no members) yields no delta; a move that changes both lanes to
   one value yields one `Both` entry, and a one-lane change one `Left` or `Right`.
   *Test value: it turns red if composition re-emits unchanged targets, which moves a settled lane's
   bits.*
3. **Clamp and restore.** A member at +20 dB in a VCA moved to +24 dB has effective +24 dB; moving
   the VCA back to 0 dB returns exactly +20 dB.
   *Test value: it turns red if the state stores the clamped effective value as the member's own.*
4. **Transaction.** After a mix of edits, `rollback` restores every VCA value, member value,
   emitted value and the strip-mute owner's `vca_mute` to their pre-transaction state, and
   `commit` keeps them; a second transaction shadows afresh.
   *Test value: it turns red if any mirror is left changed by a refused submission.*
5. **Precedence and follow.** With the strip-mute owner seeded from a session, `set_vca_mute` on a
   soloed track's strip makes `strip_delta` yield its mute and `effective_mute` true; un-soloing
   keeps it; a solo-safe submix's `set_vca_mute` mutes it; and `LiveRouteMuteFollow::delta` with
   that owner's `effective_mute` yields exactly the following sends of the VCA-muted strips.
   *Test value: it turns red if solo clears a VCA mute, if solo-safe exempts it, or if the VCA mute
   does not reach the follow composition.*
6. **Allocation.** After `try_new`, a sequence of setters, deltas, `record_emitted_db`, `commit` and
   `rollback` (with the strip-mute owner's `set_vca_mute`) allocates and frees nothing:
   `allocations == 0` and `frees == 0` on the current thread, measured with
   `bench_support::alloc`'s thread-scoped counters after warm-up; and `try_new` on a model with no
   VCA allocates nothing. `retained_bytes()` equals the net bytes `try_new` keeps, measured as
   `requested_bytes - released_bytes` of `bench_support::alloc`'s thread counters around `try_new`
   (`tools/bench-support/src/alloc.rs:67-84`; the transient `vca_reach()` and
   `effective_strip_faders()` vectors are freed before it returns); `largest_allocation_bytes()` is
   positive and at most that for a built state; both are 0 for `empty()`.
   *Test value: it turns red if a setter or delta allocates on the audio thread, if the VCA-free path
   allocates arrays that would move a host's retained bytes, or if the reported bytes omit the
   shadow or a table.*
7. **Workspace and policy.**
   - `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host (`host-core` is in it), or CI's
     `aarch64-debug` at the batch push (recorded "at batch push").

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer, and the mutation that turned it red
  (recorded in `crates/host-core/tests/MUTATIONS.md`).

### Attempt 1 record (Terra)

- **D1** `crates/host-core/src/vca.rs`, re-exported (`LiveVcaState`, `LiveVcaFaderDelta`), with
  the frozen signatures. Per VCA `db`/`mute`, per strip own `db` and emitted effective `db`
  (seeded from `effective_strip_faders()`), the reach flattened from `vca_reach()` into CSR tables
  (`reach_start`/`reach`, ascending VCA ID; inverse `reached_start`/`reached`, ascending strip,
  built by count, prefix sum and a transient cursor), and a shadow of the four mutable arrays.
  `effective_db` calls `session::vca_effective_db(own, reach offsets)`; nothing re-spells it. A
  model with no VCA returns `empty()` before `vca_reach()` runs. Indices are `usize` (4 bytes on
  wasm32). `retained_bytes`/`largest_allocation_bytes` sum/max the twelve boxed arrays. Zero fills
  only (`repeat_n(0)`); the rest are copies, so no splatted non-zero store (cross-targets: host-core
  still 4 `memset_pattern16` calls). No name contains `free`.
  - Out-of-range behaviour (not frozen by D1, chosen on the solo/route precedent): unknown
    VCA/strip -> `false` or no-op; `effective_db` of an unknown strip/lane is `0.0`; of an unreached
    strip, the value preparation baked; `fader_delta` of an unreached strip is empty.
- **D2** `solo.rs`: `set_vca_mute(strip, [bool; 2]) -> bool`, `vca_mute_shadow` reserved in
  `try_new`, copied in `shadow()`, restored in `rollback()`; `covers` became `pub(crate)` for the
  VCA module; the module/field docs that said the term had no setter and no shadow are corrected.
- **Tests** (`crates/host-core/tests/vca_live.rs`; mutations in `tests/MUTATIONS.md`, 1244-H1..H15,
  all 15 red; driver and logs `/tmp/claude-1002/kv-1244/`):
  - `a_live_recompute_equals_preparation` (gate 1, 32 seeds, 2-8 tracks, 1-4 submixes, up to 10
    VCAs over 4 levels, up to 64 edits, values across the domain plus edge/tiny words): red if live
    composition differs from preparation in order, reach, clamp or lane, if a member's own value is
    overwritten by an effective one, or if the inverse table misses a nested member (H1, H2, H3,
    H9). Reach: 193 multi-VCA strips, 394 diamond (VCA, strip) pairs, 79 reached submixes, 5,721
    clamped-lane observations, 3 refused unreached member edits.
  - `a_composition_never_owes_a_redundant_record` (gate 2, randomized, 16 seeds x 48 edits): red if
    a delta re-emits an unchanged target, misses a moved one, or the mirror is not seeded from the
    prepared values (H5, H13, H14). Shapes: 3,954 empty, 55 `Both`, 703 one-lane, 136 two-record.
  - `an_unchanged_effective_value_owes_no_record_and_the_delta_shape_follows_the_lanes` (gate 2,
    named cases: members clamped at -144 before and after, a member-less VCA, `Both`, `Right`,
    `Left`+`Right`, a member's own move, an unreached strip refused): red if a clamped or
    unreached move owes a record or a selector disagrees with the moved lanes (H5, H13, H14).
  - `a_clamped_member_returns_to_its_own_value` (gate 3): red if the effective value is stored as
    the member's own (H4).
  - `rollback_restores_every_mirror_and_commit_keeps_them` (gate 4): red if a refused submission
    leaves any mirror changed, including the strip-mute owner's VCA term (H6, H7, H15).
  - `a_vca_mute_wins_over_solo_and_reaches_the_following_sends` (gate 5): red if solo clears a VCA
    mute, solo-safe exempts a submix, or the VCA mute misses the follow composition (H8, H9).
  - `the_command_path_allocates_nothing_and_the_retained_bytes_are_measured` (gate 6; 63 command
    passes after one warm-up: `allocations == 0`, `deallocations == 0`; `retained_bytes ==
    requested - released` around `try_new`; VCA-free `try_new` allocates nothing): red if a setter
    or delta allocates, the VCA-free path allocates, or the bytes omit a table or shadow (H10, H11,
    H12).
  - `live_vca_moves_render_as_a_fresh_plan` (the caller's settled-render proof, beyond the frozen
    gates; 12 seeds x 8 blocks): random batches of VCA dB/mute moves (one lane or both), member
    moves, solos and user mutes go through `LiveVcaState`, the strip-mute owner and the route mirror
    by the D3 flow, owed records pushed at zero smoothing, or the batch is rolled back; every block
    equals, bit for bit, a plan freshly prepared from the session the test edited itself (its own
    intent, not the states' readback). A randomized differential, judged by reach: 82 fader, 40
    mute and 25 route records, 15 one-lane VCA mutes, 100 VCA-muted submix observations, 14 follow
    records of VCA-muted sources, 33 solos, 19 rollbacks. Red under H1, H3, H4, H8, H9, H14, H15.
  - `MISO_ENGINE_RANDOMIZED_SCALE=25` (release): all green.
- No test superseded; no digest, oracle or pin moved.
- **Gates** (x86-64-v3 AVX2 host, tree = this commit):
  - 7: workspace test command rc 0 (115 binaries, 1,275 passed, 0 failed, 9 ignored);
    `cargo fmt --check` ok; clippy `--all-features -D warnings` clean; `cargo doc -D warnings`
    clean; host-core, realtime and workspace `check-*`/`test-*` ok; `check-cross-targets.sh` PASS
    (host-core 4 `memset_pattern16` calls, at its ceiling); `run-aarch64-tests.sh debug`: at batch
    push (no arm64 host).
  - Because `solo.rs` is on the browser's command path: `build-web-audioworklet.sh --named-twin`
    and `check-web-audioworklet.sh` ok (callgraph included); `check-browser-expected-resources.py
    --artifacts` ok, no re-pin.
- **For #1245** (INFO-1 of the #1243 verdict, corrected by this slice's verdict MINOR-1): the state
  keeps about `2 x usize` per (strip, reaching VCA) pair plus `O(strips + VCAs)` mirrors, which
  `retained_bytes` reports for the host to charge. The browser's only document bound is the raw
  byte length (1 MiB), and whitespace-free JSON is accepted, so its worst case is a chain of 1,292
  submixes under 4,842 VCAs: **6.26 M pairs**, about 48 MiB retained on wasm32 (95.6 MiB native),
  1.9 s in `try_new`, and **10.6 ms natively per VCA dB move** (3.6 ms per mute), about four
  128-frame quanta. (~1.7 M pairs, ~13 MiB and 2.9 ms per move is the canonical, indented JSON
  case, still over a quantum.) #1245's bounds (256 VCAs, 16,384 pairs) cap it in the browser.

### VCA follow-up record (after the attempt 1 PASS verdict)

Applied in the VCA batch follow-up commit (on `5248f94c4`, branch `codex/batch-vca`):

- **MINOR-1, step 1.** The "For #1245" bullet above and `crates/host-core/src/vca.rs`'s `# Size`
  doc state the verdict's measured worst case (the raw-byte bound, whitespace-free JSON: 6.26 M
  pairs, ~48 MiB on wasm32, 10.6 ms per VCA dB move; canonical JSON: 1.73 M, ~13 MiB, 2.9 ms) and
  that #1245's boot bounds (256 VCAs, 16,384 pairs) now cap it in the browser. **Step 2** was
  #1245's amendment A1.
- **NIT-2.** `set_vca_db` and `set_member_db` say the value must be finite and in -144 to +24 dB
  and that the host validates it; a new "Preconditions" doc section says what a non-finite or
  out-of-domain value does. `effective_db`'s doc says an unreached strip returns the value
  preparation baked, never a later own move.
- **NIT-3, taken (a deviation from D1's "seeded from `effective_strip_faders()`").** `try_new` seeds
  the emitted mirror with `vca_effective_db` over each strip's own value and its flattened reach's
  offsets, in the same ascending VCA-ID order, instead of a second `vca_reach()` inside
  `effective_strip_faders()`: the same function over the same values in the same order, so the
  same bits, for half the construction time and transient. `a_live_recompute_equals_preparation`
  still compares every live value with `effective_strip_faders()`. Mutation (the mirror seeded from
  the own values): RED in `a_composition_never_owes_a_redundant_record` and
  `an_unchanged_effective_value_owes_no_record_and_the_delta_shape_follows_the_lanes`. host-web's
  transient projection, which charged two reach results, now over-counts by one (#1245 record).
- **NIT-1** (`record_emitted_db` without its shadow, unreachable through D3): not applied.

## Verdict

- **Attempt 1** (`1db0aacc3`): Sol PASS. One MINOR, three NITs and two INFOs; the MINOR and NITs 2-3
  are applied above, and the INFOs went to #1245. `docs/handoffs/submix-sends-2026-10-02/verdicts/1244-attempt1.md`; probes
  `docs/handoffs/submix-sends-2026-10-02/verdicts/1244-attempt1-verifier-scratch.rs`.

## Dependencies

- *Apply VCA offsets and mutes at preparation* (#1242)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Never emit a redundant record. Nothing here allocates after construction.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the VCA batch branch; do not push until the batch closes.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
