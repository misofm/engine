# Validate each effect descriptor once per type, not once per prepared instance

Stream J of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-15).
Code anchors verified on `main` at `6fb211594`.

D15-15 names two independent wins in different files with different merge orders. This issue is
the first: descriptor validation once per type (effect-contract only). The second is *Resolve graph
node IDs to dense indices once per graph compilation* (#1384).

## Product outcome

Session preparation on both hosts (the browser rebuild and the C ABI's
`SESSION_TRANSACTION_APPLY` rebuild) stops re-validating a static effect descriptor for every
prepared instance and every bank member. Each descriptor is validated once, when its factory enters
a `NativeEffectRegistry`. No rendered bit and no diagnostic a host can observe changes.

## Context

- **The evidence.** A phase profile of the browser module's preparation
  (`docs/handoffs/decision-15-2026-10-05/PLAN-2026-10-05-adversary-round1.md`, C4, "Cost", scratch
  profiles, not committed) attributes 8-9 % of preparation to per-instance descriptor validation.
  The frozen workload is #1289's four boot documents (`artifacts/steps/web-rebuild-base/report.md`:
  64-track boots 23.4-39.2 ms p50 under V8).
- **Validation at registration.** `NativeEffectRegistry::new`
  (`crates/effect-contract/src/lib.rs:2285-2313`) runs `validate_descriptor` on each factory's
  descriptor (`:2291`) and refuses it with `RegistryError { code: "effect.descriptor.invalid" }`
  (`:2292-2295`); it also validates the response-analysis descriptor (`:2297-2304`).
- **Validation again per instance.** `validate_prepare_request` (`:2459-2466`) starts with
  `validate_descriptor(d)` (`:2463`). `expected_prepared_metadata` (`:2543-2550`) calls it, and every
  effect calls that once per prepared instance and once per bank member, for example
  `crates/delay/src/lib.rs:646`, `crates/multiband-compressor/src/lib.rs:1553`, `:1561`, `:1598`,
  `crates/transient-shaper/src/lib.rs:852`, `:923`, `:932`, `crates/soft-clip/src/lib.rs:1017`,
  `:1099`, `crates/parametric-eq/src/lib.rs:3249`, `:3300`, `:3302`.
- **`validate_descriptor`** (`:708`) walks every parameter, port, quality and observation row and
  collects errors into a `Vec`. A descriptor is `&'static` and immutable, so its validity cannot
  change after the registry checked it.
- **Every production preparation goes through a registry.** Session preparation builds one with
  `launch_native_effect_registry()` (`crates/effect-compiler/src/prepare.rs:202-213`) at
  `crates/host-core/src/prepare.rs:1342-1343`, and hands it to `prepare_native_session_effects`.
  `crates/host-core/src/live_delta.rs:424` and `crates/host-core/src/response.rs:534` do the same.
  The registry is built per preparation, so "once per registry" is once per type per preparation.
- **No test pins the registry refusal of a main descriptor.** `crates/effect-contract/tests/response_analysis.rs:216-230`
  covers the response-analysis descriptor only. Nothing outside `lib.rs` names
  `effect.descriptor.invalid` except `docs/EFFECT_CONTRACT_V1.md:213`.

## Decisions frozen for this slice

- **D1. The registry is the one validation point.** Remove the `validate_descriptor` call from
  `validate_prepare_request`. Its doc comment states the precondition: `d` is the descriptor of a
  factory a `NativeEffectRegistry` admitted (or one the caller validated itself, as
  `crates/conformance/src/effect.rs:1048` does).
- **D2. Debug builds keep the check.** Replace the call with
  `debug_assert!(validate_descriptor(d).is_ok(), "...")`, so every test that prepares an
  unregistered descriptor directly still fails loudly. Release builds pay nothing.
- **D3. No signature change.** `validate_prepare_request`, `expected_prepared_metadata`,
  `NativeEffectFactory` and every effect crate stay as they are. No cache, no global state, no lock:
  the work is removed, not memoized.
