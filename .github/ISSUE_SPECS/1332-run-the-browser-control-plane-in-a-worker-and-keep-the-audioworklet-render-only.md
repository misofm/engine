# Run the browser control plane in a Worker and keep the AudioWorklet render-only

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10, D15-11, D15-17).
Code anchors verified on `main` at `6fb211594`.

This spec is the first slice of the Worker control plane: boot and disposal move to a Worker, and
the worklet only renders. The rest of D15-10 is split into ordered slices: the predecessor
*Ship the browser module with one imported shared memory at every instantiation site* (#1380), and
the successors *Swap and retire browser plans through the Worker's service loop* (#1381) and
*Admit browser live edits in the Worker through the committed model* (#1382).

## Product outcome

On a cross-origin-isolated page, the browser engine parses, compiles and prepares its session in a
dedicated Worker, and disposes it there. The AudioWorklet instance shares the Worker's one
`WebAssembly.Memory`. After boot it only renders, and it never allocates or frees. A 64-track boot
no longer stalls the audio thread: today it costs 23.9 ms (V8 p50) against a 2.667 ms quantum
(round-1 C4). A page that is not isolated keeps the same API and the same artifact. It runs one
instance on a local shared memory, as #1380 ships it. The host reports which
mode it runs in.

## Context

- **Boot runs on the audio thread today.** The worklet compiles the module into one instance with
  no imports (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:268` `initialize`, `:286`
  `new WebAssembly.Instance(init.module, {})`). Boot (`miso_engine_web_v1_boot`,
  `hosts/host-web/src/ffi.rs:3681`) and dispose (`:4372`) run there. Boot reaches
  `AudioWorkletEngineHost::boot_with_spectrum_config` (`hosts/host-web/src/lib.rs:2043`) and then
  `compile_ready` (`:6617`).
- **The main realm** compiles the module, then `audioWorklet.addModule` and
  `new AudioWorkletNode(..., { processorOptions })`
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:676`, `:1659`, `:1665-1669`). The
  worklet cannot read `crossOriginIsolated`, so the main realm must pass it.
- **Handles are per instance.** `LIVE_HOST` and `BOOT_STAGING` are `thread_local!`s
  (`hosts/host-web/src/ffi.rs:503-512`). With atomics, each instance has its own TLS block, so a
  host booted in the Worker is not visible in the worklet's `LIVE_HOST` without an explicit
  transfer.
- **Growth.** The worklet checks `this.exports.memory.buffer !== this.memoryBuffer` at seven sites
  (`miso-engine-v1-audio-worklet.js:1005`, `:1231`, `:1308`, `:1358`, `:1422`, `:1595`, `:1823`).
  A changed buffer is treated as a fault. With a shared memory, the Worker may grow it at any
  time. The old `SharedArrayBuffer` stays valid, but views must be rebuilt over the new one.
- **Allocator.** With atomics, std's Wasm dlmalloc takes a global spin lock. If the worklet
  allocates or frees while the Worker holds that lock, the worklet waits without bound (round-1
  C4, requirement 2). This is why the worklet must never allocate or free after boot.
- **Policy gates today** forbid imports, shared memory and atomics in the module
  (`scripts/check-web-audioworklet.sh:363-380`). In the JS they forbid `Worker(`, `Atomics` and
  `memory.grow` (`:512-522`). The artifact set is seven frozen files (`:180-193`). The build
  script copies them (`scripts/build-web-audioworklet.sh:141-145`). The call-graph gate checks
  only `render`, `meter_poll` and `command_submit` (`:476-500`).
- **Isolation.** The first-party app already sends COOP `same-origin` and COEP `require-corp`
  (round-1 C4 evidence, app repository). The qualification server sends neither
  (`hosts/host-web/qualification/server.mjs`).

## Decisions frozen for this slice

- **D1. Mode.** The main realm selects the mode once at engine creation:
  - `worker` when `globalThis.crossOriginIsolated === true`;
  - `single` otherwise.

  The mode is reported in the boot result as `instanceMode`. The SDK exposes it as
  `engine.instanceMode` (`"worker" | "single"`).
  The host status gains one saturating counter, `blockingRebuilds`. It is 0 in `worker` mode.
  In `single` mode, *Replace the running browser session in the Rust host* (#1290) increments it
  once per structural edit that its single-mode apply runs on the audio thread. Nothing else in
  the public API changes.
- **D2. Worker mode, boot.**
  1. The main realm creates one shared `WebAssembly.Memory`, using the module's declared maximum
     (#1380 pins it).
  2. It starts the control Worker from the new artifact file `miso-engine-v1-control-worker.js`,
     resolved beside the worklet module URL. It posts the compiled `WebAssembly.Module` and the
     memory to the Worker.
  3. The Worker instantiates the module first. It boots through the existing boot exports,
     unchanged.
  4. A new export, `miso_engine_web_v1_host_release(handle) -> u64`, moves the booted host out of
     the Worker's handle table into a transfer token (a raw pointer to a heap box). This is a move:
     nothing is dropped.
  5. The Worker also reserves the worklet instance's stack and TLS block through
     `miso_engine_web_v1_instance_reserve() -> u32`, which follows the recipe that #1331 recorded.
  6. The main realm passes the memory, the module, the stack/TLS address and the token to the
     worklet in `processorOptions`.
- **D3. Worker mode, worklet.**
  1. The worklet instantiates the module on the same memory and initializes its stack and TLS
     from the reserved block.
  2. It calls `miso_engine_web_v1_host_adopt(token) -> u32`, which puts the host in its own
     handle table without allocating.
  3. From then on it calls only the exports it calls today after boot.
  4. It never calls `boot`, `dispose`, `host_release` or `instance_reserve`.
  5. Every export the worklet calls after `instance_reserve`, including `host_adopt` and
     `host_release`, wraps its body in #1333's `render_locked`. This widens #1333 D2's locked set
     to the worklet's whole post-boot export set, as #1333 D2 anticipates. The mechanism does not
     change.
  6. Source submission and seeks stay on the worklet thread in this slice, because they are where
     they are today; each source keeps one producer thread. They move to the Worker in
     *Move browser source submission and seeks into the Worker* (#1387). This is ordering, not
     an interim design: the C ABI shape puts the producers in the control half.
- **D4. Disposal.**
  1. A dispose request reaches the worklet. It stops rendering and releases the host with
     `host_release`, which moves it out and frees nothing.
  2. It posts the token to the Worker.
  3. The Worker adopts the token and calls `miso_engine_web_v1_dispose`, then terminates.

  Only the Worker ever frees. A worklet that is torn down without a dispose request leaks
  nothing: the memory is reclaimed with both realms.
- **D5. Growth.** A changed `memory.buffer` is growth by the other instance. The worklet rebuilds
  its views over the new buffer at each of the seven sites and continues. It no longer turns
  this into a sticky `REPREPARE`. In `single` mode the worklet never grows memory after boot, so
  the same code is never reached there.
- **D6. Failure.** A Worker boot failure returns the same result codes and diagnostic bytes as
  today's worklet boot. The worklet is never started for a failed boot. If the Worker raises
  `error` or `messageerror` before its boot reply, boot fails with the typed reason
  `worker-failed`. The page never falls back to `single` mode silently.
- **D7. Ports and wire protocol.**
  - This slice creates two ports:
    - a page-to-control-plane port, held by the main-realm host;
    - a control-plane-to-worklet `MessageChannel` port.
  - In `worker` mode the host routes control messages to the Worker. In `single` mode it routes
    them to the worklet's own control handler.
  - Later slices add their messages on these ports (#1294, #1381, #1382).
  - The Worker and worklet messages are internal (D15-11). They are not
  documented in `shipped-host.d.ts` and do not appear in the SDK's public types.
- **D8. Policy gates are rewritten to the new invariants, never deleted** (round-1 C4,
  requirement 4).
  - The artifact set gains `miso-engine-v1-control-worker.js` (eight files).
  - The frozen export set gains `host_release`, `host_adopt` and `instance_reserve`.
  - `host_adopt` and `host_release` each get the full call-graph gate (no allocator, no
    deallocator, no drop glue), like `meter_poll`.
  - JS capability rules:
    - main realm: exactly one pinned `new Worker(` site, which constructs the control Worker;
    - worklet: still no `Worker(`, `setTimeout`, `setInterval`, `WebSocket` or `memory.grow`,
      and no `Atomics.wait` anywhere;
    - control Worker: no `fetch(`, `WebSocket` or `memory.grow`.
  - The module-shape checks of #1380 (one imported shared memory, with its maximum) stay as
    they are.

## Deliverables

1. D2-D5 in `hosts/host-web/src/ffi.rs` (three exports) and the three JS files. The new
   `hosts/host-web/web/miso-engine-v1-control-worker.js`.
2. D1 and D6 in the main-realm host and in `sdk/src/browser/engine.ts` (`instanceMode`).
3. D8 in `scripts/check-web-audioworklet.sh`, plus the one copy line in
   `scripts/build-web-audioworklet.sh`.
4. The qualification server serves COOP/COEP on an isolated leg and omits them on a non-isolated
   leg. `npm run qualify` runs both.
5. `hosts/host-web/DEPLOYMENT.md` and `BROWSER_DEPLOYMENT_MATRIX.md` state the isolation
   requirement and what `single` mode does.

## Authorized paths

- `hosts/host-web/src/ffi.rs`, `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`
- `hosts/host-web/web/` (the three host JS files, `.d.ts`, the new Worker file)
- `hosts/host-web/qualification/`, `hosts/host-web/DEPLOYMENT.md`,
  `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`
- `sdk/src/browser/engine.ts`, `sdk/src/browser/shipped-host.d.ts`, `sdk/` packaging of the new
  artifact file
- `scripts/check-web-audioworklet.sh`, `scripts/check-web-audioworklet-callgraph.py`
- The frozen export lists that #1293 names, for the three new exports only:
  `tools/parameter-metadata/src/abi_layout.rs` (`EXPORTS`), `scripts/check-abi-layout-v1.py`, and
  the regenerated `sdk/assets/miso-engine-v1-abi-layout.json` and `sdk/src/generated/abi.ts`.
  The first two are outside stream H's ownership; root sequences them.
- `scripts/build-web-audioworklet.sh`, the copy line only. This file is outside stream H's
  ownership, so root orders this edit after #1334.

## Non-goals

- No plan exchange, successor, retirement queue or service loop. These belong to
  *Swap and retire browser plans through the Worker's service loop* (#1381).
- No change to where commands, sources, meters, observations or spectrum are admitted or read
  in this slice. They stay on the worklet thread and must be allocation-free there (gate 2).
  Live edits move to the Worker in #1382.
- No toolchain change (#1334). No module-shape change (#1380). No new static
  allocation gate (#1333).

## Hazards

- A host that the Worker drops while the worklet still holds it is a use-after-free. D4's order
  (release, post, then dispose) is the only allowed path.
- `host_adopt` must not touch a `thread_local!` that registers a destructor. #1333 makes them
  const and destructor-free first.

## Objective gates

1. **Worker mode renders the same bits.** In each of Chromium, Firefox and WebKit, on the isolated
   leg, the qualification's native-digest fixtures render a PCM digest equal to the `single`
   leg's digest and to the native digest the gate already pins.
2. **The worklet never allocates after boot.** On the isolated leg,
   `miso_engine_web_v1_render_allocation_count` (#1333 D3) reads exactly 0 for the worklet
   instance. The window runs from `host_adopt` to `host_release`,
   over a workload of 350 blocks with live commands, meter polls, an observation read, a spectrum
   read, source submits and seeks. The gate's mutation self-test still turns red.
3. **Disposal frees only in the Worker.** A native test in `hosts/host-web/src/tests.rs` boots a
   host and moves it through `host_release`. It adopts it with `host_adopt` on a second thread
   and renders 8 blocks, then releases and disposes it on the first thread. It asserts:
   - `bench_support::alloc` counts `allocations == 0 && frees == 0` on the second thread from
     adopt to release;
   - the first thread's frees are greater than 0.
4. **Growth is not a fault.** On the isolated leg, the qualification harness grows the shared
   memory from the Worker by one page after `host_adopt` (a qualification-only message). The
   worklet renders 64 more blocks with an unchanged digest and reports no `REPREPARE`.
5. **Mode reporting.** The isolated leg reports `instanceMode === "worker"`, the non-isolated leg
   `"single"`. Both legs pass the full existing qualification matrix.
6. **Policy self-tests.** Each new rule in `check-web-audioworklet.sh` has a red mutation in the
   script's self-test:
   - a second `new Worker(` in the main realm;
   - `Atomics.wait` in the worklet;
   - drop glue planted in `host_adopt`'s closure.
7. **Commands** (verified in `.github/workflows/qualification.yml`):
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `bash scripts/check-web-audioworklet.sh` (self-building form), and the CI form:
     `rm -rf target/ci/qualification-artifacts target/ci/qualification-named-twin && mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts && bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`
   - `bash scripts/test-web-audioworklet.sh`
   - `cd hosts/host-web/qualification && npm ci && npm run qualify -- --artifacts "$PWD/../../../target/ci/qualification-artifacts" --sdk-root "$PWD/../../../sdk" --browser <chromium|firefox|webkit> --check-matrix --self-test-mutations`
     for all three browsers. CI runs this under a private PulseAudio null sink.
   - `bash scripts/check-sdk-types.sh`, `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`, `bash scripts/sdk-package.sh check target/ci/qualification-artifacts`
   - `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`,
     `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1: turns red if the worklet's joined instance runs on the wrong stack or TLS, or reads
  views over stale memory, and so renders different bits.
- Gate 2: turns red if any export the worklet calls after boot reaches the allocator. The static
  gate can miss this behind `call_indirect`.
- Gate 3: turns red if `host_release` or `host_adopt` drops or allocates (for example, a
  `RefCell<Option<_>>` replace that drops the old value), or if dispose runs on the render side.
- Gate 4: turns red if growth by the Worker is still treated as a sticky fault, which would break
  every later slice that allocates in the Worker while audio plays.
- Gate 5: turns red if an isolated page silently runs `single` mode, or a non-isolated page fails
  instead of running it.
- Gate 6: each mutation proves its rule can still fire.

## Dependencies

- *Prove two Wasm instances on one shared memory in three browser engines and on iOS* (#1331).
  Its verdict, including the real iOS device and the memory maximum, gates this slice.
- *Gate AudioWorklet render against allocation statically and at runtime* (#1333): the runtime
  counter, and const, destructor-free thread-locals.
- *Build the browser artifact on a pinned nightly toolchain* (#1334).
- *Ship the browser module with one imported shared memory at every instantiation site*
  (#1380).

## Successor slices

- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Admit browser live edits in the Worker through the committed model* (#1382).
- *Move browser source submission and seeks into the Worker* (#1387).
