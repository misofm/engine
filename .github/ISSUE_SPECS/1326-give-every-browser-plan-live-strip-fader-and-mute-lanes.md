# Give every browser plan live strip fader and mute lanes

Stream D of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Every plan the browser engine prepares carries each strip's live fader/mute and matrix/pan lanes,
whatever the app's live-control options say. The engine can then ramp any strip's mute on a
running browser plan, which a D15-9 transition (duck-swap, two-phase removal) needs. An app that
did not opt in to live commands still cannot send them, and its rendered output does not change by
one bit.

## Context

- **The C ABI already has these lanes on every plan (verified, no work here).**
  `prepare_runtime` (`crates/control-plane/src/compile.rs:578`) always passes
  `control_queue_depth: Some(LIVE_QUEUE_DEPTH)` (`:594-597`) with `C_ABI_LIVE_LANES` (`:25-28`,
  `FADER_AND_MATRIX` plus effect lanes). Its only callers are the boot compile (`:721`) and the
  structural arm (`crates/control-plane/src/control.rs:1043`), which prepares every successor with
  the same lanes. host-core attaches one control request per strip, tracks then submixes
  (`crates/host-core/src/prepare.rs:1421-1443`). The test
  `a_live_fader_after_a_carrying_structural_commit_reaches_the_successor`
  (`crates/capi/src/runtime/live_tests.rs:930`) proves a successor's lanes work.
- **The browser attaches them only on opt-in.** `live_control_request`
  (`hosts/host-web/src/lib.rs:7221`) maps `live_control_command_queue_records == 0` to
  `control_queue_depth: None` (`:7225-7228`), and host-core then attaches no lane at all
  (`HostLiveControlRequest`, `crates/host-core/src/prepare.rs:309-321`). The SDK's default policy
  is 0 (`sdk/test/browser-defaults-evals.mjs:422-456`), so fan playback has no live mute.
- `compile_ready` (`hosts/host-web/src/lib.rs:6617`) prepares through one of three host-core
  entries: `prepare_host_runtime_with_live_controls_and_spectrum` (`:6646`),
  `prepare_host_runtime_with_live_controls_and_spectrum_collection` (`:6661`) and
  `prepare_host_runtime_with_selected_meters_between_render_calls` (`:6675`). All three pass
  `HostLiveLanes::ALL` inside host-core (`crates/host-core/src/prepare.rs:1078-1088`, `:843-900`);
  none takes a lane selection. `HostLiveLanes::FADER_AND_MATRIX` (`:390`) exists.
- The browser infers "live commands enabled" from the strip producers being present:
  `live_controls_attached` (`hosts/host-web/src/lib.rs:3029-3033`, gate at `:3334`),
  `ready.controls.is_empty()` (`:4899`, `:4943`), and `handles.strip_controls.is_empty()` for the
  input-filter shadows (`:6715`) and the live VCA state (`:6989`).
- A settled live fader lane runs the same `gain_mute_block` arithmetic as the prepared-only stage
  (`FaderRampStage` doc, `crates/builtins/src/lib.rs:2640-2652`).

## Decisions frozen for this slice

- **D1. Lanes always.** `live_control_request` always returns `control_queue_depth: Some(..)`:
  the app's `live_control_command_queue_records` when it is nonzero, else
  `host_core::STRIP_LIVE_QUEUE_DEPTH`.
