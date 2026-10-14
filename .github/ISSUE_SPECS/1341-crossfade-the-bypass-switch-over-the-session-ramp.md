# Crossfade the bypass switch over the session ramp

Stream E of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-13 E5).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A live bypass toggle on any effect that has a bypass shunt no longer steps the output by
`wet - dry`. It crossfades linearly between the effect's wet output and its latency-matched dry
signal, over the session's mute ramp (`control_smoothing.mute_ms`, or the default table). In this slice
the render paths (per node and banked) crossfade, a plan swap carries a crossfade in flight, and
the C ABI's bypass transactions use it. The effect's latency does not change. From the first sample
after the ramp, the output is bit-identical to today's whole-block select. A session whose
`mute_ms` is 0 renders exactly today's bits. The browser's `COMMAND_EFFECT_BYPASS` gets the session
ramp in *Crossfade the browser's live bypass command over the session ramp* (#1393).

## Context

- **The shunt** (`crates/effect-contract/src/live.rs:822-990`). The wet path always runs. The dry
  signal is delayed by exactly the declared latency. "Selection is whole-block, never per sample"
  (`:841-843`), done by copies, so `-0.0` survives (`apply`, `:979-984`; `dry`, `:987-990`).
  `capture` (`:948-976`) must run on every block when the shunt has a latency line (`feeds_line`,
  `:941-946`). At zero latency it is skipped on blocks that no reader needs.
- **The bypass state.** `EffectControlRecord::Bypass(bool)` (`live.rs:95`) sets
  `EffectControlLane::bypass` at the block's `stage` (`:398-401`). `bypassed()` (`:245`) is the
  only reader. The witness term `UNBYPASSED` is seeded from it (`:175`, `:201`) and declines mono
  collapse while an upstream stage is bypassed (`crates/effect-contract/src/symmetry.rs:38`,
  `:99-106`).
- **Two render paths read it:**
  - **Per node**, `NodeKind::LiveControlEffect` (`crates/graph/src/runtime.rs:3510`):
    `capture_dry = bypassed || shunt.feeds_line()` (`:3537`), and `shunt.apply` after `process`
    when bypassed (`:3582-3585`).
  - **Banked**, `LiveControlEffectBankStage` (`crates/rack/src/lib.rs:937`): `any_bypassed`
    (`:1321-1323`) gates the capture (`:1348-1360`). The restore copies the dry words of exactly
    the bypassed lanes, strided by `lane_count` (`:1441-1462`).
- **Producers of a bypass record:**
  - the C ABI classifier (`crates/host-core/src/live_delta.rs:391-393`);
  - the browser decode (`hosts/host-web/src/lib.rs:4489-4501`), which today refuses a non-zero
    `smoothing_samples` word as `MALFORMED`;
  - the SDK's `bypass(enabled)` (`sdk/src/core/live-controls.ts:943`), which has no ramp option.
- **A drift-free, vectorisable law already exists.** `IndexedRamp`
  (`crates/lane/src/kernels.rs:1073-1100`) computes `c(k) = round(round(k * step) + start)` for
  `1 <= k < length`, and assigns `target` exactly for `k >= length`. A step is `length == 0`. It
  is a pure function of the frame index, so it vectorises over frames, and a settled ramp is
  exactly the target. The live routes already use it.
- **The ramp length.** #1054 D3 maps the bypass crossfade to `mute_ms`, through
  `LiveRamps::mute_samples` (`crates/host-core/src/live_delta.rs:33-58`). The C ABI qualification
  document says "The switch is a step: there is no bypass crossfade (decision 14, F7)"
  (`docs/C_ABI_V1_QUALIFICATION.md:450-452`).
