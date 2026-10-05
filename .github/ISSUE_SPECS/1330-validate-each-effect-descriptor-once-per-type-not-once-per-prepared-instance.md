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
  (`/home/bl/misofm/submix-verdicts/PLAN-2026-10-05-adversary-round1.md`, C4, "Cost", scratch
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

- Node-ID string compares in graph compilation (the proposed successor issue).
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
