# Accept only the launch sample rates in effect descriptors

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

**Owner ruling (2026-09-28):** Approved: accept only the launch rates.

**Blocked on an owner ruling.** Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`,
section 7, R5. The ruling to record: "The engine supports exactly 44.1, 48, 88.2 and 96 kHz. The
extended research rates (176.4, 192, 352.8 and 384 kHz) are removed from every accepted set."

## Context

- **No rendering path accepts an extended rate today.** `crates/session/src/validate.rs:36-43`
  refuses any non-launch session rate (`SampleRateUnsupportedAtLaunch`). host-web boot, host-core
  and capi all compile through `session`. No native effect declares an extended quality row: see,
  for example, `crates/delay/src/lib.rs:265-270`.
- **They survive only as descriptor metadata:**
  - `crates/engine/src/lib.rs:55-84`: `EXTENDED_COMPATIBILITY_SAMPLE_RATES` and the predicate
    `is_extended_compatibility_sample_rate`. The unit test
    `sample_rate_tiers_are_exact_sorted_disjoint_and_classified` is at `:111`.
  - `crates/effect-contract/src/lib.rs:44` and `:761`: `validate_descriptor` accepts a quality row
    at a launch **or** extended rate, while still requiring all four launch rates (`:781-788`);
  - `crates/effect-package/src/wire.rs:13-14`: the descriptor-wire decoder uses the same predicate.
    This part goes with `R6-…` if that lands first.
  - `fixtures/conformance/v1`: 4 of its 11 files use extended rates. CI checks it with
    `cargo run -p conformance --example conformance_fixtures -- --check`.
- **Other uses:** no coefficient or math table depends on the extended rates (searched, not
  exhaustively). The true-peak limiter's exact box-sum bound `R <= 961` is derived for rates up to
  96 kHz (`true-peak-limiter/src/lib.rs:94-99`).
- **Shipped module:** `validate_descriptor` is in its closure, so the module may change slightly.

## Smallest closable slice

1. Delete `EXTENDED_COMPATIBILITY_SAMPLE_RATES` and its predicate, and reduce the engine test to
   the launch tier.
2. Make `validate_descriptor`, and the descriptor-wire decoder if it still exists, accept launch
   rates only. Flip every descriptor test and reference vector that accepted an extended row to
   expect refusal, and list them; `scripts/effect-descriptor-v1-reference.py` is one candidate.
3. Regenerate or trim `fixtures/conformance/v1` to launch rates, and update its manifest and
   `conformance_fixtures --check`.
4. Update AGENTS.md ("176.4 … 384 kHz are extended compatibility/research evidence only") and
   `docs/EFFECT_CONTRACT_V1.md`.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p dsp-reference -p conformance`
     passes.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change. `bash scripts/run-wasm-gates.sh` passes. Rates are not part of any console row's
   render.
3. **Shipped artifact.** Build base and change on one machine, as audit section 11 describes. The expected change is the removed
   extended-rate branch of `validate_descriptor`. Explain every changed function from
   `wasm-objdump -d`, and re-pin.
4. **CI routing.** `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   passes with the updated corpus. `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   `check-effect-descriptor-v1.sh` passes, if it still exists.
5. **No live claim lost.** Launch-rate descriptor acceptance and session rate refusal keep their
   tests. The only tests removed or flipped are the extended-rate ones. List them from the
   `-- --list` diff and the test diff (audit section 11).

## Dependencies

The owner ruling. It can land before or after `R6-…`.

## Standing rules for the implementer

- Commit on `codex/<issue>-launch-rates-only`. Do not run timed benchmarks.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F9. The recommendation stands; the scope is larger than
"descriptor metadata, about 50 lines".

1. **A shipped-module predicate also accepts extended rates.**
   `builtins::validate_builtin_filter_cutoff` (`crates/builtins/src/lib.rs:343-370`) admits
   176.4-384 kHz through `is_extended_compatibility_sample_rate`, and `BuiltinChain::new` relies
   on it (pinned by `builtins/tests/contract.rs:297-309`). It is reached from
   `builtins-compiler/src/lib.rs:4937` and `builtins/src/lib.rs:3217`, both in the shipped module.
   Add it to step 2. The module changes in `builtins` as well as `effect-contract`.
