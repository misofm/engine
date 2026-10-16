# Seek the timeline and every source from the browser module export and the headless SDK

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A1 (A1.2), under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

The shipped engine module exports the session seek of #1642: one function on both hosts, with no
browser-only seek logic in Rust. It seeks the timeline and every source to one frame under one new
generation, all or nothing, and every consumer applies it at the next render block. The headless
SDK moves the playhead with one call, `seek(frame)`, which resolves with the generation it used;
the caller then submits each source's PCM from that frame under that generation. Until a source's
new PCM arrives it underruns, as after any seek today. The browser SDK and the PCM feed use this
export in #1644.

## Context

- **The browser's per-source seek.** The export `miso_engine_web_v1_source_seek`
  (`hosts/host-web/src/ffi.rs:3787-3812`) reads the staged source ID and calls
  `AudioWorkletEngineHost::seek_source` (`hosts/host-web/src/lib.rs:3170-3208`), which seeks the
  producer in `ready.host.sources` (a `SourceControlSet`) and then calls
  `plan.prepare_source_seek` (`:3193-3203`). Errors map through `source_result` (`:6543`). These
  are the anchors of `6ee64f484`; before this slice lands, *Move browser source submission and
  seeks into the Worker* (#1387) moves the seek to the Worker on an isolated page, and *Swap and
  retire browser plans through the Worker's service loop* (#1381) moves the set into the Worker's
  `SessionState`. Both arrive through #1293, a dependency.
- **#1642's function.** `SourceControlSet::seek_session(generation, timeline_sample, anchor)`
  in host-core checks the timeline and every source producer, then pushes to all of them.
  `SessionState::session_seek` in `crates/control-plane` wraps it, as `seek` wraps the source seek.
  The C ABI calls the wrapper; this slice makes the browser call the same wrapper, as #1381 D2 makes
  it call `SessionState`'s other source routing.
- **Headless.** `OfflineEngine.seekSource` (`sdk/src/headless/engine.ts:299-305`; class at `:114`)
  calls `WasmBoundary.seekSource`, which calls the export directly
  (`sdk/src/core/boundary.ts:1082-1107`). The existing headless seek eval is at
  `sdk/test/capability-evals.mjs:162`.
- **Export mirrors.** `scripts/check-web-audioworklet.sh` (`expected_exports`, `:203`),
  `tools/parameter-metadata/src/abi_layout.rs` (`EXPORTS`, `:135`), `scripts/check-abi-layout-v1.py`
  (`:191`) with its fixture `scripts/fixtures/abi-layout-v1-self-test.json`,
  `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts` and the hermetic mock in
  `scripts/test-web-audioworklet.mjs` (`:2277`).
- **The anchored browser form** is *Export transaction apply and anchored seek from the browser
  engine module* (#1293), which lands first. This slice exports the anchored session seek beside
  #1293's anchored source seek (D1).

## Decisions frozen for this slice

- **D1. The export.** `miso_engine_web_v1_session_seek(handle, generation: u64, frame: u64) -> u32`.
  It calls #1642's `SessionState::session_seek` (#1642 D1's wrapper, which synchronizes the
  epochs and calls `SourceControlSet::seek_session(generation, frame, None)` on the newest
  providers), the source routing every browser source entry point uses after #1381 D2, on the
  control half #1387 D2 picks (the Worker in `worker` mode, the worklet's control handler in
  `single` mode). It maps an error through `source_result`. It does **not** call `prepare_source_seek`: every consumer
  observes the seek through its command queue at the next block, the timeline included, so all of
  them change in one block. On an instance that holds no control half it refuses with
  `RESULT_REFUSED_LIFECYCLE` and changes nothing, the rule #1293 D1 sets for every control-half
  export. Beside it, `miso_engine_web_v1_session_seek_at(handle, generation: u64,
  frame: u64, anchor: u64) -> u32` passes `Some(anchor)`, with #1293's anchor rules, so a browser
  host loops or starts the session in exact time (README A1.2).
- **D2. Headless `seek`.** `OfflineEngine.seek(frame: bigint, options?: { generation?: bigint }):
  { generation: bigint } & EngineCallResult` calls a new `WasmBoundary.sessionSeek` once, which
  calls the export. The generation is `options.generation`, else one above the largest generation
  this engine object has passed through `seek`, `seekSource` or `submitSource` (every source and
  the timeline start at 1). A stale generation is refused with `source.generation.stale` and
  nothing changes. The caller then submits each source's PCM from its clamped frame,
  `min(frame, source frames)`, with that generation, as after `seekSource`.
