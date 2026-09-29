# Remove host-core's orphaned observation-demand API

Successor of #1064 (Sol verdict attempt 2, finding 2). Owner decision 9 (`docs/rulings/engine-footprint-2026-09-29.md`) keeps the ordinary observation path and removes the unadopted protected path. #1064 removed the browser adapter's side; host-web was the only non-test consumer of host-core's observation-demand API, which is now orphaned in production. The compiler does not flag it only because the items are `pub`.

## Smallest closable slice

Remove, after proving non-use by making them crate-private and compiling every target:
- `crates/host-core/src/observation_demand.rs` (about 2,500 lines);
- the four `prepare_host_runtime_with_observation_demand*` functions;
- the controlled-spectrum machinery in `spectrum.rs` (`ControlledSpectrumCandidate`, `ControlledSpectrumSlot`, `reset_for_controlled_stage`, `retire_controlled_after_receipt`) if nothing on the ordinary path uses it;
- `crates/host-core/tests/observation_demand.rs` (25 tests), which is the only remaining caller.

Keep everything the ordinary observation path, capi or host-web uses.

## Gates

1. `cargo check --workspace --all-targets --all-features`; the wasm `simd128` check; the aarch64 iOS/Android checks; clippy and fmt.
2. host-core, host-web and capi tests; the SDK tests; `check-web-audioworklet.sh`; console digests unchanged; the shipped module changes only by removed code.
3. Every policy script.

## Attempt 1 evidence

Terra, 2026-09-29. Branch `codex/1076-remove-observation-demand` from #1064's verified `9c7b3f64`;
implementation commit `3b073997`. Rust 1.97.1, Node 22.23.2.

### Proof of non-use

- The candidates were made crate-private: `observation_demand` became a private module with its
  re-exports gone; the four `prepare_host_runtime_with_observation_demand*` functions and
  `HostPrepareReport::observation_demand_resources` became `pub(crate)`; host-core's
  `GraphObservationActivationConfig` re-export (added with #825 for
  `HostObservationPreparation::activation`) was removed. `tests/observation_demand.rs` was deleted,
  and the module's own unit test was pointed at the private paths.
- Every target still compiled:
  - `cargo check --workspace --all-targets --all-features`;
  - host-web `--all-targets` on wasm32 `simd128`;
  - the 25 product crates, `--all-targets --all-features`, on `aarch64-apple-ios` and
    `aarch64-linux-android`.
- The only warnings were host-core's own: 44 dead-code warnings, identical on every target and
  with or without features. 32 are in `observation_demand.rs`, 4 are the prepare functions, and 8
  are the controlled spectrum machinery in `spectrum.rs` (`ControlledSpectrumCandidate`,
  `ControlledSpectrumSlot`, `reset_for_controlled_stage`, `retire_controlled_after_receipt` and
  the paired collection).

### What was removed

- `observation_demand.rs`, 2,501 lines, and its 15 unit tests.
- In `prepare.rs`:
  - the four entries;
  - the `observation_demand` parameter. It was `None` from every remaining caller, so its
    branches were removed: controlled builtins, controlled spectrum resources and bindings,
    activation binding, the owner and its budget, and `observation_diagnostics`;
  - `preflight_spectrum_handles`;
  - the `_with_hop` wrapper. Its hop fed only the controlled cadence, so the body is now
    `prepare_host_runtime_with_console_policy_and_spectrum`;
  - the report field, which is always `ZERO` on the ordinary path.
- In `spectrum.rs`:
  - the controlled slots, candidate, descriptor and paired collection;
  - `prepare_controlled_capture_collection`, `controlled_spectrum_capture_collection_resources`
    and `CONTROLLED_SLOTS_PER_ENTRY`;
  - the two controlled methods on `SpectrumCapture`;
  - the `CaptureBindingPolicy` enum, whose only remaining variant was `Permanent`;
  - the two unit tests of the paired collection.
- `tests/observation_demand.rs`: 2,064 lines, 26 tests (25 by default and 1 under
  `test-support`).
- One doc reference in `tools/console-workload` was renamed.
- Totals: 6 files, +30 / −5,833 lines. host-core's `src` goes from 14,815 to 11,076 lines and its
  `tests` from 8,737 to 6,673.

### Why the ordinary path is unchanged

- Every removed branch needed `Some` demand.
- The two checks that followed binding (`host.graph.resource.limit` and `host.resource.limit`)
  added zero with no demand. Each is implied by an identical earlier check with the same code, so
  neither could fire.

### Kept deliberately

Some code is now reachable only through graph's controlled activation, or only from unit tests:
- `SpectrumCaptureObserver`'s `activation_changed` hook and the records' `observation_generation`;
- the `maximum_pops` bound of `try_read_record` and `try_read_continuous_record`. Production passes
  `None`.

