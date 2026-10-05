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

- **The shunt** (`crates/effect-contract/src/live.rs:822-998`). The wet path always runs. The dry
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
  (#1280 D2-D6). This slice lands after both (plan risk 3) and extends that carry.

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
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-effect-runtime-policy.sh`,
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
