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

## Dependencies

- *Apply VCA offsets and mutes at preparation* (#1242)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Never emit a redundant record. Nothing here allocates after construction.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the VCA batch branch; do not push until the batch closes.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
