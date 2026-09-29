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

## Sol verdict, attempt 1

**PASS.** Sol, 2026-09-29. Verified on a scratch merge of `a09cf0ee` into the batch-6 head
`codex/batch-slim-6` at `56900df4` (owner decision 11). Rust 1.97.1, Node 22.23.2.

### The merge

It is clean: `ort`, no textual conflict. Batch 6 adds only the ruling and #882's rescoped spec.
- The router gives `route=full` and `self_tests=[]` over `56900df4..merge`.
- Base module: 3,299,125 B, `303a0f3f…`. Merge: 3,254,230 B, `885aa117…`. Both match the
  branch's figures.
- The six other delivery files are byte-identical, including the ABI layout and the parameter
  metadata. Both modules declare `initial=18`, and their 117 exports have identical signatures.

### Checks

1. **Render safety improved, and nothing regressed.**
   - I rebuilt the direct-call closure of `<GraphExecutor as PreparedPlanExecutor>::render` from
     both disassemblies, using the gate's own `parse`/`closure`/`FORBIDDEN`.
     - Base: 59 functions, 333 traps. Five members are `FORBIDDEN`: `__rust_dealloc`,
       `__rdl_dealloc`, and dlmalloc `free`, `unlink_chunk` and `insert_large_chunk`. All five
       are reached through `apply_candidate`.
     - Merge: 50 functions, 320 traps, and no `FORBIDDEN` member.
     - The merge loses nine members: `apply_candidate`, the five above, `observe_one`, and the
       now-inlined `write_resident_lane` and `invalidate_observers_after_failure`. It gains none.
   - `Runtime::begin_observation_block` no longer exists.
     - Render goes from the host-plane shape check straight to the source work, then calls
       `observe_unit` for every active unit.
     - These functions are textually identical to base: `observe_unit`, `observe`,
       `observe_output`, `observe_output_one`, `write_resident_lane`, `execute`, `unit_inert` and
       `complete_pending`.
     - The two `invalidate_observers` functions lose only branches that needed activation state,
       which no production bind created.
     - The builtins meter's `emit` loses only the generation field. `observe_input`,
       `observe_input_banked` and `reset` are identical to base.
   - Gates, all green:
     - `check-web-audioworklet.sh`, with and without metadata regeneration: render 8/5,
       `meter_poll` 9, `command_submit` 36, 15 kernels, `f32x4_arith` 14,007.
     - `check-realtime-policy.sh` and its mutation tests: 57 → 54 regions, all three inside the
       deleted file.
     - audit capi, delay, compressor, parametric-eq and gate-expander: 0 violations each.
     - The builtins, builtins-graph and graph 1,000,000-block traces.
     - The protocol allocation audit.
     - The realtime, builtins and builtins-graph probe mutations.
     - The 1,000,000-block realtime syscall trace.
     - The effect-contract 1,000,000-block audit.
2. **The ordinary path behaves as before.**
   - *Differential.* I wrote a new lockstep harness, preloaded into Node, that pairs every
     instantiation of the base module with the merge module.
     - Each call runs on both modules. Before the call, the input staging (options, document,
       source, command, selection, spectrum, response, EQ and companion regions) is copied from
       base to merge. After the call, every return value and every staging region is compared.
       Pointer returns are compared only as zero or nonzero.
     - The harness caught a one-bit mutant: the low bit of the meter's `sample_peak` flipped,
       reported in the meter frame.
     - Inputs: the SDK's 20 eval files (284/284 pass under the harness), plus a seeded driver.
       The driver ran 8 seeds × 50 scenarios.
       - 304 SDK scenarios, 222 booted and 82 seeded refusals: random 1–64-track sessions with
         effects, buses and taps; console words; budgets; spectrum queries, collections and hops;
         reboots; lanes; meters; observations; spectrum and track-response reads.
       - 96 raw-ABI scenarios: every export with misuse arguments, handle 0, stale and
         double-dispose calls, and the Worker-side stream-analysis import.
   - *Result.*
     - 2,271,689 calls; 31,692,389 region and 1,568,308 resource-report comparisons; 1,062
       successful boots.
     - All 116 exports were called and all reached `ok`.
     - **0 mismatches and 0 one-sided traps.**
     - Allowed differences: 464,648 address-only pointers, and linear-memory size in 71,055
       records. The peak is equal, at 25,362,432 B.
   - *Every resource row that moved:*
     - The three graph rows are −200 B in all 1,568,308 reports.
       - This is base's per-graph `observation_runtime_state_bytes`: the inline activation owner
         state, `size_of::<GraphExecutor>() − size_of::<GraphExecutorWithoutObservationActivation>()`,
         which base charged to every graph.
       - The delta is constant across 1–64 tracks, so it is that constant and no per-op term. It
         is −256 B native (`direct-route`).
     - `bridgeMetadataBytes` and `bridgeRetainedBytes` are −8 B per track meter.
       - host-web charges `meter_count × size_of::<Option<MeterSnapshot>>()`.
     - `builtinRetainedBytes` is −80 B per meter, in all 880,621 metered reports.
       - That is the meter SPSC's 8 + 1 slots × −8, plus `MeterAccumulator`'s −8.
       - I confirmed it at 1, 5 and 17 tracks: bridge −8/−40/−136, builtin −80/−400/−1,360.
     - 18 budget diagnostics differ only in the byte count, by −200 − 8·meters − 24·spectrum
       targets. The −24 is the spectrum observer's −8 plus its two-slot record queue's −16.
     - No admission flipped.
   - *`MeterSnapshot` is not an ABI type.*
     - It is not in `ABI_LAYOUT`, `sdk/assets` or `sdk/src/generated/abi.ts`, all unchanged.
     - The meter frame publishes only each lane's `sample_peak`.
     - `check-sdk-generated.sh` passes against the merge's artifacts. No published layout
       changed, and nothing breaks for the app.
   - *Behaviour gates.*
     - The 17 console digests are identical on base and merge.
     - Browsers, CI mode (`--check-matrix --self-test-mutations`): Chromium 151.0.7922.34,
       Firefox 153.0 and WebKit 26.5 pass all qualification gates.
     - SDK headless on the merge: 284/284.
