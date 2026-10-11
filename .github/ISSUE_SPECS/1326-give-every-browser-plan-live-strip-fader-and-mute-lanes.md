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
  is 0 (`sdk/test/browser-defaults-evals.mjs:422-456`), so a plan booted without live controls has no live mute.
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

## Attempt record

### Attempt 1 (implementer)

**Change.** D1-D4 as frozen, on `2a589d360`.

- `crates/host-core/src/prepare.rs`: new `pub const STRIP_LIVE_QUEUE_DEPTH: NonZeroUsize` = 16
  (re-exported in `lib.rs`). `prepare_host_runtime_with_live_controls_and_spectrum`,
  `..._and_spectrum_collection` and `prepare_host_runtime_with_selected_meters_between_render_calls`
  gain a trailing `lanes: HostLiveLanes`. The last one used to go through the policy wrapper, whose
  `selected_meters` parameter no other caller set; it now calls
  `prepare_host_runtime_with_live_controls_policy_and_spectrum` directly (as the live-lanes entries
  do) and the wrapper lost that parameter. Doc comments that said "every entry but
  `prepare_host_runtime_with_live_lanes` passes ALL" now say what is true.
- `crates/control-plane/src/compile.rs`: `LIVE_QUEUE_DEPTH` is `host_core::STRIP_LIVE_QUEUE_DEPTH`.
- `hosts/host-web/src/lib.rs`: `live_control_request` always returns `Some(depth)` (option, else
  `STRIP_LIVE_QUEUE_DEPTH`); `live_lanes(options)` is `ALL` for a nonzero option, else
  `FADER_AND_MATRIX`, passed to all three branches; `ReadyOwnership.commands_enabled` (the option)
  replaces `controls.is_empty()` in `live_controls_attached`, the two VCA/route refusals, the
  input-filter shadows and the live VCA state. The `WebBootOptions` doc says what 0 means now.
- Call sites: host-core `tests/{prepare,strip_meters,spectrum}.rs` and three host-web mirrors in
  `tests.rs` (those pass `live_lanes(options)`, the production choice). `tools/console-workload`
  calls none of the three entries (only a doc comment names one), so it is unchanged.
- One new test, `every_browser_plan_carries_the_strip_lanes_and_renders_the_lane_free_bits`.

