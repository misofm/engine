# Admit observation bindings by the exact encoded bytes of the ended event

Successor of #1676 (cap the observation bindings by what one ended event holds) and #1584 D6a
(root ruling A: one `OBSERVATION_ENDED` per commit). Code anchors verified on the batch 13b fold
branch `codex/controls-b13b-fold`. Issue number: assigned at filing (written here as #1685).
Batch 14, the first K (core) slice. Feature sprint mode (owner, 2026-10-13): one implementer,
normal tests, a batch review.

## Product outcome

A mixer can watch a meter on every insert of a large session. The host refuses a subscription only
when the one `OBSERVATION_ENDED` event that a commit could emit would not fit in one control frame,
measured with the real IDs of the armed taps, not with the longest IDs the grammar allows.

## Context

- **The cap today.** `observation_binding_cap(frame_bytes)` (`crates/control-plane/src/compile.rs`,
  near `:483-523`) encodes an `ObservationEnded` event with keys whose strip ID and effect ID are
  both `session::STABLE_ID_MAXIMUM_BYTES` (127) bytes, and returns how many such keys one frame
  holds: 3 at 1 KiB, 12 at 4 KiB, 51 at the browser's 16 KiB (`CONTROL_FRAME_BYTES`,
  `hosts/host-web/src/lib.rs`). `SessionState::observation_cap()` (`crates/control-plane/src/control.rs`,
  near `:2677-2683`) keeps that count, and `admit_and_push_observation_batch` (near `:2718`) refuses
  the arm that would leave more taps armed (`ObservationRefusal::Limit`, near `:2835-2845`).
- **What it costs.** Real IDs are short. A 64-track session with a compressor insert on every
  track already needs 64 taps, more than the browser's 51, and the C ABI at 4 KiB allows 12.
- **The event.** A commit that ends bindings emits one `OBSERVATION_ENDED` with every ended key, in
  key order, after its `SESSION_COMMITTED` (#1584 D6a, root ruling A). The room is reserved with
  the commit before anything is prepared (`attach_observation_ended`, `control.rs` near
  `:1616-1660`); an event over one frame is refused as a resource fault, a defence only.
- **The reserve.** `ended_event_ceiling(bindings)` (`compile.rs` near `:590-603`) charges each key
  as `size_of::<protocol::ObservationTapKey>()` plus two maximum IDs, and the binding table rows
  (`observation_binding_rows`) the same.

## Decisions frozen for this slice

- **D1. Admit by bytes.** The session keeps the exact encoded size of the `OBSERVATION_ENDED`
  frame that would carry every armed key (the frame with no key, plus each armed key's own encoded
  bytes, with the real strip and effect IDs). A batch is admitted when that size, after every
  removal and arm of the batch, is at most the control frame limit. The arm that crosses is refused
  with `ObservationRefusal::Limit` (its index; the reason names bytes, not a key count), with
  nothing queued and no state moved. One naming per tap and the room check stay as they are.
- **D2. No allocation.** The per-key size is computed from the key's lengths with the codec's own
  size rules (no encode into a buffer), so admission stays allocation-free in the browser
  worklet's message handler.
- **D3. One event per commit.** Ruling A stands: a commit emits at most one `OBSERVATION_ENDED`,
  and every binding a commit can end is armed, so its keys are a subset of the admitted set and
  the event always fits. The room is still reserved at commit, before anything is prepared.
- **D4. The reserve follows the bytes.** The ended-event heap and the binding table rows charge
  what the admitted keys can own, as ceilings. `observation_cap()` and its key-count figures are
  removed with the docs that state them.

## Deliverables

1. D1 to D4 in `crates/control-plane`.
2. The browser's `observationLimit` reason (15) and its message keep their meaning; the SDK hint
   text says the frame is full, not that a count is reached.
3. Docs: `docs/CONTROL_PROTOCOL_SEMANTICS.md` (the observation paragraph's 3/12/51 figures).

## Authorized paths

- `crates/control-plane/src/{control.rs,compile.rs}`, `crates/control-plane/tests/observation_bindings.rs`
- `hosts/host-web/src/lib.rs` (comments only), `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/control_parity.rs`
- `sdk/src/core/observation-subscriptions.ts` (the hint text only), its eval
- `docs/CONTROL_PROTOCOL_SEMANTICS.md`

## Non-goals

- A second `OBSERVATION_ENDED` per commit, or splitting the event over frames.
- A larger control frame.
- The C ABI observe edit itself (#1584 D1-D5); it calls the same core admission (#1584's batch 13b
  amendment).

## Objective gates

1. **Large session.** A 64-track session with one compressor insert per track subscribes the
   compressor's meter tap on every insert at the browser's 16 KiB frame: all 64 admitted; on the C
   ABI (no default frame: the host sets `maximum_control_frame_bytes`; the capi tests use 4 KiB),
   the same, through #1584's observe edit when it is on `main`, else at the core through
   `SessionState` with a 4 KiB limit.
2. **The edge.** Keys sized so that the frame holds exactly `k` of them: `k` arms are admitted,
   arm `k + 1` is refused `Limit` at its index, a disarm in the same batch makes room, and removing
   the strips commits with one `OBSERVATION_ENDED` that holds all `k` keys and fits the frame.
3. **Realtime.** Admission allocates nothing (`bench_support::alloc` thread counters, statics
   warmed).
4. Commands: `cargo test --locked -p control-plane --features test-support`,
   `cargo test --locked -p host-web --features host-web/test-support`, the browser qualification,
   clippy `-D warnings`, `fmt --check`.

## Test value

- Gate 1: red if admission still assumes 127-byte IDs (64 taps are refused).
- Gate 2: red if the byte count is off by one key or by the frame header, or if the commit can
  emit an event over one frame.
- Gate 3: red if admission encodes into an allocated buffer.

## Dependencies

- #1676 and #1677 (the cap, the stable key; batch 13b).
- #1584 D6a (one event per commit).
