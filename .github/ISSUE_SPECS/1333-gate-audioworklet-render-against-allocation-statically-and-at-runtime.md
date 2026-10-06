# Gate AudioWorklet render against allocation statically and at runtime

Stream H(a) of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Browser qualification proves, in Chromium, Firefox and WebKit, that the exports the AudioWorklet
calls on its render thread made exactly zero allocator calls during a real workload, including
everything the plan executor does behind dynamic dispatch. Today nothing can see that code. The
static gate also refuses new dynamic dispatch on the render path without review, and refuses
thread-local destructor registration and atomic waits there, so the worklet keeps decision 15's
rule ("never allocates or frees after boot") when it moves onto shared memory.

## Context

- The static gate checks three exports by their direct `call` edges only:
  `miso_engine_web_v1_render`, `_meter_poll` (with a trap owner) and `_command_submit`
  (`--allocation-only`) in `scripts/check-web-audioworklet.sh:474-502`.
  `scripts/check-web-audioworklet-callgraph.py` parses only `call N` (`CALL`, `:96`; `HEADER` is `:95`) and its
  `closure()` follows only those edges (`:294-313`). Forbidden names: `FORBIDDEN` (`:103-106`).
- Blind spot: the whole executor sits behind `executor: Option<Box<dyn PreparedPlanExecutor>>`
  (`crates/engine/src/realtime/plan.rs:550`) and is reached by `executor.render(...)` (`:943`), a
  `call_indirect`. Round 2 measured render's direct closure at 8 functions. Resolving indirect
  calls by signature flags 59 forbidden names, mostly drop glue, so a static proof through the
  executor is not sound; a runtime proof is needed.
- The three render-thread exports: `miso_engine_web_v1_render` (`hosts/host-web/src/ffi.rs:3816`),
  `miso_engine_web_v1_command_submit` (`:3839`), `miso_engine_web_v1_meter_poll` (`:3936`). All go
  through `with_host_mut` (`:549-565`), which reads `LIVE_HOST`.
- Thread locals in `hosts/host-web/src/ffi.rs:503-513`. `LIVE_HOST: RefCell<Option<LiveHost>>` is
  `const`-initialised but needs drop. `BOOT_STAGING: RefCell<BootStaging>` is lazily initialised by
  `BootStaging::new()`, which allocates (`options: Box<WebBootOptions>`, `document: Vec<u8>`,
  `:70-76`). `WebBootOptions` is `Copy + Default` (`hosts/host-web/src/lib.rs:1204-1206`). With
  `+atomics`, std registers a destructor for a thread local that needs drop, lazily, which
  allocates. Without atomics (today) thread locals are plain statics and register nothing.
- The three staging thread locals `RESPONSE_STAGING`, `SPECTRUM_STAGING` and
  `OBSERVATION_STAGING` (`ffi.rs:510-512`) are lazily initialised by `ResponseStaging::new()` (`:430`),
  `SpectrumStaging::new()` (`:124`) and `ObservationStaging::new()` (`:257`), which allocate
  (`Box`, `vec!`, `Vec::with_capacity`). The first export that touches one allocates it. The
  worklet touches them after boot: `miso_engine_web_v1_spectrum_read` (`:2873`),
  `_spectrum_stream_read` (`:2995`), `_track_response_capture` (`:3354`), `_observation_read`
  (`:4175`) and their pointer and capacity accessors. Instances that never boot touch them too:
  the SDK's headless response and spectrum modules (`sdk/src/core/response.ts`,
  `sdk/src/core/spectrum.ts`). Nobody else owns these thread locals: this slice does.
- Exports are a frozen list: `expected_exports` in `scripts/check-web-audioworklet.sh:203`, the
  ABI layout generator's list (`tools/parameter-metadata/src/abi_layout.rs:172` area), and
  `scripts/check-abi-layout-v1.py:154` area; generated SDK copies `sdk/assets/miso-engine-v1-abi-layout.json`
  and `sdk/src/generated/abi.ts`.