**Gate 1 and 2.** The test boots three tracks (asymmetric faders, pans) and one submix (`bus`,
compressor insert, routes into it) with option 0 and option 64 through all three branches (plain,
single spectrum, spectrum collection). Option 0: four `controls` in order `t0 t1 t2 bus`, each with
fader and matrix producers, no `input`, a nonempty effect table that is all `None`, no route
producer, `live_controls_attached() == false`. Option 64 (contrast): input lane on all strips,
an effect lane present, three route producers. It renders 64 blocks and requires option 0 bit
equal to option 64 and to an independent host-core plan prepared with `HostLiveControlRequest::default()`
(no lane at all, the browser's own delivery `BetweenRenderCalls`): the plan option 0 prepared
before this change. The existing host-core bit test (`live_lanes.rs`) prepares with delivery
`Concurrent`, not the browser's, so it does not cover this.

**Gate 3.** Met by the existing `a_live_control_free_host_refuses_every_live_kind_as_unsupported`
(boots `command-session.json` with option 0: `RESULT_UNSUPPORTED`, `COMMAND_REASON_UNSUPPORTED_KIND`,
no staging buffer). That session now carries the lanes, so the test now proves the gate keys on the
option. The new test repeats one `submit_commands(1)` refusal on its lane-bearing session; the
D4 mutation turns both red, so that assertion is not a unique catch. No new gate-3 test was added.

**Mutations (each applied, `cargo test -p host-web --features host-web/test-support --lib`, then
reverted; the tree was clean afterwards).**

| Mutation | Red | Other tests red |
| --- | --- | --- |
| Option 0 maps back to `control_queue_depth: None` (D1) | the new test (`assertion left == right`, 0 vs 4) | none (188 others pass) |
| The collection branch prepares with `control_queue_depth: None` | the new test, Collection branch only | none |
| `live_lanes` returns `ALL` for option 0 (D3) | the new test (`Plain: no input lane`) | `live_route_ids_enumerate_in_send_index_order_through_staging_sized_for_them` (route producers appear) |
| Settled fader lane multiplies by `1 + EPSILON` (`FaderRampStage::process_plane`, `maximum == 0` path) | the new test, at the lane-free comparison | three existing fader tests (`a_both_lane_record_with_one_lane_changed_sets_the_follow_ramp`, `a_no_op_record_on_the_other_lane_never_sets_the_follow_ramp`, `native_command_timeline_digest_pins_the_wasm_parity`) |
| `live_controls_attached` reads `!controls.is_empty()` again (D4) | the new test (its refusal assertion) | `a_live_control_free_host_refuses_every_live_kind_as_unsupported` |

So the new test alone catches a dropped lane (all branches), and the lane-free bit comparison it adds
is the only check of option 0 against a plan with no lane; a broad change to the settled lane's
arithmetic is also caught by the existing fader tests that run opted-in plans. An `ALL` mapping for
option 0 is also caught by one existing route test.

**Gate 4, memory.** Native, `retained_ceilings.rs`, one-strip identity fixture: only
`builtin_retained_bytes` moves, 2,149 to 3,383 (+1,234); ceiling 2,176 to 3,776
(3,383 x 1.1, rounded up to 64). Across other sessions: nine tracks 19,179 to 30,249 (+11,070, 1,230
per strip), three strips (`observation-frame-shape`) 6,366 to 10,050 (+3,684, 1,228 per strip).
The bridge, graph and source rows are equal before and after. Shipped simd128 module
(`check-browser-expected-resources.py --artifacts`, base `2a589d360` built in a scratch export):
`builtinRetainedBytes` 2,033 to 3,219 (+1,186 for the one strip); `expected.json`
`resourceCeilings.builtinRetainedBytes` 2,048 to 3,584 (3,219 x 1.1 = 3,540.9, rounded up to 64);
no other row moves (bridge metadata and retained, graph, source rows identical). The lanes are two
16-record rings per strip. No other row was re-pinned.

**Gate 5 results (x86_64, this box).**

- `cargo test --locked -p host-core -p host-web -p capi --features host-core/test-support,host-web/test-support`:
  pass (and with `-p control-plane`).
- `cargo test --locked --release -p console-workload`: pass.
- `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts` (rows above), `test-web-audioworklet.sh`
  (exit 0), `check-sdk-headless.sh` (362 of 362, `browser-defaults-evals.mjs` unchanged):
  pass.
- `npm run qualify -- ... --check-matrix --self-test-mutations` in chromium (151.0.7922.34),
  firefox (153.0) and webkit (26.5): all qualification gates passed.
- `check-host-core-policy.sh`, `check-workspace-policy.sh`, `check-realtime-policy.sh`,
  `check-cross-targets.sh` (PASS), `cargo clippy --locked --workspace --all-targets --all-features
  -- -D warnings`, `cargo fmt --all -- --check`: pass.
- Not run: `check-capi-abi.sh` and `audit capi` (no ABI or capi source change; the only capi-side
  edit is the constant's alias, same value).

**Open items for root.** Stale wording outside the authorized paths: `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts:791`
("`0n` to attach no control channel"), `.d.ts:44`, `miso-engine-v1-audio-worklet.js:1549` and
`sdk/src/browser/shipped-host.d.ts:44` still say option 0 has no control channel; they are stream
H's files. The merge with stream B on `crates/control-plane/src/compile.rs` is the one constant line.

**Follow-ups after the PASS verdict** (comment-only edits; no code moved). The `host-web` "input lane
never fires" comment now gives the true reason (an option-0 plan refuses every command first); the
130-column `route_controls` doc line in `host-core/src/prepare.rs` is rewrapped; the extra doc line
above `LIVE_QUEUE_DEPTH` in `control-plane/src/compile.rs` is removed, so only the constant line
differs from `2a589d360` (the original doc already stays true); the `four_lanes` ceiling comment in
`retained_ceilings.rs` says the 3,776 is inferred from the x86-64 measurement, because AArch64 runs
only in CI. The stale `.d.ts`/`.js` wording and STREAMS.md's hot-file row stay with root.
