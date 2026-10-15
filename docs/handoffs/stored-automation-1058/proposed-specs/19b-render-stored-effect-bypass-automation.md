# Render stored effect bypass automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), root's ruling on finding F20 (2026-10-14, README "Root's rulings (2026-10-14)"),
under decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, C3 and C6). A draft for root
to file. Written by the controls coordinator on 2026-10-14. Code anchors verified on `2e6343a99`.
Batch R3, after draft 19.

## Product outcome

A producer automates an insert's or a console slot's bypass, for example a saturator that is in
only for the chorus. On every host the bypass switches at the exact timeline sample, through the
existing crossfade over the session's bypass ramp (#1341), aligned with the audio at the effect's
output, latency included. A bypass automation edit is a carried rebuild. A hand bypass of an
automated instance is stored and not heard (automation wins, OQ1 = C) and commits `model_only`. A
session with no bypass automation renders the bits it renders today.

## Context

- **The row.** An effect section's control 0 is the bypass: a shared toggle, family `Effect`,
  `Motion::Fixed`; live when `lowers_session_bypass(effect_id)`, otherwise ruled live, not built
  (the delay, #1339; the multiband compressor, #1340) (`crates/controls/src/effect_rows.rs:210-240`).
  `automatable` is computed, so the live bypass rows are targets today and a prepared bypass is
  refused `control.automation.target`. Nothing renders an entry (A10).
- **The crossfade.** `BypassShunt` keeps a latency-matched dry copy and mixes it with the effect's
  output (`crates/effect-contract/src/live.rs:1621`); `crossfade` and `crossfade_bank` mix on the
  `IndexedRamp` law from a `position` (`:1764`, `:1850`). The bypass cell holds a `BypassSwitch {
  bypassed, ramp_samples }` (`:157`, #1341 D1); its ramp is `LiveRamps` row `BypassCrossfade`, key
  `mute_ms` (`crates/effect-contract/src/control.rs`). The step runs once per block (draft 17b's
  amendment: "D3 also keeps the bypass crossfade step of #1341a per block").
- **Effect programs.** Draft 18a binds stored effect programs to the effect stage with the node's
  arrival `a(n)`; draft 18b adds the seek step and the browser; draft 19 classifies and carries
  effect automation edits. #1622 carries the ramp in the bypass record.

## Decisions frozen for this slice

- **D1. The cell.** One bypass cell per automated instance (row 0, shared), with draft 07's
  builder: `step` only (amended draft 02's toggle rule), so jumps only, no grid; jump row
  `BypassCrossfade` (draft 12's row cell).
- **D2. Time at the output.** The switch acts on the effect's output, so its node time is the
  output's: `a(n) + latency(n)`. A step at timeline sample `t` starts the crossfade at the render
  sample where the node's output carries `t`; there the wet and the latency-matched dry words both
  carry input `t`. The switch therefore meets a downstream event at `t` on the same timeline sample,
  as a parameter `Point` at input `t` first reaches the output there.
- **D3. In-block start.** At an event at offset `o` in a block, the stage starts the crossfade at
  `o`: frames `[0, o)` mix as before, frames `[o, end)` follow the new `BypassSwitch` from position
  0, with the rule #1341 gives a switch that arrives while a ramp runs. The shunt gains an offset
  form of `crossfade` and `crossfade_bank` (per-lane offsets in a bank, whose lanes keep
  independent cells). The effect's `process` call is not split: a bypass changes no effect state,
  and the shunt captures before the call and mixes after it, as today. All per-sample work stays in
  the existing unfused crossfade arithmetic.
- **D4. Seek.** Where a session seek reaches the node's output, the bypass is set exactly
  (settled, no ramp).
- **D5. Edits** (one rule on both hosts). Row 0 joins draft 10 D1's `automation_row_renders`. A
  bypass automation edit is a carried rebuild; the cell's event state carries by `(strip, section,
  effect, control 0)` with draft 19's carry. A `SET_CONTROL bypass` on an automated instance stores
  the fallback, gives no record and commits `model_only`; the control list shows `automated = true`
  (draft 11b D4). A bypass record never reaches an automated instance's cell (draft 19's rule).
- **D6. Prepared bypasses.** The delay's and the multiband compressor's rows stay rebuild-only, so
  the row rule refuses their automation, unchanged. When #1339 or #1340 makes a row live, that
  effect renders through this slice's path with no change; that issue adds it to gate 1.
- **D7. Memory and work.** One program per automated instance (`32 + 32·n` bytes) and one event
  state (48 bytes); no span window term (a bypass stages no span). At most `⌈q/64⌉ + 1` switches per
  block and instance (the 64-sample spacing rule of #1626).
- **D8. The acked-batch question: can an ack ever precede a drop? No.** Events come from the plan;
  every check precedes the first cell write; a cell write cannot fail.

## Deliverables

1. D1, D2 and D4 in host-core preparation and the effect stage's event read.
2. D3 in the shunt and the per-node and bank stages.
3. D5 in `crates/controls` (the rendering row) and the classifier.
4. `docs/SESSION_SCHEMA_V1.md` and `docs/EFFECT_CONTRACT_V1.md`: bypass automation renders at the
   output's timeline sample.

## Authorized paths

- `crates/effect-contract/src/live.rs` (the shunt's offset form), `crates/effect-contract/tests/live_control.rs`
- `crates/graph/src/runtime.rs` (the per-node stage), `crates/rack/src/lib.rs` (the bank stage)
- `crates/host-core/src/{prepare.rs,live_delta.rs}`, `crates/host-core/tests/effect_automation.rs`
- `crates/controls/src/automation.rs` and its tests
- `crates/graph-compiler/tests/bypass_shunt_identity.rs`, `crates/capi/src/runtime/live_tests.rs`,
  `hosts/host-web/tests/control_parity.rs`, `crates/control-plane/tests/bypass_automation_realtime.rs` (new)
- `docs/SESSION_SCHEMA_V1.md`, `docs/EFFECT_CONTRACT_V1.md`

## Non-goals

- Live bypass for the delay and the multiband compressor (#1339, #1340).
- A per-edit bypass ramp (the bypass has none, #1341).
- Bypass of a builtin section (no such row).

## Hazards

- **Hot files.** `crates/graph/src/runtime.rs`, `crates/rack/src/lib.rs` and
  `crates/host-core/src/prepare.rs` are shared with drafts 17b-19; root sequences the merge.
- **The shunt identity.** `bypass_shunt_identity` and `bypass_cohorts` must stay green: the offset
  form at `o = 0` is today's call.

## Objective gates

Commands: `cargo test --locked -p effect-contract`, `cargo test --locked -p graph --features
test-support`, `cargo test --locked -p graph-compiler --test bypass_shunt_identity`, `cargo test
--locked -p rack`, `cargo test --locked -p host-core --features test-support`, `cargo test --locked
-p controls`, `cargo test --locked -p capi`, `cargo test --locked -p host-web --features test-support
--test control_parity`, `cargo test --locked -p control-plane --features test-support`,
`./target/release/realtime_policy`, `bash scripts/check-cross-targets.sh` (README F19), clippy
`-D warnings`, `cargo fmt --all -- --check`.

1. **Flat equals static** (C ABI and browser). For each effect whose bypass is live, a bypass entry
   constant at 1 renders the bits of the session with static bypass `true`, and constant at 0 the
   bits of `false`.
2. **Exact sample and latency.** A step to 1 at a timeline sample inside a block, on the true-peak
   limiter: the crossfade starts on the output sample that carries that timeline sample; after the
   ramp the output equals a static-bypass plan fed the same PCM. At quanta 128 and 100.
3. **Banks equal per node.** Four tracks in one compressor bank, two with different bypass curves,
   render at Simd4 and Simd8 as their per-node instances do.
4. **Seek.** A seek into a bypassed span renders the settled bypass from its first sample.
5. **Edits.** A bypass automation edit gives `rebuild` and completes `exact`; a hand bypass on the
   automated instance gives `model_only` and moves no bit; a hand bypass on an instance without
   bypass automation in the same session crossfades live, as today.
6. **Realtime.** Events and crossfade starts allocate and free nothing on the render thread
   (`bench_support::alloc` thread counters); `audit capi` reports every violation count 0.
7. **No rendered bit moves** for a session without bypass automation (`audit capi` `pcm_digest`,
   PR evidence; the browser legs with unchanged digests).

## Test value

- Gate 1 turns red if the stored bypass takes another path than the shunt's identity.
- Gate 2 turns red if the switch uses the node's input arrival instead of its output, or snaps to
  a block boundary.
- Gate 3 turns red if one lane's bypass event reaches a neighbour lane in a bank.
- Gate 5 turns red if a hand bypass fights the curve or an automation edit commits live.

## Dependencies

- Draft 19 *Classify and carry effect automation edits* (and drafts 18a and 18b, in its push): the
  effect programs, the seek step, both hosts, the carry.
- #1341a1 and #1341a2 (landed): the crossfade. #1622 (ordering only: one bypass record spelling).
- Batch R3, after 19. A complete outcome alone; not in a must-land group.
