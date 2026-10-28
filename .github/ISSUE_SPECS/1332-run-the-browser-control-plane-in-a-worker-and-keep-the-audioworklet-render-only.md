# Run the browser control plane in a Worker and keep the AudioWorklet render-only

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10, D15-11, D15-17).
Code anchors verified on `main` at `6fb211594`.

This spec is the first slice of the Worker control plane: boot and disposal move to a Worker, and
the worklet only renders. The rest of D15-10 is split into ordered slices: the predecessor
*Ship the browser module with one imported shared memory at every instantiation site* (#1380), and
the successors, in order: *Move browser source submission and seeks into the Worker* (#1387),
*Swap and retire browser plans through the Worker's service loop* (#1381) and *Admit browser live
edits in the Worker through the committed model* (#1562).

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
  - Later slices add their messages on these ports (#1294, #1381, #1562).
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
  Live edits move to the Worker in #1562.
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

Root note (2026-10-12, by named exception, from #1530 open question 1): this slice adds a gate that runs the observation wait (#1530) on a realtime `AudioContext` through the rewired worklet and proves no one-block gap between `renderedQuanta` and the observation drain; #1530's `OfflineAudioContext` renders in a burst and cannot detect it.

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

Root note (2026-10-13): *Share one in-flight status read in the browser host* (#1536) lands before this slice, and the Worker rewire keeps its sharing rule.

## Successor slices

- *Move browser source submission and seeks into the Worker* (#1387), next.
- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Admit browser live edits in the Worker through the committed model* (#1562).

## Amendment (decision 17, root 2026-10-13)

**Single instance first.** `single` mode is the browser's first deployment and is in place
before this slice: #1381a (`SessionState`, the service tick and the control-allocation counter of
D1), #1562a and #1570b (the `control` and `control-event` messages). This slice is the second
deployment of the same core entries. The `worker` mode boots in the Worker, hands the render half
to the worklet (D2-D4), routes the existing control messages to the Worker (D7) and handles growth
(D5). It adds no admission, service or event logic.

- D1's `CONTROL_ALLOCATIONS` counter and `singleModeControlAllocations` are #1381a's.
  `blockingRebuilds` is #1290's.
- Gate 2's "live commands" are `SET_CONTROL` `control` messages.
- It lands after #1565A (the old path, which writes cells on the worklet thread, is gone) and
  after #1381a.
- #1331 means #1331a (the desktop legs) and #1331b (the iOS leg, an owner-scheduled step). #1331
  D9's outcome rule stands.
- The "before H #1332" orders of #1530, #1532, #1534, #1535 and #1536 stand.

Split, all in this issue: (a) the Rust exports and `worker_handoff.rs`; (b) the JS modes, the
Worker file, the ports, `instanceMode` and the isolated leg; (c) growth and gate 4.

## S4 evidence (2026-10-27): the host split in place, one thread

Slice S4 of the #1332 build plan (#1332(a1)): a pure refactor of `hosts/host-web/src/lib.rs`, with
no ABI, export, JS or SDK change. Branch `codex/issue-1332-host-split` from main `0512957d5`.

- **Split.** `AudioWorkletEngineHost` holds `control: Option<ControlHalf>` and `render: RenderHalf`.
  `ControlHalf` is the old `ReadyOwnership` without its render fields: the control plane, the
  queries, the preparer record, `blocking_rebuilds`, the `ControlWords` it decides and its own
  `last_result`. `RenderHalf` owns `WebStatus`, the staging buffers and `RenderReady` (the plan,
  `shape`, `shape_epoch`, the meter delivery, the meter level word, `meter_applied`, the
  `live_values_superseded` handle and the retired tables). `render_next` and `poll_meters` are
  `RenderHalf` calls. A control call records its result in the control half and the status word,
  as before; a render call records it in the status only. The published status word does not
  change in single mode.
- **Tables travel with the plan.** `RenderCompanions` carries the plan's boxed `PlanShape`. The
  preparer moves the plan's meter frame producer into it before publication, through the new
  `RuntimePreparer::keep_meter_frame_producer` (the default keeps the producer in the epoch, so the
  C ABI does not change). `RenderReady::adopt_shape` takes the tables from the adopted plan's
  companions. `pending_shapes`, the room check and `name_successor` are gone. A superseded
  candidate's tables drop with its plan.
- **Control words.** The control half leaves `ControlWords`: the newest epoch, the master index in
  the newest strip order, the structure epoch, and the address of the newest replacement's meter
  header, which the preparer leaves in the record. The render half applies the master and
  structure words to the plan of that epoch only, at once between blocks (the single instance
  hands them over after each control call) and at adoption. It names the successor header while a
  newer plan waits. The retired-header rule (#1290d1) is unchanged.
- **Retired tables.** The block that adopts a plan moves the retired tables, by exchange with an
  empty slot, into one of `RETIRED_SHAPES = 4` slots. The bound is exact: 3 retirement credits
  (`RETIREMENT_CAPACITY`) plus the one in-place adoption before the first block (#1597), and a
  change to either needs a fifth slot. If all slots are full, the tables stay in the adopted plan's
  companions and drop with that plan on the control thread, which is safe. The memory timing
  changes: retired tables now live until the next `service()` tick, not the next control call that
  reclaimed them before, bounded by the four slots. The accounting rows do not change.

Gates, all green on the branch: `cargo test -p host-web -p control-plane -p capi --all-targets`
with the workspace debug step's features for them (501 passed); workspace `cargo clippy
--all-targets --all-features -D warnings`; `cargo fmt --check`; `RUSTDOCFLAGS='-D warnings' cargo
doc --workspace --no-deps`; every lint-job policy script, including `check-lane-policy.sh`;
`realtime_policy` (285 marked regions); `build-web-audioworklet.sh`, `check-web-audioworklet.sh`
(render closure 35 members, 11 pinned indirect sites unchanged, no allocation, `atomic_waits=0`),
`test-web-audioworklet.sh` and `check-browser-expected-resources.py --artifacts`; and both local
browser legs, `npm run qualify -- --check-matrix --self-test-mutations` on Chromium 151 and WebKit
26.5 with a private null PulseAudio sink.

No bit moved (PR evidence, not a committed test): `direct-oracle.mjs` in print mode over the base
module (main `0512957d5`, `df4a8800…`) and the S4 module (`87691d75…`). Every PCM digest (8),
status snapshot, result transcript and timeline is equal. Four resource rows grew within their
ceilings: `graphSessionPlusPlanBytes` and `graphIncrementalPlanBytes` by 4 B (the companions'
tables pointer), and `bridgeMetadataBytes` and `bridgeRetainedBytes` by 96 B (the two halves'
shells and the retired slots).

Notes for S5/S6:
- `ControlWords` describe one epoch. Across threads, render could adopt a candidate after a later
  commit replaced the words. That plan would then keep its preparation defaults (no master,
  structure epoch 0) until the words name it. S5 must carry the words for each epoch, or stamp
  them into the plan before publication.
- The retired-table slots are emptied by the host's service step on the same thread. Across
  threads, S6 needs a bounded hand-back from render to control. The free must not happen until
  the worklet has switched its meter views off the retired header (`postMeterFrame` runs after
  `render_next` in the same `process()`), so the hand-back should wait at least one block.
- The control exports still read the render half's `status` (state, quantum, clock) and the
  shared staging `buffers` (control request, diagnostic, source staging). In worker mode the
  Worker needs its own copies.

## S5 evidence (2026-10-27): every control-to-render signal crosses two threads

Slice S5 of the #1332 build plan (#1332(a1')), branch `codex/issue-1332-cross-thread` from S4,
with main `02630e950` merged. No export, ABI, JS or SDK change.

- **Per-plan words.** Each plan carries its own words (`PlanShape::words`, an engine seqlock
  record `[master + 1, structure epoch]`). The control plane calls the new
  `RuntimePreparer::stamp_before_publication` (default: nothing) just before
  `reservation.commit`, with the committed session, the master and the structure epoch; the
  browser preparer publishes them through the writer the plan's preparation left in the record.
  After the call the control half keeps that writer and writes the plan's words again only after
  a later call that publishes no plan (a live or monitor commit). The render half reads the
  plan's own words at adoption and before every block. The only global words are the successor
  record `[newest epoch, successor header]`, written by the control half (seqlock, `Release`) and
  read by the render half (`Acquire`), which names the waiting replacement's header (#1290d1).
  `ControlWords` and `receive_words` are gone. The single instance still applies the words at
  once after each control call, so the header names them before the next block, as before.
- **Retired tables.** The adopting block parks the retired tables in `RenderReady::parked`. The
  next block, before it renders, pushes them into a bounded SPSC hand-back ring
  (`RETIRED_SHAPES = 4`, render end in `RenderReady`, control end in `ControlHalf`). The service
  step pops the ring and drops them on the control thread. So the tables live at least until the
  adopting block's `process()` has returned. The bound is exact: three retirement credits after
  a service step, plus the one shape still parked at it (or, before the first block, the in-place
  `SPECTRUM_CONFIGURE` adoption). A full ring keeps the tables parked, and the next retired
  tables drop with their plan on the control thread. The pop is in the realtime-policy
  control-side allowlist (`ControlHalf::reclaim_shapes`).
- **No control call reads render memory.** The control half owns its staging (control request,
  diagnostic, source ID and PCM, plane references). A shared lifecycle word (`Arc<AtomicU32>`)
  carries a sticky failure both ways: a control failure sets it and the render half silences from
  its next block; a render failure sets it and every control call refuses. Control calls check
  the word, read the quantum from the boot report, and lend the render plan only while it is on
  the same thread. A render failure's fixed diagnostic stays in the render half; the single
  instance copies it into the control diagnostic after the call, so the page reads one buffer.
- **Rust API for S6.** `AudioWorkletEngineHost::release_render() -> Option<RenderHost>` and
  `adopt_render(RenderHost)` (refuses a foreign or second half). `RenderHost` has `render_next`,
  `poll_meters`, `status`, `output_pcm`, `meter_header` and `diagnostic`.
- **Accounting.** The bridge rows add the plan's words record, the successor record, the
  lifecycle word and the ring (`cross_thread_bytes`); a replacement's rows add its words record.

Test: `hosts/host-web/tests/worker_admission_realtime.rs` now holds the two-thread form. The
control thread boots, designates a master, releases the render half and then commits 13 muted-track
rebuilds (some back to back, so supersession runs) and 39 live edits, feeds the source and runs the
service step while the render thread renders 360 blocks inside the render-locked window. Asserts:
render-locked allocator calls `0`; every block's header shows the words of the plan that rendered
it (track count, master `= track count`, structure epoch of its commit); the bits equal the
single-thread run of the same script. 200 consecutive runs: 200 green. The strip-fader
single-thread test was deleted (superseded); the every-owner-kind single-thread test stays (its
session has inserts, console slots, sends and a VCA, which the swap-carrying session cannot have).
Red evidence: plan not stamped before publication: red at the first adoption (`(11, 0, 0)` for
`(11, 11, 3)`), and three unit tests red; retired tables dropped on the render thread: red (589
render-locked allocator calls); hand-back at the adopting block (no delay): the unit test
`a_strip_count_change_publishes_the_successors_frame_from_the_adoption_block` red. Not made red:
the S4 form (master and structure from a global epoch-gated word) fails only if render adopts a
plan within the very block a later commit publishes; 0 of 20 runs hit it. A `Relaxed` load where
`Acquire` is needed is not observable on x86-64.

Gates: `cargo test -p host-web -p control-plane -p capi --all-targets` with their test-support
features (500 passed); clippy `-D warnings` workspace; fmt; `cargo doc -D warnings`; every
lint-job policy script including `check-lane-policy.sh`; `realtime_policy` (286 regions) and its
tests; `build-web-audioworklet.sh`, `check-web-audioworklet.sh` (render closure 38 members, 11
indirect sites, no allocation, `atomic_waits=0`; the pinned sites moved by inlining only:
`RenderReady::adopt_shape` 2 + `RenderHalf::render_next` 2 for the host's 4, and
`RenderHalf::poll_meters` 2 for the host's 2), `test-web-audioworklet.sh`,
`check-browser-expected-resources.py --artifacts`; both local browser legs (Chromium 151, WebKit
26.5, null PulseAudio sink): all qualification gates passed.

No bit moved (PR evidence): `direct-oracle.mjs` print mode over main `02630e950` (module
`98634c73…`) and S5 (`e35bf9e0…`): every PCM digest, status snapshot and timeline equal; only
`bridgeMetadataBytes` and `bridgeRetainedBytes` grew, by 368 B, within their ceilings.

Verify folds (S5 verify PASS, folded in one commit):
- A deterministic form of the race: `a_plan_adopted_after_a_later_commit_shows_its_own_words_and_names_the_waiting_successor`.
  A test-support-only hook (`RenderHost::pause_before_adoption`, absent from the shipped module)
  stops render in B's adoption block just before it takes B's tables, with a two-step `Barrier`.
  The control thread commits C to the end of its call, then releases render. Asserts: B's
  adoption block shows B's own words, B's header names C's while C waits, and nothing waits after
  C. Red: the S4 form (master and structure words applied only while the newest epoch is the
  rendering plan's) shows `(10, 0, 0)` for `(10, 10, 3)`; a render half that never reads the
  successor record names no successor. The same test asserts that the status shell
  `release_render` leaves refuses a render (`RESULT_WRONG_STATE`) and fails nothing (red when
  the shell's render records a failure: `255` for `3`). `RenderHalf::render_next` on the shell
  now refuses, so the shell's status and the shared lifecycle word cannot disagree.
- `seqlock.rs` now says a reader may be on the render thread: bounded at 64 attempts, then
  `Busy`, and the browser render half keeps its words for one more block.

**S6/S7 requirement (verifier MAJOR).** On two threads the successor record gives no advance
notice: the control half publishes it after the commit, and the render half reads it at a
block's start, so a replacement is usually adopted in the same block in which render first names
its header. The single instance's guarantee (the page cuts its meter views in the message
handler that commits the plan, #1290d1) does not carry over. S6/S7 need a protocol so the
worklet never cuts meter views inside `process()`, for example: adoption deferred until the
worklet acknowledges the header the Worker names in a `postMessage`. Also, the successor pointer
is advisory: after a supersession it may name freed tables for up to one block, so the page must
never dereference it on two threads without that protocol. The header and `SuccessorWords` docs
in `hosts/host-web/src/lib.rs` now say so.

S6 still has to do: the exports (`host_release`, `host_adopt`, `instance_reserve`) over
`release_render`/`adopt_render`; the Worker's status export (the released host keeps a status
shell) and the worklet's diagnostic export (`RenderHost::diagnostic`); `OBSERVATION_READ` and the
in-place `SPECTRUM_CONFIGURE` adoption from the Worker (with the render half away the command runs
without a lent plan, as on the C ABI, so the plan's readers are not reached); `seek_source`
without the render plan (routing only); the lease export on the worklet side; per-instance
`RENDER_ALLOCATIONS`.

## S3 evidence (2026-10-28): a shared-memory growth is not a fault

Slice S3 of the #1332 build plan (#1332(c)), branch `codex/issue-1332-growth` from #1380
(`d0c5b81b5` merged). No Rust, export or ABI change.

- **Worklet.** The `process()` check that silenced a quantum when `memory.buffer` was a new
  object is deleted. The memory is shared (#1380), so a growth gives `memory.buffer` a new
  `SharedArrayBuffer` object while every view over the older one stays valid for its bytes;
  `process()` renders over the views it holds. The message handlers still re-cut every view
  (`rebindMemoryViews`) for their own reads of grown bytes. `staleViewQuanta` stays in the
  status and is always 0 (owner decision O5: no API break); the `.d.ts` says so.
- **Policy.** `memory\.buffer` joins `process_policy_re` (`scripts/check-web-audioworklet.sh`):
  the render-callback body may not read it. Red mutation in `scripts/test-web-audioworklet.sh`:
  the old check put back in the body fails the policy, and passes with the regex before S3.
- **Hermetic test.** The fake module's memory in `scripts/test-web-audioworklet.mjs` is now a
  real shared `WebAssembly.Memory`, grown by `memory.grow`. The root-ruling-A case is rewritten:
  after a growth with no handler since, the next quantum renders through the views over the
  older buffer, `process()` does not re-bind and the next handler does. Red on the old worklet:
  `the quantum after a growth renders` (`[]` rendered). No test asserts `staleViewQuanta === 0`:
  it is a constant now.
- **Browser row.** The 1 MiB `SESSION_REPLACE` row (single mode, the growth happens in the
  handler: about 1.9 MiB to 20.7 MiB) now renders 80 quanta (16 + 64) over a feed submitted
  after the commit, with the meter stream leased. Gate `structural-replace-grown` (run.mjs):
  precondition, the memory grew; the capture digest equals the module's render of the same
  replace and feed under Node (`structural-edit-reference.mjs`, new `sourceAfterReplace`) and
  the 64 quanta after the sixteen are not silent; at least one meter frame after the growth
  carries the 192 strips with finite peaks. Red mutation `structural-replace-grown-output`.
  Both legs green locally (Chromium 151.0.7922.34, WebKit 26.5); digest `383be91c…` on both.
  (Verify fold: the `staleViewQuanta` leg and the observation-read leg were dropped; S9 adds the
  real two-thread stale check.)
- **Limit, stated.** The extended row stays green on the old worklet in single mode (run on
  Chromium: `staleViewQuanta` 0). The handler that grows the memory re-binds before the next
  quantum, so the old check could not fire on one thread. The row discriminates once the Worker
  grows the memory with no worklet handler between (S9's isolated leg runs the same row); until
  then the red evidence is the hermetic test and the policy mutation.

## S6 evidence (2026-10-28): the render half crosses instances through the exports

Slice S6 of the #1332 build plan (#1332(a2)), branch `codex/issue-1332-handoff` from S3, with main
`9c46475ea` (#1380, S3) merged. No JS change; S7 wires the exports.

- **Seven exports** (`hosts/host-web/src/ffi.rs`). `host_release(handle) -> u64`: on the instance
  that booted the host, the render half leaves it (the host keeps its control half and the status
  shell); on a render-only instance, the half it adopted leaves it, after the bound stack canary
  is checked. `0` for a wrong handle, nothing to release, or a full registry (the half stays).
  `host_adopt(token) -> u32`: into an empty slot only: an instance with no session becomes a
  render-only session under the token's host handle (`token >> 32`), and the host that released
  the half takes it back. `RESULT_WRONG_STATE` for a taken slot, `RESULT_INVALID_ARGUMENT` for a
  token no release issued, one already taken, or another host's half; the token is unconsumed on
  every refusal. `instance_reserve(tls_size, tls_align) -> u32`: one 2 MiB stack (O6) with a
  64-byte canary below it and one TLS block, leaked, and a 16-byte record `{struct_size,
  tls_base, stack_top, stack_bottom}`. **Deviation:** the coordinator's `instance_reserve()` takes
  `__tls_size`/`__tls_align` as arguments, because a Wasm global is not addressable from Rust; the
  page reads the exported globals. `instance_bind(record) -> u32`: the second instance names its
  reservation (only one this module made), so `host_release` checks its canary; an overwritten
  canary fails the session (`web.instance.stack_canary`, sticky `RESULT_INTERNAL`) and the half
  still goes back, so the Worker sees the failure. `render_diagnostic_ptr/_bytes(handle)`: the
  render-only session's fixed failure text. `successor_header_ptr(handle)`: the Worker's meter-view
  hook (below). The module links `--export=__wasm_init_tls,__tls_size,__tls_align,__tls_base,
  __stack_pointer`.
- **Registry** (`hosts/host-web/src/handoff.rs`). Four entries of process statics, one copy on the
  shared memory: a deposit claims a free entry by a CAS of its half word, then stores the token
  (`handle << 32 | sequence`); an adoption claims by a CAS of the token to `CLAIMED`, then
  confirms or redeposits. No allocation, free, lock or wait. A concurrent adoption of the same
  token is refused while another instance decides (an adopter never waits).
- **No allocation in a release or an adoption.** `AudioWorkletEngineHost` holds its render half in
  a box and a spare status shell allocated at boot; release and adoption exchange the two boxes,
  and every refusal path forgets or redeposits what it holds. The call-graph gate now holds
  `host_adopt` and `host_release` (zero pinned indirect sites, `--render-thread`); red on a release
  that boxes a new shell (`rust_oom` site and six trap owners appear). The bridge rows grow by
  2312 B (two `RenderHalf` boxes counted in `bridgeMetadataBytes`), inside their ceilings.
- **Render exports on either instance.** `render`, `meter_lease`, `meter_poll`,
  `meter_header_ptr`, `status_ptr`, `buffer_ptr`/`buffer_capacity` (output PCM and meter frame)
  serve a render-only session. On a host whose half is away, render and lease refuse
  `RESULT_WRONG_STATE`, poll answers `0`. The lease and the output-quantum check moved into the
  render half. The Worker's status is the shell: its `state` mirrors the lifecycle word at each
  control call, `last_result` is the control half's, and a rebuild published while the half is
  away counts no blocking rebuild (`blockingRebuilds` stays 0 in worker mode). Dispose on a
  render-only instance refuses; dispose on a host whose half never came back sets the lifecycle to
  disposed (that half silences) and frees the control half.
- **Per-instance `RENDER_ALLOCATIONS`**: a const `thread_local!` `Cell<u32>` like the flag (#1331
  recipe rule 3). The control counter stays one shared copy.
- **Seek, spectrum, track response with the half away.** `seek_source` already routes through the
  control plane without the plan (S5). `SPECTRUM_CONFIGURE` after the release publishes a
  successor that the first block adopts (no in-place adoption: that needs the plan, and worker
  mode configures boot taps through the boot command, #2166); its capture and a strip
  `RESPONSE_GET` are served from the Worker while the worklet renders (test below).

**Not done: `OBSERVATION_READ` from the Worker (design choice, stopped as briefed).** The browser
preparer moves the observation readers into the plan's `RenderCompanions` (#1589 R2), because
the render half folds their gain reduction and pairs them at the swap block. With the half away,
`command_with_document` reads the epoch's own reader list, which is empty for the browser, so a
bound tap answers `observation.unbound`. The readers are `Arc` slots readable from any thread, but
the plan that holds them is owned by the render thread, so the Worker cannot borrow them. Options:
(a) the preparer gives each tap a second read-only reader endpoint kept in the epoch (control
side), built only in worker mode so single-mode rows do not move; (b) the readers go back to the
epoch (the C ABI's shape) and the render half gets its own endpoints for the fold and the carry;
(c) a page-side protocol in which the worklet answers reads. (a) or (b) touches the engine
observation slot, the control-plane epoch and the accounting rows. Root to choose.

**Meter-view hook for S7 (S5 verify MAJOR).** Deferring adoption until the worklet acknowledges
a header needs a gate inside the engine's plan mailbox (the epoch is known only after the claim),
which is beyond S6; S6 does not add it. What S6 exposes instead is the hook for a protocol that
never cuts a view in `process()`: `successor_header_ptr` names the header of the newest committed
replacement. Proposed for S7: the worklet keeps its meter views over the whole shared memory
(one `Uint32Array`/`Float32Array` pair, re-cut only in a message handler), so following a retired
header's `successor_pointer` in `process()` reads through views it already holds. After each
control call that may publish a plan, the Worker posts `{header, memoryBytes}`; the worklet's
handler re-cuts the whole-memory views when the memory has grown. `process()` posts no meter frame
for a header or frame beyond its views and counts that quantum (possible only when a growth and an
adoption fall inside one handler round trip). The pointer is advisory: it may name freed tables
after a supersession, and the worklet reads through it only after render adopted that plan. If
root wants the acknowledgement gate instead, it is an engine change (`enter_block` must not claim
an epoch above a host-set limit) for a successor issue.

Test: `hosts/host-web/tests/worker_handoff.rs` (test-support, the module's
`RenderLockedAllocator`), three tests, each thread one instance:
1. Thread 1 boots, reserves, releases (its render-locked count does not move); its render and
   lease refuse; it configures the spectrum with the half away (`blockingRebuilds` 0, a successor
   header named). Thread 2 binds, adopts, takes the lease, renders 28 blocks with the poll, then
   releases: its render-locked count does not move from before the adoption to after the release
   (every allocation and free of its exports). Meanwhile thread 1 arms and reads the spectrum
   (`Ready`) and asks a strip `RESPONSE_GET` (`RESPONSE_READY`, ready). Thread 1 adopts back
   (count unchanged), and its dispose frees. The 28 blocks equal, bit for bit, a single instance
   running the same commands (whose configure adopts in place).
2. Two hosts release; the worklet adopts A; B is refused `RESULT_WRONG_STATE`; A renders; a
   consumed token adopts nothing; host A refuses host B's token (`RESULT_INVALID_ARGUMENT`) and B
   later adopts its own (so it was unconsumed); both dispose.
3. An overwritten canary: a second bind refuses, the release fails the session, the Worker
   adopts it, its status is failed, its control calls refuse, the diagnostic names
   `web.instance.stack_canary`.
40 consecutive runs green. Red on mutants: canary check removed (test 3, state `2` for `3`); a
render-only instance that takes a second half (test 1, "the slot is taken", `1` for `3`); a
release that boxes a new shell (test 1, release count `1`); the shell's lease answering OK (test
1); a rebuild counted as blocking with the half away (test 1, `Some(1)`); a global render counter
(the `render_lock` unit test's new per-thread case, `10` for `5`).

Gates, all green: `cargo test -p host-web -p control-plane -p capi --all-targets` with their
test-support features; `parameter-metadata` and `realtime-policy` tests (the ABI layout self-test
fixture regenerated); workspace clippy `-D warnings`, fmt, `cargo doc -D warnings`; every
lint-job policy script including `check-lane-policy.sh`; `realtime_policy` (290 regions;
`handoff.rs` joins the unsafe allowlist); `build-web-audioworklet.sh`,
`check-web-audioworklet.sh` (export set 55 functions plus the five linker symbols),
`test-web-audioworklet.sh`, `check-abi-layout-v1.py --self-test`,
`check-browser-expected-resources.py --artifacts`; the SDK job's scripts (generated `abi.ts`
changed); both local browser legs, `npm run qualify -- --check-matrix --self-test-mutations` on
Chromium 151.0.7922.34 and WebKit 26.5 with a private null PulseAudio sink: all gates passed.

No bit moved in single mode (PR evidence): `direct-oracle.mjs` print mode over the base module
(`e18fb827a`, `2daf4fb3…`) and S6 (`87157e1c…`): every PCM digest, status snapshot and timeline
equal; only `bridgeMetadataBytes` and `bridgeRetainedBytes` grew, by 2312 B.

Remains for S7: the Worker file and both JS halves (`__wasm_init_tls`, `__stack_pointer`,
`instance_bind`, `host_adopt` in the worklet; `instance_reserve`, `host_release`,
`successor_header_ptr` in the Worker), the meter-view protocol above, the handler names the
render-thread table already lists (`adoptRenderHalf`, `releaseRenderHalf`,
`readRenderDiagnostic`), and reporting the shared control counter as shared in worker mode.

### S6 verify folds (S6 verify PASS, 2026-10-28)

- `instance_bind` marks a reservation bound with a compare-and-swap: a second bind of the same
  reservation refuses, from the same or another instance, and an instance binds once. Test row in
  `worker_handoff.rs` test 1 (red without the CAS: the Worker binds the worklet's stack).
- The canary is checked before every block of a bound render-only session (eight `u64` volatile
  compares) and fails the session sticky with `web.instance.stack_canary` at the first bad block;
  the release check stays. Test `an_overwritten_stack_canary_fails_the_next_block` (red without
  the per-render check: `0` for `255`).
- `CONTROL_ALLOCATIONS` is a per-instance const `thread_local!` `Cell<u64>`, like the render count.
  Test 1 asserts the Worker's control count does not move while the worklet renders, copies and
  releases (red with a shared counter).
- Test `a_release_into_a_full_registry_keeps_the_half`: four hosts fill the registry; a fifth
  release returns `0` and that host still renders (red when the half is forgotten instead of taken
  back).
- The ABI layout document publishes `structures.instanceReservation` (16 bytes: `structSize`,
  `tlsBase`, `stackTop`, `stackBottom`, `u32` each); `abi.ts` and the self-test fixture regenerated.
  The call-graph gate also holds `instance_bind`, `render_diagnostic_ptr` and
  `render_diagnostic_bytes` (no allocator, free or drop glue, no indirect site).

### Root rulings (2026-10-28)

- **R-S6b.** While the render half is away, `OBSERVATION_READ` uses option (b): the control side
  holds read-only observation endpoints (a new read-only type: an `Arc` clone with no
  `acknowledge`) in the epoch's `ObservationIndex`, as on the C ABI; carried taps pair at
  promotion from the swap record (reusing #1674's C ABI code); the render half keeps its
  acknowledging readers for the meter fold. This amends root decision #1589 R2. Class A, no SDK
  or C ABI change. Slice S6b.
- **R-S7-meters.** The two-thread meter-view protocol: (1) `process()` reads only the header render
  reports as current (`meter_header_ptr` after `render()`, wrapped, call-graph pinned), never a
  retired header or a successor pointer. (2) The Worker posts `{header, memoryBytes}` after every
  control call that may publish a plan or grow memory; the worklet's message handler re-cuts the
  whole-memory views if they are too short and cuts the set for that header, recording its frame
  pointer and track and submix counts. (3) When the pointer changes, `process()` switches to the
  pre-cut set only if the pointer, the frame pointer and the counts match (compares only);
  otherwise it bounds-checks, skips the frame and counts the skip. The S6 builder's proposal above
  (follow the retired header's successor pointer) is withdrawn: it reads freed memory.
- **S8 note.** An orphaned render half (the Worker disposed while the half was away) must be
  disposable: `instance_bind` marks the instance render-only, and dispose on an unbound instance
  may free an adopted half.

## S6b evidence (2026-10-28): `OBSERVATION_READ` from the Worker

Slice S6b, branch `codex/issue-1332-observation-endpoints` from S6 (`62e6771ba`). Root ruling
R-S6b, option (b): the control side holds read-only observation endpoints in the epoch's
`ObservationIndex`, as the C ABI does; the render half keeps its acknowledging readers. This
amends #1589 R2 (the readers moved into the plan so the swap block could pair them and lending
them to control was free on one thread). Class A: no rendered bit moved; no SDK or C ABI change
(same wire, capability bit 15).

- **The endpoint** (`crates/engine/src/realtime/observe.rs`): `ObservationView`, an `Arc` clone of
  the tap's seqlock cell with `read`/`read_state` and no `acknowledge`, made by
  `ObservationReader::view` (a reference-count increment). The cell keeps one writer of its
  `consumed` word: the acknowledging reader, which the meter fold uses.
- **The index holds the views** (`crates/host-core/src/observation.rs`): `ObservationIndex::new`
  takes the handle list its positions name and keeps one view per reader in handle order, with a
  start table per handle. `read_with`/`read_into` no longer take a handle list: the read goes
  through the index alone, so it runs on whatever thread holds the epoch. The tap's descriptor
  comes from the index entry (the producer's, the same static).
- **Pairing at promotion, from the swap record** (#1674's code): `CarriedTaps` is now made for
  every meter-delivery plan with readers, wherever they are kept. Its render-side write moved into
  `CarriedTaps::record`, called by the C ABI's attachment and, through a new preparer hook
  `RuntimePreparer::keep_carried_taps`, by the browser's `RenderCompanions::adopt_predecessor`
  (the plan's one attachment). `pair_observations` exchanges the indexes' views with
  `ObservationIndex::pair_carried` beside the C ABI's handle swap, with the same three states
  (written, pending, reclaimed).
- **One read path.** The lent-readers branch (`command_with_plan`'s `readers` argument, the
  epoch-mismatch fault) is deleted: single mode reads through the views too, paired at the
  promotion its own service step makes before the read.

Accounting. Per tap one pointer (the view; no new slot). Per plan the view table and the start
table (one `u32` per handle and one more). A browser plan with readers now also has the carried-tap
record (8 B per handle and its shared header), which the C ABI already had; `RenderCompanions`
gains one pointer. The direct oracle over S6 (`87157e1c…`) and S6b (`1cacfbbf…`): every PCM
digest, status snapshot and timeline equal; `bridgeMetadataBytes`/`bridgeRetainedBytes` +96 B
(the host shell's inline epoch structures: 16 B per `ObservationIndex` on wasm32) and
`graphIncrementalPlanBytes`/`graphSessionPlusPlanBytes` +4 B (the companions' pointer); the
identity fixture declares no tap, so no table row moves. Against main (`2daf4fb3…`) the same, plus
S6's 2312 B.

C ABI delta (no wire or API change): `ObservationIndex::retained_bytes` now charges the view and
start tables, so a C ABI plan with taps reports more retained bytes, and its admission charge rises
by 8 B per tap plus 4 B per handle and one more (64-bit). Its carried-tap record is unchanged.

Tests (one per claim):
1. `hosts/host-web/tests/worker_handoff.rs`,
   `the_worker_reads_an_observation_tap_while_the_worklet_renders`: the Worker arms `comp0`'s
   console compressor tap and releases; the worklet renders six blocks; the Worker's read equals a
   single instance's row (Ready, a nonzero reduction). The Worker then rebuilds (a spectrum
   configure), the worklet renders across the swap that carries the tap, and the Worker's read is
   Ready and equal to the single instance's. Red on S6 (`Err("observation.unbound")`), and with
   `ObservationIndex::pair_carried` removed (`Pending` for `Ready`).
The view type has no `acknowledge`, so the control side cannot write the `consumed` word; a
read moving no word stays `selected_observation_reads_keep_same_track_owner_windows_and_sequences`'s.
(A fold-acknowledges test was written and deleted at verify: the word has no production reader.)
Single mode is unchanged: every existing host-web observation test passes through the new path.

Verify fold (S6b verify PASS): the control plane never replaces a preparer's host attachment
with its own carried-tap attachment; a preparer that returns the record for a plan that already
carries an attachment is refused (`host.observation.attachment`), and the hook's documentation
says it goes with `kept_observations`. No unit test: no preparer in the tree reaches the
refusal, and building one needs a tap fixture and an attachment type for one branch.

Gates, all green: the lint job's steps (fmt, workspace clippy `-D warnings`, `cargo doc -D
warnings`, every policy script); `cargo test -p host-core -p control-plane -p capi -p host-web -p
engine --all-targets` with the CI feature set; `realtime_policy` (291 regions: `CarriedTaps::record`
is the new one, its body moved from the C ABI's attachment hook); `build-web-audioworklet.sh`,
`check-web-audioworklet.sh` (render closure allocation-free and trap-free as before),
`check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py`,
`test-web-audioworklet.sh`, the render-lock probe; both local browser legs, `npm run qualify --
--check-matrix --self-test-mutations` on Chromium 151.0.7922.34 and WebKit 26.5 with a private null
PulseAudio sink: all qualification gates passed.

## S7 evidence (2026-10-28): the control Worker, the boot handoff and `instanceMode`

Slice S7 of the #1332 build plan (#1332(b1)), branch `codex/issue-1332-worker-js` from S6
(`c31ea5ae5`, the tree main `b0bec8ef9` merged).

- **Control Worker** (new artifact `hosts/host-web/web/miso-engine-v1-control-worker.js`; the
  set is eight files: `check-web-audioworklet.sh`, `build-web-audioworklet.sh`,
  `sdk/codegen/generate.mjs`, `sdk/src/assets.ts` (`controlWorkerModule`), the package smoke, the
  qualification server). It instantiates on the page's memory first, boots through the existing
  boot exports (the #2166 boot command included), calls `instance_reserve(__tls_size,
  __tls_align)` and `host_release`, and replies `miso.worker-ready.v1` `{handle, token,
  reservation, metersAttached}` plus the ready fields. It then serves `miso.control.v1`, sources,
  seeks, the session seek, seek-at, seek report, plan latency, the session map and the status's
  control fields, and runs the service step, the watermark and the protocol events after every
  request and on its own pinned service tick. A refused boot answers the worklet's result code.
- **Main realm.** `mode = crossOriginIsolated ? "worker" : "single"`. Worker mode creates the
  shared memory at the layout pages, starts the Worker beside the worklet module URL (one pinned
  `new Worker(workerUrl)` site), and constructs the node only after the boot reply, with
  `processorOptions {instanceMode, module, memory, token, reservation, metersAttached}`; the
  Worker's `MessageChannel` port reaches the worklet as `miso.control-port.v1` on the node's port
  (processor options cannot transfer a port). Requests route to the Worker except the meter and
  telemetry leases and the render allocation count, which go to the worklet. No main-realm tick in
  worker mode. A Worker `error`/`messageerror` before the reply rejects with
  `{tag: "miso.worker-failed.v1", requestId: 0, result: 255, reason: "worker-failed"}`; no node is
  built and nothing falls back. `host.instanceMode`, `engine.instanceMode`; `BrowserBootError`
  gains `worker-failed`.
- **Decision for root (not in the brief): an explicit `instanceMode: "single"` factory option.**
  The SDK's PCM feed drains in the worklet's `process()` through `source_submit`, which a
  render-only instance refuses, and the feed needs isolation, so on every page that loads it the
  default would pick worker mode and break the feed (the isolated `pcm-feed-suspended` row and the
  console). The SDK passes `instanceMode: "single"` whenever `pcmFeed` is set, until S11 drains the
  feed in the Worker; the host refuses `"worker"` on a page without isolation. This is explicit, so
  it is not a silent fallback; remove it with S11.
- **Worklet worker branch** (`initializeWorker`): instantiates on `{env: {memory}}`, sets
  `__stack_pointer` to the record's `stackTop`, then `__wasm_init_tls(tlsBase)`, checks the ABI,
  `instance_bind(record)`, `host_adopt(token)`; never boots, disposes or reserves. Ready is
  `{tag, requestId: 0, result, backend, instanceMode: "worker"}`; single mode's ready gains
  `instanceMode: "single"`. Dispose (D4, pulled forward because it is three messages): page ->
  Worker; Worker -> `miso.release.v1`; worklet `host_release`, posts `miso.released.v1 {token}`
  and stops; Worker adopts back, disposes, answers; the page terminates the Worker. S8's failure
  paths (an adoption refused after a later check, an orphaned half) are not done.
- **Status in worker mode**: the Worker asks the worklet (`miso.render-status.v1`) for its render
  half's status and counters and merges its own control fields (state when not ready, last result
  when render reports none, `blockingRebuilds` 0, `memoryBytes`). `singleModeControlAllocations`
  is the worklet instance's own counter (0); the Worker's control allocations are not reported.
  New counter `meterFramesSkipped` in both modes (0 in single mode).
- **R-S7-meters.** `meter_header_ptr` is wrapped in `render_locked`, held by the call-graph gate
  (closure 1, no indirect site) and listed wrapped in `render-thread-exports.json` (callers
  `syncMeterViews`, `postMeterFrame`). The Worker posts `miso.meter-view.v1 {header,
  memoryBytes}` (`successor_header_ptr`) after each control call or tick when either changed; the
  worklet's handler re-cuts its views, including one whole-memory `Uint32Array`, and cuts the set
  for that header as pending. In worker mode `postMeterFrame` reads `meter_header_ptr` after
  render; on a change it switches only when the pending set's header, frame pointer and track and
  submix counts equal the live header's words (bounds-checked through the whole-memory view),
  else it skips and counts. It never reads a retired header's successor. Single mode unchanged.
- **Policy** (`check_capability_policy`, `--self-test-capabilities`, 10 red mutations): main realm
  one `\bWorker\(` site, spelled `new Worker(workerUrl);`; worklet no `Worker(`/`Atomics`; Worker
  file no `fetch(`, `WebSocket`, `memory.grow`, `Atomics.wait`, `Worker(`, `setTimeout`,
  `importScripts`, exactly one pinned `setInterval(`.
- **Hermetic tests** (`scripts/test-web-audioworklet.mjs`): main realm worker mode (memory pages,
  Worker URL, processor options, routing, no main tick, dispose terminates, `worker-failed` for
  `error` and `messageerror` with no node, a refused boot's code, explicit single, `worker`
  refused unisolated); worklet worker mode (no boot, TLS after stack, bind, adopt, render, a
  refused adoption fails without dispose, Worker-owned tags refused, meter switch on a full match,
  skip and count when the counts differ or no set was cut, release posts the token); the control
  Worker file (boot, reserve, release, control, service, meter view on change and on growth,
  merged status, dispose adopts before freeing, refused boot). Red on mutants: worklet boots in
  worker mode; no `instance_bind`; no `host_adopt`; meter switch without the compares (`counts
  differ: the frame is skipped`); host mode always single; Worker error not typed; Worker error
  resolves; Worker skips `host_release`; Worker posts no meter view; Worker disposes without
  adopting back; Worker skips `instance_reserve`.
- **Worker-mode smoke** (one run each, not a gate; S9 makes it a leg): the isolated qualification
  server, an `OfflineAudioContext`, the empty session plus the `empty-boot-frame.json` structural
  transaction, 64 fed blocks. Chromium 151.0.7922.34 and WebKit 26.5: `instanceMode` worker,
  revision 2, peak 0.25, 64 quanta, `blockingRebuilds` 0, `singleModeControlAllocations` 0,
  `renderAllocationCount` 0, `meterFramesSkipped` 0, meter frames carry the new plan's one track,
  dispose clean. The same script unisolated: single, `blockingRebuilds` 1.

### S7 attempt 2 (verify FAIL folds, 2026-10-28)

- **R-S7-meters amended (root).** A meter set's identity is the four-part key: header pointer,
  frame pointer, track count, submix count. `process()` compares the active set's key with the
  live header's words on every block, even when the pointer is unchanged (a freed header address
  may be reused by a later plan); on a mismatch it switches to the pending set only if its key
  matches, else skips and counts. The meter-view handler and `syncWorkerMeterViews` decide whether
  to cut by the same key (`meterSetMatches`, compares only). Test: a header reused at the active
  address with another frame and counts is skipped and counted, then converges on the next view
  (red on attempt 1: "a reused header address is not the active set").
- **Pending set kept.** `syncWorkerMeterViews` makes the current header's set active and keeps a
  newer pending set unless it is the one made active. Test: B and C committed back to back,
  render adopts B uncut, then C: one skip, then C's frame (red on attempt 1's rule: "the pending C
  survives B's adoption").
- **Folds.** The host keeps the Worker's handlers attached between its boot reply and the
  worklet's ready (messages kept in order and replayed, an error fails the host; red: "expected
  rejection"). `releaseRenderHalf` posts `miso.released.v1 {token: 0n}` when the release throws.
  The Worker refuses control calls after a dispose began with `WRONG_STATE` in each reply's own
  shape, answers status reads from its own record then, and answers every in-flight status read
  before it frees the host (red: "a control call after the dispose began is refused", "the status
  read in flight across the dispose is answered"). Policy: an `importScripts` mutation.

### S7 attempt 3 (verify FAIL folds, 2026-10-28)

- **No request can fail a dispose.** Page side, once `dispose()` was sent every other request is
  refused locally with result 3 (as `status()` was), and a second `dispose()` returns the first
  one's promise (a failed dispose may be retried). When the dispose completes, every request it
  overtook rejects with result 3. Worker side, a session map is answered normally until the host
  is freed, any other request after the dispose began is refused `WRONG_STATE` in its own reply
  shape and never sticky, and a second dispose joins the first: one release, both answered.
  Tests red on attempt 2: "a second dispose is the first one's promise", "nothing is posted
  behind the dispose, once", "a second dispose posts no second release"; without the settle, the
  overtaken request hangs.
- **Meter view cut rule.** The handler skips the cut only when the posted header is the pending
  set's, or is the header render reports now with the active set's key. A later plan that reuses
  the active set's address and words while render reports another header is cut as pending (red
  on attempt 2: "C at A's address with A's words is cut and adopted").
- Header addresses from the `i32` exports are normalised with `>>> 0`. The memory is capped at
  512 MiB (8192 pages), so no address reaches 2 GiB.

### S7 attempt 4: the worker-mode dispose contract (root decision, 2026-10-28)

- **Root decision.** In `worker` mode a dispose always ends in teardown. (1) A clean dispose
  resolves and a failed one rejects with the typed error; either way the page terminates the
  Worker and disconnects the node exactly once (`#disposeWorkerMode`: `try`/`finally` around one
  `#teardown`). (2) The dispose promise is created once and every later `dispose()` returns it,
  settled or not; there is no retry in worker mode (a failed host is unusable). (3) The Worker
  answers a dispose request that arrives after its dispose finished (freed, or failed by a trap
  in the adoption or the dispose, which it catches and answers 255) at once with that outcome,
  never by joining a dispose that will not complete. `single` mode keeps its retry: a failed
  single-mode dispose drops its promise and restarts the service tick it stopped.
- Tests red on attempt 3: a Worker dispose that traps rejects the page's dispose with 255, still
  terminates the Worker and disconnects the node once, and a second `dispose()` returns the same
  rejected promise ("a failed dispose still terminates the Worker"); the clean dispose test checks
  the node is disconnected; a dispose after the host is freed is answered at once ("a dispose
  after the host is freed is answered at once with the outcome"); a trap in the Worker's dispose
  is answered 255 (red without the catch).
- Attempt 4 verify PASS; folds: a single-mode test that a failed dispose leaves exactly one live
  service tick (red without the restart), and a worker-mode test that a dispose the Worker
  answers 255 rejects with 255 and still terminates the Worker and disconnects the node once.

### S8 requirement (root, 2026-10-28)

- A dispose must end in teardown even when nobody answers: when `context.state === "closed"`, or
  the release or the dispose reply does not arrive within a bounded time, the page tears down
  directly (terminates the Worker, disconnects the node) and rejects with a typed reason; the
  Worker then disposes with no token.

## S8 evidence (2026-10-28): the failure paths of `worker` mode

Slice S8 of the #1332 build plan, branch `codex/issue-1332-failure` from S7 (`28bb9f849`).

- **Teardown when nobody answers (root S8 requirement).** Two bounds. The page waits at most
  `DISPOSE_TIMEOUT_MS` = 5 s for a dispose reply; past it, it tears down (terminates the Worker,
  disconnects the node) and rejects with `{tag: "miso.teardown.v1", requestId: 0, result: 255,
  reason: "dispose-timeout"}`. The control Worker waits at most `RELEASE_TIMEOUT_MS` = 2 s for
  the worklet's `miso.released.v1`; past it, it disposes the host with no token and answers the
  dispose `{tag: "miso.teardown.v1", requestId, result: 255, reason: "release-timeout"}`, which the
  page rejects with (and tears down). 2 s is about 750 quanta at 48 kHz / 128 and leaves the Worker
  3 s to dispose and answer inside the page's bound. A closed context (`node.context.state ===
  "closed"`) rejects `context-closed`: in `single` mode at once, posting nothing; in `worker` mode
  after the Worker disposed its control half with no release (`miso.dispose.v1 {renderGone:
  true}`, bounded by the same 5 s). In `single` mode a teardown is terminal: no retry and no tick
  restart. New public type `MisoTeardown` (`.d.ts` and `sdk/src/browser/shipped-host.d.ts`).
- **Timers.** The page has no clock and one timer site, so the deadline uses it: the site is now
  `startTimer(periodMs, run)` between `SERVICE_TICK_BEGIN`/`SERVICE_TICK_END`, called by the
  single-mode service tick and by the dispose deadline (fires once, cleared); the timer policy
  is unchanged (one site, in the markers), its comment updated. The Worker keeps its service
  tick running during a dispose for the release deadline only, and reads `performance.now()`.
- **Orphaned render half (S8 note).** `miso_engine_web_v1_dispose` on an instance that bound no
  reservation frees a render-only session (a half adopted after its host was disposed); a bound
  instance (`instance_bind`, the worklet) still refuses `WRONG_STATE`. The Worker reclaims a
  `miso.released.v1` that arrives after its dispose finished: adopt (into the empty slot), then
  dispose. Native test `an_orphaned_render_half_is_freed_by_the_worker_that_adopts_it_back`
  (`hosts/host-web/tests/worker_handoff.rs`).
- **Render failure reaches the page.** `process()` posts no fatal, so the Worker's tick sends
  `miso.render-check.v1` to the worklet at most every 50 ms; a worklet whose render failed sticky
  posts `miso.error.v1 {requestId: 0, result}` to the page once, and the worker-mode host fails
  sticky with that typed record (single mode unchanged).
- **Construction failure after adoption.** The worklet sets `workerMode` before any check (so it
  takes the control port even when its options are refused); a construction that fails after the
  adoption releases the half at once (`host_release`, render-locked) and answers the Worker's
  release with that token, once; before the adoption it answers `0n`. The page, when the factory
  fails after the node was built, sends the Worker `miso.dispose.v1 {requestId: 1}` and waits for
  its answer (bounded, 5 s) before terminating it. With a zero token the Worker also adopts the
  boot's token, so a half the worklet never adopted comes back from the registry.
- **Worker `error` after boot** fails the host and terminates the Worker; a later dispose rejects
  at once with the host's error and posts nothing.
- **Tests, red on mutants** (`scripts/test-web-audioworklet.mjs`): page deadline removed ("no
  reply within the bound tears down"); worker-mode closed context not told ("a closed context
  tears down"); single-mode closed check removed (same); Worker error not terminating; dispose
  after a Worker error posting ("the dispose tears down at once"); requestId-0 fatal untyped (255
  !== 9); factory skips the Worker dispose; `release-timeout` reply refused; worklet ignores the
  render check; worklet keeps the half after a failed construction; worklet answers a release
  twice; Worker release without deadline, without the boot token, dropping the orphan, asking a
  release of a closed context's worklet, posting no render check. Native: dispose always refusing
  a render-only session, and dispose freeing on a bound instance, both abort the test.
