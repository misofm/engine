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
  (Attempt 2, verify MAJOR: `createHost` is public, so an app's own host can drop `bootCommand`.
  The browser engine therefore sends the same `SPECTRUM_CONFIGURE` after boot as a confirmation:
  when the boot carried the taps it is the equal configuration, `rebuild` false at epochs 1/1, and
  counts no rebuild; a reply that rebuilt or whose epochs differ fails `createEngine` with a
  `boot` error naming `bootCommand`, diagnostic `sdk.boot.command`. The headless boundary stages
  its own command, so no injected host can drop it there.)
- **D6. Behaviour change.** A boot spectrum configuration the engine refuses now fails the boot
  (`refusedDocument`/`refusedBudget`, with the spectrum code as the diagnostic) instead of the
  spectrum refusal error. The browser host carries only the result code. Recorded in
  `sdk/README.md` "Upgrading from 0.8.0"; mapping it back was not small (a code-to-status table
  on two paths, and no diagnostics on the browser path).
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

Attempt 1, branch `codex/issue-2166-boot-spectrum` (from `main` `0d326c5c8`, merged with
`origin/main` `3374b21a0` after the 0.8.0 release).

- **Change.** `crates/control-plane/src/compile.rs` (`compile_children_with_boot_command`,
  `boot_command_spectrum`, the boot preparation and monitor adoption in `compile_children_warm`);
  `crates/control-plane/src/control.rs` (`checked_spectrum_configuration`, shared by
  `spectrum_configure` and `boot_spectrum_configuration`); `hosts/host-web/src/lib.rs`
  (`AudioWorkletEngineHost::boot_with_command`, `compile_ready`); `hosts/host-web/src/ffi.rs`
  (`miso_engine_web_v1_boot_command_ptr`, `BootStaging::command`); the worklet and its host
  (`bootCommand`); `sdk/src/core/boundary.ts` (`bootSpectrumCommand`, staging in `stage`);
  `sdk/src/browser/engine.ts` (`createHost`'s `bootCommand`, `requireBootSpectrumCapability`).
- **Red on named mutants** (each restored after):
  - Gate 1 is red on the defect itself (the host boots plain, then runs the frame through the
    command entry before the first block): `the boot rebuilt nothing`, left `1`.
  - Gate 1 is red when the boot drops the configuration (`boot_spectrum` filtered to `None`) and
    when the monitor skips the boot adoption: the stream start is refused.
  - Gate 2 is red when `run_control` counts a rebuild only when the committed revision changed:
    left `0`. `hosts/host-web/tests/control_parity.rs` stays green on that mutant, so no existing
    test catches it.
  - Gate 3 (SDK) is red when the browser engine leaves `bootCommand` out of the `createHost`
    request: the parity row and the five other served-core spectrum evals fail.
- **Parity rows.** `scripts/test-web-audioworklet.mjs`: the host hands the worklet a private copy
  of `bootCommand` and refuses an empty or non-`Uint8Array` one; the processor stages it through
  `boot_command_ptr` before `boot`. `sdk/test/boot-evals.mjs`: the headless boot's and reboot's
  boot command carries the hop, and no `SPECTRUM_CONFIGURE` frame follows the boot.
- **Gates run on the merged tree, all green:** `cargo fmt --all -- --check`; `cargo clippy
  --locked --workspace --all-targets --all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings'
  cargo doc --locked --workspace --no-deps`; every policy script of the lint job, including
  `check-lane-policy.sh`; the workspace debug tests and doctests with the job's excludes and
  features; `cargo build --release -p realtime-policy && ./target/release/realtime_policy`;
  `scripts/run-wasm-gates.sh`; `check-web-audioworklet.sh --without-metadata-regeneration` (export
  set, call graph and vector roster), `check-browser-expected-resources.py --artifacts`,
  `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh`; the SDK job
  (`check-sdk-generated.sh`, `check-sdk-deletions.py`, `check-sdk-types.sh`,
  `check-sdk-headless.sh`, `sdk-package.sh check`) and the source-only asset tests.
- **Attempt 2** (verify of `0e2af089e`: one MAJOR, three MINORs).
  - MAJOR: the browser confirmation (D3). `spectrum-browser-evals.mjs` "a host that drops the boot
    command fails the boot, naming bootCommand" is red on `0e2af089e`'s `engine.ts` and on the
    mutant that skips the rebuild/epoch check. Gate 1 now also sends the confirmation and asserts
    `rebuild` false, epochs (1, 1) and `blocking_rebuilds() == 0`.
  - MINOR 1: `control-plane/tests/spectrum_monitor.rs`
    `a_boot_command_that_is_not_a_spectrum_configure_refuses_the_boot` (malformed and wrong-kind
    frames refuse with `control.boot.command`), red on the wrong-kind `else` arm returning
    `Ok(None)`; `host-web` `a_refused_boot_takes_the_staged_boot_command`, red on moving the
    `mem::take` below the document check.
  - MINOR 2: `miso_engine_web_v1_document_ptr` frees a staged command.
  - MINOR 3: recorded as D6.
- **Not run locally:** the three-browser qualification and the AArch64 jobs. The release pin
  `miso-engine-v1-audio-worklet-artifact.sha256` is left at 0.8.0's: the module changes (one new
  export), and the 0.8.1 release re-pins it (docs/RELEASE.md).
- **SDK surface.** Additive only: `CreateEngineOptions.createHost`'s request and the shipped
  host's `CreateMisoAudioWorkletHostOptions` gain the optional `bootCommand`; the generated ABI
  export list gains `miso_engine_web_v1_boot_command_ptr`. No public call changes.

### Attempt 2 verdict (bounded opus-high verify, 2026-10-27): PASS

- The confirming `SPECTRUM_CONFIGURE` is free when the boot carried the taps: `spectrum_configure` returns early on an equal configuration (no rebuild, no epoch or revision change, no watermark effect), and rendering cannot change the monitor's configuration between boot and confirmation.
- The failure path disposes the host (time-bounded) and closes the context; no subscription exists yet.
- Red evidence confirmed for `a_refused_boot_takes_the_staged_boot_command` and `a_boot_command_that_is_not_a_spectrum_configure_refuses_the_boot`.
- Folded MINOR: sdk/README's "Upgrading from 0.8.0" lists the custom-`createHost` case under "Changed" (a host that drops `bootCommand` now fails `createEngine` loudly; the owner chose no boot rebuild, so the silent 0.8.0 behaviour is not kept).
