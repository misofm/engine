# Remove a strip in two phases: ramp out, then a scheduled swap

Stream D of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

When a structural edit removes a track or a submix while audio plays, the strip fades out over the
session mute ramp and only then leaves the plan. Today its contribution stops dead at the swap
block (#1269 P4), which is a click. The removed strip's source keeps playing until it is gone, so
the fade is made of real audio, not of underrun zeros. This slice also builds the shared
"ramp on the predecessor, then a scheduled swap" step that the duck-swap (#1324) reuses.

## Context

- Today a structural transaction prepares a successor and publishes it for the next block:
  `SessionState::command`'s structural arm (`crates/capi/src/runtime/control.rs:907` onward,
  `prepare_runtime` with a `SuccessorBase`). #1309 moves this into a portable control-plane crate.
- A pending candidate makes the next structural edit BACKPRESSURE
  (`crates/capi/src/runtime/control.rs:959`); #1310 replaces that with compare-and-swap supersession.
- Source calls address the newest committed session: `newest_providers` (`control.rs:1488`), used by
  `submit` (`:1501`), `seek` (`:1514`) and `seek_at` (`:1530`). A source the transaction removed is
  refused at once as `source.id.unknown`, and its accepted PCM is discarded with its plan
  (`crates/capi/include/miso_engine_v1.h:85-94`).
- The default source ring hides 100 ms (`default_source_ring_frames`,
  `crates/host-core/src/prepare.rs:65`), but a session mute ramp may be up to 1000 ms (#1054 bounds),
  so the ramp needs PCM submitted after the commit.
- Every C ABI plan has each strip's live fader/mute lane (`C_ABI_LIVE_LANES`,
  `crates/capi/src/runtime/compile.rs:18`, attached in `prepare_runtime`, `:572-600`). The browser
  gets them from #1326.
- Render publishes the start of its next block after every block: `SharedPlanState::render_sample`
  (`crates/capi/src/runtime/plan.rs:10`, stored at `:236`).
- Live mute ramps: `LiveRamps::for_session(..).mute_samples` (`crates/host-core/src/live_delta.rs:52`),
  0 until #1054.

## Decisions frozen for this slice

- **D1. Which strips.** Every strip (track or submix) whose ID is in the displaced plan's committed
  model and absent from the transaction's model.
- **D2. Phase 1, the ramp.** After the transaction's fallible steps and before it is acknowledged,
  the control plane writes, for each removed strip, a mute of both channels with
  `ramp = N = LiveRamps::for_session(next model).mute_samples` into the displaced plan's strip lane
  (its latest-target cell, #1312, which cannot refuse a write). Then it reads `p =
  render_sample`.
- **D3. Phase 2, the scheduled swap.** The successor is published to adopt no earlier than
  `S = ceil_q(p + q + N)` (#1311), where `q` is the quantum and `ceil_q` rounds up to a multiple of
  it. Proof: the write precedes the read of `p`, so render drains it no later than the block that
  starts at `p + q`; the ramp ends by `p + q + N <= S`. Between the ramp's end and `S` the strip is
  settled at exact `+0.0`. A paused host keeps `S` valid: render resumes at `p`.
- **D4. The source retires with phase 2.** Until the successor is adopted, `submit`, `seek` and
  `seek_at` for a source absent from the newest committed session but present in the plan that
  still renders go to that plan's producer. From the adoption on they are refused with
  `source.id.unknown`, as today. Its ring and producer retire with the displaced plan through the
  existing retirement path; PCM queued past `S` is discarded with it (documented, as today). The host
  learns the adoption from the watermark (#1314) and stops feeding then.
- **D5. Reporting.** The response path is `rebuild` (#1313). The revision completes when render
  adopts at `S`: the watermark (#1314) reports first sample `S` with `EXACT` (or `SUPERSEDED` when
  #1310 displaces it) and counts it in `exact_count`. A planned D15-9 transition is the designed
  result of this edit, not a fallback: it never sets `PREROLL_FALLBACK` or `TRANSITION_FALLBACK`
  nor their counters, which belong only to the catch-up fallback (#1358).
- **D6. Supersession.** A structural edit during phase 1 supersedes the candidate (#1310) and
  inherits `S` (the newer not-before is the maximum of both). The phase-1 mutes count as records
  pushed to the displaced plan (D15-7's P1.4 base), so a newer transaction that restores a removed
  strip carries it and its unmute is an ordinary ramped retarget. If render has already adopted the
  older candidate, the newer one is re-targeted to it (#1310) and D2-D4 run against that plan.
- **D7. Shared step.** D2, D3 and D6 are one host-core function, called by the control plane:
  `plan_strip_transition(committed, next, inventory) -> StripTransition` returning the strips to
  duck, `N`, and the strips the successor arms (empty here; #1324 fills it). The not-before sample
  is computed by the control plane from `p` after the writes. One transaction that both removes and
  adds strips uses one `S`; added strips fade in by #1288.
- **D8. Realtime and the acked-batch question.** Render work is the existing mute ramp and one
  not-before comparison at block entry (#1311). Every fallible step (preparation, publication and
  retirement credit, D15-17) runs before the cell writes and the commit, and a cell write cannot
  fail, so no ack precedes a drop.

## Deliverables

1. `crates/host-core/src/transition.rs` with D7, exported from `crates/host-core/src/lib.rs`.
2. The control plane's structural arm and source routing (D2-D6), in the files #1309 creates.
3. The header paragraph (`miso_engine_v1.h:85-94`) and `docs/C_ABI_V1_QUALIFICATION.md` state the
   two phases, the feeding duty until the adoption, and `S`.
4. Tests in a new `crates/capi/tests/strip_transitions.rs` (its own binary, so `bench_support`'s
   allocator serves it, as `plan_swap_race.rs` does).

## Authorized paths

- `crates/host-core/src/transition.rs` (new), `crates/host-core/src/lib.rs`
- the control-plane crate's structural transaction and source-routing files that #1309 creates
  (stream B owns them; root sequences the merge)
- `crates/capi/include/miso_engine_v1.h` (comments only; #1317 edits the same header)
- `crates/capi/tests/strip_transitions.rs` (new), `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- Edited strips (#1324) and added strips (#1288).
- The browser path: the browser runs the same control-plane crate once *Run the browser control
  plane in a Worker and keep the AudioWorklet render-only* (#1332) lands, on the lanes #1326 adds.
- No crossfade between plans. A true crossfade with ghost strips is deferred by D15-9; it reopens
  on a measured, audible dip in a listening test.

## Objective gates

All through the exported C entries, quantum 128, 48 kHz, a session `controlSmoothing.muteMs` that
gives `N = 2000`, a source ring of 1024 frames, single-threaded (each command is submitted between
two render calls, so the ramp starts at `p`).

1. **Fade, then removal.** Tracks A and B play distinct, never-zero sources. Between blocks `k` and
   `k + 1` a transaction removes B. The reference run instead applies a value-only transaction that
   mutes B at the same point. Every block of the two runs is bit-identical until block
   `S/q + 8`, and the watermark covers the removal's revision with first sample `S` and
   `MISO_ENGINE_V1_OUTCOME_EXACT` only (`transition_fallback_count` unchanged), never earlier.
2. **The source feeds phase 1.** In gate 1, B's source is fed one block ahead only (the ring cannot
   hold the ramp). Every submit for it before the adoption returns OK and its frames are the ones
   the ramp plays (gate 1's bit-identity holds). The first submit after the adoption returns
   `MISO_ENGINE_V1_INVALID_ARGUMENT` with `source.id.unknown`.
3. **Supersession inherits S.** During phase 1 a second transaction adds track C: it returns OK
   (no BACKPRESSURE), the adoption happens at or after `S`, and gate 1's blocks up to `S` are
   unchanged.
4. **Realtime.** On the render thread, every block from `k + 1` through the adoption block makes
   zero allocations and frees (`bench_support::alloc` thread-scoped counters, statics warmed).
5. Commands:
   - `cargo test --locked -p capi --test strip_transitions`
   - `cargo test --locked -p host-core -p capi --features host-core/test-support`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` (as in
     qualification.yml's `audit-native` job)
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/check-realtime-policy.sh`,
     `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: a removal that still stops at the swap block, or an adoption before the ramp ends (an
  `S` computed from `p` without the `+ q`), turns it red.
- Gate 2: a source that retires with the commit makes the ramp play underrun zeros and refuses the
  phase-1 submits; it turns red.
- Gate 3: a superseding candidate that drops the inherited `S` swaps mid-ramp; it turns red.
- Gate 4: a schedule check or cell drain that allocates on the render thread turns it red.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310).
- *Adopt a successor plan no earlier than a scheduled sample, with a return queue* (#1311).
- *Hold live values in latest-target cells on both hosts* (#1312).
- *Report each transaction's edit path in its response* (#1313).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054).