- **D4. Diagnostics.** `effect.descriptor.invalid` stays a frozen code; it is now raised only by
  `NativeEffectRegistry::new`. Session preparation already built the registry before preparing any
  effect (`host-core/src/prepare.rs:1342-1343`, mapped to `host.effect.registry`), so a host
  observes the same refusal as before. Add one sentence saying so under "Stable diagnostics" in
  `docs/EFFECT_CONTRACT_V1.md`.
- **D5. Bits.** Validation is a pure check, so no rendered bit can move. The PR still records the
  one-time "no bit moved" comparison of gate 4 as evidence.

## Deliverables

1. The D1/D2 edit in `crates/effect-contract/src/lib.rs`.
2. The test of the Test value section.
3. The D4 sentence.
4. The before/after measurement of gate 5, with its two step records committed under
   `artifacts/steps/`.

## Authorized paths

- `crates/effect-contract/src/lib.rs` (`validate_prepare_request` and its doc only)
- `crates/effect-contract/tests/registry.rs` (new)
- `docs/EFFECT_CONTRACT_V1.md` ("Stable diagnostics" only)
- `artifacts/steps/d15-15-descriptor-before/`, `artifacts/steps/d15-15-descriptor-after/` (new)

## Non-goals

- Node-ID string compares in graph compilation: *Resolve graph node IDs to dense indices once per
  graph compilation* (#1384).
- A witness type for validated descriptors, or any change to the factory trait.
- Any other preparation cost (parse, identity hashing, bind/lower).
- Optimizing to improve the number. The measurement is descriptive (`AGENTS.md`, benchmarks).

## Objective gates

1. `cargo test --locked -p effect-contract` passes, including the new test.
2. `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
   and `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p delay -p compressor -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper -p soft-clip -p parametric-eq -p builtins -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`
   pass (the qualification `test-debug-a` and `test-debug-b` lines). Every effect's tests run with
   the D2 debug check active.
3. `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   and `bash scripts/check-workspace-policy.sh` pass.
4. **No bit moved (PR evidence, not a committed test).** On the base commit and on the branch, run
   `cargo run --locked --release -q -p host-web --example sdk_render_oracle -- 64 < F` for each of
   `fixtures/session/v1/parametric-eq-nine-track.json`, `console-sixty-four-track.json`,
   `console-sixty-four-track-app.json` and `console-sixty-four-track-sends.json`. The four digests
   are equal; the PR lists them.
5. **Frozen-workload before/after (descriptive).** On the base commit, then on the branch, each in
   a fresh empty directory:
   `bash scripts/run-web-mixing-automation-benchmark.sh prepare W`,
   `bash scripts/run-web-mixing-automation-benchmark.sh rebuild-preflight W`,
   `bash scripts/run-web-mixing-automation-benchmark.sh rebuild-run W --step d15-15-descriptor-before`
   (then `-after`). One invocation each: one warmup and two measured rounds, no retry, no tuning.
   Both runs use the same host and the same control state; if the load ceiling refuses, both set
   `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1` and the records say so. The PR states the p50 change per
   document. There is no pass threshold.

## Test value

- **`registry_refuses_an_invalid_main_descriptor`** (`crates/effect-contract/tests/registry.rs`): a
  factory whose descriptor has `contract_major: 2` makes `NativeEffectRegistry::new` return
  `RegistryError { code: "effect.descriptor.invalid", id: Some(<its id>) }`. Defect it catches:
  someone deletes or weakens the registry's `validate_descriptor` call. After D1 that call is the
  only release-build check, and no existing test exercises it for a main descriptor (only the
  response-analysis branch is covered).
- No test is superseded.

## Dependencies

- None. File-disjoint from stream A; it may merge at any time.

## Attempt record

### Attempt 1 (implementer, 2026-10-05)

Base `3ade8e969` (branch `codex/d15-stream-j`); implementation commit `a9f09e29a`.

- **D1/D2.** `validate_prepare_request` (`crates/effect-contract/src/lib.rs`) no longer calls
  `validate_descriptor`; it opens with `debug_assert!(validate_descriptor(d).is_ok(), ...)` naming
  the descriptor id, and its new doc comment states the precondition (registry-admitted, or
  validated by the caller) and lists the codes it still raises. No signature changed.
- **D4.** One paragraph under "Stable diagnostics" in `docs/EFFECT_CONTRACT_V1.md`.
- **Test.** `registry_refuses_an_invalid_main_descriptor` (`crates/effect-contract/tests/registry.rs`).
  The fixture also asserts its `contract_major: 1` twin validates and registers, so the refusal is
  the main-descriptor check and not a fixture defect. *Defect it catches:* the registry's
  `validate_descriptor` call deleted or weakened, which after D1 would let an invalid main
  descriptor prepare in release builds. *Mutation:* `if validate_descriptor(d).is_err()` in
  `NativeEffectRegistry::new` became `if false && validate_descriptor(d).is_err()`;
  `cargo test --locked --no-fail-fast -p effect-contract` was RED on exactly this test (panic
  "a descriptor with contract_major 2 must not enter the registry") and every other effect-contract
  test, `response_analysis.rs` included, stayed green; reverted, GREEN.
- **Gate 1.** `cargo test --locked -p effect-contract`: pass.
- **Gate 2.** `test-debug-b` line: pass (812 tests). `test-debug-a` line: the first run failed one
  test, `crates/capi/tests/resource_lifecycle.rs`
  `live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free` (run 5: raced
  final block `[0.0, 0.0]` against the fresh plan's `[-0.1212493, 0.039765462]`), on a host at
  load average ~98 on 32 CPUs. That race test prepares only registry-admitted descriptors, so the
  removed check could not have refused or changed anything there (with the debug assertion active,
  an invalid descriptor would have panicked, not rendered zeros). Five isolated reruns of it passed
  and a full rerun of the `test-debug-a` line passed (1427 tests, 0 failed). Recorded as a
  pre-existing load-sensitive intermittent outside this issue's paths, for the coordinator to file.
- **Gate 3.** `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings`, `bash scripts/check-workspace-policy.sh`: pass.
- **Gate 4 (no bit moved; PR evidence).** `sdk_render_oracle -- 64`, identical on base and branch:

  | fixture | digest |
  |---|---|
  | `parametric-eq-nine-track.json` | `2938bb7859ebb8ab57edc9bbdd5e2ee368a590078787df29c6da095fa8e1f5c8` |
  | `console-sixty-four-track.json` | `24607288d129370f1aa1d617c7d7ac69ff4cb6122520fb26a3873116accffce8` |
  | `console-sixty-four-track-app.json` | `ac23bd045e27fda2095937e70b78a8df62ff6d3fd301815d82f2807b888430cf` |
  | `console-sixty-four-track-sends.json` | `58c54351ab7e89bf3afe3bad3b33c075d628fad429359a22da37f85581d5f883` |

- **Gate 5 (descriptive).** `prepare`, `rebuild-preflight`, `rebuild-run` once each, fresh empty
  workdirs, one warmup and two measured rounds, no retry or tuning. The host was above the load
  ceiling for both, so both ran with `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1` and both records say
  `uncontrolled`; both pinned to CPU 31; both validators accepted. Records:
  `artifacts/steps/d15-15-descriptor-before/` (module `a9a51862...` at `3ade8e969`, loadavg 95.49)
  and `artifacts/steps/d15-15-descriptor-after/` (module `b723ff9f...` at `a9f09e29a`, loadavg
  39.22). Boot p50, mean of the two rounds:

  | document | before ms | after ms | change |
  |---|---|---|---|
  | nine_track_eq | 4.892 | 3.531 | -27.8 % |
  | sixty_four_track_console | 48.498 | 33.206 | -31.5 % |
  | sixty_four_track_app_shape | 45.311 | 32.550 | -28.2 % |
  | sixty_four_track_console_sends | 77.748 | 54.380 | -30.1 % |

  Candidly: the host load fell from ~95 to ~39 between the two runs, so most of this ~30 % is the
  load difference, not the change; the profile's 8-9 % attribution is the better estimate of the
  change's share. The two records cannot separate them, and per the benchmark rules the run was
  not repeated.
