# Run the browser control plane in a Worker and keep the AudioWorklet render-only

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10, D15-11, D15-17).
Code anchors verified on `main` at `6fb211594`.

This spec is the first slice of the Worker control plane: boot and disposal move to a Worker, and
the worklet only renders. The rest of D15-10 is split into ordered slices: the predecessor
*Ship the browser module with one imported shared memory at every instantiation site* (#1380), and
the successors, in order: *Move browser source submission and seeks into the Worker* (#1387),
*Swap and retire browser plans through the Worker's service loop* (#1381) and *Admit browser live
edits in the Worker through the committed model* (#1382).

## Product outcome

On a cross-origin-isolated page, the browser engine parses, compiles and prepares its session in a
dedicated Worker, and disposes it there. The AudioWorklet instance shares the Worker's one
`WebAssembly.Memory`. After boot it only renders, and it never allocates or frees. A 64-track boot
no longer stalls the audio thread: today it costs 23.9 ms (V8 p50) against a 2.667 ms quantum
(round-1 C4). A page that is not isolated keeps the same API and the same artifact. It runs one
instance on a local shared memory, as #1380 ships it, and runs its control work in the worklet's
message handler, never inside `process()`. The host reports which mode it runs in and, in that
mode, how many control allocations ran in the worklet.

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
- **The process body.** `scripts/check-web-audioworklet.sh:4` bans `new `, `subarray`,
  `WebAssembly` and more inside the `PROCESS_POLICY_BEGIN`/`END` body, so `process()` cannot
  rebuild a view. The seventh growth site (`:1823`) is inside `process()` (`:1813`); the other six
  are in message handlers.
- **Growth of a shared memory.** A grown shared memory returns a new `SharedArrayBuffer` from
  `memory.buffer`. The old one stays valid, keeps its old length and aliases the same bytes, so a
  view over it still addresses every byte below the old length.
- **Isolation.** The first-party app already sends COOP `same-origin` and COEP `require-corp`
  (round-1 C4 evidence, app repository). The qualification server sends neither
  (`hosts/host-web/qualification/server.mjs`).

## Decisions frozen for this slice

- **D1. Mode.** The main realm selects the mode once at engine creation:
  - `worker` when `globalThis.crossOriginIsolated === true`;
  - `single` otherwise.

  The mode is reported in the boot result as `instanceMode`. The SDK exposes it as
  `engine.instanceMode` (`"worker" | "single"`).
  The host status gains two saturating counters. Both read 0 in `worker` mode. Nothing else in
  the public API changes.
  - `blockingRebuilds`: in `single` mode, *Replace the running browser session in the Rust host*
    (#1290) increments it once per structural edit that its single-mode apply runs in the
    worklet's message handler.
  - `singleModeControlAllocations`: in `single` mode, the allocations, reallocations and frees
    that ran in the worklet instance outside the render-locked window (boot, control messages, the
    service step). It is how decision 15 counts and reports the one place this engine allocates on
    an audio thread (D15-10, single mode; `AGENTS.md`). The render-locked count stays exactly 0 in
    both modes.

  Mechanism: #1333's `RenderLockedAllocator` gains a second counter, `CONTROL_ALLOCATIONS`, that
  each allocator call increments when the render-locked flag is clear (one `Relaxed` add). A new
  export, `miso_engine_web_v1_control_allocation_count() -> u32`, returns it. In `single` mode the
  worklet reads it in its message handler, never in `process()`, and reports it in the host
  status. In `worker` mode the counter also counts the Worker's allocations, which are not on an
  audio thread, so the status reports 0.

  Isolation moves boot and rebuild work off the audio thread. No edit's exactness depends on it:
  the Worker runs no catch-up, and a latency growth is adopted by render in both modes (D15-8
  (round-5 amendment); *Check the warm-successor deadline in the browser Worker's service loop and
  report its outcome*, #1361).
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
     nothing is dropped. Move semantics, exactly:
     - `LIVE_HOST` (`hosts/host-web/src/ffi.rs:504`, today `RefCell<Option<LiveHost>>`; #1333 D5
       makes it `RefCell<ManuallyDrop<Option<LiveHost>>>`) becomes
       `RefCell<ManuallyDrop<Option<Box<LiveHost>>>>`, const-initialised. It keeps #1333 D5's
       `const _: () = assert!(!core::mem::needs_drop::<_>())` for its new type, so the thread
       local still registers no destructor. Boot boxes the host once, in the Worker.
     - `host_release` takes the box out through the `ManuallyDrop` with `Option::take()` and
       returns `Box::into_raw`. No allocation, no free: the token is the same box.
     - `host_adopt` rebuilds the box with `Box::from_raw` and stores it only into a slot that
       holds `None`, by matching on the `borrow_mut()` slot. A slot that already holds a host
       refuses with `RESULT_WRONG_STATE` and leaves the token unconsumed. It never assigns or
       `replace`s over a `Some`, which would run the old host's drop glue.
     - The staging references are `Copy` and go into `Cell<Option<&'static _>>` slots, so
       setting them drops nothing.
     - One box travels Worker, worklet, Worker. Only the Worker's dispose frees it: it takes the
       box out with `take()` and drops it explicitly, as #1333 D5's dispose does.
     - The token also carries the three staging references that boot allocated (#1333 D5:
       `&'static` references, `Copy`), so the worklet never allocates a staging.
  5. The Worker also reserves the worklet instance's stack and TLS block through
     `miso_engine_web_v1_instance_reserve() -> u32`, which follows the recipe that #1331 recorded.
  6. The main realm passes the memory, the module, the stack/TLS address and the token to the
     worklet in `processorOptions`.
- **D3. Worker mode, worklet.**
  1. The worklet instantiates the module on the same memory and initializes its stack and TLS
     from the reserved block.
  2. It calls `miso_engine_web_v1_host_adopt(token) -> u32`, which puts the host in its own
     handle table and the three staging references in its own staging thread locals, without
     allocating.
  3. From then on it calls only the exports it calls today after boot.
  4. It never calls `boot`, `dispose`, `host_release` or `instance_reserve`.
  5. Every export the worklet calls after `instance_reserve`, including `host_adopt` and
     `host_release`, wraps its body in #1333's `render_locked`. This widens #1333 D2's locked set
     to the worklet's whole post-boot export set, as #1333 D2 anticipates. The mechanism does not
     change.
  6. Source submission and seeks stay on the worklet thread in this slice, because they are where
     they are today; each source keeps one producer thread. They move to the Worker in the next
     slice, *Move browser source submission and seeks into the Worker* (#1387), before any
     session state lives in the Worker (#1381). This is ordering, not an interim design: the C ABI
     shape puts the producers in the control half.
- **D4. Disposal.**
  1. A dispose request reaches the worklet. It stops rendering and releases the host with
     `host_release`, which moves it out and frees nothing.
  2. It posts the token to the Worker.
  3. The Worker adopts the token and calls `miso_engine_web_v1_dispose`, then terminates.

  Only the Worker ever frees. A worklet that is torn down without a dispose request leaks
  nothing: the memory is reclaimed with both realms.
- **D5. Growth.** A changed `memory.buffer` is growth, never a fault. Both modes grow: the Worker
  in `worker` mode, and the worklet's own control handler in `single` mode (decision 15's single-mode control work
  allocates there).
  1. `process()` never checks or rebuilds a view. Every view it uses (status, output planes,
     meter frame) was built over memory that existed when it was built, and a view over an older
     `SharedArrayBuffer` stays valid for the bytes it covers after growth. The check at `:1823`
     is deleted. The process body keeps every ban of `check-web-audioworklet.sh:4`.
  2. Each message handler (the six other sites) compares `memory.buffer` with its cached buffer
     at entry. On a change it rebuilds every cached view over the new buffer, then continues.
     The sticky `REPREPARE` at those sites is deleted.
  3. Rebuilt views replace the boot-time ones that `process()` reads only between two `process()`
     calls, because handlers and `process()` run on the same thread.
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
  - The frozen export set gains `host_release`, `host_adopt`, `instance_reserve` and
    `control_allocation_count`.
  - The process body may not read `memory.buffer` (D5.1).
  - `host_adopt` and `host_release` each get the full call-graph gate (no allocator, no
    deallocator, no drop glue), like `meter_poll`.
  - JS capability rules:
    - main realm: exactly one pinned `new Worker(` site, which constructs the control Worker
      (today's ban, `check-web-audioworklet.sh:512-515`, becomes this pin; the single-mode
      service tick's one pinned `setInterval(` is *Swap and retire browser plans through the
      Worker's service loop* (#1381) D6's rule change, not this slice's);
    - worklet: still no `Worker(`, `setTimeout`, `setInterval`, `WebSocket` or `memory.grow`,
      and no `Atomics.wait` anywhere;
    - control Worker: no `fetch(`, `WebSocket` or `memory.grow`.
  - The module-shape checks of #1380 (one imported shared memory, with its maximum) stay as
    they are.

## Deliverables

1. D2-D5 in `hosts/host-web/src/ffi.rs` (four exports) and the three JS files. The new
   `hosts/host-web/web/miso-engine-v1-control-worker.js`. D1's control counter in
   `hosts/host-web/src/render_lock.rs`.
2. D1 and D6 in the main-realm host and in `sdk/src/browser/engine.ts` (`instanceMode`).
3. D8 in `scripts/check-web-audioworklet.sh`, plus the one copy line in
   `scripts/build-web-audioworklet.sh`.
4. The qualification server serves COOP/COEP on an isolated leg and omits them on a non-isolated
   leg. `npm run qualify` runs both.
5. `hosts/host-web/DEPLOYMENT.md` and `BROWSER_DEPLOYMENT_MATRIX.md` state the isolation
   requirement and what `single` mode does: isolation changes where control work runs, never what
   an edit renders.

## Authorized paths

- `hosts/host-web/src/ffi.rs`, `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`,
  `hosts/host-web/src/render_lock.rs` (the control counter only)
- `hosts/host-web/tests/worker_handoff.rs` (new), `hosts/host-web/Cargo.toml` (the `bench-support`
  dev-dependency)
- `hosts/host-web/web/` (the three host JS files, `.d.ts`, the new Worker file)
- `hosts/host-web/qualification/`, `hosts/host-web/DEPLOYMENT.md`,
  `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`
- `sdk/src/browser/engine.ts`, `sdk/src/browser/shipped-host.d.ts`, `sdk/` packaging of the new
  artifact file
- `scripts/check-web-audioworklet.sh`, `scripts/check-web-audioworklet-callgraph.py`
- `scripts/test-web-audioworklet.mjs` (the fake exports and messages for the mode, the token
  hand-off and growth)
- The frozen export lists that #1293 names, for the four new exports only:
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
- `host_adopt` must not touch a `thread_local!` that registers a destructor, or allocate a
  staging. #1333 makes all five const and destructor-free first, and D2.4 hands the stagings
  over.

## Objective gates

1. **Worker mode renders the same bits.** In each of Chromium, Firefox and WebKit, on the isolated
   leg, the qualification's native-digest fixtures render a PCM digest equal to the `single`
   leg's digest and to the native digest the gate already pins.
2. **The worklet never allocates after boot.** On the isolated leg,
   `miso_engine_web_v1_render_allocation_count` (#1333 D3) reads exactly 0 for the worklet
   instance. The window runs from `host_adopt` to `host_release`,
   over a workload of 350 blocks with live commands, meter polls, an observation read, a track
   response capture, a spectrum read, source submits and seeks. On the non-isolated leg the same
   workload reads 0 too. The gate's mutation self-test still turns red.
3. **Disposal frees only in the Worker.** Integration test binary
   `hosts/host-web/tests/worker_handoff.rs`. Decision 15 rules that host-web's native
   allocation-count gates live in an integration binary, never in `src/tests.rs`, which already
   registers a `#[global_allocator]`. It links `bench_support::alloc` and calls
   `assert_installed()` first. Through the ffi exports, it boots a host on thread 1 and moves it
   through `host_release`. It adopts it with `host_adopt` on thread 2 (a fresh thread-local
   block, as a joined instance has), renders 8 blocks, reads observations, a track response and a
   spectrum, then releases. Thread 1 adopts and disposes. It asserts:
   - thread 2's thread-scoped counters read `allocations == 0 && frees == 0` from adopt to
     release;
   - thread 1's frees are greater than 0;
   - on thread 2, adopting a second booted host's token into the occupied slot returns
     `RESULT_WRONG_STATE`, frees nothing, and the first host
     still renders.
4. **Growth is not a fault.** On the isolated leg, the qualification harness grows the shared
   memory from the Worker by one page after `host_adopt` (a qualification-only message). On the
   non-isolated leg, the same message grows it from the worklet's control handler. On both legs,
   the worklet renders 64 more blocks with an unchanged digest, then answers a meter poll and an
   observation read over rebuilt views, and reports no `REPREPARE`.
5. **Mode reporting.** The isolated leg reports `instanceMode === "worker"`, the non-isolated leg
   `"single"`. Both legs pass the full existing qualification matrix. The non-isolated leg reports
   `singleModeControlAllocations > 0` after boot (boot allocates), and the isolated leg reports
   0.
6. **Policy self-tests.** Each new rule in `check-web-audioworklet.sh` has a red mutation in the
   script's self-test:
   - a second `new Worker(` in the main realm;
   - `Atomics.wait` in the worklet;
   - drop glue planted in `host_adopt`'s closure;
   - a `memory.buffer` read planted in the process body.
7. **Commands** (verified in `.github/workflows/qualification.yml`):
   - `cargo test --locked -p host-web --features host-web/test-support` (runs gate 3's binary)
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
  `RefCell<Option<_>>` replace that drops the old value, or a staging the joined instance
  allocates on first read), or if dispose runs on the render side.
- Gate 4: turns red if growth is still treated as a sticky fault, or if a handler keeps a stale
  view after growth. Either would break every later slice that allocates while audio plays.
- Gate 5: turns red if an isolated page silently runs `single` mode, a non-isolated page fails
  instead of running it, or single mode's control allocations go uncounted.
- Gate 6: each mutation proves its rule can still fire.

## Dependencies

- *Prove two Wasm instances on one shared memory in three browser engines and on iOS* (#1331).
- *Gate AudioWorklet render against allocation statically and at runtime* (#1333).
- *Build the browser artifact on a pinned nightly toolchain* (#1334).
- *Ship the browser module with one imported shared memory at every instantiation site* (#1380).
- *Design: one edit API on every host over the core's committed session model* (#1057).

#1331's verdict, including the real iOS device and the memory maximum, gates this slice. #1333
supplies the runtime counter and the const, destructor-free thread locals. The ports of D7 carry
#1057's edit API messages, so its design note fixes what they serve.

## Successor slices

- *Move browser source submission and seeks into the Worker* (#1387), next.
- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Admit browser live edits in the Worker through the committed model* (#1382).
