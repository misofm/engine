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