These are the observer half of the controlled-activation API that graph and builtins-compiler
still expose. That API is the successor finding below.

### Shipped module

**Size and digest.** The module goes from 3,322,821 to 3,299,125 B (−23,696 B), and its sha256
from `5afb47e3…` to `6ee91a0d…`.

| section | change |
|---|---|
| code | −17,693 B |
| data | −1,104 B |
| name | −4,814 B |
| functions | 2,557 → 2,524 |

**Unchanged surface.**
- `initial=18` pages.
- The 116 function exports and `memory`, with identical signatures.
- The six other delivery files are byte-identical.

**Only removed code moved.** I compared the demangled functions, with closure numbering
normalized and the renamed body mapped:
- 2,325 are identical and none is new.
- 25 are gone. All are demand or controlled code, their monomorphizations and drop glue, and
  graph's and builtins-compiler's activation binding.
- 16 changed:
  - the rest shrink;
  - the three growths are inlinings into a function that became the single caller:
    - `into_bound_with_source_set` grows +12,288 B. It absorbs `prepare_binding_validation`
      (9,808 B) and `restore_graph_bind_failure`;
    - `SpectrumCapture::try_read_continuous` absorbs `try_read_continuous_record`;
    - `prepare_capture_with_handle` absorbs `validate_capture_request`.
- The render closure is unchanged at 8/5, `meter_poll` at 9, `command_submit` at 36 and the kernel
  shape at 15. The spectrum observer's `capture` and `observe_resident` keep their exact sizes.

**Browser resources.** `bridgeMetadataBytes` goes from 1,146,259 to 1,146,179 and
`bridgeRetainedBytes` from 1,166,768 to 1,166,688. The −80 B is the removed report field, whose
host shell holds a `PreparedHost` inline. Every other row, the three PCM digests and the timelines
are identical to base.

### Gates, all green

