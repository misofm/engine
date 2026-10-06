# Render stored effect parameter automation across seeks and on both hosts

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1 (A1.4, A1.6), A4, A5, A7 and A11, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R3.

## Product outcome

Stored effect parameter automation, which draft 18a compiles and binds, follows a session seek
and renders identically on both hosts and at any quantum. Where a session seek reaches an effect
node, each automated cell stages one `Point` to the curve at the seek target, and the grid resumes
from the new node time. The browser host renders the same automation as the C ABI. The same
session renders the same bits at quantum 128 and at quantum 100. The contract and schema docs say
which spans render. This slice, draft 18a, draft 19 and #1306 land in batch R3, in one push
(README "Must-land-together groups").

## Context

- **What draft 18a gives.** Each automated `Block` cell of a span-driven effect has a program
  compiled by draft 07's builder (grid period and ramp `L` from the descriptor, 64 or 128 for the
  delay time; completion `End`; "no restart" on the delay time), bound to the effect's stage with
  the node's `a(n)`. The stage processes the node in pieces at events (draft 17b).
- **The ramp law.** A span-driven effect reaches a `Point`'s target at `τ + L - 1`
  (`LinearRamp::next_value`, `crates/effect-runtime/src/ramp.rs:140-155`).
- **The seek rule** (A1.4): where a seek reaches a node, its node time steps; builtin lanes are set
  exactly there, and each effect cell stages a `Point` there, which the effect ramps over its
  smoothing. Drafts 05 and 06a add the session seek on the C ABI and in the browser module; draft 07's
  generator emits the step as an event of kind `Jump` for an effect cell.
- **The browser host** boots through host-core preparation (`hosts/host-web/src/lib.rs:6661`), so it
  compiles the same programs; its own S derivation (`:6574-6583`) is deleted by #1306 D3 in this
  batch.
- **The quantum.** Any nonzero quantum is valid (`crates/session/src/validate.rs:48-53`).
- **The docs.** The contract says runtime automation is a `Point` at the block's first sample
  (`docs/EFFECT_CONTRACT_V1.md:149-155`). The schema says the table renders nothing
  (`docs/SESSION_SCHEMA_V1.md:220-225`).

## Decisions frozen for this slice

- **D1. The seek step.** Where a session seek reaches the node (its node time steps), each cell
  stages a `Point` at that sample: draft 07's `Jump` kind for effect cells, to the curve at the
  step sample plus `L - 1`. The grid resumes from the new node time (A1.4).
- **D2. One path on both hosts.** The browser host takes the programs from the same host-core
  preparation as the C ABI. It adds no browser-only automation code; its tests prove the bits
  equal.
- **D3. Docs.** `docs/EFFECT_CONTRACT_V1.md:149-155` states that stored automation reaches an
  effect as `Point`s at the first sample of a piece, that a process call may cover part of a
  quantum, and that the delay time follows a curve as a sequence of 128-sample crossfades.
  `docs/SESSION_SCHEMA_V1.md:220-225` lists the effect rows as rendered (the EQ after draft 20).

## Deliverables

1. D1 through draft 07's generator in the stage (draft 17b), and the step's node-time mapping.
2. D2's tests on both hosts.
3. D3's text.

## Authorized paths

- `crates/rack/src/lib.rs`, `crates/graph/src/runtime.rs` (the step's event in the stage only).
- `crates/host-core/tests/effect_automation.rs`, `crates/capi/src/runtime/` tests,
  `hosts/host-web/src/tests.rs` (tests).
- `docs/EFFECT_CONTRACT_V1.md`, `docs/SESSION_SCHEMA_V1.md`.

## Non-goals

- Compiling and binding cells (draft 18a). The session seek itself (drafts 05, 06a, 06b).
- The EQ (draft 20). Classifier and carry (draft 19).
- Cross-target bit identity of a whole session (drafts 23a, 23b).

## Hazards

- **The delay time.** A time curve that moves faster than one crossfade per 128 samples renders as
  a sequence of crossfades, each to the curve at its completion. That is the delay's law, not a
  defect; D3's paragraph states it. A seek inside a running crossfade waits for its end (draft 07's
  "no restart" rule).
- **Hot files.** `hosts/host-web/src/tests.rs` is stream H's; the stages are stream A's. Root
  sequences the merge.

## Objective gates

1. **Seek** (new cases in `crates/host-core/tests/effect_automation.rs`). After a session seek
   (draft 05) to `T`, the first stored `Point` after the step is at the sample where the node's time
   steps, with the curve's value at `T + L - 1`, and the next ones are on the grid of the new node
   time (read through a test-support trace of staged stored `Point`s). On the delay time, a seek
   inside a running crossfade stages at the crossfade's last sample plus one.
2. **Browser equals C ABI.** For each of the seven effects, a session with a flat automation at `v`
   renders, through the browser host, the bits of the same session with static `v` (new test in
   `hosts/host-web/src/tests.rs`, 64 blocks), and a session with a moving curve renders the bits
   the C ABI renders for it (draft 18a's gate-2 session, compared in the test).
3. **Quantum independence.** A one-track session whose only moving part is an automated
   compressor renders the same master output bits at quantum 128 and at quantum 100, with and
   without a session seek at the same timeline position.
4. **Allocation.** The seek block and the 1,000 blocks after it make 0 allocations and 0 frees
   (`bench_support::alloc::current_thread_delta_since` after one warm block).
5. **No rendered bit moved (PR evidence).** `cargo build --locked --release -p audit -p capi`, then
   `target/release/audit capi` at base and head: every violation count 0 and the same
   `pcm_digest`. The browser legs of `qualification.yml`'s `browser` job pass with unchanged
   digests.
6. **Workspace.**
   - `cargo test --locked -p host-core --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p host-web --features test-support`
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     then
     `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
     `memset_pattern16` call; fix one in code, never by a ceiling)
   - `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1 turns red if a seek leaves a cell's cursor at the old position, stages no `Point`, takes
  the value at the step sample instead of `+ L - 1`, or keeps the old grid phase.
- Gate 2 turns red if the browser host compiles or binds the programs differently from the C ABI.
- Gate 3 turns red if any event time depends on the quantum or on where a seek landed within a
  block.
- Gate 4 turns red if the seek's reposition allocates.

## Dependencies

Batch R3. Direct dependencies:

- Draft 18a *Compile and bind stored effect parameter automation* (same push).

Draft 17b's piece loop, draft 07's events, the seek of draft 05 *Seek the timeline and every source
in one C ABI call* and draft 06a *Seek the timeline and every source from the browser module export
and the headless SDK*, and *Size each effect's automation span window from the producers its plan
has* (#1306) arrive through draft 18a.
