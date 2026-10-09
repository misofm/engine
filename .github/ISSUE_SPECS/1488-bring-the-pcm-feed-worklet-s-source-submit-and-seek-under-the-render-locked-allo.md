# Bring the PCM-feed worklet's source submit and seek under the render-locked allocation count

Stream H follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10). Filed
2026-10-08 by root from the #1478 attempt-2 verdict, MAJOR M2 and open item 1
(`/home/bl/misofm/submix-verdicts/1478-attempt2.md`).

Root's ruling (2026-10-08), verbatim:

> (2) The real gap: file a stream H issue now: the PCM-feed worklet's source_submit and
> source_seek run on the audio thread in process(), so they are render-thread code: bring both
> exports under render_locked so the render-allocation count covers them, and correct the
> render_lock.rs and ffi.rs headers; gates: the count sees an allocation injected into either
> export (red), zero on the real code.

## Problem (verified on `main` at `a059cdd03`)

- **The runtime proof.** #1333 made `hosts/host-web/src/render_lock.rs` the browser module's
  global allocator: it counts every allocator call made while the thread is inside a
  `render_locked` window, and browser qualification reads the count through
  `miso_engine_web_v1_render_allocation_count` and asserts zero (D15-10). The module header
  (`render_lock.rs:6-7`) says "every export the worklet calls on its render thread runs inside
  [`render_locked`]", and the `ffi.rs` header (`hosts/host-web/src/ffi.rs:14-23`) says the same of
  "every export the AudioWorklet calls on its render thread after boot", then lists the set.