| gate | result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo check --workspace --all-targets --all-features` | pass |
| `cargo clippy --workspace --all-targets --all-features -D warnings` | pass |
| `cargo doc -D warnings` | pass |
| `check-cross-targets.sh` | PASS: aarch64 iOS and Android check and clippy, wasm `simd128` rows, refusal rows, and the #1018 memset rows unchanged |
| wasm `simd128` `check` of host-web (`--all-targets --all-features`), target-smoke, protocol, dsp-reference and conformance | pass |
| wasm `simd128` `clippy -D warnings` of host-web | pass |
| host-core, host-web and capi tests | 316 with all features and 313 by default; 0 failed |
| parameter-metadata and audit tests | 48 passed |
| `console-workload` release tests (the console digests) | 62 passed, 0 failed |
| SDK after `npm ci` | `check-sdk-generated`, `check-sdk-deletions`, `check-sdk-types`, `sdk-package.sh check` pass; headless 284/284 |
| `check-web-audioworklet.sh` | pass, with and without metadata regeneration |
| `test-web-audioworklet.sh` | pass |
| `check-browser-expected-resources.py --artifacts` | pass |
| `check-browser-expected-resources.py --self-test` | pass: 32 red mutations |
| V8 spill gate | pass |
| `check-scalar-oracle-absent.py --wasm` and `--native` | pass |
| `check-capi-abi.sh` | pass: shared and static |
| `check-console-fixtures.sh` with the release `session_validator` | pass |
| realtime audit probes and the 1,000,000-block syscall trace | pass |
| policy, lint, docs, route and gate-self-test commands from `qualification.yml` (61, Python under `python3 -B`) | 61/61 pass; the router gives `route=full` |

One pre-existing failure, not a gate: wasm clippy of host-web with `--all-targets` fails on
`drop_non_drop` at `hosts/host-web/src/tests.rs:5332` (`drop(file)`). It fails identically at
`9c7b3f64` and no CI row runs it; the lib row passes.

### `cargo test --workspace --all-features -- --list`

2,127 → 2,084 tests: −43, +0.
- 26 from `tests/observation_demand.rs`.
- 15 from `observation_demand` unit tests.
- 2 from `spectrum::tests::controlled_*`.
- The two `PreparedHost` doctests only move lines, 388/395 → 376/383.

### Finding for a successor

graph's and builtins-compiler's controlled observation activation now has no production caller.
It lost its last one here. The items:
- `into_bound_with_source_set_and_observation_activation`;
- `bind_with_source_set_and_observation_activation`;
- `prepare_controlled_session_builtins_*`;
- `GraphNodeObserverBinding::controlled`;
- `GraphObservationController`.

They are already absent from the shipped module. Because they are `pub`, the compiler does not
flag them. Together with the observer hooks listed under "Kept deliberately", they need an issue
that removes them or rules to keep them.

## Sol verdict, attempt 1

**PASS.** Sol, 2026-09-29. Verified on a scratch merge of `9f3ccc2a` into the batch head
`codex/batch-slim-5` at `c04bc8ea` (#1064, #1065, #1045 and #1051). Rust 1.97.1, Node 22.23.2.

### The merge

It is clean, with no textual conflict and no semantic conflict.
- The batch's only new host-core caller is #1051's `tests/randomized.rs`. It uses
  `prepare_host_runtime_with_console` and `prepare_host_runtime_between_render_calls`, which are
  both kept, and it passes on the merge.
- The batch's graph and builtins-compiler edits delete tests only.
- The router gives `route=full` and `self_tests=[]` over `c04bc8ea..merge`.
- The modules:
  - Base (`c04bc8ea`): 3,322,821 B, `0605b376…`.
  - Merge: 3,299,125 B, `303a0f3f…`. That is −23,696 B, the branch's figure.
  - The six other delivery files are byte-identical.
  - The 116 function exports have identical signatures (checked with `wasm-objdump`), and both
    modules declare `initial=18`.

### Checks

1. **The ordinary path behaves as before.**
   - *Source.* The deleted branches all needed `Some` demand. The two resource checks after
     binding cannot fire:
     - The earlier `host.graph.resource.limit` check was guarded by `observation_demand.is_none()`,
       which is always true now, and its operand is identical.
     - The earlier `host.resource.limit` check tests the same `largest.max(session)` expression,
       plus the builtin-retained clause.
     - `prepare_capture_with_handle` always bound `Permanent` on the ordinary path.
     - The explicit hop never reached host-core on the ordinary path: host-web applies it at stream
       start.
   - *Differential run.* I drove the base and merge modules with identical seeded call sequences
     and compared them record by record.
     - Size: 192 scenarios (8 seeds × 24), 578,312 calls, all 116 exports, 1,818 successful boots.
     - Boots: 12 sessions from 1 to 64 tracks. Plain, single-target, collection and explicit-hop
       boots, plus refused options, documents, hops and budgets, and handle 0, stale and
       double-dispose calls.
     - Exercised to `ok`: meter lease and poll, `observation_read`, and spectrum arm, read,
       select, cancel and stream start, read, select, reset and stop.
     - Compared: every return code and a digest of the staging each call can change (status,
       command report, meter header and frame, output PCM, observation, spectrum result, capture
       and stream metadata, response and live-response results).
     - Result: **0 mismatches.**
     - The only differences:
       - `bridgeMetadataBytes` and `bridgeRetainedBytes` are −80 B in all 2,001 resource reports.
       - 54 budget refusals (`host.budget.retained_projection` and `retained_exact`) print a byte
         count 80 B lower against the same budget.
       - Linear memory size (finding 3).
   - *Tests.*
     - The SDK headless evals pass 284/284 on both base and merge.
     - All three browsers pass in CI mode (`--check-matrix --self-test-mutations`) on both:
       Chromium 151, Firefox 153 and WebKit 26.5.
     - The console digests (`gain_pan_profile digests`) are identical on base and merge.
2. **Nothing live was deleted.**
   - Every target compiles with the items gone. No removed name appears in capi, host-web, the
     SDK, tools, scripts, CI or tests.
   - The remaining mentions are historical, in specs, `docs/handoffs/` and
     `docs/rulings/825-…`. The `ObservationReadError`, `PublicationKind` and `is_closed` hits are
     host-web's or graph's own items.
   - `HostPrepareReport::observation_demand_resources` is on no wire, ABI or SDK surface. It was a
     Rust-only field, `ZERO` on every ordinary preparation, and filled only by the protected path.
     Its only reader was `tests/observation_demand.rs`.
   - Why 80 B:
     - `HostObservationResources` is 80 B on wasm32 and on native: `Option<GraphObservationActivationResources>`
       takes 48 B, since it has no niche, and four `u64` take 32 B.
     - The report sits inline in `PreparedHost`, and so in `AudioWorkletEngineHost`, whose
       `size_of` feeds both bridge rows.
     - #820 recorded the same 80 B being added.
   - #1060's check passes on the merge artifact: 1,146,179 of 1,159,360 and 1,166,688 of
     1,181,888. Its self-test is green at every ceiling, with 32 red mutations.
3. **The successor is real.** I proved it with the crate-private method in a scratch copy of the
   merge.
   - I made builtins-compiler's four entries `pub(crate)`:
     - `into_bound_with_observation_activation`;
     - `into_bound_with_source_set_and_observation_activation`;
     - `prepare_controlled_session_builtins_with_console`;
     - `prepare_controlled_session_builtins_between_render_calls`.

     `cargo check --workspace --lib --bins --all-features` compiles, and rustc reports all four as
     dead. The same holds for host-web's lib on wasm32 `simd128` and for the 25 product crates on
     `aarch64-apple-ios` and `aarch64-linux-android`.
   - I then put those four and `MeterBindingPolicy::Controlled` under `cfg(test)`, and made
     graph's `bind_with_observation_activation`, `bind_with_source_set_and_observation_activation`
     and `GraphNodeObserverBinding::controlled` `pub(crate)`. Every production target still
     compiles, and all three are reported dead. Only builtins-compiler's `test-support` helpers
     still name `controlled`.
   - Proposed scope, two lines:
     > Remove graph's and builtins-compiler's controlled observation activation, orphaned in production since #1076: the bind/into-bound activation entries, `GraphNodeObserverBinding::controlled`, `prepare_controlled_session_builtins_*`, `MeterBindingPolicy::Controlled`, graph's `observation_activation` module with the executor's per-block activation hook, and host-core's observer half (`activation_changed`, `observation_generation`, the `maximum_pops` bound).
     > Gate: crate-private-then-compile on every target, the ordinary-path differential and console digests unchanged, and the shipped module changing only by removed code (about 34 activation functions, including `Runtime::begin_observation_block`'s `apply_candidate`).
4. **Test lists** (workspace, all features, on the merge): 2,114 → 2,071.
   - 45 removed and 2 added. The two added are `PreparedHost` doctests that only moved lines.
   - That is −43 net: the 26 integration tests, the 15 `observation_demand` unit tests and the 2
     `spectrum::tests::controlled_*`.

### Gates on the merge

All pass:
- `cargo fmt --check`.
- `cargo check` and `clippy -D warnings`, workspace, `--all-targets --all-features`.
- `cargo doc -D warnings`.
- `check-cross-targets.sh`, covering aarch64 iOS and Android and the wasm rows.
- wasm `simd128`:
  - `check` for host-web (`--all-targets --all-features`), target-smoke, protocol, dsp-reference
    and conformance;
  - `clippy -D warnings` for host-web's lib.
- The workspace release check under the unwind override.
- Tests:
  - `test-debug-a`'s full set with its features: 1,112 passed.
  - host-core, host-web and capi by default: 314 passed.
  - The audit, bench and console-workload release tests, and the console digests.
- The SDK: `npm ci`, `check-sdk-generated`, deletions, types, headless and `sdk-package.sh check`.
- `check-web-audioworklet.sh`, with and without metadata regeneration: render 8/5, `meter_poll` 9,
  `command_submit` 36, kernel shape 15.
- `test-web-audioworklet.sh`.
- The V8 spill gate and its self-test.
- `web-audioworklet-identity.py --self-test`.
- `check-browser-expected-resources.py --artifacts` and `--self-test`.
- `check-scalar-oracle-absent.py`, `--wasm` and `--native`.
- `check-capi-abi.sh` and its self-test.
- `check-console-fixtures.sh` against the release `session_validator`.
- The realtime audit probes and the 1,000,000-block syscall trace.
- The routing tests.
- Every hermetic policy step of the route, docs-gates, lint, release-shape and gate-self-tests
  jobs: 56/56, with Python under `python3 -B`.

### Findings, by severity

No defect.

1. **Low: "already absent from the shipped module" is only half true.**
   - The public activation entries are gone from the module.
   - The machinery they gate still ships: 34 functions of graph's `observation_activation`
     (`prepare_activation`, `preflight_observation_activation`, snapshot storage and rings, and
     `RealtimeObservationActivation::apply_candidate`, reached from the render path's
     `Runtime::begin_observation_block`), and the drop glue of `Option<GraphObservationController>`.
   - This strengthens the successor: removing it also shrinks the module and the render path.
2. **Low, optional (as #1064's finding 3): the #1060 bridge ceilings were not lowered.**
   - The rows fell 80 B further. The checker passes.
   - #1060 raises ceilings only on growth.
3. **Info: linear memory differs in 8 of 192 differential runs.**
   - The final size differs by ±17 pages or −1 page, in both directions.
   - I traced four runs to `response_close`: `ResponseStaging::reset` builds the new 1 MiB staging
     before it drops the old one, so page growth depends on heap layout, which the smaller module
     shifts.
   - No return code or staged byte differs.
4. **Info: the first Firefox run on the merge failed `sdk-spectrum-hop`.**
   - That gate counts real-time publications.
   - It did not reproduce: Firefox passed 3/3 later runs on the merge and 3/3 on base.
5. **Info, pre-existing:** wasm clippy of host-web with `--all-targets` fails on `drop_non_drop` at
   `hosts/host-web/src/tests.rs:5332`, on base and merge alike. No CI row runs it.
6. **Info: coverage limit.** In the differential, `eq_target_prepare`, `input_filters_prepare`,
   `response_query`, `prepared_command_submit` and `eq_target_config_copy` returned only refusals.
   Their codes still matched, and none of them reaches the changed code.