- **D3. Mirrors.** The two new exports join every list in the context; the generated SDK files are
  regenerated with the repository's generators; every checker's self-test passes.
- **D4. The acked-batch question.** The export is #1642's all-or-nothing function: an OK means
  every consumer holds the seek, and a refusal changes nothing. No ack can precede a drop.

## Deliverables

1. D1 in `hosts/host-web/src/ffi.rs` and `hosts/host-web/src/lib.rs`, with a native test.
2. D2 in `sdk/src/headless/engine.ts` and `sdk/src/core/boundary.ts`, with an eval.
3. D3's mirrors and generated files.

## Authorized paths

- `hosts/host-web/src/ffi.rs`, `hosts/host-web/src/lib.rs`, `hosts/host-web/web/` and
  `crates/control-plane/src/` as #1387 and #1381 leave them (the session seek export, its routing
  beside the source seek's, and its call into #1642's function only),
  `hosts/host-web/src/tests.rs`
- `sdk/src/headless/engine.ts`, `sdk/src/core/boundary.ts`, `sdk/src/generated/abi.ts`,
  `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/test/capability-evals.mjs`
- `scripts/check-web-audioworklet.sh` (export list only), `scripts/check-abi-layout-v1.py`,
  `scripts/fixtures/abi-layout-v1-self-test.json`, `scripts/test-web-audioworklet.mjs` (the mock
  export only), `tools/parameter-metadata/src/abi_layout.rs`
- No new path for gate 1: `hosts/host-web/src/tests.rs` reads the timeline through #1641's
  host-core re-export (host-web has no `graph` dependency).

## Non-goals