- **Hot file, and the order.** `crates/effect-contract/src/live.rs` is edited first by
  *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345), which turns
  the live bypass into a one-word cell drained first (#1345 D1, D3). Then *Carry live-controlled
  effect lanes across a plan swap* (#1280) carries those cells and the shunt words in move mode
  (cells #1280 D2, retarget D4, shunt words D5). This slice lands after both (plan risk 3) and
  extends that carry.

## Decisions frozen for this slice

- **D1. The bypass value and its ramp travel as one unit.** #1345's bypass cell widens from one
  word to two, `bypassed` and `ramp_samples`, written and drained as one unit (D15-2: a cell holds
  its target words and its ramp together). The producer fills `ramp_samples` from
  `LiveRamps::for_session(model).mute_samples` (#1054 D4). The bypass is the one live row with no
  per-edit length (D15-1 recorded resolution): a host asks for a step by setting `mute_ms` to 0.
  The browser decode, which this slice does not change in behaviour, writes `ramp_samples = 0`;
  *Crossfade the browser's live bypass command over the session ramp* (#1393), which needs #1364's
  browser `LiveRamps`, gives it the session ramp. Its `smoothing_samples` word stays required
  zero.
- **D2. Lane state.** `EffectControlLane` holds:
  - the target (`bypass`);
  - a one-word mix ramp `m`, where `m = 1` is wet and `m = 0` is dry, as an `IndexedRamp` that uses
    word 0;
  - its frame position `k`.

  A record whose target equals the current settled state changes nothing. Otherwise it starts
  `IndexedRamp::new([m_now, 0, 0, 0], [target_m, 0, 0, 0], ramp_samples)` with `k = 0`. `m_now`
  is the mix at the current position, so a reversal mid-ramp continues from where the output is.
  Every rendered frame advances `k`, which saturates at the length. A length above
  `INDEXED_RAMP_LENGTH_MAXIMUM` is refused where the record is produced (C ABI: `Domain` in the
  classifier; browser: `DOMAIN`). At the launch bound (1000 ms at 96 kHz, 96,000 samples) this
  cannot happen.
- **D3. The mix.** For a ramping lane and frame `k`:
  `out = dry + (wet - dry) * m(k)`, as a separate subtract, multiply and add. There is no fused
  multiply-add, as base Wasm SIMD requires.
  - A settled-wet lane keeps the wet words untouched.
  - A settled-dry lane copies the dry words, as today.
  - Settled lanes are never computed through the formula, so the after-ramp bits are today's.
  - The same `m(k)` applies to left and right, so the switch never makes the channels differ.
  - Linear in amplitude, the engine's one ramp law family (#1055 `FINDINGS.md`, section 3): dry
    and wet are the same signal, latency-matched, and are highly correlated, so an equal-gain
    crossfade keeps the level where an equal-power one would bulge by up to 3 dB at the midpoint.
- **D4. Vectors, not scalars.**
  - **Per node:** `m(k)` is computed for a whole run of frames by the indexed law, and the mix runs
    lane-wide over frames.
  - **Banked:** the interleaved frame holds `lane_count` contiguous words. The mix runs lane-wide
    on one vector of per-lane `m`. A select then keeps the wet words of settled-wet lanes and the
    dry words of settled-dry lanes, and takes the mix for ramping lanes.
  - The kernels live next to `BypassShunt` in `live.rs` and use the `lane` crate's vector types.
    The same kernel shape serves every target; only the lane width differs.
- **D5. Capture.** Both paths capture the dry block when any lane is bypassed or ramping, or when
  the shunt feeds a line.
- **D6. Mono collapse.** A ramping lane counts as bypassed for the witness: `UNBYPASSED` is false
  from the record until the ramp settles at wet. Collapse is declined during a ramp exactly as it
  is under a bypass.
- **D7. Carry.** The mix ramp and `k` are live lane state, carried with the bypass cell by #1280's
  move-mode lane carry: the swap block moves them, and the predecessor then retires. A swap in the
  middle of a ramp continues the ramp.
- **D8. Acked-batch question.** No queue or admission rule changes, and a record still applies at
  its block boundary. A bypass record is never acked without effect: its ramp starts at the next
  block.

## Deliverables

1. D1-D4 and D6 in `crates/effect-contract/src/live.rs` (record or cell, lane state, kernels), with
   the shunt's doc rewritten ("Selection is whole-block" becomes the crossfade rule).
2. D4-D5 at the two call sites: `crates/graph/src/runtime.rs:3530-3585` and
   `crates/rack/src/lib.rs:1318-1462`.
3. D1 producers: the classifier (`live_delta.rs:391-393`) with the session mute ramp; the browser
   decode (`hosts/host-web/src/lib.rs:4489-4501`) with `ramp_samples = 0` (D1); every other
   construction of the record or cell (mechanical).
4. D7 in #1280's move carry.
5. Docs: `docs/C_ABI_V1_QUALIFICATION.md:450-452` and `docs/EFFECT_CONTRACT_V1.md`'s bypass
   text.
6. The tests below.

## Authorized paths

- `crates/effect-contract/src/live.rs`, `crates/effect-contract/src/symmetry.rs`
- `crates/graph/src/runtime.rs`, `crates/rack/src/lib.rs` (stream A owns them; root sequences the
  merge)
- `crates/host-core/src/live_delta.rs` (stream B owns it), `crates/host-core/tests/live_delta.rs`,
  `crates/host-core/tests/successor_swap.rs`
- `crates/graph-compiler/tests/bypass_shunt_identity.rs`, `crates/graph-compiler/tests/bypass_cohorts.rs`
- `crates/capi/src/runtime/live_tests.rs`, `docs/C_ABI_V1_QUALIFICATION.md`
- `docs/EFFECT_CONTRACT_V1.md`
- Mechanical record-shape updates in any file that builds a bypass record or cell, including
  `hosts/host-web/src/lib.rs` (D1's zero only; stream H owns it)

## Non-goals

- **The browser's bypass command.** *Crossfade the browser's live bypass command over the session
  ramp* (#1393): host-web fills `ramp_samples` from the `LiveRamps` it keeps for the running
  session (*Resolve an absent live ramp to the session default on the browser and in the SDK*,
  #1364, D2), the `COMMAND_EFFECT_BYPASS`, shipped-host and SDK `bypass` docs, and the gate that a
  C ABI bypass transaction and a browser bypass command render the same bits. It depends on this
  issue and #1364.
- No crossfade for a prepared bypass. The delay and the multiband compressor get shunts in
  *Give the delay a live bypass shunt* (#1339) and *Give the multiband compressor a live bypass
  shunt* (#1340). Once they land, D3 covers them with no further edit.
- No equal-power or raised-cosine shape. #1055's section 5.5 what-if is research, not a decision.
- No ghost-strip crossfade for a plan swap (D15-9 defers it).
- No change to latency, PDC, bank cohorts or the prepared bypass path.

## Hazards

- **Existing equality tests.** A test that pushes a live bypass and compares the next block with
  a prepared-bypass or authored control now sees a ramp. Where the claim is the switch itself,
  author `control_smoothing.mute_ms = 0` (#1054 D6). Otherwise compare after the ramp. Do not
  weaken a comparison.
- **Zero-latency effects** skip the capture on un-bypassed blocks. A ramp toward wet ends on a
  block whose capture must still run (D5).

## Objective gates

1. **The exact law, per node** (`bypass_shunt_identity.rs`, new). Render one per-node live
   instance three times from the same input, as the existing per-node harness does (`:538-608`):
   - W, never bypassed;
   - D, bypassed from preparation;
   - X, bypassed by a record at block 3 with a 480-sample ramp at quantum 128, so the ramp ends
     mid-block.

   Every sample of X before block 3 equals W. Inside the ramp it equals
   `D + (W - D) * m(k)`, with `m` computed independently by `IndexedRamp::new([1,0,0,0],
   [0,0,0,0], 480)`. From the ramp's end it equals D, bit for bit. Run it for a zero-latency effect
   and for one with latency (the true-peak limiter).
2. **Bank equals node** (same file; extend `:610-760`'s live-toggle test). Four lanes (eight on an
   AVX2 width), with toggles on lanes 1 and 3 at different blocks and one reversal mid-ramp. Each
   lane equals the per-node render of the same track bit for bit, and untouched lanes equal W.
   Banking never moves a lane's bits.
3. **Reversal.** In gate 2's reversal, the bypass record lands at block 3 and the un-bypass record
   at block 5, at `k = 256` of the 480-frame ramp. From block 5 the output follows
   `IndexedRamp::new([m(256), 0, 0, 0], [1, 0, 0, 0], 480)`, with no jump, and equals W exactly
   from 480 frames later.
4. **Zero ramp.** With `mute_ms = 0`, every existing shunt-identity and bypass-cohort test passes
   unchanged, and gate 1's X equals D from the first sample of block 3.
5. **C ABI** (`crates/capi/src/runtime/live_tests.rs`). A bypass transaction on a playing session
   follows gate 1's law with the session's `mute_ms`, then equals a control booted bypassed. The
   ramp blocks allocate nothing (`bench_support::alloc` thread counters, statics warmed).
6. **Carry** (`crates/host-core/tests/successor_swap.rs`). A swap at a block inside a ramp renders
   bit-identically to the same session without the swap.
7. **Cross-target.** `bash scripts/check-cross-targets.sh` and the browser legs pass. The mix uses
   no fused multiply-add, so the native, AArch64 and Wasm bits match.
8. **Commands:**
   - `cargo test --locked -p effect-contract`,
     `cargo test --locked -p graph-compiler --test bypass_shunt_identity --test bypass_cohorts`
   - the workspace debug leg and the DSP-crates debug leg, as `.github/workflows/qualification.yml`
     runs them (`test-debug-a`, `test-debug-b`)
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit && ./target/release/audit capi`
   - `bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`,
     `bash scripts/test-web-audioworklet.sh`
   - `cargo run --locked --release -q -p realtime-policy`, `bash scripts/check-effect-runtime-policy.sh`,
     `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
9. **Digests.** A checked-in digest of a render that toggles a live bypass moves. Each one is
   listed in the PR with the reason "bypass crossfade, decision 15 E5" and re-pinned on its own
   (decision 15, risk 4).

## Test value

- Gate 1 turns red if the mix uses `m` from an accumulated sum, or an FMA, if the ramp is
  mis-anchored (off by a frame at the block boundary), or if the after-ramp path computes the
  formula instead of copying. No test checks a ramped bypass today.
- Gate 2 turns red if a bank lane reads a neighbour's `m`, if the select keeps a settled lane on
  the formula path, or if the bank and per-node kernels diverge.
- Gate 3 turns red if a reversal restarts from the settled endpoint (a jump) instead of from the
  current mix.
- Gate 4 turns red if a zero length is not a step, which would move today's bits for an explicit
  0.
- Gate 5 turns red if the C ABI producer forgets the session ramp (the toggle steps).
- Gate 6 turns red if the carry drops the ramp position, so the successor jumps to the endpoint.

## Dependencies

- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054), for `LiveRamps::mute_samples` and D3's row table.
- *Research: default ramp lengths for live mute, fader and pan changes (cited, measured, listened)*
  (#1055), whose decision-15 addition confirms `mute_ms` for the bypass row.
- *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345): D1 widens
  its bypass cell.
- *Carry live-controlled effect lanes across a plan swap* (#1280): D7 extends its carries.

Dependent: *Crossfade the browser's live bypass command over the session ramp* (#1393) depends on
this issue.

## Amendment (decision 17, root 2026-10-13, Q2)

- This slice gains one gate taken from #1563B gate 3: the SDK's `bypass` builder (a `SET_CONTROL`
  on control `0` of an effect section) renders, on the headless engine and through the browser's
  core path, the same crossfade over the session ramp as the C ABI, bit for bit per block.
  #1563B no longer waits for this slice; only that row does.

## Decision record (decision 17, lane K, batch 5)

### The a1 / a2 / b split

The slice ships in three coherent commits, each with its own gates.

| Part | Content | Gates |
|---|---|---|
| **a1** (this commit) | Per node: D1 (the cell and the producers), D2, D3 and D5 (per node), D6; the C ABI producer; the docs; the per-node render and its tests. The bank steps unchanged. | 1, 3 (per node), 4, 5, 7, 9 |
| **a2** (next) | D4's banked kernel: the interleaved mix with a per-lane select, the bank's capture and restore. | 2, 3 (banked) |
| **b** (after #1280) | D7: the mix ramp and its position carried across a plan swap in #1280's move-mode lane carry. | 6 |

The SDK `bypass` builder row (Q2 amendment) is **pending**: its browser leg runs only after #1562a
(the browser core path), and its headless leg needs the browser's `LiveRamps` of #1364. Nothing in
a1 builds it.

### Adaptations to today's code

- **The cell is already a latest-target cell** (#1345 landed): `EffectCells::bypass` is a
  `LatestCell<2>` now (`bypassed`, `ramp_samples`), read in the one drain.
- **The record keeps its shape.** `EffectControlRecord::Bypass(bool)` is matched exhaustively by the
  browser host, whose source another lane owns, so it stays the *step* spelling (the same write with
  a ramp of 0, which is D1's browser value). The ramp-carrying unit is `BypassSwitch { bypassed,
  ramp_samples }`, written by `EffectCellWriter::write_bypass` and
  `EffectControlProducer::write_bypass` (preflight `check`/`preflight_bypass`, refusal
  `EffectWriteRefusal::Ramp` above `INDEXED_RAMP_LENGTH_MAXIMUM`). The classifier's
  `LiveEffectRecords::bypass_ramp_samples` carries the session ramp beside the `Bypass` record, and
  the control plane writes the pair as one `BypassSwitch`. Folding the ramp into the record itself,
  and giving the browser's `COMMAND_EFFECT_BYPASS` the session ramp, is #1393 with the browser lane.
- **Registry.** The bypass row's motion is `Motion::Fixed` (no per-edit length, so `SET_CONTROL`
  on control 0 cannot carry a length) and `LiveRampRow::BypassCrossfade` maps to `mute_ms`; the
  delay and the multiband bypass stay `rebuild_only` through `lowers_session_bypass`. The control
  list now reports the bypass row's `default_ramp_samples` as the session's mute ramp (was 0).
- **Ramp index.** Frame `f` of a block at ramp position `p` takes `k = p + f + 1`, the convention of
  `route_mix_ramp_block`: the first ramp frame is already one step along, the ramp reaches its target
  on the `length`-th frame, and `m_now` at a reversal is `m(p)`, the last rendered frame's mix. The
  settled frames are never computed.
- **Lane API.** `needs_dry()` (bypassed or ramping, D5), `bypass_ramping()`, `bypass_mix()`,
  `apply_bypass::<L>(shunt, left, right)` (mix and advance) and `advance_bypass_mix(frames)` on
  `EffectControlLane`; `BypassShunt::crossfade::<L>` is the kernel, with the outlined `f32` tail.
  D6: a record admits `Bypass(true)` into the witness whenever the target is dry or the ramp is
  longer than 0; the advance that settles a lane at wet admits `Bypass(false)`.
- **The bank, until a2.** `LiveControlEffectBankStage` still restores dry words by the lane's
  target (`bypassed()`), so a banked lane steps at its record exactly as before. It calls
  `advance_bypass_mix(frames)` once per block so each lane's ramp state, and the `UNBYPASSED` term it
  restores, stay in step with the timeline. A console slot always banks, so on the C ABI only a
  per-node insert (a remainder group) crossfades until a2 lands; a2 deletes the interim.
- **Test adaptations (hazard 1).** The C ABI tests that compare a live bypass with a rebuild wait
  `window(latency, mute ramp, quantum)`, and the two that compare with the browser's stepping lane
  author `control_smoothing = hard switch`.

### Evidence

- `bypass_shunt_identity.rs`: the exact law per node (gate 1) for a zero-latency effect (compressor)
  and one with latency (true-peak limiter), at the host width and at scalar width; the reversal at
  `k = 256` and a repeated target (gate 3, per node); the zero ramp (gate 4).
- `live_control.rs`: the witness, `needs_dry`, the repeated target, a zero ramp and the refusal.
- `live_delta.rs`: the record's ramp is the session's `mute_ms` (no per-edit length) and a ramp above
  the law's bound is `Domain`.
- C ABI (gate 5, `live_tests.rs`): a bypass transaction follows the law with the session's
  `mute_ms` and lands on a control booted bypassed, for a zero-latency effect and one with
  latency, at `mute_ms` 10 and 0, with no allocation in a ramp block. The law holds there to a few ulps
  of the block peak, because the strip's gain stages follow the effect and round; gate 1 pins it
  exactly at the effect's output.

## Decision record: #1341a1 (batch 8)

The per-node crossfade (commit `b6bbc44e3`, built on a pre-flip base) was re-applied onto `main` at
`32cee58d0`, after the batch 6-7 flip made the browser a thin client of the core's `SessionState`.

### What shipped

The a1 content of the batch 5 decision record above, unchanged: `BypassSwitch { bypassed,
ramp_samples }` as the two-word bypass cell, the per-node lane crossfade (`EffectControlLane`,
`BypassShunt::crossfade`), `LiveEffectRecords::bypass_ramp_samples` filled from the session's
`mute_ms`, the control plane's `write_effect_record`, the bypass row's `default_ramp_samples`, and
the docs. A bank (a console slot, or any banked cohort) still steps; that is a2.

### Departures from the batch 5 text, and why

- **The browser needs no separate producer.** `COMMAND_EFFECT_BYPASS` and the browser's own live
  lane are gone from `main`: the browser sends `SET_CONTROL` frames into the core, whose control
  plane writes the `BypassSwitch` (with the session ramp) for every host. So #1341a1 crossfades the
  browser too, and #1393 (already superseded by #1563) has no work left. D1's "browser writes a ramp
  of 0" no longer applies. The stale comments that said so were corrected
  (`EffectControlRecord::Bypass`, `live_tests.rs`, `C_ABI_V1_QUALIFICATION.md`).
- **D7 (carry) does not apply now.** The #1280 carries are not on `main`, and
  `crates/engine/src/realtime/latest_cell.rs` is not touched. The mix ramp and its position `k` are
  carried by #1280's move-mode lane carry; that extension moves with #1341b.

### Amendment gate (SDK `bypass` builder, from #1563B gate 3)

The browser leg is reachable now, as the browser is the core's `SESSION_STATE` through frames. The
gate is three tests, one per layer, because the C ABI, the headless engine and the browser host run
one core:

- C ABI: `capi` `a_live_bypass_crossfades_over_the_session_mute_ramp_and_lands_on_the_bypassed_control`
  (the law, at `mute_ms` 10 and 0, with zero allocation in a ramp block).
- C ABI against the browser host, bit for bit per block, at 44.1, 48, 88.2 and 96 kHz:
  `hosts/host-web/tests/control_parity.rs` `a_live_bypass_crossfades_alike_on_both_hosts` (new), plus
  the scripted insert and console-slot bypass rows of
  `both_hosts_give_the_same_result_for_the_same_control_edits`. The new test fails on a step (a step
  spans at most one block, a ramp spans `ceil(mute_ms * rate / 128)` or one more) and on a wrong
  length, and requires the render to equal the booted-bypassed one from the ramp's end.
- SDK: `sdk/test/live-controls-evals.mjs` "the SDK bypass builder crossfades an insert over the
  session ramp": the builder's `SET_CONTROL`, through the headless engine and through the browser
  transport (`createBrowserLiveControls` over `engineHost`), renders the same bits in every block,
  with the same count of crossfade blocks as the law, and settles on the booted-bypassed document.
  The "effect bypass on a console slot" row left `todo` and gates (it settles on the booted
  document; the console slot steps until a2).

### Test adaptations (hazard 1)

- `sdk/test/console-evals.mjs`: the two evals that compare a live insert bypass with the authored
  one author `controlSmoothing` of zero (`HARD_SWITCH`); the claim there is addressing, not fading.
- `hosts/host-web/tests/control_admission.rs`: the insert "bypass" row compares with the fresh plan
  from block 16 (after the ramp and the strip's delay compensation) instead of from block 0.

### Conflicts met on `main`

`crates/control-plane/src/control.rs` (the effect-record write loop moved under the `if let
Some(stamp)` publication block: kept main's shape, both write sites call `write_effect_record`) and
`crates/host-core/src/control_provider.rs` (`control_row` became `Shape::of`: the bypass arm of the
`Motion::Fixed` match moved there).

## Decision record: #1341a2 (written in batch 8, lands in batch 9)

### What shipped

D4's banked kernel, and with it the deletion of the a1 interim. A bank now renders each lane's
bypass crossfade exactly as the per-node path renders it, so a placement change (a console slot, a
cohort of 4-7 inserts on a 4-lane target, a remainder insert) moves no rendered bit.

- `BypassShunt::crossfade_bank::<L>` (`crates/effect-contract/src/live.rs`, a realtime-policy
  region): the interleaved block holds `L::WIDTH` words per frame, one per bank lane. Per frame and
  channel one vector op computes each lane's `m = fma(k, step, start)` from a vector of per-lane ramp
  indices `k = position + f + 1` (one `add` per frame advances all of them; exact below `2^24`) and
  the mix `dry + (wet - dry) * m` as a separate subtract, multiply and add, the per-node order.
  `select(k < length, mix, select(settled_dry, dry, wet))` then keeps the wet words of a settled-wet
  lane and the dry words of a settled-dry lane (bit selects: `-0.0`, NaN payloads and subnormals pass
  untouched, and a settled lane never goes through the formula). A bank with no ramping lane skips
  the mix and only selects, which replaces the old scalar strided restore loop with a vector select
  over the same words.
- State: `BankCrossfade` (five `[f32; 8]` arrays and a flag, inline in `LiveControlEffectBankStage`,
  no allocation, nothing new in the heap resource report) is rewritten every block from
  `EffectControlLane::bypass_crossfade_lane()`; a lane with no control lane is `BankCrossfadeLane::WET`.
- The stage (`crates/rack/src/lib.rs`): the dry block is captured when any lane `needs_dry()` (D5,
  was `bypassed()`), the kernel runs after the bank and the taps, and the lanes then advance by the
  block (`advance_bypass_mix`, kept: it is the lane's state step and restores the witness term). The
  interim, "restore dry words by the target and only advance", is gone. The collapsed (mono) body
  shares the code: a ramping lane counts as bypassed for the witness, so a collapsed block has no
  ramping lane.

### Gates and tests

- Gate 2 (bank equals node) and gate 3 banked:
  `graph-compiler` `bypass_shunt_identity` `a_bank_crossfades_every_lane_exactly_as_the_node_does`.
  The real `LiveControlEffectBankStage` through `BankChain` at the build's bank width, with eight
  lanes' schedules (four on a 4-lane build): a bypass over 480 frames that ends inside a block (the
  per-node kernel's scalar tail), a reversal at `k = 256`, a lift over 300, a step, a lane bypassed
  without a channel, a lane without a control lane, a never-switched lane, and a lane whose ramp is
  exactly one block followed by a bypass. Every lane equals its own per-node render (`needs_dry`,
  capture, effect, `apply_bypass`) bit for bit in every block, for the compressor (no latency) and the
  true-peak limiter (latency). Red when the bank steps: first diff is lane 1, block 3, a `-0.0` where
  the node has `+0.0`.
- Widths. An effect binds only the build's own bank width (no runtime dispatch, D4), so the stage
  test runs at the host width (8 lanes on this AVX2 host). The 4-lane and 8-lane kernels are both
  pinned lane by lane without a processor in `effect-contract`
  `live_control::a_bank_kernel_crossfades_every_lane_exactly_as_the_node_kernel_does`: eight lane
  schedules (ramp, reversal, lift, step, never, session-bypassed, short ramps, a ramp ending
  mid-block), wet words with `-0.0`, infinities and NaNs, at `Simd4` and `Simd8` against the per-node
  `apply_bypass` at the host width. The scalar tail is the per-node kernel's (the bank has none: its
  frame is exactly one vector). The 4-lane build itself runs the stage test under
  `scripts/run-aarch64-tests.sh` and, as simd128, the SDK headless evals below.
- M1 (lift through the real runtime): `capi`
  `a_live_bypass_crossfades_over_the_session_mute_ramp_and_lands_on_the_bypassed_control` now runs,
  per effect (compressor with no latency, soft clip with 31), per `mute_ms` (10, and 0), a bypass and
  a lift, and the effect as an insert (a remainder, rendered per node) and as a console slot (always
  banked). The lift boots the document with the effect bypassed, commits the lift, and asserts
  `D + (W - D) * m(k)` with `m` running 0 to 1 frame by frame, then W exactly, with no allocation in
  a ramp block. Mutation: a lift ramping the wrong way, and a bank that steps, each turn it red.
- `render_per_node` in `bypass_shunt_identity.rs` is **kept**: the C ABI test holds the law to a few
  ulps of the block peak (the strip's gain stages round after the effect), and it is the only test
  that pins the law exactly at the effect's output and at scalar kernel width.
- SDK: `sdk/test/live-controls-evals.mjs` "the SDK bypass builder crossfades ..." now has an insert
  case and a console-slot case (headless and browser transport, bit for bit per block, the ramp's
  block count, settles on the booted bypassed document). The console-slot row of
  `liveEqualsBooted` keeps its settle check, its comment now says the bank crossfades.
- `hosts/host-web/tests/control_parity.rs`: the scripted console-slot bypass rows already compare the
  two hosts bit for bit; the module doc no longer says the bank steps. A console-slot variant of
  `a_live_bypass_crossfades_alike_on_both_hosts` was tried and dropped: this session's strips hold
  effects after the console slot, whose history outlasts the ramp, so the render is not equal to the
  booted bypassed one for 55 blocks (a property of the fixture, not of the crossfade).

### Hazard 1 adaptation

`capi` `a_live_eq_bypass_beside_a_gain_change_renders_like_the_browsers_lane`: the console EQ banks
and stepped before a2; it compares with a reference lane pushed a bare `Bypass` record (a step), so
its session authors the hard switch, as the insert test beside it already does.

### Docs

`docs/C_ABI_V1_QUALIFICATION.md` (the bank crossfades, no per-node-only sentence),
`docs/EFFECT_CONTRACT_V1.md` (the bank kernel paragraph), the rack stage's docs.

### Evidence

No checked-in digest moved: `check-effect-runtime-fixtures.sh` and `check-console-fixtures.sh` pass
unchanged (none of them toggles a live bypass).
