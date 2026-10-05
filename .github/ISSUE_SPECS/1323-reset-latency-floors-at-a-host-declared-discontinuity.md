# Reset latency floors at a host-declared discontinuity

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Latency does not ratchet up for the life of a session. *Keep every node's latency from dropping
during playback* (#1285) keeps a removed limiter's latency while audio plays. When the host
declares a discontinuity (it has stopped, or it is about to seek every source), the engine drops
every latency floor. The next render runs the session at its natural latency, and the plan
resource report says so. A declaration with nothing to reset changes nothing.

## Context

- Floors are recorded in the predecessor's inventory and passed on to every successor (#1285 D2).
  Without a reset they never come down.
- No C ABI call lets a host say "continuity is not needed". The render clock must stay contiguous
  ("render.time.discontinuity", `crates/capi/include/miso_engine_v1.h:72-75`). Seeks are per
  source (`miso_engine_v1_source_seek`, `miso_engine_v1_source_seek_at`, `:293-303`). On
  `6fb211594` the last feature bit is `MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT` (`:136-142`;
  `crates/capi/src/abi.rs:64-71`). Other decision-15 entry points may take bits before this slice
  merges.
- The successor entry points take a `SuccessorBase` (`crates/host-core/src/prepare.rs:641-647`).
- D15-17: a host-declared stop turns any pending catch-up into a plain rebuild with no continuity
  constraint, applied at the next render. The accumulated read-ahead `ΣP` of the warm successor
  resets at the same declaration (D15-8). *Give a plan a source-read clock that leads its render
  clock* (#1396), a slice of *Pre-roll a successor whose latency grows* (#1287), introduces that
  read-ahead and resets it at the declaration defined here.

## Decisions frozen for this slice

- **D1. The declaration.**
  - A new C ABI entry point, `uint32_t miso_engine_v1_declare_discontinuity(miso_engine_v1_session
    *session)`, guarded by `MISO_ENGINE_V1_FEATURE_DECLARE_DISCONTINUITY`. Its value is the next
    free bit when this slice merges, and the mask grows to match. This spec fixes no bit number;
    code, tests and docs name the bit by its symbol (decision 15, D15-12: a feature bit per
    addition).
  - It is a control-thread call. Its meaning: the next block the host renders need not continue the
    previous output, because the host has stopped or is about to seek every source.
  - The header documents the duty: a host calls it at a stop, and before a seek of every source.
  - The ABI growth rule of *Document the seek contract and the C ABI growth rule in the header*
    (#1317) applies: one feature bit, the mask never compared with `==`.
- **D2. What it does.**
  - If the newest plan (the pending candidate, or else the active plan) was compiled with any floor
    above its node's natural arrival, the control plane prepares the committed model as a successor
    at a discontinuity. That successor has no floors and carries only the source consumers (ring,
    generation and read position). Every other owner starts at rest: there is no continuity
    constraint. It is published for adoption at the next block.
  - The call lives in `crates/control-plane/src/`; the capi entry point only forwards to it.
  - A pending candidate is superseded by *Supersede an unadopted candidate plan by
    compare-and-swap* (#1310).
  - If no floor is raised, the call returns OK and publishes nothing.
- **D3. Host-core.** `SuccessorBase` gains `discontinuity: bool`. When it is true, the successor
  is compiled with an empty floor map, and the join carries sources only. Its source-read offset
  is 0 (#1396 D2, which lands after this slice and reads this flag). The declaration may or may not
  be followed by a seek: a stop without a seek follows #1396 D3. Each carried consumer keeps its
  read position, so no frame is repeated or skipped, and the source-read clock steps back to the
  render clock at the adoption block. The inventory records
  natural arrivals beside floored ones, so D2's test is a comparison of two numbers.
- **D4. Acked-batch rule.** Preparation can fail (budgets, ceilings). On failure the call returns
  the preparation error and nothing changes. On success the plan is published with its retirement
  credit reserved, as a structural transaction is. The model and the revision do not change. No
  acknowledged edit is lost: the committed model is the source of the successor.
- **D5. Both hosts.** The call is implemented in the control plane, in the crate of *Extract the C
  ABI control plane into a portable crate both hosts call* (#1309). The C ABI exposes it here. The
  browser reaches the same function when *Run the browser control plane in a Worker and keep the
  AudioWorklet render-only* (#1332) lands, and its SDK surface belongs to *Apply session
  transactions from the browser SDK* (#1296).

## Deliverables

1. D1: the entry point in `crates/capi/src/ffi.rs`, the feature bit and mask in
   `crates/capi/src/abi.rs`, the header prototype and text, and `docs/C_ABI_V1_QUALIFICATION.md`.
2. D2 and D4 in `crates/control-plane/src/` (package `control-plane`, lib `control_plane`, created
   by #1309).
3. D3 in `crates/host-core/src/prepare.rs`.
4. Tests in `crates/host-core/tests/successor_swap.rs` and `crates/capi/src/runtime/tests.rs`.

## Authorized paths

- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`
- `crates/capi/src/ffi.rs`, `crates/capi/src/abi.rs`, `crates/capi/src/runtime/tests.rs`,
  `crates/capi/include/miso_engine_v1.h`, `docs/C_ABI_V1_QUALIFICATION.md`, and
  `crates/control-plane/src/` (#1309). These are stream B's files; root orders the merge after
  #1309 and #1310.
- `scripts/check-capi-abi.sh` (only if its symbol list must learn the new entry point).

## Non-goals

- No read-ahead reset: #1396 sets the discontinuity successor's source-read offset to 0 (its D2)
  and defines the stop without a seek (its D3).
- No browser export (#1332, #1296).
- No change to the render clock's contiguity rule.

## Objective gates

1. **Floors reset, host-core.** A has three tracks, the first with a true-peak limiter insert. B
   removes that track and is swapped in after block 6 with A's floors (#1285). Then prepare B again
   with `discontinuity: true` and swap it in after block 10. From block 11 on, its output equals a
   fresh B, compiled without floors, whose sources were fed the same PCM from the frames the carried
   consumers stood at. `latency_samples` is B's natural latency.
2. **Floors reset, C ABI.** The same edit as a `RemoveTrack` transaction, then
   `miso_engine_v1_declare_discontinuity`. After one render, `miso_engine_v1_plan_resources`
   reports the natural `latency_samples`. Before the declaration it reported A's.
3. **Nothing to reset.** On a session with no raised floor, the declaration returns OK, publishes
   no plan (the swap counter is unchanged), and leaves the output bit-identical to a run without
   the call.
4. **Failure changes nothing.** With a resource cap set so that the floorless preparation fails,
   the call returns the preparation error, and the active plan, its floors and the committed model
   are unchanged.
5. **ABI surface.** `query_capabilities` reports `MISO_ENGINE_V1_FEATURE_DECLARE_DISCONTINUITY`,
   and the mask test checks that the mask is the OR of every feature symbol, the new one
   included (by symbol, never by a literal number).
6. Commands:
   - `cargo test --locked -p host-core -p control-plane -p capi --features host-core/test-support`
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`
   - `cargo build --locked --release -p audit && ./target/release/audit capi`
   - `bash scripts/check-workspace-policy.sh`, `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`

## Test value

- Gate 1: a reset that keeps the floors, or one that carries DSP state, diverges from the fresh
  floorless reference. It turns red.
- Gate 2: a declaration that clears floors in the inventory but never publishes a plan keeps
  reporting A's latency. It turns red.
- Gate 3: a declaration that always rebuilds resets every effect tail on every stop. It turns red.
- Gate 4: a declaration that drops floors before its preparation succeeds leaves a plan whose next
  successor loses latency mid-playback. It turns red.

## Dependencies

- *Keep every node's latency from dropping during playback* (#1285).
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310).
