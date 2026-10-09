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

## Decisions

- **D1. Both exports run inside `render_locked`.** `miso_engine_web_v1_source_submit` and
  `miso_engine_web_v1_source_seek` wrap their whole bodies in `render_locked(|| ...)`, as `render`
  does. Behaviour, result codes and the `SAFETY` reasoning are unchanged. No other export changes.
- **D2. The headers say what is true.** `render_lock.rs:1-18` and `ffi.rs:14-23` name the set as
  it then is: the static gate's three exports, the staging reads, the post-boot staging accessors,
  and `source_submit` and `source_seek` (called from `process()` on the SDK feed path and from the
  worklet's port handlers). The sentence that every post-boot worklet call on the render thread is
  render-locked becomes true; the implementer checks it export by export against both worklet
  files and records the list.
- **D3. A native gate.** A new integration binary, `hosts/host-web/tests/render_locked_source.rs`,
  registers `RenderLockedAllocator<System>` as its global allocator and holds exactly one test
  (as `render_locked_staging.rs` does, so nothing else shares the process-wide counter). It boots
  the observation session (`hosts/host-web/qualification/observation-session.json`, source
  `live-control-source`), renders a few blocks, then, on a fresh thread, calls through the exports:
  `source_submit` with a full-quantum chunk, with an end-of-region short chunk, and with a refused
  chunk (an invalid argument); `source_seek` with an accepted seek and a refused one. It asserts
  that the count is unchanged after each call. If the source stagings cannot be written natively,
  add a `native_staging` helper next to the existing ones (`ffi.rs:4564`), written the same way.
- **D4. Zero on the real code is the claim.** If the real code allocates in either export (the
  gate reads non-zero once D1 is in), the slice stops and reports the allocating call path to root;
  it does not loosen the gate or remove the wrap.

## Authorized paths

- `hosts/host-web/src/ffi.rs` (the two exports, the header, and a `native_staging` helper if D3
  needs one)
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
   see it). Record the three runs in the attempt record.
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

*Test value.* `render_locked_source.rs` is red if `source_submit` or `source_seek` allocates on
the render thread, or if either export loses its `render_locked` wrap while an allocation is
present; no existing test reaches either export inside a render-locked window.

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