2. **Files the slice omits** (`rg 'EXTENDED_COMPATIBILITY_SAMPLE_RATES|is_extended_compatibility_sample_rate'`
   lists 12 files outside `engine/src/lib.rs`):
   - the conformance mock effect declares eight quality rows, four of them extended
     (`crates/conformance/src/effect.rs:99-107`); a tightened `validate_descriptor` rejects it, so
     `conformance` tests go red unless the mock is trimmed;
   - `conformance/src/block.rs`, `src/fixture.rs`, `tests/fixtures.rs` and
     `examples/conformance_fixtures.rs`;
   - `builtins/tests/response.rs`, `tests/stage.rs` (RBJ-oracle and stage tests at extended rates);
   - `engine/src/realtime/mod.rs:688`; `effect-contract/tests/response_analysis.rs:71-74`;
   - `fixtures/effect-descriptor/v1/comprehensive-b.json` and
     `fixtures/effects/runtime-v1/valid/descriptor.toml:5` (hash-checked; goes stale).

   Failure scenario: an implementer follows the listed files only and the `conformance` and
   `builtins` test binaries go red on files the draft never named.
3. **Gates.** Add `-p builtins -p conformance` to gate 1's test run, and extend gate 3's expected
   module diff to the `builtins` cutoff branch.
4. **Mobile scope: no change.** A device whose hardware rate is outside the launch set is the
   separate "no implicit SRC" rule, which this ruling does not touch.

## Attempt 1 evidence