- **D2. One depth constant.** host-core exports `pub const STRIP_LIVE_QUEUE_DEPTH: NonZeroUsize`
  = 16 (#1053 D4's value). capi's `LIVE_QUEUE_DEPTH` (`crates/control-plane/src/compile.rs:17`)
  becomes an alias of it, so both hosts read one number.
- **D3. Lane selection.** With a nonzero option the browser keeps `HostLiveLanes::ALL` (today's
  behaviour). With 0 it selects `HostLiveLanes::FADER_AND_MATRIX`: no input, effect or route lane,
  so no strip's builtin tail turns infinite and no route loses its fold. The three host-core entries
  above gain a trailing `lanes: HostLiveLanes` argument; every existing caller passes
  `HostLiveLanes::ALL`, which keeps its behaviour.
- **D4. The public command gate keys on the option, not on the lanes.** `ReadyOwnership` gains
  `commands_enabled: bool` (= option nonzero). `live_controls_attached`, the two
  `ready.controls.is_empty()` refusals and the two `handles.strip_controls.is_empty()` builders
  read it instead. With 0, every command is refused exactly as today
  (`RESULT_UNSUPPORTED`, `COMMAND_REASON_UNSUPPORTED_KIND`), and no input-filter shadow or VCA state
  is built. The lanes serve the engine's own transitions and, later, `engine.apply` (D15-11).
- **D5. Memory is measured once and accepted** (round 1 C5.4). The PR records the retained bytes
  per strip that D1 adds for a session with option 0, from the boot resource report, and re-pins
  any `expected.json` ceiling the rows exceed, each with its reason.
- **D6. Realtime.** No render-path change: the lanes exist, settled, and receive no record.

## Deliverables

1. D1-D4 in `hosts/host-web/src/lib.rs` and `crates/host-core/src/prepare.rs` (plus the
   re-export in `crates/host-core/src/lib.rs`), the alias in `crates/control-plane/src/compile.rs`.
2. Call-site updates for the new argument (D3) in host-core tests and `tools/console-workload`.
3. Tests below; the measured bytes and any ceiling re-pin in
   `hosts/host-web/tests/browser-v1/expected.json` with reasons.

## Authorized paths

- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/browser-v1/expected.json`, `hosts/host-web/tests/retained_ceilings.rs`,
  `hosts/host-web/tests/boot_transient_budget.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/src/lib.rs`,
  `crates/host-core/tests/spectrum.rs`, `crates/host-core/tests/strip_meters.rs`,
  `crates/host-core/tests/prepare.rs` (argument only)
- `tools/console-workload/src/lib.rs` (argument only)
- `crates/control-plane/src/compile.rs` (the one constant line; stream B owns this file, root
  sequences the merge)

## Non-goals

- No change to the SDK, its defaults or its error text (stream H owns `sdk/`).
- No browser structural edit path: *Replace the running browser session in the Rust host* (#1290)
  and *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332)
  bring it. This slice only guarantees the lanes those paths drive.
- No cells: *Hold live values in latest-target cells on both hosts* (#1312) converts these lanes
  with the C ABI's.

## Objective gates

1. **Lanes on every browser plan.** A host-web unit test boots a session of three tracks and one
   submix (built in the test; the browser fixtures have no submix) with option 0
   through each of the three preparation branches (no spectrum, single, collection) and asserts
   one fader/mute and one matrix/pan producer per strip, tracks then submixes, and no input,
   effect or route producer.
2. **No bit moves.** The same test renders 64 blocks with option 0 and with option 64 (no command
   submitted) and asserts every output word bit-identical between the two.
3. **The gate keys on the option.** With option 0, `submit_commands` of one fader record returns
   `RESULT_UNSUPPORTED` with `COMMAND_REASON_UNSUPPORTED_KIND`, and the SDK's
   `sdk/test/browser-defaults-evals.mjs` stays green unchanged.
4. **Memory recorded.** `cargo test --locked -p host-web --features host-web/test-support` (with
   `retained_ceilings.rs` and `boot_transient_budget.rs`) and
   `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`
   pass; the PR lists every re-pinned row and the per-strip bytes.
5. Commands:
   - `cargo test --locked -p host-core -p host-web -p capi --features host-core/test-support,host-web/test-support`
   - `cargo test --locked --release -p console-workload`
   - `bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`
   - `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`
   - in `hosts/host-web/qualification`: `npm ci && npm run qualify -- --artifacts ../../../target/ci/qualification-artifacts --sdk-root ../../../sdk --browser chromium --check-matrix --self-test-mutations` (and `firefox`, `webkit`)
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: a fix in only one of the three preparation branches (the spectrum boots keep no lanes),
  or option 0 still mapping to `None`, turns it red.
- Gate 2: attaching the lanes switches a settled strip to a kernel with different arithmetic (a
  signed zero or a reordered multiply in the fused bank); it turns red.
- Gate 3: the command gate still reads `controls.is_empty()`, so an app that never opted in can
  suddenly send commands; it turns red.

## Dependencies

- none (it does not wait for #1312; that issue converts these lanes on both hosts).
