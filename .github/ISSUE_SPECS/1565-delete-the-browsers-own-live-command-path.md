# Delete the browser's own live-command path

Slice S10b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point C1:
"a behaviour that exists on one host only is a defect"; "What this supersedes"). Code anchors
verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal
tests, a batch review; no per-attempt verifier or mutation runs.

This slice lands after #1562 (the Worker commits `SET_CONTROL`) and after #1563 and #1564 (the SDK
no longer sends a value command, calls no prepared-target export and reads no index export).

## Product outcome

host-web keeps no value admission of its own. Its value command kinds, its admission and
composition, its prepared-target exports, its index exports, its own ramp table and its live-lane
boot word are gone, and so are host-core's browser-only mirrors. Every value in the browser goes
through the core, as on the C ABI. The browser keeps only transport and its monitor operations:
observation subscriptions (kinds 7 and 8) and the meter lease, which the later browser-feature
phase moves into the core (ruling C9, C13).

## Context

- **The value kinds and their admission.** Kinds 1-6 and 9-17 (`hosts/host-web/src/lib.rs:837-949`),
  `submit_commands` (`:3477`), `admit_commands` (`:4798`) and `admit_commands_staged` (`:5051`),
  the record builders (`into_track_record` `:4443`, `into_effect_records` `:4685`), the passes for
  solo, VCA and follow, and #1548's temporary `WebEffectiveView`. The reasons that only these kinds
  use (`COMMAND_REASON_NOT_SOLOABLE` `:1076`, `_UNKNOWN_ROUTE` `:1081`, `_UNKNOWN_VCA` `:1087`).
- **The prepared-target exports.** `miso_engine_web_v1_eq_target_*` (`hosts/host-web/src/ffi.rs:3578-3632`,
  `:4058-4073`), `miso_engine_web_v1_input_filters_prepare` (`:3638`) and `_config_copy` (`:4084`),
  `miso_engine_web_v1_prepared_command_submit` (`:4025`), their codec and workspace
  `hosts/host-web/src/control_targets.rs`, and the page module `hosts/host-web/web/prepared-control.js`.
  The core designs these targets in the classifier since #1554 (EQ) and #1556 (input filters).
