# Boot spectrum configuration adopts without a blocking rebuild

Filed by root for the console (misofm/studio slice C1g-2), 2026-10-27. Code anchors verified on
`main` `0d326c5c8`.

## Problem

The browser SDK's `createEngine({ spectrum })` sent one `SPECTRUM_CONFIGURE` before any block
rendered (`sdk/src/browser/engine.ts:566-605`, `configureBootSpectrum`; the headless
`WasmBoundary` did the same in `#configureBootSpectrum`). The control plane prepared a successor
plan with the taps and, as no block had rendered, adopted it in place of the boot plan (#1597,
`SessionState::command_with_plan`). That is a new plan epoch, and
`hosts/host-web/src/lib.rs:906-938` (`run_control`) counts every epoch change as a blocking
rebuild. So a console that only sets a boot spectrum tap reported `blockingRebuilds == 1` before
it played a sample, and the boot prepared two plans. The console asserts `blockingRebuilds == 0`
and is blocked on this.

## Owner decision (2026-10-27)

Fix it in the engine and ship it in 0.8.1. The boot spectrum tap must be part of the first plan,
so there is no rebuild at all. A `configureSpectrum` call while the session runs stays a counted
rebuild.

## Smallest closable slice

- **D1. The boot carries the taps.** The control plane gains
  `compile_children_with_boot_command(document, limits, preparer, frame)`: it decodes one command
  frame under the session's frame limit, accepts only `SPECTRUM_CONFIGURE` (any other frame, or one
  that does not decode, refuses the boot with `control.boot.command`), checks the configuration
  exactly as the command checks it (one shared function, `checked_spectrum_configuration`: request
  rules, `spectrum.target.unknown`, `spectrum.duplicate`, `spectrum.frame`), prepares the first
  plan with the taps, and starts the spectrum monitor at configuration and adopted epoch 1 with its
  service made from the first plan's captures. The frame's request ID and expected revision are not
  read: a boot has no reply and starts the revision. No successor is prepared.
- **D2. The browser stages it.** One optional boot staging export,
  `miso_engine_web_v1_boot_command_ptr(len) -> u32`, stages the frame beside the document; `boot`
  takes it whatever its outcome, and dispose frees one no boot took. The browser decodes nothing in
  it (the core decodes, as for the command entry). The four-call staging sequence is unchanged; the
  export is not spectrum-specific, so #1597 D1 (no browser spectrum export) holds.
- **D3. The SDK sends the taps in the boot.** The headless `WasmBoundary` stages the frame
  (`bootSpectrumCommand`); the browser engine hands it to its host as `createHost`'s optional
  `bootCommand`, which the shipped worklet host copies and the worklet stages. Neither sends a
  boot `SPECTRUM_CONFIGURE`; both keep the `CAPABILITIES_GET` bit-16 check after boot.
- **D4. The counter is unchanged.** `run_control` still counts every published plan, so a running
  `configureSpectrum` counts one blocking rebuild. A counter exception ("an epoch adopted before
  the first block is not a rebuild") was rejected: it would hide a second plan preparation the boot
  no longer needs.
- **D5. The C ABI.** `miso_engine_v1` has no boot spectrum option and no blocking-rebuild counter,
  so nothing changes there; a C ABI boot option would call the same control-plane entry.

## Non-goals

- No change to `SPECTRUM_CONFIGURE` itself, to the in-place adoption before the first block
  (#1597), or to the C ABI.
- No new spectrum feature, export or record.

## Authorized paths

`crates/control-plane/src/`, `hosts/host-web/src/`, `hosts/host-web/web/`, `sdk/src/**`,
`sdk/test/**`, `sdk/assets/miso-engine-v1-abi-layout.json` and `sdk/src/generated/abi.ts`
(regenerated), `tools/parameter-metadata/src/abi_layout.rs`, `scripts/check-abi-layout-v1.py`,
`scripts/fixtures/abi-layout-v1-self-test.json`, `scripts/check-web-audioworklet.sh`,
`scripts/check-session-map-shape.py`, `scripts/test-web-audioworklet.mjs`, this spec.

## Objective gates

1. **Claim (a).** `hosts/host-web/src/tests.rs`
   `a_boot_spectrum_tap_is_the_first_plans_and_streams_with_no_rebuild`: a boot with a staged
   output-tap command reports `blocking_rebuilds() == 0`, the stream starts at adopted epoch 1, and
   after one 2048-frame window a stream read is `Ready` with both planes' bins; still 0 after.
2. **Claim (b).** `a_running_spectrum_configure_counts_one_blocking_rebuild`: after one rendered
   block, a `SPECTRUM_CONFIGURE` that adds a tap reports `rebuild` and `blocking_rebuilds() == 1`.
3. **SDK parity row.** `sdk/test/spectrum-browser-evals.mjs` "the browser boot hands its taps and
   the policy's hop to the host as the boot command": the host is booted with only the command the
   engine handed it, no `SPECTRUM_CONFIGURE` frame is sent, and the stream runs at the policy's hop.
4. The lint job, the workspace debug tests, the realtime policy, the Wasm gates, the AudioWorklet
   artifact gates and the SDK job pass.

## Test value

- Gate 1 turns red if the boot drops the command, if the monitor does not adopt the first plan's
  captures, or if the taps are set by a plan that replaces the boot's (the defect).
- Gate 2 turns red if the browser stops counting a rebuild that commits no edit, which a fix of
  claim (a) that excused the epoch change would do; no structural-edit test catches it.
- Gate 3 turns red if either SDK boot path still configures the taps after boot, or the hop does
  not reach the boot command.

## Evidence

(filled in by the implementing attempt)
