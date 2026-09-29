# Remove graph's and builtins-compiler's orphaned controlled observation activation

Successor of #1076 (Sol verdict attempt 1, finding 3). Owner decision 9 (`docs/rulings/engine-footprint-2026-09-29.md`) keeps the ordinary observation path and removes the unadopted protected path. After #1064 and #1076, the controlled observation-activation machinery below them has no production caller (proved by making each entry crate-private and compiling every target: the workspace, host-web on wasm32 `simd128`, and the 25 product crates on `aarch64-apple-ios` and `aarch64-linux-android`).

## Smallest closable slice

Remove: the bind/into-bound activation entries (`bind_with_observation_activation`, `bind_with_source_set_and_observation_activation`, `into_bound_with_observation_activation`, `into_bound_with_source_set_and_observation_activation`), `GraphNodeObserverBinding::controlled`, `prepare_controlled_session_builtins_*`, `MeterBindingPolicy::Controlled`, graph's `observation_activation` module with the executor's per-block activation hook (`Runtime::begin_observation_block`'s `apply_candidate`), and host-core's observer half (`activation_changed`, `observation_generation`, the `maximum_pops` bound), plus builtins-compiler test-support helpers that exist only for them.

## Gates

1. Crate-private-then-compile on every target proves non-use before deletion.
2. The ordinary-path differential (all shipped exports, seeded boots; #1076's method) shows zero mismatches; console digests unchanged; the three browsers pass in CI mode.
3. The shipped module changes only by removed code (about 34 activation functions); the render closure loses only the removed hook; render stays allocation-, lock- and syscall-free.
4. clippy, fmt, the affected tests in dev and release, every policy script.

## Attempt 1 evidence

Terra, 2026-09-29. Branch `codex/1080-remove-controlled-activation` from the batch `codex/batch-slim-5`
at `fa48dbbb` (#1064 and #1076 included). Implementation `ebc657d0`; fixture repins `5147d675` and
`ca8c361c`. Rust 1.97.1, Node 22.23.2.

### Proof of non-use

Before deleting anything, the candidates were made unreachable from outside their crate:
- graph: the six `observation_activation` re-exports and `test_only_observation_transition_entries`
  became `pub(crate)`, as did `bind_with_observation_activation`,
  `bind_with_source_set_and_observation_activation` and `GraphNodeObserverBinding::controlled`.
- builtins-compiler: the two `into_bound_*_observation_activation` entries, the two
  `prepare_controlled_session_builtins_*` entries, `MeterBindingPolicy::Controlled` (with its
  validation and binding arms), `MeterObserver::activation_changed` and the activation imports went
  under `cfg(test)`. The one test-support fixture that named `controlled`
  (`source_bind_fixture_with_observer`, compiled into the lib under `test-support`) was gated the
  same way.
- builtins: `restart_observation`, `observation_generation()` and `MeterSnapshot::observation_generation`
  became `pub(crate)`.

Every production target still compiled (`--lib --bins --all-features`):
- the workspace on x86-64;
- host-web on wasm32 `simd128`;
- the 25 product crates on `aarch64-apple-ios` and `aarch64-linux-android`.

The only warnings were the same 14 on every target: the graph entries, `controlled`, the unused
re-exports and the `observation_activation` internals, and builtins' `restart_observation` and
`observation_generation`. host-core's `maximum_pops` and record `observation_generation` were
already `pub(crate)`; production passed `None` and read the generation only as `let _ = …`. The
trait hook `activation_changed` had one caller, `Runtime::begin_observation_block`. After the
deletion, every target compiles with `--all-targets --all-features`.

### What was removed

- graph (`src` 26,380 → 23,469 lines, `tests` 2,899 → 2,137):
  - `observation_activation.rs` (1,233 lines, 6 unit tests);
  - the two activation bind entries, the `Option<GraphObservationController>` in the bind result,
    and `preflight_observation_activation` with its error-code map and `PreparedObservationActivation`;
  - `GraphNodeObserverBinding::controlled`, its flag and `is_controlled`;
  - `GraphRuntimeObserver::activation_changed`;
  - the runtime's activation state, dispatch cursor and failure flag, `begin_observation_block`
    (its `apply_candidate` hook), `has_observation_activation`, `has_active_observation`,
    `observe_active_unit`, `observe_active_entry`, `observer_at_entry`, `observe_one`, and the
    activation branches of `invalidate_observers` and `invalidate_observers_after_failure`;
  - the `*WithoutObservationActivation` layout witnesses and the metadata estimate's
    `observation_runtime_state_bytes`, the inline activation state every graph was charged for.
    `Runtime::new_with_observation_activation` is now `new_with_output_unit`.
- builtins-compiler (`src` 13,363 → 12,538): the four entries, `MeterBindingPolicy` and the
  request seal's `binding_policy`, the controlled request validation
  (`builtin.meter.controlled_*`), the meter's activation hook, and the fixture arm.
- host-core (`src` 11,076 → 10,780): `SpectrumCaptureObserver::activation_changed` and its two scalar
  resets, `observation_generation` in the observer, buffers and records, and the `maximum_pops`
  parameter of `try_read_record` and `try_read_continuous_record`.
- Tests: the tests of the removed path. The ordinary arms of mixed tests stay, as does the
  selected-meter delivery test (now `selected_meter_preparation_keeps_control_delivery_distinct`,
  comparing the concurrent and between-render-calls permanent entries). The gap-recovery and
  stale-pop spectrum tests now read through the public `try_read_continuous`. Their fixture queue
  has one slot, so the old one-pop bound was the entry population.
- Totals: 14 files, +183 / −5,132.

### Why the ordinary path is unchanged

- With no controlled observer and no configuration, `preflight_observation_activation` returned
  `Ok(None)` before any check. Render took `observe_unit` whenever `observation_activation` was
  `None`, which every production bind produced. The remaining code is that branch, unchanged.
- The deleted invalidation branches and the failure flag were reachable only with activation state.
- The continuous-spectrum window check that mentioned controlled activation stays; only its
  comment changed.
- Byte counts moved only by removed fields:
  - `GraphNodeObserverBinding` is 88 → 80 B on native, which reverses #816.
  - The builtins-compiler mutation transcript moves only through that size: restoring an 8-byte
    field restores the old hash.
  - `fixtures/builtins/v1/resources.jsonl` loses 8 B per meter. The audit's pinned binding size,
    the joined-manifest identity and `builtins_graph.rs`'s consumer pin follow it.
  - The metadata estimate drops the inline activation term: `direct-route` is −256 B native, and
    browser `graphMetadataBytes` is −200 B on wasm32.

### Ordinary-path differential

This reproduces #1076's method with a new harness, out of tree in the scratchpad. It drives the
base and candidate modules with identical seeded call sequences and compares every call record by
record.
- Base: `fa48dbbb`, `303a0f3f…`. The harness reproduces the #1076 pair (0 mismatches, −80 B bridge
  rows) and flags one-constant mutations.
- Scope: 200 scenarios (8 seeds × 25), 867,762 calls, 346,366 staged writes and 1,222,876 records
  compared, with 2,771 successful boots.
- Coverage: all 116 exports were called and all 116 reached success.
- Result: **0 mismatches**, 0 traps and 0 admission flips at a budget threshold.
- The only differences:
  - `graphMetadataBytes`, `graphIncrementalPlanBytes` and `graphSessionPlusPlanBytes` are −200 B
    in all 9,048 resource reports.
  - 240 budget refusals print an `exact_bytes`/`projected_bytes` 200 B lower against the same
    budget.
  - Linear memory size differs in 2,158 records; the peak is equal at 25,100,288.
  - 18,041 pointer returns differ in address only.
- Console digests (`gain_pan_profile digests`, release): the 17 rows are identical at `fa48dbbb` and
  on the branch.
- Browsers, CI mode (`--check-matrix --self-test-mutations`): all qualification gates pass on
  Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5. The first local attempt failed on socket
  paths longer than 108 bytes, a scratch-path artifact; with a short `TMPDIR` all three pass.
- SDK headless: 284/284.

### Shipped module

**Size and digest.** The module goes from 3,299,125 to 3,254,260 B (−44,865 B), and its sha256
from `303a0f3f…` to `aecdd83e…`.

| section | change |
|---|---|
| code | −36,341 B |
| data | −888 B |
| name | −7,564 B |
| type | −25 B (109 → 107 types) |
| functions | 2,524 → 2,481 |
| indirect table | 504 → 502 entries: the two `activation_changed` impls |

**Unchanged surface.**
- `initial=18` pages.
- The 116 function exports and `memory`, with identical signatures.
- The six other delivery files are byte-identical.

**Only removed code moved.** I compared functions by demangled name, with call targets by name and
type indices and ≥1 MiB data addresses normalized:
- 2,374 are identical.
- 45 are gone:
  - 37 are the activation module, its preflight, hooks and monomorphizations;
  - 3 were reached only from them (`BTreeMap<u32,_>::get`, `Arc<AtomicBool>::drop_slow`, a
    `BTreeMap` drop glue);
  - 5 were inlined into a caller that became their only one: `write_resident_lane` into `observe`,
    `invalidate_observers_after_failure` into `render`, `<bool>::from_elem`, and two drop glues into
    `preflight_sequential`.
- 2 are new: `RawVec<UnitIdentity>::grow_one` and `Vec<FoldLane>::into_boxed_slice` are now outlined
  from `build_sequential`, which shrinks by 54 B.
- 105 changed:
  - 62 differ only in layout constants: the spectrum record is 16,432 → 16,424 B, executor field
    offsets move, and the resource-estimate stack slots shift.
  - 43 change size, −3,822 B in all. The two growths are `render` (+159: the inlined invalidation
    at 5 sites, minus 3 `apply_candidate` and 2 `observe_one` calls) and `preflight_sequential`
    (+115: the inlined drop glue).
  - The largest shrinks are the policy-free builtins preparation (−1,206), `bind_optional_source_set`
    (−567), `bind_with_source_set` (−508), `observe` (−385), `MeterRequestSeal`'s sorts and
    `invalidate_observers` (−300).
- Kernel shape is identical: 15 kernels, `f32x4_arith` 14,007.

**Render closure.**
- `miso_engine_web_v1_render`'s direct-call closure is unchanged at 8 functions / 5 traps. The
  executor's `render` sits behind the plan's `dyn` call.
- The direct-call closure of `<GraphExecutor as PreparedPlanExecutor>::render` goes from 59 to 50
  functions (120,231 → 114,069 B).
  - It loses the hook `apply_candidate`, together with `__rust_dealloc`, `__rdl_dealloc` and dlmalloc
    `free`/`unlink_chunk`/`insert_large_chunk`, which were reachable from `render` only through it.
  - It also loses `observe_one`, and `write_resident_lane` and `invalidate_observers_after_failure`,
    which are now inlined.
  - Nothing is added.
- `meter_poll` stays at 9 and `command_submit` at 36.

**Browser resources.** `graphMetadataBytes` goes 4,131 → 3,931, and `graphIncrementalPlanBytes` and
`graphSessionPlusPlanBytes` go 32,402 → 32,202. The other rows, the PCM digests and the timelines are
identical.

### Gates, all green

| gate | result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo check --workspace --all-targets --all-features` | pass, no warnings |
| `cargo clippy --workspace --all-targets --all-features -D warnings` | pass |
| `cargo doc -D warnings` | pass |
| wasm `simd128` check of host-web (`--all-targets --all-features`), target-smoke, protocol, dsp-reference, conformance; host-web lib clippy `-D warnings` | pass |
| `check-cross-targets.sh` | PASS: aarch64 iOS/Android check and clippy, the #1018 memset rows as expected |
| graph, builtins-compiler, host-core, host-web, capi, graph-compiler, builtins tests, dev, all features | 727 passed, 0 failed |
| the same five crates' tests, release, all features | 483 passed, 0 failed |
| `test-debug-a`'s command and features | 1,086 passed, 0 failed |
| audit, bench, console-workload release tests | 107 passed |
| `audit-native`, all 20 steps | pass: the graph and builtins 1,000,000-block traces, realtime probes and the 1,000,000-block syscall trace, `check-capi-abi.sh` and self-test, graph determinism 100/100, builtins fixtures, console fixtures |
| `check-realtime-policy.sh` and its mutation tests | ok (54 marked regions in 15 files) |
| `check-web-audioworklet.sh`, with and without metadata regeneration; `test-web-audioworklet.sh`; `web-audioworklet-identity.py --self-test` | pass |
| V8 spill gate and self-test; `check-scalar-oracle-absent.py --wasm`; `--self-test` | pass |
| `check-browser-expected-resources.py --artifacts` (both modules) and `--self-test` | pass: 32 red mutations |
| SDK after `npm ci`: generated, deletions, types, headless, `sdk-package.sh check` | pass; headless 284/284 |
| wasm-guests: protocol wasm parity, `run-wasm-gates.sh --without-v8-spill --without-native` | pass |
| every hermetic step of route, docs-gates, lint, release-shape and gate-self-tests (38 steps, Python under `python3 -B`) | 38/38 pass; the router gives `route=full`, `self_tests=[]`. The DSP research mutations need stdin closed when run outside CI. |

### `cargo test --workspace --all-features -- --list`

2,071 → 2,045: −27, +1.
- graph: the 6 `observation_activation` unit tests, 5 lib activation tests, and the 5 rt9 controlled
  tests.
- builtins-compiler: 7 tests.
- host-core spectrum: 4 tests.
- The one addition is the rename to `selected_meter_preparation_keeps_control_delivery_distinct`.

### Findings for the owner

1. **Open #882 plans to use what this removes.** "Bind web console meters through the controlled
   observer policy" selects `MeterBindingPolicy::Controlled` and drives graph activation from the
   meter lease, and its evidence cites `observe_active_unit`. After this change that mechanism is
   gone. #882 needs a ruling: close it, or rescope it onto the ordinary path.
2. **Builtins' meter observer half.** It was kept in the first commits and removed afterwards, by
   owner ruling (decision 11). See the addendum.

### Addendum: builtins' meter observer half (owner decision 11)

The owner ruled to merge #1080, to rescope #882 to a simpler lease-driven skip (root handles that),
and to remove the builtins meter's `restart_observation` and `observation_generation`, which this
change left unused.

**Proof of non-use.** `restart_observation`, `observation_generation()` and
`MeterSnapshot::observation_generation` were made `pub(crate)`. The workspace (`--lib --bins
--all-features`), host-web on wasm32 `simd128` and the 25 product crates on `aarch64-apple-ios` and
`aarch64-linux-android` all compiled. On every target, the only warning was builtins' own "methods
`restart_observation` and `observation_generation` are never used". The field's only other writer
was the constant 0 at preparation.

**What was removed.**
- `MeterAccumulator::restart_observation` and `observation_generation()`, and the
  `observation_generation` field of `MeterAccumulator` and `MeterSnapshot`. builtins `src` goes
  5,559 → 5,538 lines. The ordinary meter path, including `reset`, the window logic, the snapshot's
  other fields and the banked and resident commits, is untouched.
- Tests:
  - `meter_observation_restart_discards_partial_window_and_preserves_lifetime_state` is removed;
  - the `Restart` event leaves the G2 peak streams and the M2 random stream, which now draws from 19
    arms instead of 20;
  - the generation word leaves the snapshot-bit helpers in builtins' meter tests and graph-compiler;
  - console-workload's assertion keeps `reset_generation`.
- Repins that move only by the removed bytes:
  - `MeterSnapshot` is 168 → 160 B and `MeterAccumulator` 240 → 232 B, reversing #818 A1;
  - the builtins-compiler mutation transcript moves only through those sizes, and adding one `u64`
    back to each restores the previous hash;
  - `resources.jsonl` loses 24 B per meter at queue depth 1 and 48 B at depth 4. The audit's two
    layout constants, the joined-manifest identity and `builtins_graph.rs`'s consumer pin follow.
- The commit's code change is 9 files, +32 / −134. The branch's code total is now 16 files, +206 /
  −5,257 (spec excluded).

**Module.**
- The module is 3,254,230 B, 30 B less than the earlier commits' module, with sha256 `885aa117…`.
  Against base that is 3,299,125 → 3,254,230 (−44,895 B).
- The whole −30 B is code. The meter's emit copies a smaller snapshot. `restart_observation` never
  shipped.
- Functions stay at 2,481, the 116 exports are unchanged, and memory stays at `initial=18`.
- The `render` closure stays at 8/5, `meter_poll` at 9, `command_submit` at 36, and the kernel shape
  at 15.
- Browser resource rows are unchanged from the earlier commits, `builtinRetainedBytes` 1,817
  included.

**Gates.**

| gate | result |
|---|---|
| fmt; `check` and `clippy -D warnings`, workspace, `--all-targets --all-features` | pass |
| default-feature `check --all-targets` | pass, no warnings |
| builtins, builtins-compiler, graph, host-core, host-web, graph-compiler and capi tests, dev, all features | 727 passed, 0 failed |
| the builtins through host-web crates, release, all features | 558 passed |
| the builtins through host-web crates, dev, default features | 553 passed |
| audit, bench and console-workload release tests | 107 passed |
| `check-builtins-fixtures.sh` | ok (50 files) |
| `check-web-audioworklet.sh`, with and without metadata regeneration | pass |
| `check-browser-expected-resources.py --artifacts` | pass |
| console digests | all 17 rows identical to `fa48dbbb` |
| `check-cross-targets.sh`, wasm `simd128` host-web `--all-targets`, `check-realtime-policy.sh` | pass |
| `cargo test --workspace --all-features -- --list` | 2,045 → 2,044, the removed restart test |