Terra, attempt 1, branch `codex/1036-launch-rates-only` from `codex/batch-slim-2` (`eca8779d`,
`main` plus #1048, #1027, #1037 and #1063). Implementation commit `9ae0f9fa`.

### What changed

The engine accepts exactly 44.1, 48, 88.2 and 96 kHz everywhere. Removed:

| item | what it covered |
|---|---|
| `engine::EXTENDED_COMPATIBILITY_SAMPLE_RATES`, `is_extended_compatibility_sample_rate` | the extended tier and its predicate |
| the extended branch of `effect_contract::validate_descriptor` | a quality row at 176.4-384 kHz was accepted; it now refuses with `Quality` at `qualities` |
| the extended fallback of `builtins::validate_builtin_filter_cutoff` (amendment 1) | an open-Nyquist cutoff domain at 176.4-384 kHz, which let `BuiltinChain::new` prepare there; only a launch rate has a domain now |
| conformance: the mock's four extended quality rows, `EffectConformanceReport::extended_compatibility_probes`, `FaultKind::ExtendedRatePreparation` | the extended probe tier; every declared row is now a launch gate |
| conformance: extended acceptance in `PlanarBlock::try_new`, `PcmFixture::parse` and `encode` | blocks and `.mepcm` fixtures at extended rates |
| `fixtures/conformance/v1/rate-{176400,192000,352800,384000}-impulse-dual-mono.mepcm` and their `MANIFEST.tsv` rows | 4 of the 11 fixtures; the other 7 are byte-unchanged and `conformance_fixtures` generates launch rates only |
| the `extended_compatibility_*` fields of `bench effect-contract --conformance` | the tier's JSON record; `check-effect-contract.sh` reads only the launch fields |
| extended rows in test descriptors (`effect-contract/tests/response_analysis.rs`, `effect-compiler/tests/native_session.rs`) and in `fixtures/effects/runtime-v1/valid/descriptor.toml` (manifest re-hashed) | descriptors the tightened validator would refuse |
| extended rates in `builtins/tests/stage.rs` and `response.rs` rate lists | RBJ-oracle and stage gates at 176.4-384 kHz; launch-rate coverage is unchanged |

`crates/builtins/tests/response.rs` (shared with #1049): only the import, the rate-list helper's
body and its comment changed, so the merge stays mechanical. The helper's and four tests' names
still say "extended"; renaming them is left until #1049 lands.

Docs: the one rate sentence in AGENTS.md, `docs/EFFECT_CONTRACT_V1.md` (evidence contract),
`docs/SESSION_SCHEMA_V1.md`, `docs/IMPLEMENTATION_PLAN.md`, `dsp-research/README.md` and
`fixtures/conformance/README.md` (whose run command also named the pre-rename crates).
`scripts/effect-descriptor-v1-reference.py`, `check-effect-descriptor-v1.sh`, the descriptor wire
and `fixtures/effect-descriptor/` went with #1037, so nothing was left there.

Diff against `eca8779d`: 305 lines added, 223 removed, in 28 text files, plus 4 binary fixtures
deleted. Rust: 268 added, 201 removed; 147 of the Rust additions are the two new host-path refusal
tests (capi 93, host-web 54), and the SDK eval adds 19 JavaScript lines. Without those tests the
change is 121 Rust lines added and 201 removed.

### Refusal on every host path

The session layer already refused extended rates (`session::validate`), so these tests pin
existing host behaviour at each entry point rather than discriminate this change; the
discriminating flips are listed under "Test inventory".

| host path | test | asserts, for 176.4, 192, 352.8 and 384 kHz |
|---|---|---|
| session parse, typed compile, canonical write | `session/tests/sample_rate_tiers.rs::extended_and_unrelated_engine_rates_reject_with_one_stable_diagnostic` (existing) | one `sample_rate.unsupported_at_launch` at `$.sample_rate_hz` |
| shared host preparation | `host-core/tests/prepare.rs::session_validation_owns_the_launch_rate_set` (existing, 176.4 and 192) | `PrepareRejection::Session`, same diagnostic |
| C ABI prepare | `capi::ffi::tests::extended_rate_session_and_chunks_are_refused_typed` (new) | `miso_engine_v1_compile_session` returns `RESULT_COMPILE_REJECTED`, publishes no handle, writes and `last_error`s `sample_rate.unsupported_at_launch\t$.sample_rate_hz\n`; a chunk at the rate into a 48 kHz plan refuses with `source.rate.mismatch` |
| browser boot (Rust) | `host_web::tests::extended_rates_refuse_typed_at_browser_boot` (new) | `AudioWorkletEngineHost::boot` and the raw `miso_engine_web_v1_boot` export return `RESULT_REFUSED_DOCUMENT` with the session diagnostic, with and without `require_sample_rate_hz` set to the rate; a 48 kHz document on a context at the rate refuses `RESULT_REPREPARE_REQUIRED`, `host.session.shape` |
| browser boot (SDK, shipped module) | `sdk/test/browser-defaults-evals.mjs` "browser boot refuses the removed extended research rates typed, before any AudioContext" (new) | `scratchBootInWorker` rejects with `MisoEngineError`, phase `boot`, code `refusedDocument`, `sample_rate.unsupported_at_launch` at `$.sample_rate_hz` |
| render plan | `engine::realtime::tests::extended_and_unrelated_rates_reject_before_plan_publication` (existing, literals now inline) | `RenderError::UnsupportedRate` |

The C ABI already reported `exact_launch_rate_mask == 0x0f`; unchanged.

### Test inventory (`cargo test --workspace --all-features -- --list`)

Base 2,331, change 2,332. The whole diff:

```
- builtins/test:contract::compatibility_fallback_is_limited_to_the_exact_extended_rate_tier
+ builtins/test:contract::extended_and_unrelated_rates_have_no_cutoff_domain_and_refuse_preparation
+ capi/rlib:capi::ffi::tests::extended_rate_session_and_chunks_are_refused_typed
- conformance/test:effect_contract::descriptor_requires_launch_rows_and_accepts_optional_extended_rows
+ conformance/test:effect_contract::descriptor_requires_launch_rows_and_refuses_extended_rows
- conformance/test:effect_contract::extended_rate_failures_are_reported_but_do_not_fail_launch_gates
- conformance/test:fixtures::planar_blocks_group_launch_gates_and_extended_compatibility_inputs
+ conformance/test:fixtures::planar_blocks_and_fixtures_accept_only_launch_rates
- engine/lib:engine::tests::sample_rate_tiers_are_exact_sorted_disjoint_and_classified
+ engine/lib:engine::tests::launch_sample_rates_are_exact_and_the_only_accepted_rates
+ host-web/rlib:host_web::tests::extended_rates_refuse_typed_at_browser_boot
```

- Flipped (accept to refuse), renamed because the old names stated the removed claim:
  the builtins cutoff test (extended rates now have no domain and `BuiltinChain::new` refuses
  `FilterCutoff`); the conformance descriptor test (every non-empty subset of extended rows refuses
  with exactly `[Quality at qualities]`; the launch-only, missing-row, draft, duplicate, unordered
  and 192,001 Hz cases are kept, the unordered case now swapping two launch rows); the conformance
  fixtures test (extended `PlanarBlock`, `encode` and `parse` all refuse); the engine tier test
  (launch set exact, extended and unrelated rates not launch rates).
- Removed: `extended_rate_failures_are_reported_but_do_not_fail_launch_gates`. Its claim, that an
  extended-tier failure does not fail the launch gates, has no subject once the tier is gone;
  `validate_descriptor` refuses such a row before the harness runs (flipped test above).
- `correct_mock_passes_every_enabled_conformance_gate` keeps its launch assertions (8 prepared
  configurations, at least 800 process calls) and loses only the extended ones.
- Kept: launch-rate descriptor acceptance (every production effect's `validate_descriptor`
  test, the conformance launch-only case) and session rate refusal. The five builtins response and
  two stage tests run at the four launch rates, as before.

### Gates

- **Native and wasm build.** `cargo check --locked --workspace --all-targets --all-features` and
  `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: clean.
  `cargo fmt --all --check`: clean. `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked
  --target wasm32-unknown-unknown -p host-web -p dsp-reference -p conformance`: pass.
  `cargo check --locked --target aarch64-apple-ios` and `--target aarch64-linux-android` of
  `capi host-core session engine effect-contract builtins builtins-compiler effect-compiler
  graph-compiler rack-compiler`: pass.
- **Tests.** `cargo test --locked --all-targets -p engine -p effect-contract -p builtins
  -p conformance -p effect-compiler -p capi -p host-web -p host-core -p session --features
  builtins/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,engine/realtime-audit`:
  784 passed, 0 failed, 8 ignored in dev, and the same in release. The eight production effects'
  `conformance` tests (harness consumers): 11 passed in dev and in release.
- **Console digests.** `cargo test --locked --release -p console-workload --test gain_pan_profile
  -- --ignored --exact digests --nocapture` on base and change: the 18 digest rows are
  byte-identical; the outputs differ only in the `finished in` timing line.
- **Wasm gates.** `bash scripts/run-wasm-gates.sh`: ok (native, wasm scalar, wasm simd128, V8 EQ
  loops; Node v22.23.2).
- **SDK.** `check-sdk-headless.sh` against this commit's module: 285/285 evals pass, including the
  new one. `check-sdk-generated.sh`: current (no SDK table or error changed).
- **C ABI.** `check-capi-abi.sh` (shared and static linkage) and `check-capi-abi.sh --self-test`:
  ok.
- **CI routing and fixtures.** `cargo run --locked -p conformance --example conformance_fixtures --
  --check`: pass with the 7-file corpus. `check-ci-path-routing.py`, `test-ci-path-routing.py`:
  pass. `check-effect-runtime-fixtures.sh` and its mutation test: ok (4 files).
  `check-effect-contract.sh target/release/bench`: ok, record
  `{"launch_prepared_configurations":8,"launch_process_calls":4608,"launch_failed_gates":0}`.
- **Policy.** 60 script runs, all pass: every static policy invocation of `qualification.yml`'s
  lint job (including `check-stem-store-v1.mjs` and `check-parametric-eq-render-contract.sh`) and
  `check-sdk-deletions.py` with its self-test; Python ones with `python3 -B`.

### AudioWorklet artifact

Not re-pinned (the batch boundary does that). `build-web-audioworklet.sh --module-only` on the same
machine: base `eca8779d` builds `f58b55e9…` (3,485,448 bytes); this commit builds `52ee8595…`
(3,485,631 bytes), **183 bytes larger**: code section +260, name section -76, one function fewer.
`wasm-objdump -d`, compared per function with crate-hash disambiguators normalized (base was built
from another checkout path), shows exactly:

- removed: `builtins::filter_control::validate_input_filter_pair`. With the extended branch gone
  from the `validate_builtin_filter_cutoff` it inlines, it is small enough to be inlined itself:
  base called it from 9 sites, the change from none.
- grew, from absorbing that inlined body: `host_web::compile_ready` (6,295 to 6,538 instructions),
  `host_core::…::InputFilterPreparer::prepare` (791 to 1,053),
  `host_core::…::apply_input_filter_edit` (434 to 603), `builtins::…::prepare_input_filter_pair`
  (435 to 566).
- shrank, from losing the extended branch: `builtins::prepare_sections` (1,119 to 915) and
  `builtins_compiler::parameter_diagnostic` (1,230 to 1,019), each losing four sets of
  `i32.const 176400/192000/352800/384000` compares and the `f64` open-Nyquist compare;
  `effect_contract::validate_descriptor` (5,314 to 5,284), losing the four extended-rate compares.

Every other function is identical. The module held 33 `i32.const` 176400, 352800 or 384000
operands before and none after.

## Sol verdict, attempt 1

**PASS.**

Verified on a scratch merge of `d10edc8e` into `codex/batch-slim-2` at `8b3c0794` (#1035 and the
env-count repin included). Base for every comparison is `8b3c0794`.

### Merge

- **One textual conflict:** `AGENTS.md`, in the Sources paragraph. #1035 rewrote its first
  sentences and #1036 its rate clause. Resolution: keep #1035's host-decode sentences and #1036's
  rate clause.
- **No semantic conflict.** The merged tree builds, lints and tests green, and no removed
  identifier survives outside historical specs, handoffs and `artifacts/`.
- `git merge-tree` of `codex/1049-shrink-dominated-slow-tests` onto the merge is clean.

### Findings, by severity

No defect. An extended rate is refused on every host path, every validator mutation is caught,
launch-rate output is unchanged and every gate passes.

1. **Low: stale "extended" test names.** The helper `launch_and_extended_compatibility_rates` in
   `crates/builtins/tests/response.rs` and four test names built on it still claim
   extended-rate coverage that no longer exists. So do
   `protocol::model::tests::temporary_extended_rate_is_permitted_when_final_candidate_is_launch_rate`
   and the `"extended rate refuses"` expect messages in `host-core` spectrum.
   - The protocol test is rate-agnostic transaction semantics: any intermediate rate is permitted
     and only the final candidate is checked.
   - #1049's amendment 3 does not carry the rename. Add the rename to #1049 or to a successor
     issue, so that it is not lost.
2. **Nit: digest row count.** The evidence says the digests have 18 rows. `gain_pan_profile
   digests` prints 17, and `console-workload` is unchanged since `eca8779d`. The rows are
   byte-identical either way.

### Extended rates in accepted sets

The merged tree was grepped for `176[_,]?400`, `192[_,]?000`, `352[_,]?800`, `384[_,]?000`,
`176.4`, `352.8`, `192k` and `384k`, and for every rate set. Every survivor is one of three kinds:

- **A refusal test:**
  - engine, realtime, session, protocol and controller;
  - host-core prepare and spectrum;
  - capi, host-web and SDK;
  - conformance;
  - builtins `contract.rs` and `filter_response.rs`;
  - parametric-eq response;
  - effect-compiler owner;
  - native-pcm-runner.
- **A documented non-acceptance:**
  - AGENTS.md;
  - `EFFECT_CONTRACT_V1.md`, `SESSION_SCHEMA_V1.md` and `IMPLEMENTATION_PLAN.md`;
  - the dsp-research and fixture READMEs.
- **An unrelated number:** frame counts, probe frequencies, byte bounds or vendored constants.

The accepted sets hold the launch rates only:

- `docs/session-v1.schema.json`: `sample_rate_hz` enum;
- the C ABI header: `EXACT_LAUNCH_RATE_MASK = 15`;
- `sdk/src`: `SessionSampleRateHz`;
- `fixtures/builtins/v1`: `rate_hz` values.

### Refusal on every host path

Each validator was mutated to also admit 192 kHz, and at least one test went red for each:

| mutation | tests that went red |
|---|---|
| `validate_descriptor` | `descriptor_requires_launch_rows_and_refuses_extended_rows` |
| `validate_builtin_filter_cutoff` | `extended_and_unrelated_rates_have_no_cutoff_domain_and_refuse_preparation` |
| `session::validate` | capi `extended_rate_session_and_chunks_are_refused_typed`; host-web `extended_rates_refuse_typed_at_browser_boot`; host-core `session_validation_owns_the_launch_rate_set`; session, protocol, controller and native-pcm-runner rate tests |
| `session::validate`, in a module rebuilt with it | SDK eval "browser boot refuses the removed extended research rates" (284/285) |
| `engine::is_launch_sample_rate` | 10 tests, including the engine tier, realtime, conformance fixtures, capi and host-web |
| `PlanarBlock::try_new`, `PcmFixture::parse`, `PcmFixture::encode` (each mutated separately) | `planar_blocks_and_fixtures_accept_only_launch_rates` |
| `RenderEnvelope::validate` | `realtime::tests::extended_and_unrelated_rates_reject_before_plan_publication` |

### Launch rates

- **Console digests:** `gain_pan_profile digests` prints 17 rows, byte-identical on base and merge.
- **All four launch rates prepare and render on every path. Tests that pass on the merge:**
  - `realtime::tests::launch_sample_rates_prepare_and_render`;
  - `session` `launch_rates_parse_compile_and_canonicalize`;
  - capi `direct_and_c_render_match_one_and_ten_tracks_across_launch_rates`;
  - host-web `meter_spans_cover_all_launch_rates_and_a_nine_track_tail`;
  - SDK boot evals;
  - the conformance mock (8 launch configurations) and all 8 production effects' conformance
    tests.
- **Builtins cutoff:**
  - The new function is equivalent to the old one for `Some(maximum)`.
  - `representable_cutoff_domain_is_shared_by_descriptors_and_preparation` is unchanged and passes
    on base and merge.
  - Mutating `<= maximum_hz` to `<` turns it and `descriptor_domains_are_exhaustive_at_launch_rates`
    red.

### Module

- **Size:** base 3,485,448 bytes, merge 3,485,631, so +183.
- **Function-level `wasm-objdump -d` diff,** with crate hashes normalized:
  - `builtins::filter_control::validate_input_filter_pair` is removed. Base had 9 call sites in
    the four functions that grew; the merge has none, and each of those functions gains the
    launch-rate compares that the removed function carried.
  - `prepare_sections`, `parameter_diagnostic` and `validate_descriptor` shrank.
  - The 44 `i32.const` operands for the extended rates drop to 0.
  - Every other function is identical.
- **Gates:** the callgraph gates (render, meter_poll, command_submit, kernel-shape), the V8 spill
  gate and its self-test, and the boot budget all pass.

### Test list

`cargo test --workspace --all-features -- --list` goes from 2,272 to 2,273. The diff matches the
evidence's inventory exactly: 4 tests flipped and renamed, 1 removed, 2 added. No launch-rate claim
is lost.

### Gates on the merge

All pass:

- `cargo check` and `cargo clippy -D warnings` (`--workspace --all-targets --all-features`), and
  `cargo fmt --check`;
- `cargo check` for aarch64-apple-ios and aarch64-linux-android (capi and the product crates), and
  for wasm `simd128` (host-web, dsp-reference, conformance);
- tests of the affected crates, dev and release: 1,153 passed, 0 failed;
- the conformance tests of the effect crates: 11 passed;
- `run-wasm-gates.sh`;
- SDK: 285/285 evals; `check-sdk-generated`, `check-sdk-types` and `check-sdk-deletions` with its
  self-test;
- `check-capi-abi.sh` and its self-test;
- `conformance_fixtures --check`;
- every `scripts/check-*.sh`, `check-*.py` and `check-*.mjs`, with the argument-requiring ones run
  as `--self-test` or on the built module;
- `test-ci-path-routing.py` and `test-effect-runtime-fixtures.sh`.

**Expected red: the AudioWorklet pin.** `check-web-audioworklet.sh` and `check-sdk-headless.sh`
run without an argument fail only there. The pin is already red on base: base builds `e34073a2…`
against the pin `f7bd75ca…`. The batch boundary re-pins it.