- The shipped host message, the browser SDK `seek` and the PCM feed's ring rule (#1644).
- The anchored source seek's export (#1293), which lands first.
- Any change to the routing of *Move browser source submission and seeks into the Worker* (#1387),
  which lands first (through #1293): the session seek takes the thread the source seek takes.
- Any change to the per-source seek's semantics.

## Hazards

- **Stream H owns host-web** (`docs/handoffs/decision-15-2026-10-05/STREAMS.md`). Root sequences
  the merge with #1293, #1332 and #1387.
- **A browser app that keeps using ring seeks** after a session seek at generation `g` makes the
  feed call the per-source export with `g`, which is refused as stale and stalls that ring. No
  browser path calls the new export until #1644 adds its ring rule; both merge in batch R1.
- **The shipped artifact changes.** Report its new digest and size in the PR; the committed pin is
  the release fingerprint and is not re-pinned here (`scripts/build-web-audioworklet.sh:6-10`).

## Objective gates

1. **One block for everything** (`hosts/host-web/src/tests.rs`, new). A two-source session at
   quantum 128, playing. Call the `extern "C"` export with generation 2 and frame 6,000, submit one
   generation-2 quantum per source from its clamped frame, render one block: both sources read
   their seek frames and the timeline reads 6,000 (#1641's test reader) in that block. A stale
   generation returns `RESULT_INVALID_ARGUMENT` and the next block equals a run without the call.
   One full source slot returns `RESULT_BACKPRESSURE` and nothing changes.
2. **Headless** (`sdk/test/capability-evals.mjs`, new case beside the seek case at `:162`). After
   `seek(4096n)` and one quantum per source at the returned generation, the next render reads frame
   4,096 from a source. The SDK has no timeline reader and no stored automation renders before
   #1647, so this case does not observe the timeline; gate 1 does, through the same export
   (#1641's reader, re-exported by host-core). A second `seek` without
   options uses the next generation. A stale explicit generation returns the typed refusal.
3. **Mirrors.** Each mirror list contains the export, and the checkers fail if one omits it.
3a. **Role and pending candidate** (`hosts/host-web/src/tests.rs`, new). Each of the two exports,
   called on an instance without a control half, returns `RESULT_REFUSED_LIFECYCLE` and changes
   nothing (as #1293 gate 5). Commit a structural transaction that adds a source through
   `miso_engine_web_v1_apply` (#1293), then call the session seek before the swap: at the swap
   block the carried source, the added source and the timeline all read the seek's frame (#1642
   gate 4, on the browser).
4. **No rendered bit moves** for a session that never calls the new export: the browser legs of the
   `browser` job in `.github/workflows/qualification.yml` pass with unchanged digests.
5. **Commands:**
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`
   - `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
     `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`
   - `python3 -B scripts/check-abi-layout-v1.py` with its self-test
   - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
     `memset_pattern16` call; fix one in code, never by a ceiling)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`, `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1 turns red if the browser export seeks consumers in different blocks (for example through
  `prepare_source_seek` for the sources only), seeks with logic of its own, or pushes before every
  slot is checked.
- Gate 2 turns red if the headless SDK picks a stale generation or does not call the session seek
  export.
- Gate 3 turns red if a mirror lets the SDK call a name the module lacks.
- Gate 3a turns red if the export runs on the render half, or if the browser seeks the running
  plan's producers instead of the newest committed session's while a candidate waits.

## Dependencies

- #1642 *Seek the timeline and every source in one C ABI call* (its D1 function; #1640 and
  #1641 come with it).
- *Export transaction apply and anchored seek from the browser engine module* (#1293): the
  anchored export's rules and its mirrors.
- Batch: R1. #1644 *Seek the timeline and every source from the browser SDK and the PCM feed*
  builds on it.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **Single instance first** (#1562's decision 17 amendment). The control half is the worklet's
  message handler (`single` mode); #1332 later deploys the same exports in the Worker with no other
  logic. D1's "the Worker in `worker` mode" waits for #1332 and adds nothing here.
- **Anchors.** `miso_engine_web_v1_source_seek` is `hosts/host-web/src/ffi.rs:3251`; the control
  exports of #1562a are `:3308-3383`.
- **Dependencies.** #1293 (open) for the anchored export's rules; #1381a has landed, #1381 and #1387
  are open. Decision 17 C13 applies as in #1642's amendment.

## Amendment (controls coordinator, root rulings, 2026-10-15)

- **Gate 3a uses a dropped export.** Root ruling B15-R8 (2026-10-15, recorded in #1293) drops #1293's
  D1 staging, and decision 17 dropped `miso_engine_web_v1_apply`. Gate 3a commits its structural
  transaction as a `SESSION_TRANSACTION_APPLY` frame through #1562's generic entry
  (`miso_engine_web_v1_control_request_ptr` and `miso_engine_web_v1_control_command`), and reads
  the outcome from the response frame. The rest of the gate is unchanged.
- **Discontinuity first.** Root ruling B15-R7 (2026-10-15, recorded in #1323): the headless `seek`
  (D2) calls `miso_engine_web_v1_declare_discontinuity` (#1323) before it calls the session seek
  export, as a C ABI host declares before its session seek (#1642). So browser latency floors reset
  as they do on the C ABI. `WasmBoundary` gets the call. #1323 adds the export and its mirrors and
  is a dependency. Gate 2 asserts the declaration comes first.

## Amendment (controls coordinator, batch 16 check, 2026-10-16)

Checked against `35ac17545`.

**Landed dependencies.** #1309, #1316, #1323 (with `miso_engine_web_v1_declare_discontinuity`,
`hosts/host-web/src/ffi.rs:3354`) and #1562 are closed and on main. #1642 and #1293 are open and in
this batch; both land first. #1387 and #1332 stay open: `single` mode only, as the amendment of
2026-10-14 says. #1644 is open and not in this batch.

**Moved anchors.**
- `miso_engine_web_v1_source_seek` is `hosts/host-web/src/ffi.rs:3251-3276`. `seek_source` is
  `hosts/host-web/src/lib.rs:3533-3565`; it calls `ready.control.seek_unserviced` (`:3551-3553`),
  then `plan.prepare_source_seek` (`:3556`). `ready.control` is the `SessionState` (#1381a).
- D1 "maps an error through `source_result`": `SessionState` returns `SourceFailure`, so the map
  is `source_failure_result` (`lib.rs:5197`), which calls `source_result` (`:5181`).
- Headless: `OfflineEngine` is `sdk/src/headless/engine.ts:125`; `submitSource` `:419-427`;
  `seekSource` `:429-435`. `WasmBoundary.seekSource` is `sdk/src/core/boundary.ts:1144-1167`. The
  headless seek eval is `sdk/test/capability-evals.mjs:197`.
- Mirrors: `expected_exports` `scripts/check-web-audioworklet.sh:274`; `EXPORTS`
  `tools/parameter-metadata/src/abi_layout.rs:126` (105 today, 108 after #1293, 110 after this
  slice) and `scripts/check-abi-layout-v1.py:120`. The list is sorted: `session_seek*` goes after
  `service`, #1293's `source_seek_*` after `source_seek`. Merge #1293 first.
- The mock in `scripts/test-web-audioworklet.mjs` (`:2741`) lists only exports the worklet script
  calls. This slice adds no worklet call, so the mock needs no edit.

**The code's truth.**
- **No generation tracking today.** The headless SDK passes the caller's generation straight
  through (`seekSource`, `submitSource`); it keeps no generation state. D2's default adds the
  first such state to `OfflineEngine`.
- **No discontinuity call today.** No SDK file calls `miso_engine_web_v1_declare_discontinuity`
  (only `sdk/src/generated/abi.ts` names it). B15-R7's `WasmBoundary` call is new here.
- **Prepare.** The browser's plain seek prepares the consumer at the call, so new PCM enters at
  once. D1's session seek does not (C ABI parity): a ring full of old-generation PCM refuses the
  first submission with backpressure until one block renders. Gates 1 and 2 feed rings with room.
- **Thread.** In `single` mode the worklet instance is the control half, so only the caller keeps
  the export out of `process()` (the SDK calls it from the message handler, as for
  `declare_discontinuity`). Gate 3a's "not ready" case cannot show a render-half call.
- **Paths.** `crates/control-plane/src/` and `hosts/host-web/web/` need no edit:
  `SessionState::session_seek` is #1642's, and no worklet script changes.
- Gate 3a's structural transaction goes through #1562's control entry, as amended on 2026-10-15.

**Hazard (#1644 not in this batch).** After this slice, the headless `seek` is the export's only
SDK caller. The browser SDK and the PCM feed do not call it until #1644, so the stale-ring stall
in the hazards cannot happen in this batch. The headless default generation counts `seekSource`
and `submitSource` too, so a headless caller that mixes them stays ahead.

**Decision 17.** D1 calls the one core function (`SessionState::session_seek`); host-web keeps
pointer checks and the result map. Open points are in the coordinator's report: the two new
exports and the C13 list, the SDK-chosen generation (D2), and a parity row in
`hosts/host-web/tests/control_parity.rs` (browser session seek against the C ABI session seek).

## Amendment (root rulings, batch 16, 2026-10-16)

- **B16-R1.** The two exports `miso_engine_web_v1_session_seek` and `_session_seek_at` are in
  decision 17 C13's list of approved direct exports (seek transport behind
  `SessionState::session_seek`).
- **B16-R5, D2 stands.** The SDK chooses the generation, one above the largest it has used. It is a
  host transport token, as on the C ABI (#1642 D3). The core refuses a stale one (and, under
  #1642's B16-R2 fence, a generation not above every generation published on the session).
- **B16-R6, new gate 6 (parity row).** In `hosts/host-web/tests/control_parity.rs` (now an
  authorized path): a plain session seek and a late anchored session seek on both hosts at 48 and
  96 kHz, two sources of different lengths. After each block, from the call through the applying
  block and after, the browser and C ABI seek reports of both sources are equal field by field,
  and the timeline (#1641's host-core test reader) reads the same value on both hosts. Red if
  either host splits the seek across blocks, clamps differently or drops the lateness. It needs
  #1642's C export.

## Decision record: #1643 (batch 16)

Built on `5567f7487` (worktree `codex/controls-batch-16b`).

**What shipped.**
- **D1.** `miso_engine_web_v1_session_seek(handle, generation, frame)` and
  `miso_engine_web_v1_session_seek_at(handle, generation, frame, anchor)` in
  `hosts/host-web/src/ffi.rs`, one shared body `AudioWorkletEngineHost::session_seek(generation,
  frame, Option<anchor>)` in `lib.rs`: it calls `SessionState::session_seek` (the C ABI's call,
  after its service step) and maps a refusal through `source_failure_result`. host-web holds no seek
  logic and never calls `prepare_source_seek`. A host that is not ready refuses with
  `RESULT_REFUSED_LIFECYCLE` and changes nothing; both exports run the service step, so the page
  calls them from the message handler only (neither is `render_locked`).
- **D2.** `OfflineEngine.seek(frame, options?) -> { generation } & EngineCallResult`
  (`sdk/src/headless/engine.ts`) and `WasmBoundary.declareDiscontinuity()` / `sessionSeek()`
  (`sdk/src/core/boundary.ts`). `sessionSeek` calls `miso_engine_web_v1_declare_discontinuity`
  first (B15-R7), stops at its refusal, then calls the session seek export. The generation is
  `options.generation`, else one above the largest this object has passed (starts at 1). The
  counter (`#highestGeneration`) is the only state added; `seek`, `seekSource` and `submitSource`
  raise it on an `ok` result only, so a refusal (backpressure, stale) never moves it.
- **D3.** Mirrors: `check-web-audioworklet.sh` `expected_exports`, `abi_layout.rs` `EXPORTS` (108 ->
  110), `check-abi-layout-v1.py` `EXPORTS` and the fixture, `sdk/assets/miso-engine-v1-abi-layout.json`
  and `sdk/src/generated/abi.ts` (regenerated with `node codegen/assets.mjs` and `node
  codegen/generate.mjs`). The ABI self-test gains two mutations (each new export dropped): 36
  caught, from 34. The test-worklet mock needed no edit.

**Departures.**
- **Test-support plumbing outside the listed paths** (mechanical): `hosts/host-web/Cargo.toml`
  `test-support` now also names `control-plane/test-support` and `engine/test-support`, so a native
  host-web test reaches the active plan's timeline through `PlanState::test_owner_mut` and
  `RealtimePlanOwner::active_plan_mut` (B16-R8) and `host_core::test_only_timeline_at`:
  `AudioWorkletEngineHost::timeline_at_for_test` and `native_staging::timeline_at`, both
  `cfg(feature = "test-support")`. The workspace test steps already name both features
  (`check-test-support-ci.py` passes, 13 packages). No file under `crates/engine/src/realtime/`
  changed.
- **Gate 6: the C ABI's timeline is not read.** `capi::Plan` and `Session` expose no accessor
  outside the crate (`test_plan_timeline_at` is `cfg(test)` and crate-private), and a new public
  capi test hook is not mechanical. The parity row therefore compares the browser's and the C
  ABI's seek reports of all three sources field by field (at the call, after every block through
  the applying one and after), the output bits, and reads the browser's timeline: it moves in the
  applying block, runs on one quantum per block, and in the late seek lands on the frame the long
  sources' reports (equal on both hosts) name. The C ABI's timeline is pinned by capi's own gate
  (`runtime::tests::session_seek`). The row is `cfg(feature = "test-support")`.
- **Gate 1/6: the short source is not fed at its clamped frame.** A source clamped to its end has
  no frame to submit there (a chunk from the end is `RESULT_INVALID_ARGUMENT`), so the long source
  is fed and the short source underruns; its report still names the clamped frame.
- **Gate 1's full-slot case** holds the slot with an anchored `source_seek_at` (which, unlike the
  plain `source_seek`, does not prepare the consumer and so does not drain the slot).
- **A refused session seek still declares the discontinuity first** (the SDK holds no
  generation state beyond its counter, so it cannot know the core would refuse). The declaration
  resets latency floors and publishes nothing else; it is the C ABI host's order.
- **Gate 4** (no rendered bit moves) is the unchanged `browser` job digests; this slice adds no
  render code and no worklet call. Not run here (CI job).

**Gates -> tests.**
- Gate 1 (`hosts/host-web/src/tests.rs`, `tests::session_seek`):
  `both_forms_move_the_timeline_and_every_source_at_one_block` (plain seek at 2/6,000: long at
  6,000, short at 4,000, timeline 6,000 in one block; anchored seek held two blocks then landing
  at the anchor), `a_stale_generation_changes_nothing` (next block bits and timeline equal a run
  without the call; a per-source seek to 5 makes a session seek at 4 stale), 
  `one_full_slot_refuses_every_consumer` (`RESULT_BACKPRESSURE`; the short source's slot and the
  timeline stay untouched). All call the `extern "C"` exports.
- Gate 2 (`sdk/test/capability-evals.mjs`): "session seek declares the discontinuity first and moves
  the sources in the next block" (export call order `declare_discontinuity`, `session_seek`; output
  equals a `seekSource` twin and is not silent; default generations 2, 3, 10, 13, 14; a stale
  explicit generation is `invalidArgument`; usage errors reach no export).
- Gate 3: `check-web-audioworklet.sh` (compares the wasm's exports with `expected_exports`),
  `check-abi-layout-v1.py` and `--self-test` (36 mutations), `check-sdk-generated.sh`.
- Gate 3a (`tests::session_seek`):
  `the_session_seek_exports_refuse_a_host_with_no_control_half_and_change_nothing` and
  `a_seek_before_the_swap_reaches_the_carried_and_the_added_source_and_the_timeline` (the
  transaction goes through `native_staging::control_command`, the `control_command` export; the
  seek before the swap reaches the carried source, the added source and the timeline in the swap
  block, active epoch 0 -> 1).
- Gate 6 (`hosts/host-web/tests/control_parity.rs`):
  `the_session_seek_is_alike_on_both_hosts_for_plain_and_late_seeks` (48 and 96 kHz).

**Commands run (all pass).** `cargo test --locked -p host-web --features host-web/test-support`
(lib 145 passed, control_parity 20 passed, every other binary green); `cargo clippy --locked -p
host-web -p parameter-metadata --all-targets --all-features -- -D warnings`; `cargo fmt --all --
--check`; `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p host-web --all-features`;
`build-web-audioworklet.sh --named-twin`; `check-web-audioworklet.sh`; `test-web-audioworklet.sh`;
`check-sdk-generated.sh`, `check-sdk-types.sh`, `check-sdk-headless.sh` (565 SDK tests pass);
`check-abi-layout-v1.py` (document and `--self-test`); `check-session-map-shape.py` (and
`--self-test`); `check-cross-targets.sh` (PASS, no new `memset_pattern16` call);
`check-workspace-policy.sh`; `check-test-support-ci.py`; `cargo run -p realtime-policy` (153
regions, unchanged). The workspace-wide clippy runs at the batch gates.

**Artifact.** `miso-engine-v1-audio-worklet.simd128.wasm`: 4,207,840 bytes, sha256
`2fc4d8aaa1b9fb49f18c71cc8f2e0fc8fd1f096b96324e86bc6312e64bf92a74` (named twin 4,830,433 bytes,
`f6cb1e2c5d2a5d1ea88327d6331c2473ff7eeaa5d5f75eff550a8b19a2521b01`). The committed pin is not
re-pinned.

**Successors.** (1) A capi test hook that reads the active plan's timeline would let the parity row
compare both timelines directly. (2) #1644 (browser SDK `seek` and the PCM feed's ring rule) is the
next caller of the exports.

## Amendment (root ruling B16-R9, batch 16b review, 2026-10-16)

- **B15-R7's order is retired.** The headless `seek` (D2) and `WasmBoundary.sessionSeek` no longer
  call `miso_engine_web_v1_declare_discontinuity`: a plain session seek is a declared discontinuity,
  and the engine declares it in the same call, after every check, so a refused seek declares
  nothing. An anchored session seek keeps continuity and declares nothing (the loop and exact-start
  form; README A1.2). `WasmBoundary.declareDiscontinuity()` stays for a stop.
- **Gate 2 amended.** It asserts the SDK does not call the declaration before a session seek (the
  call log is `["session_seek"]`), and a new eval holds the B16-R9 rules on the headless path.

## Fold (batch 16b review), part B / B16-R9

- **SDK.** `sdk/src/core/boundary.ts` `sessionSeek` and `sdk/src/headless/engine.ts` `seek`: no
  declaration call; the doc comments say the engine declares in the same call. The browser export
  (`miso_engine_web_v1_session_seek`) runs the plain form through the host's `run_control`, so a
  plan the declaration publishes keeps its tables and counts as a blocking rebuild; the anchored
  export declares nothing.
- **Evals (`sdk/test/capability-evals.mjs`).** `session seek calls the session seek export alone and
  moves the sources in the next block` (gate 2, the call log is `["session_seek"]`; red with the
  SDK's declaration restored) and `a plain session seek declares the discontinuity itself, and a
  refused one declares nothing` (B16-R9 gate 4 on the parity session: t3's limiter removed through
  the new `remove_t3_limiter` structural frame, so #1285 floors hold the latency; a stale plain seek
  keeps the floors, an accepted one resets them, and no separate declaration is called; red with
  the SDK's declaration restored, and red against an artifact built with the core's declaration
  removed). The headless SDK has no anchored session seek, so rule 3 is held on the C ABI and the
  parity row only.
- **Fixture.** `hosts/host-web/tests/structural_frames_fixture.rs` writes one more frame,
  `remove_t3_limiter` (`RemoveTrackEffect` of t3's limiter), into
  `hosts/host-web/tests/fixtures/structural-frames.json` (blessed; the byte gate passes).
- **Browser tests.** `tests::session_seek::the_session_seek_exports_refuse_a_host_with_no_control_half_and_change_nothing`
  now asserts no seek waits in a slot after the refusals (`native_staging::session_seek_slots_empty`,
  test-support; MINOR f; red when the export pushes before its lifecycle refusal); its doc says the
  failed host keeps its control state. The `one_full_slot_refuses_every_consumer` comment is
  corrected (MINOR e: the slot frees at the next block's pop; it is full only because no block
  renders between the calls).