- **The index exports.** `miso_engine_web_v1_live_control_{track,submix,route,vca}_{count,id}`
  (`ffi.rs:4148-4221`), read by the worklet's session map
  (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:597-660`). #1564 moves the SDK's strip order
  to the control list.
- **The ramp table.** `SessionRamps` (`lib.rs:1549`), built in `compile_ready` (`:6960`); #1562 D6
  leaves it serving only the old path.
- **The boot word.** `live_control_command_queue_records` (offset 32 of the boot options,
  `lib.rs:1251`): nonzero allocates the command staging (`:6498-6502`) and is required by
  observation taps (`:6638-6648`). Since #1562 the core chooses the live lanes. *Make the browser
  live-command boot word an on/off flag* (#1524) planned to rename it; this slice deletes it.
- **host-core's browser-only mirrors.** `LiveVcaState` (`crates/host-core/src/vca.rs:84`),
  `LiveControlSoloState` and `StripMuteSeed` (`crates/host-core/src/solo.rs:112-129`),
  `LiveRouteState` and `LiveRouteMuteFollow` (`crates/host-core/src/live_route_state.rs:67`,
  `:232`), re-exported at `crates/host-core/src/lib.rs:136`, `:164`, `:183`. Only host-web and their
  own tests use them (`crates/host-core/tests/vca_live.rs`); the C ABI diffs committed models (#1558,
  #1559).
- **The static gate.** `scripts/check-web-audioworklet.sh:494-511` holds `command_submit` to the
  allocation-only call graph on the worklet thread; `:258` lists `prepared_command_submit`.
- **Tests.** About 68 of the 168 tests in `hosts/host-web/src/tests.rs` use a value kind. The
  raw-module oracle `hosts/host-web/tests/browser-v1/direct-oracle.mjs` has a command-record
  timeline (kinds 1-6); #1564 added the `SET_CONTROL` timeline beside it.

## Decisions frozen for this slice

- **D1. Deleted kinds.** Kinds 1-6 and 9-17 are removed. A record with one of those kinds is
  refused `COMMAND_REASON_MALFORMED` at its index, before anything is admitted, as an unknown kind
  is today (`hosts/host-web/src/lib.rs:1031`, `:4564`). Their numbers are never reallocated. `COMMAND_RECORD_BYTES`, the record
  layout and `ABI_VERSION` do not change: kinds 7 and 8 keep the record.
- **D2. Deleted admission.** Every value arm, pass and helper of `admit_commands_staged`, the
  record builders, `WebEffectiveView`, `SessionRamps`, and the reasons that only deleted kinds use
  are removed. What stays in `admit_commands_staged` is the observation path only.
- **D3. Deleted exports.** `miso_engine_web_v1_prepared_command_submit`, every
  `miso_engine_web_v1_eq_target_*`, `miso_engine_web_v1_input_filters_prepare`,
  `miso_engine_web_v1_input_filters_config_copy` and every
  `miso_engine_web_v1_live_control_{track,submix,route,vca}_{count,id}`, with
  `hosts/host-web/src/control_targets.rs` and `hosts/host-web/web/prepared-control.js`. The
  worklet's `preparedCommand` message and its session-map fields for routes and VCAs go; its strip
  order, if the worklet still needs one for meters, comes from the core's list (#1564 D5).
- **D4. The boot word.** Offset 32 is renamed `reserved1: u64` and must be zero (refused
  `web.options.reserved1` otherwise). The command staging is allocated when
  `live_control_observation_taps != 0`, and kinds 7 and 8 are accepted only then (otherwise
  `RESULT_UNSUPPORTED` with `COMMAND_REASON_UNSUPPORTED_KIND`, as a command on a host with no live
  controls attached is today, `lib.rs:3489-3491`).
  `BOOT_OPTIONS_BYTES`, every other offset and `ABI_VERSION` do not change. The SDK's
  `policy.liveControls.commandQueueRecords` option is deleted; `host-mirror.ts` writes 0.
- **D5. Deleted mirrors.** `LiveVcaState`, `LiveVcaFaderDelta`, `LiveControlSoloState`,
  `LiveControlMuteDelta`, `StripMuteSeed`, `LiveRoute`, `LiveRouteState`, `LiveRouteStateError`
  and `LiveRouteMuteFollow` are removed from host-core with their modules' code and
  `crates/host-core/tests/vca_live.rs`. Doc comments elsewhere that point at them point at
  `controls::effective` instead.
- **D6. Tests.** Each host-web test that used a deleted kind is either rewritten to send
  `SET_CONTROL` through #1562's entry and keeps its claim, or deleted because a core test or the
  #1560 parity test holds the same claim; the PR lists each deleted test and the test that now holds
  its claim. The command-record timeline of `direct-oracle.mjs` is removed and `expected.json`
  loses its digest; the `SET_CONTROL` timeline of #1564 stays the cross-target owner.
- **D7. Gates and mirrors follow.** `scripts/check-web-audioworklet.sh` drops
  `prepared_command_submit` and keeps the `command_submit` allocation leg (kinds 7 and 8 still run
  on the worklet thread). The ABI layout document and its generator, `sdk/src/generated/abi.ts`,
  `sdk/assets/miso-engine-v1-abi-layout.json` (regenerated), `scripts/check-abi-layout-v1.py` and
  its self-test fixture, `tools/realtime-policy/render-thread-exports.json`, the shipped host's
  `.d.ts` files and `scripts/test-web-audioworklet.mjs` drop every deleted export, kind and field.
- **D8. Benchmarks.** `scripts/web-mixing-automation-benchmark.mjs`,
  `scripts/web-mixing-automation-lib.jq` and `tools/console-workload/src/mixing_automation.rs`
  send their edits as `SET_CONTROL` through the `control` message. Their record keys follow D4.

## Deliverables

1. D1-D4 and D6-D8 in host-web, the worklet scripts, the checkers and the generated mirrors.
2. D5 in host-core.
3. The SDK option deletion of D4 (`sdk/src/browser/{policy.ts,host-mirror.ts,engine.ts}`,
  `sdk/src/core/boundary.ts`).

## Authorized paths

- `hosts/host-web/src/`, `hosts/host-web/web/`, `hosts/host-web/tests/`,
  `hosts/host-web/qualification/`, `hosts/host-web/MUTATIONS.md`, `hosts/host-web/DEPLOYMENT.md`.
- `crates/host-core/src/{vca.rs,solo.rs,live_route_state.rs,lib.rs}`,
  `crates/host-core/tests/{vca_live.rs,MUTATIONS.md}`.
- `scripts/check-web-audioworklet.sh`, `scripts/check-web-audioworklet-callgraph.py`,
  `scripts/test-web-audioworklet.mjs`, `scripts/check-abi-layout-v1.py`,
  `scripts/fixtures/abi-layout-v1-self-test.json`, `scripts/check-browser-expected-resources.py`,
  `scripts/web-mixing-automation-benchmark.mjs`, `scripts/web-mixing-automation-lib.jq`.
- `tools/parameter-metadata/src/abi_layout.rs`, `tools/parameter-metadata/tests/{abi_layout.rs,round_trip.rs}`,
  `tools/realtime-policy/render-thread-exports.json`, `tools/console-workload/src/mixing_automation.rs`.
- Regenerated only: `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts`.
- `sdk/src/browser/{policy.ts,host-mirror.ts,engine.ts,shipped-host.d.ts}`,
  `sdk/src/core/boundary.ts` (D4 only), `sdk/test/**` (D4 only).

## Non-goals

- No change to observation subscriptions, the meter lease or spectrum (the later phase).
- No change to the core, the classifier or any record type. (Whether builtins-compiler's record
  types get private fields stays with #1423.)
- No new export.

## Objective gates

Commands: `cargo test --locked -p host-web --features host-web/test-support`,
`cargo test --locked -p host-core --all-features`, `cargo test --locked -p capi`,
`bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh`,
`python3 -B scripts/check-abi-layout-v1.py`,
`python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`,
`bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
`bash scripts/check-sdk-types.sh`, `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
`bash scripts/check-workspace-policy.sh`, and the browser qualification for chromium, firefox and
webkit on both legs.

1. **Deleted kinds refuse.** For each of kinds 1-6 and 9-17, a one-record command is refused
   `COMMAND_REASON_MALFORMED` at index 0 and changes no cell.
2. **The boot word.** A nonzero offset 32 is refused `web.options.reserved1`; a boot with
   observation taps and offset 32 zero accepts kinds 7 and 8; a boot without taps refuses them
   with `COMMAND_REASON_UNSUPPORTED_KIND`.
3. **Every value still works.** The #1560 parity test and #1562's gates 1-6 pass unchanged.
4. **Observation still works.** Every observation test and every observation row of the browser
   qualification passes unchanged.
5. **Realtime.** `hosts/host-web/tests/worker_admission_realtime.rs` and the browser's
   `render_allocation_count` gate pass unchanged.

## Test value

- Gate 1 turns red if a value kind still reaches the cells without the core.
- Gate 2 turns red if the deleted word still changes behaviour, or if observation loses its
  staging.
- Gate 3 turns red if a deletion removed something the core path still needed.
- Gate 4 turns red if the deletion broke the monitor operations that stay.
- Gate 5 turns red if the remaining command path allocates on the render thread.

## Dependencies

- #1562, #1563, #1564.

## Supersedes and shrinks

- Supersedes *Make the browser live-command boot word an on/off flag* (#1524): the word is deleted.
- Supersedes *Test that a refused batch does not undo a committed browser VCA batch* (#1525): the
  browser VCA mirror and its commit are deleted; #1559 gate 3 holds the shared-path half.
- Shrinks *Make live strip records valid by construction* (#1423): no host builds a strip record
  any more; the private-field change stays as defence in depth for host-core.