- **Two render-thread exports are outside it.**
  - The SDK ships `sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js` (asset URL
    `sdk/src/assets.ts:34`). Its `process()` (`:306-328`) drains each shared ring
    (`drainSharedRing`, `:334`) before it calls `super.process()`, which renders. The drain calls
    `miso_engine_web_v1_source_submit` (`:386`) for each chunk and, through `applySharedSeek`
    (`:341`, `:441`), `miso_engine_web_v1_source_seek` (`:445`). These run on the AudioWorklet
    thread inside `process()`, every block.
  - Without the feed, the engine worklet's port handlers call the same exports on the same thread
    (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1010-1012`, `receiveSource` `:1525`,
    `receiveSeek` `:1746`).
  - Neither export wraps its body in `render_locked`: `ffi.rs:3846` and `:3922` call
    `with_host_mut` directly. Every other per-block export (`render` `:3950`, `command_submit`,
    `meter_poll` `:4074`, the staging reads) is wrapped.
- **Consequence.** An allocation in `source_submit` or `source_seek`, or in what they call
  (`AudioWorkletEngineHost::submit_source`/`seek_source`, `hosts/host-web/src/lib.rs:3142`,
  `:3184`, and host-core's `SourceControlSet::submit`/`seek`), happens on the render thread and
  the count does not see it. The headers overclaim.

## Root's amendment (2026-10-09)

Root's scope amendment, verbatim intent:

> also spectrum_arm, spectrum_cancel, spectrum_stream_start, spectrum_stream_stop and
> spectrum_selection_epoch, which are not render-locked either — amend #1488's local spec first so
> its 'every post-boot worklet call on the render thread is render-locked' sentence is true, or
> name each exception with its reason.

**Inventory (attempt 1, read-only, base `1d294c700`).** Every `miso_engine_web_v1_*` export in
`hosts/host-web/src/ffi.rs` was checked for a `render_locked` wrap and against both worklet files
for a post-boot call on the AudioWorklet thread (`process()` or a port handler dispatched from
`receive()`). Besides the two exports of the original brief and root's four spectrum exports, five
more are called after boot from port handlers without a wrap: `input_filters_config_copy`
(`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1599`) and `eq_target_config_copy` (`:1600`)
and `eq_target_config_ptr` (`:1612`) in `receiveEqTargetConfig`; `prepared_command_submit`
(`:1663`) in `receivePreparedCommand`; `meter_lease` (`:1695`) in `receiveMeterLease`. Two more
unwrapped post-boot calls are `dispose` (`:992`, the `miso.dispose.v1` handler; `:866` is the
boot-failure path) and `render_allocation_count` (`:1073`).

**`spectrum_selection_epoch` has no call site in either worklet file.** Its only callers are the
`ffi.rs` unit tests. The other mentions are export-name lists, none a caller:
`sdk/src/generated/abi.ts`, `sdk/assets/miso-engine-v1-abi-layout.json`,
`scripts/check-web-audioworklet.sh`, `scripts/check-abi-layout-v1.py` and its self-test fixture,
and `tools/parameter-metadata/src/abi_layout.rs`.
The amendment's premise does not hold for it, so it is not wrapped, and the coordinator reports
this to root. The header sentence speaks only of exports the worklets call after boot on the
AudioWorklet thread, so it stays true without it.

**Coordinator's decisions (2026-10-09, under root's "wrap, or name each exception with its
reason").**

- Wrap the five port-handler exports found by the inventory, as well as `source_submit`,
  `source_seek`, `spectrum_arm`, `spectrum_cancel`, `spectrum_stream_start` and
  `spectrum_stream_stop`: they are post-boot render-thread calls, and #1332 D3.5 puts every
  post-boot worklet export inside `render_locked`.
- Named exceptions, each with its reason, in this spec and in both headers: `dispose` (teardown:
  it frees the host by design, and it is also the boot-failure path); `render_allocation_count`
  (it is the reader of the count; wrapping it measures nothing); `spectrum_select` and
  `spectrum_stream_select` (they allocate today; #1492 removes the allocation and wraps them).
- D4 applies to every wrapped export.

**Root's confirmation (2026-10-09).** Root confirmed the report above, verbatim: "confirmed:
spectrum_selection_epoch stays unwrapped (no worklet calls it), as recorded."

## Decisions

- **D1. Eleven exports run inside `render_locked`** (amended 2026-10-09).
  `miso_engine_web_v1_source_submit`, `source_seek`, `spectrum_arm`, `spectrum_cancel`,
  `spectrum_stream_start`, `spectrum_stream_stop`, `prepared_command_submit`, `meter_lease`,
  `eq_target_config_copy`, `eq_target_config_ptr` and `input_filters_config_copy` wrap their whole
  bodies in `render_locked(|| ...)`, as `render` does. Behaviour, result codes and the `SAFETY`
  reasoning are unchanged. No other export changes.
- **D2. The headers say what is true.** `render_lock.rs:1-18` and `ffi.rs:14-23` name the set as
  it then is: the static gate's three exports, the staging reads, the post-boot staging accessors,
  `source_submit` and `source_seek` (called from `process()` on the SDK feed path and from the
  worklet's port handlers), the live-control exports `prepared_command_submit`, `meter_lease`,
  `eq_target_config_copy`, `eq_target_config_ptr` and `input_filters_config_copy`, and the
  spectrum exports `spectrum_arm`, `spectrum_cancel`, `spectrum_stream_start` and
  `spectrum_stream_stop`. They name the only exceptions, each with its reason: `dispose`,
  `render_allocation_count`, `spectrum_select` and `spectrum_stream_select` (see Root's
  amendment). The sentence that every post-boot worklet call on the render thread is render-locked
  becomes true apart from those named exceptions; the implementer checks it export by export
  against both worklet files and records the list.
- **D3. A native gate.** A new integration binary, `hosts/host-web/tests/render_locked_source.rs`,
  registers `RenderLockedAllocator<System>` as its global allocator and holds exactly one test
  (as `render_locked_staging.rs` does, so nothing else shares the process-wide counter). It boots
  the observation session (`hosts/host-web/qualification/observation-session.json`, source
  `live-control-source`), renders a few blocks, then, on a fresh thread, calls through the exports:
  `source_submit` with a full-quantum chunk, with an end-of-region short chunk, and with a refused
  chunk (an invalid argument); `source_seek` with an accepted seek and a refused one. Amended
  2026-10-09: it also calls each of the other nine wrapped exports through its export on the same
  thread, on the accepted path and, where one exists, a refused path (`input_filters_config_copy`,
  `eq_target_config_copy`, `eq_target_config_ptr`, `prepared_command_submit`, `meter_lease`,
  `spectrum_arm`, `spectrum_cancel`, `spectrum_stream_start`, `spectrum_stream_stop`). It asserts
  each exact result code and that the count is unchanged after each call. Where a staging cannot
  be written natively, add `native_staging` helpers next to the existing ones (`ffi.rs:4564`),
  written the same way.
- **D4. Zero on the real code is the claim.** If the real code allocates in either export (the
  gate reads non-zero once D1 is in), the slice stops and reports the allocating call path to root;
  it does not loosen the gate or remove the wrap.

## Authorized paths

- `hosts/host-web/src/ffi.rs` (the eleven exports of D1, the header, and the `native_staging`
  helpers D3 needs)
- `hosts/host-web/src/render_lock.rs` (the header only)
- `hosts/host-web/tests/render_locked_source.rs` (new)
- `docs/REALTIME_DEPENDENCY_POLICY.md`: only if J #1489 has already landed with D-M2's exception
  clause, remove that clause (one sentence)
- this spec

## Non-goals

- Moving source submission into the Worker (#1387). In `single` mode the worklet remains the
  producer after #1387, so D1 stays needed.
- The static call-graph gate's export list (`scripts/check-web-audioworklet-callgraph.py`).
- `hosts/host-web/qualification/qualification.js` and `scripts/test-web-audioworklet.mjs` (#1477
  is in review on them). The browser qualification's existing zero assertion covers the two exports
  once D1 is in, with no edit.
- Any change to the feed or engine worklet JavaScript.

## Hazards

- `hosts/host-web/src/ffi.rs` is edited by other H slices; the later slice rebases.
- The `wasm32` release module changes (two wrapped exports); the artifact identity job reports it
  CHANGED, as expected.

## Objective gates

1. **Zero on the real code.** `cargo test --locked -p host-web --test render_locked_source` passes.
2. **Red on an injected allocation (mutation, recorded).** Inject one allocation (for example
   `let _keep = std::hint::black_box(Box::new(0u8));`) into the body of `source_submit` alone:
   gate 1 is red. Revert; inject it into `source_seek` alone: gate 1 is red. Revert: green.
   Also with D1 reverted and the injection kept, gate 1 is green (the wrap is what makes the count
   see it). Amended 2026-10-09: the same injection, one run per export, into each of the other nine
   wrapped exports is red, and the D1-reverted run is made for at least `source_submit`. Record
   every run in the attempt record.
3. **Existing gates stay green.** `cargo test --locked -p host-web` (all targets),
   `cargo clippy --locked -p host-web --all-targets -- -D warnings`, `cargo fmt --all -- --check`,
   `bash scripts/check-workspace-policy.sh`, `bash scripts/check-realtime-policy.sh`,
   `bash scripts/check-cross-targets.sh`, and the worklet chain
   (`scripts/build-web-audioworklet.sh --named-twin`, `scripts/check-web-audioworklet.sh`,
   `check-browser-expected-resources --artifacts`, `scripts/test-web-audioworklet.sh`; exact
   invocations from `.github/workflows/qualification.yml`) exit 0. The browser qualification's
   render-allocation counts stay zero.
4. **Headers checked (verifier).** Each export named in D2's set is checked against the code
   (wrapped or not) and against both worklet files (called after boot on the AudioWorklet thread
   or not).

*Test value.* `render_locked_source.rs` is red if any of the eleven exports of D1 allocates or
frees inside its render-locked window, on its accepted or its tested refused path. For an
allocation in an export's own body, no existing test catches it: none calls these exports under a
counting allocator. Two existing tests overlap on accepted paths. `render_locked_staging.rs` calls
`spectrum_arm` and `spectrum_stream_start` inside its measured span. The `test-support` lib tests
`prepared_eq_owner_transaction_is_design_and_allocation_free_after_preparation` and
`prepared_mixed_eq_builtin_fader_commits_and_refuses_atomically` measure
`AudioWorkletEngineHost::submit_prepared_commands`, which `prepared_command_submit` forwards to, on
accepted batches. Only this binary catches an allocation on the tested refused paths (for example
arm while a stream is active, start with NaN smoothing, a truncated companion).

## Evidence

- Gate 1 output; gate 2's three mutation runs; gate 3's outputs; D2's export-by-export list.

## Dependencies

- None. Before #1387 (which moves the producers to the Worker in shared mode).
- J #1489 (*Correct the realtime dependency policy's unsafe-ownership statements*) prefers to come
  after this slice (its D-M2).

## Standing rules for the implementer

- Work only from this body. Read the cited lines first.
- A test that greps source or prose is refused. Allocation counts use the module's own counting
  allocator in a one-test integration binary.
- Attempt budget: two attempts, one adversarial verdict each.
- Size: half a day.

## Attempt record

### Attempt 1 (implementer, 2026-10-09, base `1d294c700`)

**Changes.**

- `hosts/host-web/src/ffi.rs`: the eleven exports of D1 wrap their whole bodies in
  `render_locked`; the module header names the set and the four named exceptions with their
  reasons; `native_staging` gains `source_id`, `source_pcm`, `command_records`,
  `prepared_companion` and `eq_target_config` (native writers and one reader for the stagings the
  worklet addresses through `u32` pointers; none is render-locked, the worklet never calls them).
- `hosts/host-web/src/render_lock.rs`: the header names both worklets and points to the set and
  its exceptions in the `ffi` header.
- `hosts/host-web/tests/render_locked_source.rs` (new, one test, own counting allocator): boots
  the observation session on a fresh thread, renders, then calls every wrapped export through its
  export and asserts its exact result and an unchanged count after each call: `source_submit` with
  a full quantum (OK), refused with `end_of_region = 2` (INVALID_ARGUMENT) and an end-of-region
  64-frame chunk after a seek to frame 1984 (OK); `source_seek` accepted (OK) and refused with an
  ID length past its staging (INVALID_ARGUMENT); `input_filters_config_copy` (OK; strip 99
  UNSUPPORTED); `eq_target_config_copy` (console slot 0 OK; retired rack 0 INVALID_ARGUMENT);
  `eq_target_config_ptr` (the live handle; handle 0); `prepared_command_submit` (one matrix record
  with a header-only companion carrying the copied host generation, OK; truncated companion
  INVALID_ARGUMENT); `meter_lease` (take OK, `2` INVALID_ARGUMENT, release OK); `spectrum_arm`
  (OK; BACKPRESSURE while a stream is active); `spectrum_cancel` (OK; WRONG_STATE while a stream is
  active); `spectrum_stream_start` (NaN smoothing INVALID_ARGUMENT; 100 ms OK);
  `spectrum_stream_stop` (OK; handle 0 INVALID_ARGUMENT).

**D2 export-by-export list** (wrapped in `render_locked` / called after boot on the AudioWorklet
thread, with call sites in `hosts/host-web/web/miso-engine-v1-audio-worklet.js` (E) and
`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js` (F)):

| Export | Wrapped | Post-boot call site |
| --- | --- | --- |
| `render` | yes | E:1852 `process()` |
| `command_submit` | yes | E:1565 `receiveCommand` |
| `meter_poll` | yes | E:1770 `process()` |
| `observation_read`, `observation_result_ptr`, `observation_result_bytes` | yes | E:1124, 1134, 1135 |
| `track_response_capture`, `track_response_result_ptr`, `track_response_result_bytes` | yes | E:1238, 1246, 1247 |
| `spectrum_read`, `spectrum_capture_ptr`, `spectrum_capture_bytes`, `spectrum_capture_capacity` | yes | E:1429, 1439-1441 (also 1381-1383) |
| `spectrum_stream_read`, `spectrum_stream_metadata_ptr`, `spectrum_stream_metadata_bytes` | yes | E:1365, 1459, 1460 |
| `spectrum_target_id_ptr`, `spectrum_target_id_capacity` | yes | E:1296, 1297 |
| `source_submit` | yes (this issue) | F:386 `process()`; E:1525 `receiveSource` |
| `source_seek` | yes (this issue) | F:445 `applySharedSeek`, from `process()` (F:341) and from the attach node's `prepare-seek` handler via `prepareSharedSeeks` (F:540, 482); E:1746 `receiveSeek` |
| `spectrum_arm`, `spectrum_cancel` | yes (this issue) | E:1426, 1428 `receiveSpectrum` |
| `spectrum_stream_start`, `spectrum_stream_stop` | yes (this issue) | E:1362, 1364 `receiveSpectrum` |
| `input_filters_config_copy`, `eq_target_config_copy`, `eq_target_config_ptr` | yes (this issue) | E:1599, 1600, 1612 `receiveEqTargetConfig` |
| `prepared_command_submit` | yes (this issue) | E:1663 `receivePreparedCommand` |
| `meter_lease` | yes (this issue) | E:1695 `receiveMeterLease` |
| `spectrum_select`, `spectrum_stream_select` | no: named exception (#1492) | E:1313, 1310 |
| `dispose` | no: named exception (teardown) | E:992 (E:866 is boot failure) |
| `render_allocation_count` | no: named exception (the reader) | E:1073 |
| `spectrum_selection_epoch` | no | none in E or F |

Every other export the worklets call (`abi_version`, `boot*`, `buffer_*`, `document_ptr`,
`status_ptr`, `resource_ptr`, `command_report_ptr`, `prepared_companion_*`, `live_control_*`,
`meter_header_ptr`, `observation_*` binding accessors, `source_count`/`id`/`channels`/`frames`,
`spectrum_request_*`, `spectrum_collection_*`, `track_response_request_*`/`snapshot_*`/`track_id_*`)
is called only on the construction path (E below line 800), before boot completes. The feed
worklet calls no other export.

**Gate 1.** `cargo test --locked -p host-web --test render_locked_source`: 1 passed. Zero
allocations on the real code for all eleven exports, accepted and refused (D4 did not trigger).

**Gate 2 (mutations).** Injection `let _keep = std::hint::black_box(Box::new(0u8));` as the first
statement inside the export's `render_locked` closure, one export per run; "unwrapped" adds a
block-local `fn render_locked<R>(body: impl FnOnce() -> R) -> R { body() }` that shadows the
import (D1 reverted for that export) with the injection kept.

| Export | Injected | Injected, D1 reverted |
| --- | --- | --- |
| `source_submit` | red: "source_submit (full quantum) allocated" | green |
| `source_seek` | red: "source_seek allocated" | green |
| `spectrum_arm` | red: "spectrum_arm allocated" | green |
| `spectrum_cancel` | red: "spectrum_cancel allocated" | green |
| `spectrum_stream_start` | red: "spectrum_stream_start (refused: NaN smoothing) allocated" | green |
| `spectrum_stream_stop` | red: "spectrum_stream_stop allocated" | green |
| `prepared_command_submit` | red: "prepared_command_submit (refused: truncated companion) allocated" | green |
| `eq_target_config_copy` | red: "eq_target_config_copy (refused: retired rack) allocated" | green |
| `eq_target_config_ptr` | red: "eq_target_config_ptr allocated" | green |
| `input_filters_config_copy` | red: "input_filters_config_copy allocated" | green |
| `meter_lease` | red: "meter_lease (take) allocated" | green |

Reverted to the real code: green.

**Gate 3.** All exit 0:
- `cargo test --locked -p host-web` (lib 185 passed and 2 ignored; `boot_transient_budget` 2;
  `render_locked_source` 1; `render_locked_staging` 1; `retained_ceilings` 1).
- `cargo clippy --locked -p host-web --all-targets -- -D warnings`.
- `cargo fmt --all -- --check`.
- `bash scripts/check-workspace-policy.sh` ("workspace policy: ok").
- `bash scripts/check-realtime-policy.sh` ("ok (89 marked regions in 25 files)").
- `bash scripts/check-cross-targets.sh` ("cross-target matrix: PASS").
- The worklet chain, with the `qualification.yml` invocations:
  `bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`
  (shipped module `ccf7c664…f9`, 3129271 B: CHANGED, as the Hazards expect);
  `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration …`;
  `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`;
  `bash scripts/test-web-audioworklet.sh` under a fresh private TMPDIR (no leftovers).

**Not run locally.** The real-browser qualification's render-allocation count
(`hosts/host-web/qualification/qualification.js`) runs only in CI's browser job; the hermetic
`test-web-audioworklet.mjs` answers `miso.renderallocations.v1` with a fake zero. The browser
count is therefore unverified until CI runs. `docs/REALTIME_DEPENDENCY_POLICY.md` is not edited
(#1489 has not landed).

### Attempt 1 follow-ups (implementer, 2026-10-09; verdict PASS, MINOR 1 and NITs 1-3)

- MINOR 1: the *Test value* paragraph now says what the test catches exactly. The two
  `test-support` lib tests in `hosts/host-web/src/tests.rs` (`:3625`, `:3957`) count allocations
  and frees around `AudioWorkletEngineHost::submit_prepared_commands` on accepted batches, and
  `miso_engine_web_v1_prepared_command_submit` only forwards to that method (`ffi.rs`, inside
  `render_locked(|| with_host_mut(..., |host| host.submit_prepared_commands(...)))`). The overlap
  is named; the claim of uniqueness is now limited to an allocation in an export's own body and to
  the tested refused paths.
- NIT 1: the `ffi.rs` header and the D2 table name the feed attach node's `prepare-seek` handler
  (F:540 -> `prepareSharedSeeks` F:482 -> `applySharedSeek` F:445 -> `source_seek`). Comment and
  prose only; the wrapped set does not change.
- NIT 2: not changed. The export returns `u32::try_from(address).unwrap_or(0)`. An exact native
  check needs the config's address, and no public accessor gives it; a new test-only accessor
  would only repeat `pointer_u32`'s mapping, and on a 64-bit host the heap address usually does
  not fit, so the live handle and handle 0 both return zero and the check would not discriminate.
  The address itself is used by the worklet in the browser chain. The test comment now says this
  without the false "both calls return zero" claim (the address can fit `u32`).
- NIT 3: the root's-amendment paragraph lists every non-caller mention of
  `spectrum_selection_epoch` (abi.ts, the ABI layout JSON, `check-web-audioworklet.sh`,
  `check-abi-layout-v1.py` and its fixture, `abi_layout.rs`).

