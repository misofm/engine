# Seek the timeline and every source from the browser module export and the headless SDK

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A1 (A1.2), under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

The shipped engine module exports the session seek of draft 05: one function on both hosts, with no
browser-only seek logic in Rust. It seeks the timeline and every source to one frame under one new
generation, all or nothing, and every consumer applies it at the next render block. The headless
SDK moves the playhead with one call, `seek(frame)`, which resolves with the generation it used;
the caller then submits each source's PCM from that frame under that generation. Until a source's
new PCM arrives it underruns, as after any seek today. The browser SDK and the PCM feed use this
export in draft 06b.

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
- **Draft 05's function.** `SourceControlSet::seek_session(generation, timeline_sample, anchor)`
  in host-core checks the timeline and every source producer, then pushes to all of them. The C ABI
  calls it; this slice makes the browser call the same function.
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
  It calls draft 05's `SourceControlSet::seek_session(generation, frame, None)` on the browser
  session's `SourceControlSet`, the one the source seek export uses (in the Worker's `SessionState`
  after #1381, on the thread #1387's routing picks), and maps an
  error through `source_result`. It does **not** call `prepare_source_seek`: every consumer
  observes the seek through its command queue at the next block, the timeline included, so all of
  them change in one block. Outside `STATE_READY` it returns `RESULT_WRONG_STATE` as
  `seek_source` does. Beside it, `miso_engine_web_v1_session_seek_at(handle, generation: u64,
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
- **D4. The acked-batch question.** The export is draft 05's all-or-nothing function: an OK means
  every consumer holds the seek, and a refusal changes nothing. No ack can precede a drop.

## Deliverables

1. D1 in `hosts/host-web/src/ffi.rs` and `hosts/host-web/src/lib.rs`, with a native test.
2. D2 in `sdk/src/headless/engine.ts` and `sdk/src/core/boundary.ts`, with an eval.
3. D3's mirrors and generated files.

## Authorized paths

- `hosts/host-web/src/ffi.rs`, `hosts/host-web/src/lib.rs`, `hosts/host-web/web/` and
  `crates/control-plane/src/` as #1387 and #1381 leave them (the session seek export, its routing
  beside the source seek's, and its call into draft 05's function only),
  `hosts/host-web/src/tests.rs`
- `sdk/src/headless/engine.ts`, `sdk/src/core/boundary.ts`, `sdk/src/generated/abi.ts`,
  `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/test/capability-evals.mjs`
- `scripts/check-web-audioworklet.sh` (export list only), `scripts/check-abi-layout-v1.py`,
  `scripts/fixtures/abi-layout-v1-self-test.json`, `scripts/test-web-audioworklet.mjs` (the mock
  export only), `tools/parameter-metadata/src/abi_layout.rs`

## Non-goals

- The shipped host message, the browser SDK `seek` and the PCM feed's ring rule (draft 06b).
- The anchored source seek's export (#1293), which lands first.
- Any change to the routing of *Move browser source submission and seeks into the Worker* (#1387),
  which lands first (through #1293): the session seek takes the thread the source seek takes.
- Any change to the per-source seek's semantics.
- A browser plan swap: the browser has no successor path yet (*Replace the running browser session
  in the Rust host*, #1290); the timeline's browser carry follows from draft 04b when it lands.

## Hazards

- **Stream H owns host-web** (`docs/handoffs/decision-15-2026-10-05/STREAMS.md`). Root sequences
  the merge with #1293, #1332 and #1387.
- **A browser app that keeps using ring seeks** after a session seek at generation `g` makes the
  feed call the per-source export with `g`, which is refused as stale and stalls that ring. No
  browser path calls the new export until draft 06b adds its ring rule; both merge in batch R1.
- **The shipped artifact changes.** Report its new digest and size in the PR; the committed pin is
  the release fingerprint and is not re-pinned here (`scripts/build-web-audioworklet.sh:6-10`).

## Objective gates

1. **One block for everything** (`hosts/host-web/src/tests.rs`, new). A two-source session at
   quantum 128, playing. Call the `extern "C"` export with generation 2 and frame 6,000, submit one
   generation-2 quantum per source from its clamped frame, render one block: both sources read
   their seek frames and the timeline reads 6,000 (draft 04b's test reader) in that block. A stale
   generation returns `RESULT_INVALID_ARGUMENT` and the next block equals a run without the call.
   One full source slot returns `RESULT_BACKPRESSURE` and nothing changes.
2. **Headless** (`sdk/test/capability-evals.mjs`, new case beside the seek case at `:162`). After
   `seek(4096n)` and one quantum per source at the returned generation, the next render reads frame
   4,096 from a source and the timeline reads 4,096 in the same block. A second `seek` without
   options uses the next generation. A stale explicit generation returns the typed refusal.
3. **Mirrors.** Each mirror list contains the export, and the checkers fail if one omits it.
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
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`, `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1 turns red if the browser export seeks consumers in different blocks (for example through
  `prepare_source_seek` for the sources only), seeks with logic of its own, or pushes before every
  slot is checked.
- Gate 2 turns red if the headless SDK picks a stale generation or does not seek the timeline.
- Gate 3 turns red if a mirror lets the SDK call a name the module lacks.

## Dependencies

- Draft 05 *Seek the timeline and every source in one C ABI call* (its D1 function; drafts 04a and
  04b come with it).
- *Export transaction apply and anchored seek from the browser engine module* (#1293): the
  anchored export's rules and its mirrors.
- Batch: R1. Draft 06b *Seek the timeline and every source from the browser SDK and the PCM feed*
  builds on it.
