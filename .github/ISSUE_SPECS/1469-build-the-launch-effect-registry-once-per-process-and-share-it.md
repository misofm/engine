# Build the launch effect registry once per process and share it

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Filed 2026-10-06 by root ruling R2 on #1462's verification
(`/home/bl/misofm/submix-verdicts/1462-attempt1.md`, R2 and flagged point 3). Code anchors verified
on `codex/d15-stream-g2` at `b5cfd2b60`; re-verify them on the branch where #1462 has landed before
starting. Ordered after #1462 and before #1372.

## Product outcome

The launch native-effect registry, with its tail-bound table, is built once per process (once per
module instance in the browser) and is then shared, immutable, by every session preparation, every
live rebuild and every EQ response preview. Today each of those builds a fresh registry, and each
build evaluates every launch effect's `tail_and_rest` for all 32 rows (8 effects x 4 launch rates x
`Normal`). The cost is nil now because every body is a constant; #1372-#1376 put certified
derivations behind `tail_and_rest`, and after this slice those derivations run 32 times per process,
not 32 times per preparation, rebuild and preview. Nothing else changes: no prepared value, no
diagnostic and no rendered bit moves.

## Context

- **The builder.** `launch_native_effect_registry()` (`crates/effect-compiler/src/prepare.rs:208-219`)
  returns a fresh owned `Result<NativeEffectRegistry, RegistryError>` on every call. Its doc comment
  (`:203-207`) says callers retain and inject the registry and that there is no render-reachable
  global catalog.