3. **Nothing live was deleted.**
   - No removed identifier appears in code under crates, hosts, the SDK, tools, scripts or
     tests. The remaining hits are historical text in specs, `docs/` and
     `crates/graph/tests/MUTATIONS.md`.
   - The deletion itself compiles everywhere. That is stronger than the crate-private proof:
     - the workspace, `--all-targets --all-features` and default;
     - host-web `--all-targets --all-features` on wasm32 `simd128`, plus its lib clippy;
     - `check-cross-targets.sh` (aarch64 iOS and Android check and clippy; #1018 memset rows
       as expected);
     - the x86 `--no-run` of both AArch64 legs.
   - `check-capi-abi.sh` and its self-test pass, with the frozen symbol set unchanged.
   - host-core's production callers passed `None` for `maximum_pops`, the parameter; only tests
     passed `Some`. `SpectrumCapturedRecord::observation_generation` was read only as `let _`.
4. **Resource pins.**
   - `check-browser-expected-resources.py --artifacts` passes, and `--self-test` is green at
     every ceiling with 32 red mutations.
     - Only the graph rows moved: 3,931, 32,202 and 32,202.
     - The bridge rows (1,146,179 and 1,166,688) and `builtinRetainedBytes` (1,817) are
       unchanged, because the fixture has no meters.
   - capi `resource_lifecycle` (#1060) passes 9/9 in release, including
     `reference_session_retained_rows_stay_within_their_budgets`. Every row only fell.
5. **Gates on the merge**, all green:
   - fmt;
   - `check` (all features, and default) and clippy `-D warnings`, workspace `--all-targets`;
   - `doc -D warnings`;
   - graph, builtins, builtins-compiler, host-core, host-web, capi and graph-compiler tests,
     dev and release, all features;
   - `test-debug-a` and `test-debug-b`;
   - audit, bench and console-workload release tests;
   - lane, math and wasm-gates release tests;
   - `audit-native`'s steps, including graph determinism 100/100 and the builtins and console
     fixtures;
   - wasm-guests' protocol parity and `run-wasm-gates.sh --without-v8-spill --without-native`;
   - the unwind release check;
   - `test-web-audioworklet.sh`, the V8 spill gate and its self-test, and the identity self-test;
   - `check-scalar-oracle-absent.py`, `--wasm` and `--native`;
   - SDK generated, deletions, types, headless and `sdk-package.sh check`;
   - every hermetic route, docs-gates, lint, release-shape and gate-self-tests step: 36/36,
     Python under `python3 -B`, stdin closed.

   Test list: 2,071 → 2,044 (−28, +1), matching the addendum.

### Findings, by severity

No defect.

1. **Low: the evidence understates which rows move.**
   - The differential and "Why the ordinary path is unchanged" predate `5ec25dad`. They say only
     the graph rows moved, and the addendum calls the browser rows unchanged. That is true only
     of the meterless qualification fixture.
   - In metered sessions:
     - the bridge rows drop 8 B per meter;
     - `builtinRetainedBytes` drops 80 B per meter;
     - spectrum admission projections drop 24 B per target.
   - Every one of these comes from a removed field. The record should say so.
2. **Low: the ruling is thinner than the addendum claims.** Decision 11 in
   `engine-footprint-2026-09-29.md` records removing the activation machinery. It does not
   mention the builtins meter's `restart_observation`/`observation_generation`, which the addendum
   attributes to it. The removal is within scope, because the meter hook was the only caller.
   One line in the ruling would close the gap.
3. **Info: one ordinary test arm was dropped.**
   - The spec says "The ordinary arms of mixed tests stay", but
     `rt9_mixed_permanent_controlled_alias_dispatch_order_and_audio_match` went whole. With it
     went its permanent arm's fixed order `[1, 0, 2, 3, 4]` for mixed direct and alias rows in a
     bank.
   - The ordinary binding order is still covered by
     `resident_meter_dispatch_preserves_binding_order_lazy_fallback_and_accepted_errors` and by
     graph-compiler's stable-handle-order tests.
4. **Info: stale historical references.**
   - `crates/graph/tests/MUTATIONS.md` rows 916-13, 936-1b and 936-5a name removed items.
   - capi `resource_lifecycle`'s baseline table comment (graph metadata 56,068) is now 256 B
     high. Its budgets are ceilings, so they still pass.
5. **Info: budget thresholds move.** They move by exactly the removed bytes, which is
   accounting only. The differential saw no admission flip.