- host-web's own unit tests register a counting `#[global_allocator]` under `#[cfg(test)]`
  (`hosts/host-web/src/ffi.rs:4524-4545`), and `tools/parameter-metadata` and
  `tools/session-validator` link the `host-web` rlib natively.
- The browser legs run `npm run qualify -- ... --check-matrix --self-test-mutations`
  (`.github/workflows/qualification.yml:362-431`); result mutations live in `mutate()`
  (`hosts/host-web/qualification/run.mjs:616`). The worklet answers port messages in `receive`
  (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:266`).

## Decisions frozen for this slice

- D1. **Runtime counter.** New module `hosts/host-web/src/render_lock.rs`:
  - `struct RenderLockedAllocator<A: GlobalAlloc>(A)`. Each of `alloc`, `alloc_zeroed`, `realloc`,
    `dealloc` first reads `RENDER_LOCKED` and, if set, does one `Relaxed` `fetch_add(1)` on
    `static RENDER_ALLOCATIONS: AtomicU32` (saturating is not needed: qualification asserts 0),
    then forwards to `A`. One load and one branch when unlocked.
  - `thread_local! { static RENDER_LOCKED: Cell<bool> = const { Cell::new(false) }; }`: const,
    no destructor, so it is instance-local under atomics and a plain static without them.
  - `#[cfg(target_family = "wasm")] #[global_allocator] static ALLOCATOR:
    RenderLockedAllocator<std::alloc::System>`. The wasm cfg is required, not a target fork: the
    native rlib links into test binaries and tools that register their own allocator.
  - `fn render_locked<R>(f: impl FnOnce() -> R) -> R` sets the flag, runs `f`, clears it. No
    nesting, no RAII guard (panic is abort).
- D2. **Lock scope.** The lock covers the dynamic extent of every export in the render-locked
  set. Each wraps its body in `render_locked`. The set is:
  - `miso_engine_web_v1_render`, `miso_engine_web_v1_command_submit` and
    `miso_engine_web_v1_meter_poll`, the set the static gate checks (D4);
  - the worklet's staging reads: `miso_engine_web_v1_observation_read`, `_track_response_capture`,
    `_spectrum_read`, `_spectrum_stream_read`, and every pointer, capacity and byte-count accessor
    of the three stagings that the worklet calls (`_observation_*_ptr`/`_capacity`/`_bytes`,
    `_track_response_*_ptr`/`_capacity`/`_bytes`, `_spectrum_*_ptr`/`_capacity`/`_bytes`).

  #1332 makes the set the worklet's whole post-boot export set; the mechanism does not change.
  The window spans everything `miso_engine_web_v1_render` does in a block, so later render work is
  counted with no new wiring: a plan swap (#1381), and a warm successor's readiness check and
  raw-frame prime (*Adopt a warm successor with a raw-frame prime at the first ready block*,
  #1355). The worklet runs no catch-up, copy or pre-roll (D15-8 (round-5 amendment)). #1361's
  browser gate reads this counter across a prime adoption.
- D3. **Reading it.** New export `miso_engine_web_v1_render_allocation_count() -> u32` returns
  `RENDER_ALLOCATIONS`. It is added to every frozen export list above and the generated SDK copies
  are regenerated. The worklet answers a new port message (in `receive`, never in `process()`)
  with the count. Qualification requests it at the end of the run, after the live-control,
  meter, observation and stall workloads, and adds gate `render-allocations`: count `=== 0`.
  `--self-test-mutations` gains mutation `render-allocations` (count set to 1), which must fail.
- D4. **Static checks** in `check-web-audioworklet-callgraph.py`, applied to each export of the
  render-thread set:
  1. Thread-local destructor registration: any closure member whose name matches std's
     registration path (`thread_local` + `destructors` + `register`, or `thread_local` + `guard` +
     `enable`) fails.
  2. Atomic wait: any closure member containing `memory.atomic.wait32` or `memory.atomic.wait64`
     fails. Parse the instruction stream with the existing `INSN` pattern.
  3. Pinned `call_indirect` sites: collect each closure member that contains `call_indirect`,
     with its name stripped of the mangling hash, and its count. Compare with a pinned table
     `INDIRECT_SITES[export]` in the script. Any added, removed or changed entry fails and prints
     the diff. The table is set from today's named twin in this PR, with a one-line reason per
     entry (for example the executor dispatch in `PreparedRenderPlan::render`).
  The direct-call allocation and trap rules stay unchanged.
- D5. **Const, destructor-free thread locals.** `LIVE_HOST` becomes
  `RefCell<ManuallyDrop<Option<LiveHost>>>`, const-initialised; dispose keeps taking the host out
  with `take()`, which drops it explicitly. `BOOT_STAGING` becomes const-initialised and
  destructor-free: `options` is stored inline (a `const` zero/default constructor replaces
  `Box::new`), `document` becomes `ManuallyDrop<Vec<u8>>` starting at `Vec::new()`, and
  `reset_after_dispose` keeps freeing it by assignment.
  - The three stagings become const-initialised and destructor-free:
    `static RESPONSE_STAGING: Cell<Option<&'static RefCell<ResponseStaging>>> = const { Cell::new(None) }`,
    and the same shape for `SPECTRUM_STAGING` and `OBSERVATION_STAGING`.
  - One accessor per staging, `response_staging()` and so on, returns the reference. If the slot
    is empty it allocates once, `Box::leak(Box::new(RefCell::new(T::new())))`, and stores it. Every
    existing `X_STAGING.with(|slot| ...)` call goes through the accessor; the `RefCell` borrow rules
    are unchanged.
  - Both boot exports (`ffi.rs:3681`, `:3687`) call the three accessors before they return. A
    booted instance therefore never allocates a staging later, inside or outside the locked window.
    An instance that never boots (the SDK's headless modules) allocates on first touch, outside any
    locked window, as today.
  - A staging lives as long as its instance, as today's lazily initialised statics do. It is never
    freed on the worklet. A `&'static` reference is `Copy`, so #1332 can hand the three references
    to the worklet's instance inside its transfer token without allocating.
  - Add `const _: () = assert!(!core::mem::needs_drop::<T>())` for all five static types.
  - `NEXT_HANDLE` is already a const `Cell<u32>` and is not touched.
- D6. **Native counters unchanged**: `bench_support::alloc`, `audit capi` and the effect allocation
  audits are not touched.
- D7. Trap-on-allocate mode is not built: the counter is the gate, and a trap would end audio in
  production.

## Amendment 1 (root, 2026-10-06)

Attempt 1 stopped on a spec problem: the runtime counter found a real render-thread allocation, and
D3 and the existing gates need paths outside the list below. Root ruled all four items in scope.
This slice lands after *Bound every drain the AudioWorklet runs on its audio thread* (#1449), which
owns the `spectrum.rs` drains; the fix in A1 is made on #1449's `spectrum.rs`.

- **A1. Render-thread allocation fix.** `PreparedSpectrumCapture::channels()`
  (`hosts/host-web/src/lib.rs`) calls `SpectrumCaptureCollection::selected_entry()`
  (`crates/host-core/src/spectrum.rs`), which clones the selected target's `String` and drops it,
  so `miso_engine_web_v1_spectrum_read` and `_spectrum_stream_read` allocate and free on every read
  of a collection capture (the counter read 312-354 in the SDK's spectrum-collection instance in
  Chromium). Add a non-cloning `selected_channels()` accessor in host-core and use it in
  `channels()`. This is a real defect; fix it here, not in #1449. A test reads a collection
  capture's spectrum and spectrum stream inside the locked window and asserts the counter reads 0;
  restoring the clone makes the counter read more than 0 (mutation run in the attempt record). The
  test lives in the gate-8 binary's single `#[test]` or in a second integration binary that also
  registers `RenderLockedAllocator<System>` and holds one `#[test]`.
- **A2. Reading the count through the host.** The worklet host refuses reply tags it does not
  expect, the worklet requires increasing request IDs, and each workload runs in its own instance
  with its own counter. The host therefore gains `renderAllocationCount()`, which sends D3's port
  message in its request sequence; `qualification.js` calls it on every instance before that
  instance's dispose, and gate `render-allocations` requires every read to be 0.
- **A3. Re-pins.** D5 makes boot reserve the three stagings, so three exact `memoryBytes` rows in
  `hosts/host-web/tests/browser-v1/expected.json` grow by 17 pages. Re-pin each row individually
  with that reason: `simd128.initialStatus` and `simd128.beforeDisposeStatus` 1310720 -> 2424832,
  `commandTimeline.beforeDisposeStatus` 1376256 -> 2490368. No PCM digest may move.
- **A4. Realtime policy.** `hosts/host-web/src/render_lock.rs` joins the approved `unsafe` list in
  `scripts/check-realtime-policy.sh` (a `GlobalAlloc` impl needs `unsafe`). Hot file: stream J's
  realtime-policy tool later reproduces this listing.

Authorized paths added by this amendment: `crates/host-core/src/spectrum.rs` (A1's accessor and its
unit test only), `hosts/host-web/src/lib.rs` (A1's `channels()` change), a second
`hosts/host-web/tests/*.rs` integration binary if A1's test uses one,
`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`,
`hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`, `sdk/src/browser/shipped-host.d.ts`
(A2), `hosts/host-web/tests/browser-v1/expected.json` (A3's three rows only) and
`scripts/check-realtime-policy.sh` (A4's listing only).

## Deliverables

1. `hosts/host-web/src/render_lock.rs` (D1) and its unit tests; D2 and D3 wiring in
   `hosts/host-web/src/ffi.rs`; D5 conversions. `RenderLockedAllocator` is `pub` (re-exported from
   the crate root) so an integration test binary can register it.
2. The integration test binary `hosts/host-web/tests/render_locked_staging.rs` (gate 8).
3. Worklet message and qualification gate and mutation (D3). The qualification workload that
   precedes the count read includes at least one observation read, one track response capture,
   one spectrum read and one spectrum stream read.
4. The three static rules with self-test cases (D4) and their invocation in
   `scripts/check-web-audioworklet.sh`.
5. Frozen export lists and regenerated SDK ABI copies.
6. `hosts/host-web/MUTATIONS.md` rows for the new tests.

## Authorized paths

- `hosts/host-web/src/render_lock.rs` (new), `hosts/host-web/src/ffi.rs`, `hosts/host-web/src/lib.rs`
  (module line and the re-export only), `hosts/host-web/src/tests.rs`
- `hosts/host-web/tests/render_locked_staging.rs` (new)
- `hosts/host-web/web/miso-engine-v1-audio-worklet.js`
- `hosts/host-web/qualification/qualification.js`, `hosts/host-web/qualification/run.mjs`
- `hosts/host-web/MUTATIONS.md`
- `scripts/check-web-audioworklet.sh`, `scripts/check-web-audioworklet-callgraph.py`
- `tools/parameter-metadata/src/abi_layout.rs`, `scripts/check-abi-layout-v1.py`,
  `scripts/fixtures/abi-layout-v1-self-test.json` (export list only)
- `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts` (regenerated only)

## Non-goals

- No change to the toolchain, the module's memory shape or the worklet's capability bans (#1334,
  #1332).
- No move of any export to a Worker, and no change to which exports the worklet calls.
- No static resolution of indirect call targets.

## Hazards

- If the new runtime gate reads non-zero on today's artifact, that is a real allocation on the
  render thread. Find it with a scratch build that records the caller, fix it in this slice if it
  is bounded, and otherwise stop and report it with the evidence. Never relax the gate.
- `scripts/check-web-audioworklet-callgraph.py` is also edited by *Anchor the worklet callgraph
  checker's C allocator names* (#1234, stream J). No dependency; rebase whichever lands second.
- On today's non-atomic module, D4.1 and D4.2 find nothing by construction (no destructor
  registration exists, and atomics are banned outright at `check-web-audioworklet.sh:377-380`).
  They are in place, self-tested, before #1332 enables atomics.
- The shipped module's bytes change (new export, wrapper). `artifact-identity` reports `ARTIFACT
  CHANGED`; no render digest may move.

## Objective gates

1. `cargo test --locked -p host-web --lib render_lock` passes.
2. `bash scripts/test-web-audioworklet.sh` passes (runs `check-web-audioworklet-callgraph.py
   --self-test` with the new cases).
3. `rm -rf <out> <twin> && mkdir -p <out> <twin> && bash scripts/build-web-audioworklet.sh
   --named-twin <twin> <out>`, then `bash scripts/check-web-audioworklet.sh <out>
   <twin>/miso-engine-v1-audio-worklet.simd128.named.wasm` and `python3 -B
   scripts/check-browser-expected-resources.py --artifacts <out>` pass.
4. Browser legs: in `hosts/host-web/qualification`, `npm run qualify -- --artifacts <out>
   --sdk-root <repo>/sdk --browser <b> --check-matrix --self-test-mutations` passes for chromium,
   firefox and webkit, with `render-allocations` reporting 0.
5. PR evidence, not committed: one scratch build with a planted `Vec::with_capacity(1)` inside
   `render_next` reads a count of at least 1 in one browser, and the gate fails.
6. `bash scripts/check-sdk-generated.sh <out>`, `cargo run --locked --release -q -p
   parameter-metadata -- --check <out>`.
7. `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features --
   -D warnings`, `bash scripts/check-workspace-policy.sh`.
8. **Stagings never allocate in the locked window.** Integration test binary
   `hosts/host-web/tests/render_locked_staging.rs` (decision 15 rules that host-web's native
   allocation-count gates live in an integration binary, never in `src/tests.rs`, which already
   registers a `#[global_allocator]`). It registers `RenderLockedAllocator<System>` as its
   `#[global_allocator]` and holds one `#[test]`, so no other test shares the global counter.
   - On a fresh thread (a fresh thread-local block), it boots a session with one observed effect
     and a spectrum capture through the ffi boot exports.
   - It calls `observation_read`, `track_response_capture`, `spectrum_read`,
     `spectrum_stream_read` and every staging accessor of D2 once each, through the exports.
   - `miso_engine_web_v1_render_allocation_count()` reads exactly 0 before and after.
   - Command: `cargo test --locked -p host-web --test render_locked_staging`.
   - PR evidence, not committed: with one boot-time accessor call removed, the count reads 1 or
     more.

## Test value

- `render_lock` unit tests (wrapper over `System`, called directly, not registered): an inverted
  flag test, a counter that is not incremented, or a `dealloc`/`realloc` path that skips the count
  turns them red; nothing tests the wrapper today.
- Browser gate `render-allocations`: an allocation or free anywhere in render, including behind the
  executor's `call_indirect`, turns it red; the direct-call static gate cannot see the executor.
- Self-test case "new indirect site": a new `call_indirect` in a render-thread closure turns it
  red; no existing rule looks at indirect calls.
- Self-test cases for D4.1 and D4.2: a closure member that registers a thread-local destructor, or
  that contains `memory.atomic.wait32`, turns them red; once atomics are allowed (#1332) no other
  rule catches either.
- `needs_drop` assertions: reverting any of the five thread locals to a type that needs drop fails
  to compile; on today's non-atomic build no other check sees it.
- Gate 8 (`render_locked_staging`): a staging that is still lazily allocated on first touch, so a
  booted worklet allocates it inside the locked window the first time it reads observations,
  responses or spectra, turns it red. The browser gate sees this only if its workload happens to
  touch each staging first inside the window; this test touches each one on a fresh thread.
- No test is superseded.

## Dependencies

- none. It does not depend on *Prove two Wasm instances on one shared memory in three browser
  engines and on iOS* (#1331) or *Build the browser artifact on a pinned nightly toolchain*
  (#1334): it gates today's stable, single-instance artifact and closes today's blind spot.
Dependent: *Run the browser control plane in a Worker and keep the AudioWorklet render-only*
(#1332) depends on this issue.

## Attempt record

### Attempt 1 (implementer, branch `codex/d15-stream-h` at `6d28a80ec`) -- STOPPED, not complete

**Status.** D1, D2, D4, D5, the D3 export and worklet message, deliverables 1, 2, 4 and 5 are
implemented and green. Deliverable 3 (the qualification gate and its mutation) and gates 4 and 5
are **not** done, and gate 3's `check-browser-expected-resources.py` and
`check-realtime-policy.sh` are red. Each needs a path outside "Authorized paths", and the
hazard below fired. Nothing was relaxed.

**Hazard fired: a real render-thread allocation on today's artifact.** A scratch run (not
committed) gave a copy of the built artifact's host a `miso.renderallocations.v1` request before
each dispose, and ran the Chromium leg with `?sdk=1`. 24 of 25 worklet instances read 0. The
SDK's spectrum-collection instance read 312, then 354 on a second run. A scratch build that
recorded the open window's closure type named all of them:
`host_web::ffi::miso_engine_web_v1_spectrum_stream_read::{{closure}}`, first allocation 7 bytes.
The cause: `PreparedSpectrumCapture::channels()` (`hosts/host-web/src/lib.rs:1496`) calls
`SpectrumCaptureCollection::selected_entry()` (`crates/host-core/src/spectrum.rs:843`). That
call clones the selected `SpectrumTarget` (its `String` id `track-a` is 7 bytes) and drops it.
So `spectrum_stream_read` and `spectrum_read` allocate and free once per read through
`AudioWorkletEngineHost::spectrum_channels` on a collection capture. The fix is bounded: add a
`selected_channels()` accessor that does not clone, in `crates/host-core/src/spectrum.rs`, and
use it in `PreparedSpectrumCapture::channels()`. Both files are outside this slice's authorized
paths (`lib.rs` is authorized for its module line and re-export only).

**Paths the slice needs that "Authorized paths" omits.**
1. `crates/host-core/src/spectrum.rs` and `hosts/host-web/src/lib.rs` (`PreparedSpectrumCapture::channels`)
   for the hazard fix above.
2. `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`,
   `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` and its checked mirror
   `sdk/src/browser/shipped-host.d.ts`. Qualification reaches the worklet only through the host,
   whose `#receive` refuses any reply tag it does not expect, and the worklet refuses a request
   id at or below its last one. So qualification cannot send `miso.renderallocations.v1` before
   `dispose()` without a host method. The workloads the spec names (live control, meters,
   observation, stall, and the reads through the SDK) each run in their own host instance with
   its own counter. So the count must be read per host, before each dispose. A raw-node workload
   only in `qualification.js` would cover one instance and copy the host's protocol. It would not
   read the instances the spec names. The planned shape: a host method
   `renderAllocationCount()` (response kind `renderAllocations`, reply fields
   `tag, requestId, result, count`). `qualification.js` calls it before each `dispose()` in
   `runLiveControlQualification`, `runObservationRun` and `runStallQualification`. The SDK's
   instances are reached through the same host. `run.mjs` adds gate `render-allocations`
   (every count `=== 0`) and mutation `render-allocations` (one count set to 1).
3. `hosts/host-web/tests/browser-v1/expected.json`: three exact `memoryBytes` rows move. Boot now
   allocates the three stagings (D5), mostly `ResponseStaging`'s fixed 1 MiB live-response
   capture. The raw direct oracle never touched them, but the shipped worklet already allocated
   them right after boot. Each row needs its own re-pin, with this reason:
   `simd128.initialStatus.memoryBytes` 1310720 -> 2424832,
   `simd128.beforeDisposeStatus.memoryBytes` 1310720 -> 2424832 and
   `commandTimeline.beforeDisposeStatus.memoryBytes` 1376256 -> 2490368 (+17 pages each). No PCM
   digest moved: `direct-oracle.mjs` asserts them before it prints.
4. `scripts/check-realtime-policy.sh`: `hosts/host-web/src/render_lock.rs` must join the list of
   approved files that may contain `unsafe`. A `GlobalAlloc` impl is `unsafe` by definition, and
   the policy already lists the other counting allocators (`boot_transient_budget.rs`,
   `bench_support::alloc`). The gate-8 binary has no `unsafe`: it stages through safe native
   writers.

**Decisions taken in this attempt.**
- D2's locked set is the set the worklet calls after boot. The spectrum request and collection
  accessors (`spectrum_request_*`, `spectrum_collection_*`) are not in it, because the worklet
  calls them only before boot, where they size and allocate the collection staging by design.
  Including them would count legal boot-time allocations. `ffi.rs`'s module doc records this.
- D5: the boot exports allocate the stagings only when boot returns a handle
  (`reserved_after_boot`). A refused boot allocates none of them. `check-web-boot-budget.mjs`
  requires a typed pre-parse refusal to grow memory by at most one page, and it went red while
  the reservation came first.
- D4: `closure()` now takes an exact name match before a substring match.
  `miso_engine_web_v1_render` is a prefix of `miso_engine_web_v1_render_allocation_count`, and
  the existing render gate otherwise stopped with "ambiguous export name". Self-test (h4) holds
  this.
- The integration binary cannot write through the `u32` pointer exports on a 64-bit host. A
  `#[doc(hidden)]` `native_staging` module was added to `ffi.rs`, behind
  `cfg(not(target_family = "wasm"))`, so it is not in the shipped artifact. It holds safe writers
  that store what the worklet writes through the exports' addresses.
- `INDIRECT_SITES`, measured on this attempt's named twin: render 2 (`PreparedRenderPlan::render`
  `invalidate_observers`, `render_inner` `executor.render`), meter_poll 0, command_submit 13 in 5
  members (the `&dyn Fn` mute predicate in `LiveRouteState::follow` and `followed_lanes`, and
  `Arc<dyn NativeEffectFactory>` in `EffectControlOwner::edit` and the producer's publish and
  preflight). The script gives the reason for each one.

**Gates run.**
- Gate 1, `cargo test --locked -p host-web --lib render_lock`: pass.
- Gate 2, `bash scripts/test-web-audioworklet.sh`: pass, with cases (h)-(h4).
- Gate 3: the build passes. `check-web-audioworklet.sh <out> <twin>` passes, with the three
  `--render-thread` runs. `check-browser-expected-resources.py --artifacts` **fails**: only the
  three `memoryBytes` rows (item 3 above).
- Gate 4: **not run as a gate**. The gate does not exist yet (item 2). The existing Chromium leg
  passes on this artifact (`--check-matrix --self-test-mutations`, Chromium 151.0.7922.34).
  Firefox and WebKit were not run.
- Gate 5: not run. It needs gate 4.
- Gate 6, `check-sdk-generated.sh <out>` and `parameter-metadata --check <out>`: pass.
- Gate 7: `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings` and `check-workspace-policy.sh` pass.
  `check-realtime-policy.sh` **fails** (item 4).
- Gate 8, `cargo test --locked -p host-web --test render_locked_staging`: pass.
- `cargo test --locked -p host-web`: all pass. `check-cross-targets.sh` was not run, because no
  engine crate changed.

**Mutation evidence. Each mutation was applied, the test went red, and the revert went green.**
- `render_lock` unit test. Each of these turned it red: an inverted flag test, no
  `fetch_add`, `dealloc` not counted, `realloc` not counted, and a window that never clears the
  flag. Test value: it catches a wrapper that counts the wrong calls, or counts outside its
  window. Nothing else tests the wrapper.
- `needs_drop` assertions. `LiveHostSlot` back to `RefCell<Option<LiveHost>>`, and
  `StagingSlot<T>` as `Cell<Option<Box<RefCell<T>>>>`, each fail to compile on the
  `const _: () = assert!(!needs_drop::<..>())` lines.
- Gate 8 (PR evidence). With the boot reservation of the observation staging removed, the count
  reads 6. With the response staging's removed, it reads 7. With the spectrum staging's removed,
  it stays 0. That is expected: boot itself touches the spectrum staging (`boot_staged`), and so
  does the worklet's pre-boot request staging. Test value: it catches a staging that a booted
  instance still allocates lazily inside the locked window.
- Callgraph self-test. Each of these turned the self-test red: the destructor-registration
  check disabled (h2, two cases), the atomic-wait check disabled (h3, two cases), the indirect
  comparison disabled (h1, h1a, h1b), hash stripping disabled (h1d, two cases), and the
  exact-name root disabled (h4). Test value: each D4 rule has a case that only that rule
  catches.