- **The registry.** `NativeEffectRegistry` (`crates/effect-contract/src/lib.rs:2512`) holds a
  `BTreeMap<&'static str, RegistryEntry>`; each entry is an `Arc<dyn NativeEffectFactory>`
  (`NativeEffectFactory: Send + Sync`, `:1846`) and a `Box<[RegisteredTailBound]>` (plain `Copy`
  data, `:2443`). `NativeEffectRegistry::new` (`:2520`) evaluates `(d.tail_and_rest)(rate, quality)`
  once per declared quality row (`:2545`, #1462 D1) and checks it. That line is the only production
  evaluation of `tail_and_rest` (#1462 D3).
- **The three production call sites** (R2), each building a fresh registry:
  - session preparation: `crates/host-core/src/prepare.rs:1347-1348`, on every
    `prepare_host_runtime*` call (both hosts prepare through it);
  - live rebuild: `crates/host-core/src/live_delta.rs` `classify_live_delta` (`:224`) keeps
    `let mut registry = None` (`:279`) and threads `&mut Option<NativeEffectRegistry>` through
    `effect_records` (`:357`) and `parameter_records` (`:435`) to `load_registry` (`:476-487`), which
    builds one per `classify_live_delta` call that needs it;
  - EQ response preview: `crates/host-core/src/response.rs:533-534`, in
    `prepare_response_preview` (`:502`), on every preview.
- **Tools and tests** also call it (re-grep `launch_native_effect_registry` across `crates/`,
  `hosts/` and `tools/` at start): `tools/session-validator/src/lib.rs:377`,
  `tools/parameter-metadata/src/lib.rs:142`, `tools/console-workload/src/lib.rs` (`:1337`, `:2571`,
  `:3188`), `tools/bench/src/console.rs`, `crates/host-core/src/control_provider.rs` (tests, `:685`,
  `:747`), `crates/graph-compiler/src/lib.rs` and `src/tests/*` (test modules), and the integration
  tests of `effect-compiler`, `graph-compiler`, `host-core`, `parameter-metadata`,
  `session-validator` and `console-workload`.
- **Where preparation runs.** On the C ABI, on the caller's control thread; several engines may
  prepare on several threads at once. In the browser, on the AudioWorklet thread at processor
  construction today, and on a Worker after #1332 (stream H). The browser module is built for
  `wasm32-unknown-unknown` with `+simd128` and no `atomics` (`scripts/build-web-audioworklet.sh:113`),
  so it is single-threaded, and its statics live in the module instance's own linear memory.
- **#1457** caches the builtin input section's design bounds (`builtins::input_section_bounds`),
  keyed by design and bounded only by the track count; its D2 (where its cache lives, its entry
  bound and its eviction) is still open. It never names `NativeEffectRegistry` (R2). This slice does
  not share a home with it: the launch registry is a fixed, small, process-constant value (eight
  effects, 32 rows) that needs no key, bound or eviction.

## Decisions frozen for this slice

- **D0. Root ruling R2 (2026-10-06).** Made by the decision-15 root coordinator under the owner's
  no-shortcuts delegation (`no-shortcuts-correctness-first`): the launch native-effect registry and
  its tail table live for the whole process, once per module instance in the browser; they are built
  once, at first use or at boot (allocation is allowed there and never on render); they are immutable
  and shared by session preparation, live rebuild and the EQ preview; no behaviour changes.
- **D1. Where it lives.** One `static` in `crates/effect-compiler/src/prepare.rs`:
  `std::sync::OnceLock<Result<NativeEffectRegistry, RegistryError>>`, filled by a private builder that
  holds today's body (the eight launch factories, in today's order). It is built lazily, on the first
  call, which is the first preparation, live rebuild or preview of the process; no eager boot hook is
  added. A failed build is cached too (the descriptors are `&'static`, so a rebuild gives the same
  error) and every call returns a clone of the `RegistryError`. The builder is not public.
  *(Root ruling, 2026-10-06.)* A cached `RegistryError` is permanent for the process (for the
  module instance in the browser). The registry's inputs (the eight factories and their `&'static`
  descriptors) are compiled in, so a retry cannot succeed. There is no retry, reset or rebuild
  path.
- **D2. The one entry point.** `launch_native_effect_registry()` keeps its name and becomes
  `pub fn launch_native_effect_registry() -> Result<&'static NativeEffectRegistry, RegistryError>`.
  Every production call site reads the shared registry through it:
  - `host-core/src/prepare.rs`: the same call, now a reference, same `host.effect.registry` error;
  - `host-core/src/response.rs`: the same, same `ResponsePreviewError::UnknownEffect` error;
  - `host-core/src/live_delta.rs`: `classify_live_delta`, `effect_records` and `parameter_records`
    take `&'static NativeEffectRegistry` (or obtain it once at the first site that needs it, with the
    same `LiveRebuild::Structure` refusal and the same refusal order); the `Option` plumbing and
    `load_registry` are deleted. A live delta that today never builds a registry (no automation
    change, no parameter change) still never reaches the call's error path, so no classification
    result moves.
- **D3. Each host's ownership.**
  - **Browser:** a `static` in a Wasm module is per module instance, because each instance has its
    own linear memory. Each AudioWorklet (or, after #1332, Worker) module instance builds the
    registry at its first preparation, on the same thread that builds it today, and keeps it until
    the instance is dropped. A new instance builds its own. No JS or SDK change.
  - **C ABI:** the static lives in the engine library once per loaded copy of it, so every C ABI
    engine in a process shares one registry. Destroying an engine does not drop it. No C ABI symbol,
    header or behaviour changes.
- **D4. Thread safety for native hosts with several engines.** `OnceLock::get_or_init` runs the
  builder exactly once; a second thread that arrives during the build waits for it on its own
  control thread, then reads the same value. After the build the registry is read-only, so shared
  reads need no lock. `NativeEffectRegistry` must be `Send + Sync`; the slice adds a compile-time
  assertion of that next to the static. No render thread calls `launch_native_effect_registry`
  (it is reachable only from preparation, live classification and preview, all control-side), so
  the one-time wait can never fall on render. Prepared plans keep their `Arc<dyn NativeEffectFactory>`
  clones (`get_shared_ascii`) as today; a plan's retirement drops a clone, never the last one.
- **D5. Tests that need a custom registry** keep building their own with `NativeEffectRegistry::new`
  and the factories they choose (`crates/conformance/src/randomized.rs:285`,
  `crates/effect-contract/tests/support/mod.rs:136`, `crates/delay/src/corpus.rs:160` and their
  peers); nothing changes for them. A test that wants the launch set reads the shared
  `&'static` registry. No public owned launch builder is added: an owned launch registry built from
  the eight factories is exactly the per-call rebuild this slice removes.
- **D6. The counter.** `crates/effect-contract` gains a `test-support` feature (it has no
  `[features]` table today) that compiles one process-global `AtomicU64`, incremented beside the
  evaluation at `NativeEffectRegistry::new` (`:2545`), and a `#[doc(hidden)]` reader
  `tail_bound_evaluations() -> u64`. Without the feature neither exists. `effect-compiler`'s
  `test-support` forwards `effect-contract/test-support`, so `test-debug-a` (which enables
  `effect-compiler/test-support` and `host-core/test-support`) turns it on through the manifests and
  `scripts/check-test-support-ci.py` passes with no workflow edit.
- **D7. Class A.** No prepared value, no diagnostic code or order, and no rendered bit moves. The
  doc comment at `prepare.rs:203-207` is rewritten to say the registry is a process-lifetime,
  control-plane-only value that no render path reaches.

## Deliverables

1. The static, the private builder, the `&'static` entry point and the `Send + Sync` assertion (D1,
   D2, D4).
2. The three production call sites on the shared registry (D2), with live delta's `Option` plumbing
   removed.
3. The `effect-contract` counter and feature forwarding (D6).
4. Gate 1's counted test and its mutation record.
5. Forced caller edits (D2's type change), field/call only.
6. The doc comment (D7) and, if it names per-call construction, the matching line in
   `docs/EFFECT_CONTRACT_V1.md`.

## Authorized paths

- `crates/effect-compiler/src/prepare.rs` (the static, builder, entry point, assertion and doc
  comment), `crates/effect-compiler/Cargo.toml` (the feature forward only)
- `crates/effect-contract/src/lib.rs` (the counter and its reader, feature-gated, at
  `NativeEffectRegistry::new` only), `crates/effect-contract/Cargo.toml` (the `[features]` table)
- `crates/host-core/src/prepare.rs` (the registry call only), `crates/host-core/src/live_delta.rs`
  (the registry plumbing and `load_registry` only), `crates/host-core/src/response.rs` (the
  registry call only)
- `crates/host-core/tests/launch_registry_once.rs` (new, gate 1) and, if the test needs it, a
  `[[test]]` entry with `required-features = ["test-support"]` in `crates/host-core/Cargo.toml`
- forced callers of `launch_native_effect_registry` (D2's type change), field/call only, in the
  files the Context list names; list each in the attempt record. Deref coercion should make most of
  them compile unchanged.
- `docs/EFFECT_CONTRACT_V1.md` (one line, only if it describes per-call construction)
- **Named exception, conditional:** `hosts/host-web/tests/browser-v1/expected.json`, only if the
  worklet chain shows a moved memory figure, with the measured reason in the commit (the registry
  now stays allocated instead of being freed after each preparation). A ceiling is never loosened to
  pass.
- this spec; `docs/handoffs/decision-15-2026-10-05/STREAMS.md` (this slice's rows)

## Non-goals

- Any effect's tail values or their derivations (#1372-#1376, #1378).
- #1457's builtin design-bound cache, and the live input bound's per-rate cost.
- Caching across processes, persisting the registry, or a registry the host injects or replaces at
  run time.
- An eager boot hook, a JS/SDK change, or a C ABI change.
- Third-party effects (owner ruling R6a).

## Hazards

- **Hot files.** `crates/effect-compiler/src/prepare.rs`, `crates/effect-contract/src/lib.rs`,
  `crates/host-core/src/prepare.rs` and `crates/host-core/src/live_delta.rs` are STREAMS hot-file
  rows; this slice lands after G #1462 and before G #1372, in the slots those rows name.
  `crates/graph-compiler/src/*` is also a hot row; its test call sites should compile unchanged, and
  any forced edit there is field/call only.
- **Test isolation of the counter.** The counter is process-global. Every test binary that builds a
  custom registry also counts, so gate 1 lives alone in its own integration-test binary with one
  `#[test]` function, and it starts by asserting the counter is 0.
- **Live delta refusal order.** Moving from a lazily filled `Option` to the shared registry must not
  change which `LiveRebuild` reason a delta reports first; host-core's `tests/live_delta.rs` stays
  green unchanged.
- **Realtime.** The `OnceLock` wait is a blocking primitive; it is acceptable only because no
  render path reaches the entry point. Do not call `launch_native_effect_registry` from any render
  or realtime-marked region; `scripts/check-realtime-policy.sh` stays green with no allowlist edit.
- **Memory pins.** The registry's allocations now persist for the life of the process or module
  instance. Rendered bits cannot move, but an exact browser memory figure might; see the named
  conditional exception.

## Objective gates

1. **32 evaluations per process (counted).** `crates/host-core/tests/launch_registry_once.rs`,
   gated on `test-support`, one `#[test]`:
   - asserts `tail_bound_evaluations() == 0` at start;
   - prepares a launch session through host-core's public preparation entry
     (`prepare_host_session` or `prepare_host_runtime*`) many times (at least 8) and at each of the
     four launch rates;
   - runs `classify_live_delta` many times (at least 8) on an EQ parameter change that reaches
     `parameter_records`, and on an automation change that reaches `effect_automation_diagnostics`;
   - runs `prepare_response_preview` for an EQ target many times (at least 8);
   - repeats a mixed set of these on at least 4 threads at once (D4);
   - asserts the counter equals the sum of `qualities.len()` over the launch registry's descriptors,
     and that this sum is 32 today;
   - asserts two calls return the same address (`std::ptr::eq`).
   **Mutation runs** (each applied: red; reverted: green; recorded in the attempt record):
   M1 `launch_native_effect_registry` bypasses the `OnceLock` and builds on every call; M2
   `host-core/src/prepare.rs` builds its own registry with `NativeEffectRegistry::new` over the
   launch factories; M3 the same in `response.rs`; M4 the same in `live_delta.rs`. Each is red in
   gate 1's test.
2. **No rendered bit moves.** `audit capi`'s `pcm_digest` unchanged (`cb10fbface44a3a4` on #1462's
   record; re-read it on the base), with 0 allocations and 0 syscalls;
   `bash scripts/run-wasm-gates.sh`; the worklet chain (`build-web-audioworklet.sh --named-twin`,
   `check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`,
   `check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh`); and
   `scripts/check-graph-determinism.sh`.
3. **Workspace gates.** fmt; workspace clippy `--all-targets` with and without `--all-features`,
   `-D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; the
   `test-debug-a` and `test-debug-b` commands from `.github/workflows/qualification.yml` and their
   doctests (gate 1's test must appear in test-debug-a's output, not be skipped);
   `python3 scripts/check-test-support-ci.py`; `bash scripts/check-effect-contract.sh`;
   `bash scripts/check-workspace-policy.sh`; `bash scripts/check-realtime-policy.sh`;
   `bash scripts/check-cross-targets.sh`; `bash scripts/check-capi-abi.sh` and `audit capi`.

## Test value

Gate 1's test turns red if any production path (preparation, live rebuild or preview) builds a
launch registry again, or if the shared registry is built more than once under concurrent first
use; no existing test counts registry builds or `tail_and_rest` evaluations across calls.

## Dependencies

- #1462 (the registry's tail table). Before #1372 in stream G, so #1372-#1376's derivations land on
  the shared registry.

## Attempt record

### Attempt 1 (implementer, 2026-10-07) -- checkpoint, gates incomplete (disk stop)

Base `34cd5d55f` (#1462 at `7a8a30f1c`/`8bb56d9b5`). Anchors re-verified on this base.

**Changes.**
- `crates/effect-compiler/src/prepare.rs`: `static LAUNCH_NATIVE_EFFECT_REGISTRY:
  OnceLock<Result<NativeEffectRegistry, RegistryError>>`; private
  `build_launch_native_effect_registry` (today's body, same order);
  `launch_native_effect_registry() -> Result<&'static NativeEffectRegistry, RegistryError>`
  (`get_or_init`, error cloned; a cached error is permanent, no reset path); a `const` `Send + Sync`
  assertion for `NativeEffectRegistry` and `RegistryError`; the doc comment rewritten (D7).
- `crates/effect-contract`: `[features] test-support = []`; a feature-gated process-global
  `AtomicU64` incremented beside the one `tail_and_rest` evaluation in `NativeEffectRegistry::new`,
  and `#[doc(hidden)] tail_bound_evaluations()`. `effect-compiler/test-support` forwards
  `effect-contract/test-support`.
- `crates/host-core/src/prepare.rs` and `response.rs`: the same call, now a reference (prepare.rs
  passes `registry` instead of `&registry`; response.rs unchanged). Same error codes.
- `crates/host-core/src/live_delta.rs`: the `Option<NativeEffectRegistry>` plumbing and
  `load_registry` deleted; the automation branch and `parameter_records` each call
  `launch_native_effect_registry()` at the point the `Option` was filled, with the same
  `LiveRebuild::Structure` mapping and the same position in the refusal order. A delta that never
  reached `load_registry` still never reaches the call. The `# Allocation` doc no longer lists the
  registry as allocated per call.
- Gate 1: `crates/host-core/tests/launch_registry_once.rs` (one `#[test]`), with a `[[test]]` entry
  `required-features = ["test-support", "control-provider"]` in `crates/host-core/Cargo.toml`.
  `control-provider` is needed for `classify_live_delta`; in test-debug-a it is on through `capi`'s
  dependency (the same route `tests/live_delta.rs` relies on). Order: counter 0; 4 threads behind a
  `Barrier`, each first reaching the registry directly, then preparing (rate per thread), classifying
  an EQ gain change (reaches `parameter_records`) and an EQ console automation (reaches
  `effect_automation_diagnostics`), previewing, and checking `ptr::eq`; then 8 rounds of preparation
  and preview at all four launch rates plus both classifications; then `ptr::eq`, sum of
  `qualities.len()` == 32, counter == 32.
- `docs/EFFECT_CONTRACT_V1.md`: no line describes per-call construction; unchanged.
- Forced callers (D2), call only: `&registry`/`&launch_native_effect_registry()...` became
  `registry`/`launch_native_effect_registry()...` (clippy `needless_borrow`, applied by
  `cargo clippy --fix`; every changed line was checked to be only that borrow removal) in
  `crates/effect-compiler/tests/native_session.rs`, `crates/graph-compiler/src/lib.rs` (also the
  three `(&registry, ...)` tuple elements of one test's `cases` array, a type error otherwise),
  `crates/graph-compiler/src/tests/{bank_padding,console_banking}.rs`,
  `crates/graph-compiler/tests/{bank_levels,bypass_cohorts,bypass_resources,live_routes,route_activity,route_coefficients,track_delay,vca_follow}.rs`,
  `crates/host-core/src/control_provider.rs` (tests), `crates/host-core/tests/{live_routes,randomized,route_mute,submix_strip}.rs`,
  `tools/console-workload/src/lib.rs`, `tools/session-validator/src/lib.rs`,
  `tools/session-validator/tests/validate.rs`; and `tools/console-workload/tests/paired_spans.rs`
  (`fn registry()` returns `&'static NativeEffectRegistry`). `tools/bench/src/console.rs` compiles
  unchanged; #1462's NIT n3 (a registry built per lane in `eq_request`) is now a read of the shared
  registry, so no per-lane build remains.

**Mutation runs** (`cargo test --locked -p host-core --features test-support,control-provider --test
launch_registry_once`; each applied: red, reverted: green). M2-M4 build an owned registry with
`NativeEffectRegistry::new` over the eight launch factories, through a temporary `pub` wrapper of
the private builder added only for the run.
- M1 (entry point bypasses the `OnceLock`, builds and leaks per call): red, `ptr::eq` fails in every
  thread.
- M2 (`host-core/src/prepare.rs` builds its own): red, counter 1184 != 32 (32 + 36 preparations x 32).
- M3 (`response.rs` builds its own): red, 1184 != 32 (36 previews).
- M4 (`live_delta.rs` `parameter_records` builds its own): red, 416 != 32 (12 classifications).
- M4b (`live_delta.rs` automation branch builds its own): red, 416 != 32.
- M5 (racy `get()`-then-`set()` instead of `get_or_init`): red 10 of 10 runs (128 or 96 != 32), so
  the concurrent-first-use half of the test-value sentence holds.
- Reverted: green (0.68 s).

**Gates run.**
- `cargo build --locked --workspace --all-targets --all-features`: pass.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: pass (after the
  `needless_borrow` call-site fixes, before `cargo fmt`; fmt then only re-wrapped lines).
- `cargo fmt --all -- --check`: pass.
- Gate 1 test: pass.

**Disk stop, then resumed.** After the all-features build `df -h /` showed 16 GB free; building
stopped. The coordinator then freed `target/debug/incremental` and `target/ci` (42 GB free), and the
gates below ran with `CARGO_INCREMENTAL=0` (disk 26 GB free at the end).

**Gates, continued (base checkpoint `1977ffbf0`).**
- clippy `--workspace --all-targets -D warnings` (no `--all-features`): pass.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps --exclude gate-expander`:
  pass. `gate-expander` is excluded for the known `corpus.rs:64` failure from #1459.
- test-debug-a (`--no-fail-fast`): **FAIL**, 2 tests in `crates/capi/tests/resource_lifecycle.rs:830`
  (see below). All other binaries pass (124 `test result: ok`). Gate 1 runs there:
  `the_launch_registry_is_built_once_per_process ... ok`. host-core `tests/live_delta.rs` is
  unchanged and passes (31 tests). test-debug-a doctests: pass.
- test-debug-b and its doctests: pass.
- `check-test-support-ci.py`: pass (11 packages; `effect-contract/test-support: test-debug-a`).
- `check-effect-contract.sh`, `check-workspace-policy.sh`, `check-realtime-policy.sh` (89 regions,
  no allowlist edit), `check-capi-abi.sh`, `check-graph-determinism.sh` (100/100),
  `check-cross-targets.sh`, `run-wasm-gates.sh`: pass.
- `audit capi`: `pcm_digest cb10fbface44a3a4` (unchanged), 0 allocations, 0 deallocations,
  0 syscalls.
- Worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
  --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
  `check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh`): pass. No browser memory
  figure moved, so `hosts/host-web/tests/browser-v1/expected.json` is unchanged.

**Open finding: the spec does not cover a resource-charge change (implementer stop).**
`capi_retained_bytes_charge_every_byte_the_compile_retains` and
`tiny_control_frame_still_accounts_three_provider_counters_exactly` fail with the observed bytes 16
below `capi_retained_bytes` (274137 against 274153, and 200459 against 200475). Cause:
`effect_control_resources` (`crates/effect-compiler/src/prepare.rs`, the owner walk that adds
`factory_allocation_bytes` once per `Arc` identity) charges the EQ control owner's factory `Arc`
allocation (an `ArcInner` of the zero-sized `ParametricEqFactory`: 16 bytes) to the prepared plan.
Before this slice the per-call registry was freed after preparation, so the owner held the last
clone and dropping the effect producers freed those 16 bytes. Now the process-lifetime registry
owns that allocation, so the plan no longer retains it and dropping the producers frees nothing for
it. The charge is now 16 bytes above what the plan retains (the safe direction, but the test checks
exact equality, which is correct for this claim). A correct fix changes the resource-accounting
contract: for example, do not charge a factory `Arc` that the process-lifetime registry owns, or
charge the registry once to the process instead of to each plan. The edit is in
`effect_control_resources` (not one of the spec's authorized `prepare.rs` regions) and maybe in its
host-web and `EqTargetPreparer` peers (`hosts/host-web/src/control_targets.rs:451,567` add
`factory_allocation_bytes` to an EQ workspace charge). No ceiling is loosened and no test is edited.
This needs a root ruling or a spec amendment.
