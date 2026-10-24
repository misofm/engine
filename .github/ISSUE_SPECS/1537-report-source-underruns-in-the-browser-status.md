# Report source underruns in the browser status

Decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Successor S3 of *Hold effect parameter, bypass and EQ-target values in latest-target cells*
(#1345), from root ruling 6 (2026-10-13): #1345 Amendment 6 had to use a peak floor as an
"underrun substitute" because the host publishes no source-underrun count. Code anchors verified at
`806de000a` (`codex/d15-stream-b3`). Verify them again on `main` before the first edit.

> **Superseded route (root decision, 2026-10-24; see the end of this spec).** The count is
> protocol counter `CounterId 20 SOURCE_UNDERRUN_BLOCKS`, served on both hosts through
> `COUNTERS_GET`, not a `WebStatus` word. D2-D4, R1 and the status-layout gates 2-4 no longer apply;
> D1, D5, D6 and R2-R4 stand. The root decision lists the slice, the paths and the gates.

## Product outcome

AGENTS.md: "underrun emits zero plus a counter". The engine counts underruns, but neither browser
status reply nor the headless SDK status shows a count, so an app cannot tell a starved stem from a
quiet one and a qualification gate must guess from peaks. After this issue the browser status
(`MisoStatus.sourceUnderrunBlocks`, from the worklet host and from the headless SDK's
`engine.status()`) carries one monotone count: the number of rendered blocks in which at least one
source had to play zeros because the host had not delivered PCM it owed.

## Context

- **What the engine counts today.**
  - Per source, on the render thread: `PcmSourceConsumer` keeps saturating `underrun_frames` and
    `underrun_events` (`crates/source/src/lib.rs:1013-1014`), raised in the missing-PCM branch of
    `begin_block_play` (`:1138-1188`; the per-call report `SourceReadReport` `:404-423`). They leave
    the render owner only after it is disarmed (`SourceConsumerTelemetry` `:438-448`,
    `copy_after_disarm_telemetry` `:1855-1878`), and no host reads them while it renders.
  - Per block, graph-wide: the source driver's `begin_block` (`:1753-1788`) sets
    `block_validity.source_underrun` when a vacant source entry has nothing to play (`:1766-1770`)
    or a consumer reports `underrun_event` (`:1774`). The type is `GraphObservationValidity`
    (`crates/graph/src/lib.rs:2717-2733`; its doc: graph-wide, not attributable to a source). The
    graph render computes it every block (`source_validity`, `:3723-3746`) and hands it only to
    observers (spectrum windows in `crates/host-core/src/spectrum.rs`). The source-set wrapper's
    `observation_validity()` is `pub(crate)` (`:2514-2516`), and `block_validity` stays set until
    the next `begin_block`.
  - Past the end of a region is not an underrun (`SourceReadReport::underrun_frames` counts
    "in-region" frames only).
- **What no host exposes.** The C ABI has no underrun counter (`crates/capi/include/miso_engine_v1.h`;
    protocol `CounterId` 1-16, `crates/protocol/src/message_wire.rs:624-643`, has none). *Anchor every
    seek on the plan's source-read clock* (#1316) gives the C host a per-source seek report with
    `cumulative_underrun_frames` and `cumulative_held_frames` (its D2, D6). The browser has nothing.
- **Held is not underrun.** *Report held source blocks apart from underruns* (#1318, stream B order
  13, open) makes a block that waits for an anchored seek *held*, not an underrun, in the report,
  the counters and the graph-wide flag (its D1-D4). After #1318 the graph flag means exactly "the
  host did not deliver PCM it owed".
- **How a status word reaches every reader** (the #1399 template, which added
  `live_values_superseded`):
  - `WebStatus` (`hosts/host-web/src/lib.rs:1295-1332`): 80 bytes; `live_values_superseded` at 48;
    `reserved: [u64; 3]` at 56, 64, 72 (`:1331`), required zero. Initial value `:2244-2245`.
    `STATUS_BYTES` `:1208`; the resource row `status_bytes` `:6219`.
  - `render_next` (`:3274-`, in a `REALTIME_POLICY` region): after `ready.host.plan.render(..)`
    returns `Ok`, it advances `next_absolute_sample` and `rendered_quanta` and loads
    `live_values_superseded` (`:3302-3309`).
  - Layout asserts: `hosts/host-web/src/tests.rs:286` (`size_of::<WebStatus>() == 80`), `:411-414`
    (offsets).
  - Layout mirrors: `status_fields` in `tools/parameter-metadata/src/abi_layout.rs:319-` (12 fields,
    `liveValuesSuperseded` at `:344`), which generates `sdk/assets/miso-engine-v1-abi-layout.json`
    (`structures.status`, `:148-162`, `"bytes": 80`) and `sdk/src/generated/abi.ts`;
    `scripts/check-abi-layout-v1.py:249` (`"status": 80`) and its fixture
    `scripts/fixtures/abi-layout-v1-self-test.json:148-162`.
  - The worklet's `readStatus()` (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:944-960`):
    throws unless the size word is 80 and words 56, 64, 72 are zero; returns the fields that the
    status reply spreads (`:1076-1084`).
  - The main-realm host's status reply check: the exact field list
    (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1016-1020`) and value checks
    (`:1129-1137`, `validU64(message.liveValuesSuperseded)` at `:1135`).
  - `MisoStatus` in both `.d.ts` twins (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts:879-899`,
    byte-identical in `sdk/src/browser/shipped-host.d.ts`).
  - The headless SDK: `EngineStatus` (`sdk/src/core/boundary.ts:291-309`, `liveValuesSuperseded`
    `:307`) and `status()` (`:444-471`), which reads words by name from the generated layout.
  - Test readers: the hermetic harness's fake status reply and its status-reply mutations
    (`scripts/test-web-audioworklet.mjs:160-167`, `:1376-1392`, `:3282-3291`); the direct oracle
    (`hosts/host-web/tests/browser-v1/direct-oracle.mjs:50-60`: size 80, words 56, 64, 72 zero);
    `hosts/host-web/tests/browser-v1/expected.json:56` (`"statusBytes": "80"`, an exact row of
    `scripts/check-browser-expected-resources.py`'s `EXACT_ROWS`, `:79-96`).
- **The three reserved words are spoken for.** *Publish the applied-revision watermark in the
  browser status* (#1349, stream B order 30, after H #1381) D2 assigns offsets 56, 64 and 72 to the
  watermark and says "size stays 80 and no expansion word remains".
- **Executor plumbing.** The engine's `PreparedPlanExecutor` trait
  (`crates/engine/src/realtime/plan.rs:273-514`) has defaulted read methods (for example
  `observation_binding_counts`, `:503-505`). `PreparedRenderPlan` (`:661-677`) sets
  `response_snapshot_valid` false at the start of `render_inner` (`:1099-1131`) and true after a
  successful block. The graph's executor is `impl PreparedPlanExecutor for GraphExecutor`
  (`crates/graph/src/lib.rs:3547-`).

## Decisions frozen for this slice

- **D1. What is counted.** `source_underrun_blocks`: the number of blocks the session rendered
  successfully in which the graph-wide `GraphObservationValidity::source_underrun` was set. One
  saturating `u64`, zero at boot, never reset, never decreasing. It is the fact the engine already
  computes once per block; it names no source (the flag is graph-wide, as its doc says). A block
  past the end of a region is not counted; a held block is not counted once #1318 is on `main`
  (dependency). Per-source frame counts are #1316's report, not this word.
- **D2. Where it is counted.** In `render_next`, after `render` returns `Ok` and before the status is
  published: if `ready.host.plan.last_block_source_underrun()` is true, add 1 (saturating) to
  `status.source_underrun_blocks`. The word lives in `WebStatus`, which the host owns and no plan
  owns, so a plan swap does not reset it. A failed block adds nothing.
- **D3. Plumbing (two defaulted methods, no new render work).**
  - `PreparedPlanExecutor::last_block_source_underrun(&self) -> bool`, default `false`, documented as
    "whether the block this executor last rendered had a source in-region underrun (graph-wide);
    read on the render thread right after a successful render".
  - `PreparedRenderPlan::last_block_source_underrun(&self) -> bool`: `false` unless
    `response_snapshot_valid`, else the executor's answer (`false` without an executor).
  - `GraphExecutor` implements it as its source set's `observation_validity().source_underrun`, and
    `false` with no source set. It reads the value the render already stored; it does no new work.
  - All three sit in `REALTIME_POLICY` regions, allocate nothing, and add no panic path to the Wasm
    render export's call graph.
- **D4. Status layout: the record grows by one word.** `WebStatus` gains
  `source_underrun_blocks: u64` at offset **80**; the size becomes **88**; `reserved: [u64; 3]` at
  56, 64, 72 stays reserved and required zero, for #1349. Every mirror in Context follows: the size
  checks (80 -> 88) in `tests.rs`, `readStatus()`, `check-abi-layout-v1.py` and its fixture, the
  direct oracle and `expected.json`'s `statusBytes`; `status_fields` gains the field (13 fields);
  the regenerated asset and `abi.ts`; the worklet returns `sourceUnderrunBlocks` (a `bigint`, offset
  80); the host's field list gains it and validates it with `validU64`; both `.d.ts` `MisoStatus`
  gain `readonly sourceUnderrunBlocks: bigint` with D1's meaning; `EngineStatus` and the headless
  `status()` gain it. A resource row computed from `size_of::<WebStatus>()` follows by construction;
  a hand-written `80` elsewhere is a finding to record. `ABI_VERSION` does not change and there is
  no feature bit: an in-place V1 field, as #1399 D2 (the browser artifact set ships host, worklet and
  Wasm together).
- **D5. Naming.** `source_underrun_blocks` / `sourceUnderrunBlocks`, unversioned (AGENTS.md: internal
  and payload names are born unversioned).
- **D6. Acked-batch question: can an ack ever precede a drop?** No queue changes. The word is a
  level the render writes; a submit's acknowledgement keeps its meaning.

## Deliverables

1. D3 in `crates/engine/src/realtime/plan.rs` and `crates/graph/src/lib.rs`.
2. D1, D2 and D4 in `hosts/host-web/src/lib.rs` and every mirror; the regenerated layout asset and
   `abi.ts`.
3. Gates 1-4.
4. This spec's Attempt record: the red-first run of gate 1 and the mutation runs.

## Authorized paths

- `crates/engine/src/realtime/plan.rs` (the trait method and the accessor only)
- `crates/graph/src/lib.rs` (`GraphExecutor`'s one method only; named exception, the file is stream
  A's)
- `hosts/host-web/src/lib.rs` (`WebStatus`, its initial value and `render_next`'s status refresh
  only), `hosts/host-web/src/tests.rs`
- `tools/parameter-metadata/src/abi_layout.rs` (`status_fields` only),
  `sdk/assets/miso-engine-v1-abi-layout.json` and `sdk/src/generated/abi.ts` (regenerated only)
- `scripts/check-abi-layout-v1.py` (the status size only), `scripts/fixtures/abi-layout-v1-self-test.json`
  (the status entry only)
- `hosts/host-web/web/miso-engine-v1-audio-worklet.js` (`readStatus()` only),
  `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js` (the status reply's field list and
  validation only), `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` and
  `sdk/src/browser/shipped-host.d.ts` (`MisoStatus` only)
- `sdk/src/core/boundary.ts` (`EngineStatus` and `status()` only), `sdk/test/capability-evals.mjs`
  (one eval)
- `scripts/test-web-audioworklet.mjs` (the fake status reply and gate 3's cases only)
- `hosts/host-web/tests/browser-v1/direct-oracle.mjs` (the status size check only),
  `hosts/host-web/tests/browser-v1/expected.json` (`statusBytes` only)
- This spec.

## Non-goals

- No C ABI counter. The C host's per-source counts are #1316's seek report; whether the C ABI also
  gets a session-wide `COUNTERS_GET` counter is an open question for root.
- No per-source attribution, no frame count, no change to `crates/source` (the flag exists).
- No change to the qualification pages. Using the word in S1's unsubscribe gate ("no underrun block
  after the acknowledgement") and in H #1387's gate 4 ("zero underruns") is their work.
- No change to the watermark words (#1349) or to status sharing (S2).

## Objective gates

Each test-value sentence answers: which plausible defect turns this red that no existing test
catches?

1. **The status counts underrun blocks (new host-web test, red first).**
   `the_status_counts_each_block_a_source_underran` in `hosts/host-web/src/tests.rs`, built like
   `finite_source_boundary_requires_only_the_actual_final_marker` (`tests.rs:5344-5366`): a host from
   `meter_tail_host_for_blocks(48_000, 128, 32, N)` (a source of `N` blocks) fed through
   `feed_and_render_finite_source` (`:5319-5342`, one submit then one render; the last block carries
   the end-of-region marker). Read `status().source_underrun_blocks` at boot and after every render:
   - Case A, `N = 8`: feed-and-render blocks 0 and 1, then `render_next()` twice with nothing
     submitted: exactly `0` at boot, then `[0, 0, 1, 2]`.
   - Case B, `N = 2`: feed-and-render blocks 0 and 1 (block 1 ends the region), then `render_next()`
     twice: exactly `0` at boot, then `[0, 0, 0, 0]`.
   Run it on the tree without D2 first and record the red.
   *Test value:* turns red if the word counts every block, never counts, counts the previous block's
   flag (a read before the render or from a stale plan), or counts silence after the end of a region
   as starvation; no test reads an underrun count from any host today.
   *Mutations (apply: red; revert: green):* add 1 every block (A red at the first render); remove
   the add (A red at the third); read the flag before `render` (A reads `[0, 0, 0, 1]`: red); count
   "no PCM copied this block" instead of the flag (B red).
2. **Layout.** `tests.rs` asserts `size_of::<WebStatus>() == 88`,
   `offset_of!(WebStatus, source_underrun_blocks) == 80` and `offset_of!(WebStatus, reserved) == 56`;
   `python3 -B scripts/check-abi-layout-v1.py` (and its self-test) passes; the regenerated asset and
   `abi.ts` match (`bash scripts/check-sdk-generated.sh`).
   *Test value:* turns red if a mirror puts the word at another offset, keeps size 80, or takes one
   of the reserved words #1349 needs.
3. **The status reply carries the word** (`scripts/test-web-audioworklet.mjs`):
   - a processor whose status block holds a nonzero word at offset 80 (and zero at 56, 64, 72)
     answers a status request with that `sourceUnderrunBlocks`, beside the existing
     `liveValuesSuperseded` case (`:3282-3291`); a size word of 80 is now refused;
   - the host accepts a fake reply with `sourceUnderrunBlocks: 3n`, and refuses (result 255) a reply
     that omits it, carries a `number`, a negative value or `1n << 64n`, as the
     `liveValuesSuperseded` cases at `:1376-1392` do.
   *Test value:* turns red if the worklet still reads an 80-byte record (every status read would
   throw after this change) or the host drops, misnames or fails to validate the field.
4. **Headless SDK** (`sdk/test/capability-evals.mjs`, one eval): a headless engine whose source is
   not fed reads `status().sourceUnderrunBlocks === 0n` at boot and `2n` after two renders.
   *Test value:* turns red if `EngineStatus` omits the word or `status()` reads it by a wrong name
   from the generated layout.
5. **Realtime and artifact** (as `qualification.yml` runs them):
   - `cargo run --locked --release -q -p realtime-policy`;
   - `bash scripts/build-web-audioworklet.sh --named-twin <empty T> <empty A>`, then
     `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration A T/miso-engine-v1-audio-worklet.simd128.named.wasm`
     (no new panic or allocation path from `process()`);
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts A`;
   - `bash scripts/test-web-audioworklet.sh`; the CI browser command once per browser, unloaded.
6. **Workspace.** `cargo test --locked -p host-web --features test-support`;
   `cargo test --locked -p engine`; `cargo test --locked -p graph`;
   `bash scripts/check-sdk-types.sh`; `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   `bash scripts/check-workspace-policy.sh`.

## Dependencies

- **After #1318** (stream B order 13; no dependencies of its own), so the word excludes held blocks
  from its first commit.
- **Before H #1387** (its gate 4 requires "zero underruns" in three browsers and needs a readable
  count) and preferably **before H #1332** (the Worker rewires the status path and should inherit the
  word).
- **#1349 then fills 56, 64, 72 and keeps size 88** (root amends #1349 D2 "size stays 80" and its
  gate 4 "size 80" to 88 when this lands first; no design change to #1349).

## Placement (root edits `STREAMS.md`)

Recommended: **stream B**, after #1318.
- Stream B owns `crates/engine/src/realtime` and, by named exception, `hosts/host-web` status code
  (#1399, #1349); this is the same kind of status word.
- Named exception: `crates/graph/src/lib.rs` (`GraphExecutor`'s one method), stream A's file.
- Hot-file rows (line numbers of `STREAMS.md` at `806de000a`): `crates/engine/src/realtime/plan.rs`
  (`:86`) and `crates/graph/src/{lib,runtime}.rs` (`:94`): B S3 in any order with the rest, the later
  slice rebases. `hosts/host-web/src/lib.rs` (`:101`) and `tests.rs` (`:108`): B S3 before H #1332,
  in any order with the B, D, E and J slices. `tools/parameter-metadata/src/abi_layout.rs` (`:106`),
  `scripts/check-abi-layout-v1.py` (`:107`), the worklet-host `.js`/`.d.ts` row (`:109`) and
  `scripts/test-web-audioworklet.mjs` (`:113`): B S3 in any order with H and B #1349, before C #1406.

Alternative: stream H, if root prefers the browser status to stay with the browser stream; it then
needs named exceptions for `crates/engine/src/realtime/plan.rs` and `crates/graph/src/lib.rs`.

## Attempt record

- Attempt 1 (counter route, root decision below), commit `7880d0a4f`. Red first: with counter 20
  served but nothing counting, gate 1 read `[0, 0, 0, 0]` for case A (expected `[0, 0, 1, 2]`).
  Mutations (each red, then restored green): add every block -> A `[1, 2, 3, 4]`; remove the add
  -> A `[0, 0, 0, 0]`; read the flag before `render_inner` -> A `[0, 0, 0, 1]`; count
  `copied_frames == 0` in the source driver -> B `[0, 0, 1, 2]`. Served-set mutant
  (`SourceUnderrunBlocks` left out of `SERVED_COUNTERS`): host-core's served-set test red, counter
  capacity 12 not 6. One path past the scope note: `crates/control-plane/src/compile.rs`, the
  `SharedPlanState` constructor's new field (one line; the struct cannot compile without it).

## Root rulings (2026-10-13)

- **R1. Layout.** WebStatus grows to 88 bytes with `source_underrun_blocks` at offset 80. Offsets
  56, 64 and 72 stay free for #1349. Root amends #1349 D2 ("size stays 80" becomes "size 88, set
  by this issue").
- **R2. Unit.** A graph-wide count of blocks in which any source underran. Per-source detail
  stays with the observers and #1316's seek report.
- **R3. C ABI parity: yes.** Add the same session-wide counter to the C ABI (`COUNTERS_GET`,
  the next free CounterId), so the two hosts report the same fact. This replaces the "No C ABI
  counter" non-goal. The C ABI qualification doc and its gates are updated with it.
- **R4. Placement.** Stream B, after #1318, before H #1332, with a named exception for
  `crates/graph/src/lib.rs` (the counting site only).

## Root decision (2026-10-24): a protocol counter, not a status word

Root-level reversal of R1 and of D2-D4's status route, under decision 17's one control
abstraction. The count is protocol counter `CounterId::SourceUnderrunBlocks = 20`
(`SOURCE_UNDERRUN_BLOCKS`), in the host-core served set, so `COUNTERS_GET` answers it on the C ABI
and in the browser (`control_command`) from one count, as #1521 did for counter 19. `WebStatus`,
`MisoStatus` and `EngineStatus` do not change. Reasons:

1. **One control abstraction.** The browser renders through `control_plane::PlanState::render` and
   answers protocol frames through the same `SessionState::command` as the C ABI, so one served
   counter reaches both hosts; a status word would be a second surface for the same fact.
2. **Cost.** The status route edits about fifteen hand-sized mirrors (layout JSON, `abi.ts`, two
   `.d.ts` twins, worklet `readStatus`, host field list, layout checker and fixture, direct oracle,
   `expected.json`, `tests.rs` offsets); the counter route adds one registry value, one setter, one
   refresh line and one wire re-pin.
3. **Semantics.** The status record is the worklet's per-block level (clock, watermark). A
   session-wide, never-resetting count is what the counter registry holds
   (`live_values_superseded`, `strips_restarted`, `live_block_errors`).

**Slice.** D1's unit and D3's executor plumbing stand. `RealtimePlanOwner` keeps
`source_underrun_blocks: u64`; `render_active` adds 1 (saturating) on its `Ok` path when the plan
reports `last_block_source_underrun()`, so a failed block adds nothing and a plan swap does not
reset it. `SharedPlanState.source_underrun_blocks: AtomicU64` takes the owner's count on
`PlanState::render`'s `Ok` path (`Release`); `refresh_provider_counters` copies it into
`SessionControlProvider::set_source_underrun_blocks`. `SERVED_COUNTERS` becomes 7.

**Paths.** `crates/engine/src/realtime/{plan,plan_exchange}.rs` (`render_active`'s `Ok` path, the
field and its accessor only: #1482 edits `enter_block`/`note_claim`, #1576 edits
`copy_response_snapshot`); `crates/graph/src/lib.rs` (the one method);
`crates/control-plane/src/{plan,control}.rs` (and `compile.rs`'s `SharedPlanState` constructor, one
field); `crates/host-core/src/control_provider.rs`; `crates/protocol/src/message_wire.rs` and its
tests' ID round-trip row; `crates/capi/src/runtime/tests.rs` (`ALL_COMMAND_RESPONSE_VECTORS[9]`
re-pin); `crates/capi/include/miso_engine_v1.h` (one sentence); `hosts/host-web/src/tests.rs` (one
test); `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/C_ABI_V1_QUALIFICATION.md`; this spec.

**Gates.**
1. `the_counters_get_reply_counts_each_block_a_source_underran` (host-web): read
   `SOURCE_UNDERRUN_BLOCKS` through a `COUNTERS_GET` frame after each render (and a service step):
   case A (`N = 8`) `[0, 0, 1, 2]`, case B (`N = 2`) all `0`, `0` at boot. Red first; mutations as
   gate 1 above (PR evidence).
2. host-core's served-set test sets `set_source_underrun_blocks(k)`; capacity stays
   `PROVIDER_COUNTER_SLOTS`.
3. `ALL_COMMAND_RESPONSE_VECTORS[9]` re-pinned (length 0x130 -> 0x160, TLV 7 -> 8); the protocol
   ID round-trip takes 20 and refuses 21.
4. Package tests, `realtime-policy`, `check-protocol-control-policy.sh`, `check-capi-abi.sh`,
   `check-workspace-policy.sh`, fmt, clippy. No browser artifact gate: no JS, `.d.ts`, layout or SDK
   change.
