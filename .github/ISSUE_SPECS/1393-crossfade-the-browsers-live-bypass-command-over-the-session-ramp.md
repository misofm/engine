# Crossfade the browser's live bypass command over the session ramp

Stream E of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-13 E5).
Code anchors verified on `main` at `6fb211594`.

Split from *Crossfade the bypass switch over the session ramp* (#1341), which delivers the
crossfade in render, the plan-swap carry and the C ABI producer. This issue owns the browser
decode, the browser parity gate, and the host-web and SDK docs.

## Product outcome

A browser app that toggles an effect's bypass live (`COMMAND_EFFECT_BYPASS`, the SDK's
`bypass(enabled)`) hears the same linear crossfade as a C ABI bypass transaction: over the
session's mute ramp (`control_smoothing.mute_ms`, or the default table), from the next block. A
session whose `mute_ms` is 0 renders exactly today's step. For the same session and the same
toggles, the browser and the C ABI render the same bits.

## Context

- **What #1341 delivers first.** The live bypass travels as one two-word latest-target cell,
  `bypassed` and `ramp_samples` (#1341 D1, on the cell of *Hold effect parameter, bypass and
  EQ-target values in latest-target cells*, #1345). Render crossfades over `ramp_samples` (#1341
  D2-D4); a length of 0 is today's whole-block select. The C ABI classifier fills the length from
  `LiveRamps::for_session(next).mute_samples`. The browser decode writes `ramp_samples = 0` (#1341
  D1), so the browser still steps.
- **The browser decode.** `CommandRecord::into_effect_records`
  (`hosts/host-web/src/lib.rs:4484-4501`) admits `COMMAND_EFFECT_BYPASS` only with channel 255,
  parameter 0, a zero `smoothing_samples` word and zero trailing values, and refuses anything else
  as `COMMAND_REASON_MALFORMED`; a value other than 0 or 1 is `COMMAND_REASON_DOMAIN`. It emits
  `EffectControlRecord::Bypass(bool)` (`:4500`).
- **The session's lengths in the browser.** *Resolve an absent live ramp to the session default on
  the browser and in the SDK* (#1364, D2) has host-web compute `LiveRamps::for_session` from the
  session it prepared and keep it with the live controls. It keeps the bypass word's zero rule: a
  sentinel or any non-zero word on bypass is `MALFORMED`. #1364 also adds the cross-host parity
  test `hosts/host-web/tests/control_smoothing_parity.rs`, with `capi` as a host-web
  dev-dependency.
- **Docs that still describe a step.** `COMMAND_EFFECT_BYPASS` (`hosts/host-web/src/lib.rs:836-844`),
  the `EffectBypass` kind in `sdk/src/browser/shipped-host.d.ts:202-209`, and the SDK's `bypass`
  (`sdk/src/core/live-controls.ts:931-943`).
- **The bypass has no per-edit length.** Decision 15, D15-1 recorded resolution:
  the bypass crossfade alone always uses the session mute ramp. *Carry an optional per-edit ramp
  length on live session edits* (#1394) gives no length to `SetEffectBypass`.

## Decisions frozen for this slice

- **D1. The length.** The browser's bypass admission writes the cell's `ramp_samples` from the
  `LiveRamps` #1364 keeps for the running session: `mute_samples`. It is read on the control
  thread at admission, never in render.
- **D2. The word stays reserved.** The command's `smoothing_samples` word stays required zero. A
  non-zero word, the `SESSION_DEFAULT_RAMP` sentinel included, is `MALFORMED`. The SDK's `bypass`
  keeps writing 0 and gains no ramp option.
- **D3. Where admission lives.** If *Admit browser live edits in the Worker through the committed
  model* (#1382) has merged when this lands, the bypass command lowers to `SetEffectBypass` and
  the shared classifier's #1341 producer already fills the length. This slice then changes no
  admission code and delivers D4 and the gates. Otherwise D1 goes in `into_effect_records`.
- **D4. Docs.** `COMMAND_EFFECT_BYPASS`, the shipped-host `EffectBypass` kind and the SDK's
  `bypass` say: the toggle crossfades linearly between the wet output and the latency-matched dry
  signal over the session's mute ramp; `mute_ms = 0` is a step; the effect's latency never
  changes.
- **D5. Acked-batch question.** No queue, admission order or ack changes; the record carries a
  different length. A bypass is never acked without effect: its ramp starts at the next block.

## Deliverables

1. D1 and D2 in `hosts/host-web/src/lib.rs` (or nothing there under D3).
2. D4 in `hosts/host-web/src/lib.rs`, `sdk/src/browser/shipped-host.d.ts` and
   `sdk/src/core/live-controls.ts`.
3. The tests below.

## Authorized paths

- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/control_smoothing_parity.rs` (stream H owns host-web; root sequences the
  merge)
- `sdk/src/browser/shipped-host.d.ts` and `sdk/src/core/live-controls.ts` (docs only),
  `sdk/test/live-controls-evals.mjs`

## Non-goals

- The render crossfade, the carry and the C ABI producer (#1341).
- A per-edit bypass length (D15-1: none exists).
- Resolving absent lengths for the other rows (#1364).

## Hazards

- **Session replacement.** If *Replace the running browser session in the Rust host* (#1290) has
  merged, the `LiveRamps` read in D1 must be the replaced session's. Read it from where #1364 keeps
  it for the running session; never cache a boot copy.
- **Existing browser tests** that toggle a live bypass and compare the next block with a prepared
  or authored control now see a ramp. Where the claim is the switch, author `mute_ms = 0`;
  otherwise compare after the ramp. Do not weaken a comparison.

## Objective gates

1. **Admission** (`hosts/host-web/src/tests.rs`, new). At 48 kHz with no `control_smoothing`, an
   admitted bypass command carries `bypassed` and `ramp_samples = 480`; with `mute_ms = 0` it
   carries 0. A word of 1 or of `u32::MAX` is `MALFORMED`. (Under D3 the same claims are made on
   the lowered transaction's committed record.)
2. **Parity** (`hosts/host-web/tests/control_smoothing_parity.rs`, extended). One session with two
   tracks and a live-controlled compressor insert on one, at 44.1 and 96 kHz: a bypass at a fixed
   block, a reversal 64 frames into the ramp, and a later bypass. The browser (native host-web,
   bypass commands) and the C ABI (`capi`, `SetEffectBypass` transactions) render bit-identical
   output on every block. The same script with `mute_ms = 0` is bit-identical between the hosts
   too.
3. **Allocation.** The render blocks of gate 2 allocate nothing on the browser host (the existing
   host-web allocation gate passes).
4. **Commands:**
   - `cargo test --locked -p host-web --features test-support`, `cargo test --locked -p capi`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`
   - `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
     `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
5. **Digests.** A checked-in browser digest of a render that toggles a live bypass moves. Each one
   is listed with the reason "browser bypass crossfade, decision 15 E5" and re-pinned on its own.

## Test value

- Gate 1 turns red if the browser keeps writing 0 (the toggle steps), reads the fader or pan key,
  or lets a non-zero word through as a length.
- Gate 2 turns red if the two hosts disagree on the length, on the block the ramp starts, or on
  the reversal's anchor. No test compares a bypass across hosts today.

## Dependencies

- *Crossfade the bypass switch over the session ramp* (#1341).
- *Resolve an absent live ramp to the session default on the browser and in the SDK* (#1364): the
  `LiveRamps` host-web keeps, and the parity test this slice extends.
