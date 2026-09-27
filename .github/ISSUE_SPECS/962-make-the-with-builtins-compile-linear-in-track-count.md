# Make the with-builtins compile linear in track count

## Product outcome

`PreparedGraphPlan::with_builtin_banks` (`crates/graph/src/lib.rs:1339-1342`) checks `self.required_bindings.contains(member)`, a linear scan of a `Vec` of about 4 x tracks entries, for every bank member (3 x tracks), so every host's compile is quadratic in track count. Measured by the builtins-less removal verification (release, `Simd8`): 8,192 tracks 3.94 s, 16,384 tracks 9.94 s, 32,768 tracks 37.4 s, 65,537 tracks **158.6 s**; with a set lookup, 16,384 tracks 2.26 s and 65,537 tracks 11.2 s. It stayed hidden because the only scale gate (`crates/graph-compiler/tests/scale.rs`) compiles without builtins at `Backend::Scalar`, where no bank attaches. AGENTS.md: track counts are constrained only by configured resources. Evidence: `docs/handoffs/builtins-less-removal-2026-09-27/VERIFY.md` section 5.

## Smallest closable slice

1. Replace the linear membership test with an ordered-set or sorted-slice lookup built once per call (compile time, off the render thread), with identical results and identical iteration order of everything it produces.
2. Search `crates/graph` and `crates/graph-compiler` for other `Vec::contains` or nested scans over per-track lists on the compile path, and fix any that is quadratic in track count, listing each.
3. Add a with-builtins scale gate: the 65,537-track session of `tests/scale.rs` compiled through `compile_with_builtins` at `Backend::current()`, asserting it completes and binds (a wall-clock bound only if CI can hold one reliably; otherwise record the time).

## Objective gates

- Every compiled artifact byte-identical before and after on the graph and builtins fixtures and every console workload (digests and unit census unchanged).
- The new scale gate passes in release and debug within CI's budget; record both times.
- fmt, clippy `-D warnings`, doc `-D warnings`, `cargo test -p graph -p graph-compiler -p builtins-compiler --features test-support`, graph policy and determinism scripts.

## Dependencies

Lands before the graph-compiler test port (the option (b) split of #959).


## Attempt 1 evidence

Implementer: Claude Opus 5.5 (attempt 1), 2026-09-27, branch `codex/962-linear-builtins-compile` on
`0109d841` (#957's attempt-1 tip, untouched). Code and gate commit `0d444f00`. Host AMD EPYC 7313P,
32 threads, rustc 1.97.1, `CARGO_INCREMENTAL=0`, the worktree's own `target/` (deleted afterwards).
The console benchmark runner was not run. Timings are descriptive: one run each, no warmup, on a
host shared with another verifier's build. Scratch probes and raw outputs are in this session's
scratchpad, `scratchpad/962/`; nothing from them is committed.

### The change

Every fix replaces a scan with an index built once per call, and every index answers exactly the
question the scan answered, so no result and no order moves. Nothing on the render path changed:
the edits are in the compile (`with_builtin_banks`, `effect_control_resource`) or in bind-time
planning (`has_valid_structural_layout`, `preflight_sequential` and its helpers,
`build_sequential`), which run on the control plane before a render plan exists. `crates/graph`
gains no `unsafe`.

1. **`PreparedGraphPlan::with_builtin_banks`** (step 1): `required_bindings.contains(member)` per
   bank member becomes a binary search of a sorted copy of the borrows. The field keeps its schedule
   order; the copy is used for membership only.
2. **`effect_control_resource`** (`graph-compiler/src/estimate.rs`, compile): for each effect-bank
   member it scanned every prepared entry for a matching controlled one. Now one set of controlled
   `(track, rack, effect)` keys.
3. **`has_valid_structural_layout`** (bind): for each bank it scanned every spec edge for one into a
   member. Now each node's incoming sources are gathered once, and each member's are checked.
4. **`units_of`** (bind): for each bank it rescanned every op for that bank's members. Now one pass
   gathers each bank's `(position, op)` pairs in op order, then sorts them as before. The unused
   `op_of_node` map is gone; the bank key is `(is_builtin, bank)`, which cannot collide.
5. **`chains_into`** (bind): for each lane it scanned every program tap. It now takes
   `taps_by_op(program)`, built once by its three callers.
6. **`build_sequential`'s split pass** (bind): for each plain single-op run it scanned every later
   run for a matrix. `chains_into(&[fader], &[matrix])` admits a matrix only as the fader's sole
   reader, so the loop now visits only the run holding that reader (`plain_run_of_op`). The loop
   body, its conditions and their order are unchanged.
7. **`scalar_split_interval_is_clear`** and the adjacent-pair `crossing_reader`: per scalar fader
   they scanned every spec edge and every op between the pair. Now `crossing_sources(spec)` and
   `ops_naming_buffers(program)` (the `op_names_buffer` relation, inverted) are built once, and
   the interval check is a `partition_point`.
8. **`BorrowedPlanningMetadata`** (bind): `has_source`, `has_binding`, `has_effect`, `has_observer`
   and `route` each scanned a per-track list. They are now sets and a map built in `new`. `route`
   keeps the last entry for a node, as the reverse search did. **`observed`** takes the tap index as
   `chains_into` does.

### Timings

Scratch probe `scratchpad/962/graph-compiler-zz_probe_962.rs`, release profile. The session is
`tests/scale.rs`'s unless stated. "Bind" is `into_bound` with identity bindings for the 65,538
external nodes. The "before" bind binaries keep fix 1, so the compile finishes, and revert fixes
3-8. The per-phase rows come from temporary `eprintln!` timing probes, which were removed before the
commit. The before-bind runs ran concurrently with each other.

**The step-1 compile** (`compile_with_builtins`, Simd8):

| tracks | before | after | plain `compile`, for scale |
|---:|---:|---:|---:|
| 8,192 | 3.31 s | 0.95 s | 0.63 s |
| 16,384 | 9.63 s | 2.08 s | 1.35 s |
| 32,768 | 40.4 s | 4.75 s | 2.91 s |
| 65,537 | **163.7 s** | **10.3 s** | 6.10 s |

**Every fix, at 8,192 and 65,537 tracks:**

| fix | path | measured as | 8,192 before -> after | 65,537 before -> after |
|---|---|---|---:|---:|
| 1 `with_builtin_banks` | compile | whole compile, Simd8 | 3.31 s -> 0.95 s | 163.7 s -> 10.3 s |
| 2 `effect_control_resource` | compile | whole compile, console strip (EQ + compressor) per track, Simd8 | 2.09 s -> 1.72 s | 36.5 s -> 19.5 s |
| 3 `has_valid_structural_layout` | bind | its phase, Simd8 | 7.88 s -> 0.127 s | 623.9 s -> 1.40 s |
| 4 `units_of` | bind | its phase, Simd8 | 4.43 s -> 4.4 ms | 332.2 s -> 62 ms |
| 5 `chains_into` taps | bind | `cohort_runs` phase, Simd8 | 244 ms -> 6.2 ms | 16.4 s -> 124 ms |
| 6 split pass, candidate run | bind | split phase, Simd8 | 88.6 ms -> 0.18 ms | 7.19 s -> 2.6 ms |
| 6 + 7 split pass, both scans | bind | split phase, Scalar | 5.06 s -> 11.4 ms | 878.0 s -> 123 ms |
| 8 metadata sets + `observed` taps | bind | `route_fold` phase, one route per track, Simd8 | 556 ms -> 8.4 ms | 41.6 s -> 173 ms |

Fixes 6 and 7 are one phase at Scalar and are reported together. At Simd8 only the ragged tail's
fader reaches fix 7's scans.

**Whole bind, before -> after:**

| session | 8,192 | 65,537 |
|---|---:|---:|
| scale, Simd8 | 12.9 s -> 0.40 s | 983.3 s -> 5.32 s |
| scale, Scalar | 5.35 s -> 0.32 s | 881.5 s -> 3.91 s |
| scale with one route per track, Simd8 | 16.9 s -> 0.51 s | 1,424.9 s -> 6.40 s |

**After, by doubling.** Simd8 compile and bind: 0.95 / 2.08 / 4.75 / 10.3 s and 0.40 / 1.03 / 2.31
/ 5.32 s at 8,192 / 16,384 / 32,768 / 65,537 tracks. The console strip compile: 1.72 / 3.85 / 8.96
/ 19.5 s. Every step is 2.2x to 2.6x, which is n log n and cache growth, not a quadratic (4x). At Simd4,
65,537 tracks: compile 10.7 s, 49,155 builtin banks, bind 5.4 s.

### Other quadratic scans (step 2)

Fixes 2-8 above. They were found by phase-timing the compile and the bind at 4,096 / 8,192 / 16,384
tracks on three session shapes: the scale session at Simd8, the same at Scalar, and the same with
one route per track. Each phase that grew by about 4x per doubling was then read. One phase
(`build_sequential`'s split pass) was fixed twice, because the Scalar shape exposed a second scan
behind the first. The bind path is `crates/graph` too, and the new gate binds, so its scans are
in step 2's scope.

Read and found linear or n log n on the compile path: `graph-compiler`'s `compile.rs`, `banks.rs`,
`schedule.rs` (`topo`, `cycle_witnesses`, `buffer_assignments`), `pdc.rs`, `ids.rs`, `canonical.rs`,
and `graph`'s `program::lower` (#925 already interned its bindable set).

Left alone:

- `scatter_redirects` (bind) grows faster than linear but stays small: 2.3 ms at 8,192 and 47 ms at
  65,537 tracks.
- Not probed at scale: binds with a source set, per-track observers or meters, and sidechains.
  `has_source` and `has_observer` are now sets, so the metadata side of those shapes is covered;
  the rest of their bind paths was not timed.
- `builtins-compiler` and `rack-compiler` are outside step 2's two crates. Their share of the probed
  compiles scales like the rest.

### The with-builtins scale gate

`crates/graph-compiler/tests/scale.rs::compiles_and_binds_65_537_tracks_with_builtins` builds the
same session as the existing gate (now a shared `scale_session()`). It compiles through
`compile_with_builtins` at `Backend::current()` and asserts:

- the 458,761-item schedule;
- `3 x 65,537` builtin bank members at a SIMD width, and none at scalar width;
- 65,538 external binding nodes;
- `into_bound` succeeds;
- one block renders.

| test | release | debug | peak RSS |
|---|---:|---:|---:|
| `compiles_and_binds_65_537_tracks_with_builtins` (new) | 17.2 s | 60.3 s | 1.31 GB |
| `compiles_65_537_tracks_or_rejects_only_a_configured_resource` | 9.2 s | 29.9 s | 0.89 GB |
| the whole `scale` binary in debug (both tests, in parallel) | – | 62.1 s | 1.87 GB |

There is no wall-clock bound. A test-harness clock is not one CI can hold reliably: runner speed
varies, and the binary's tests run in parallel. The code before the fix needs about 20 minutes
(release) for this test's compile and bind, and much longer in debug. It cannot finish inside the
15-minute `test-debug-a` job, so that job's timeout is the backstop.

### Byte identity (hard stop)

- **Graph artifacts.** The scratch probe (`probe_962_fingerprints`) compiled 28 sessions through
  `compile_with_builtins` at `Backend::current()`, Scalar, Simd4 and Simd8, giving 112 lines. The
  sessions were the 12 valid `fixtures/session/v1` sessions, effects stripped, plus the scale
  session at 1, 3, 4, 5, 7, 8, 9, 15, 16, 17, 63, 64, 65, 129, 1,000 and 4,097 tracks. Each line is
  a SHA-256 over:
  - `GraphCompiler::sha256`;
  - `Debug` of `program()`, `required_bindings`, the schedule, the levels, the buffer assignments,
    the estimate and the report;
  - every builtin bank's stage, backend, width, members and delivery;
  - every effect bank's members;
  - `external_binding_nodes` in order.

  All 112 lines are identical on `0109d841` and on the final tree.
- **Console workloads.** The scratch probe (`scratchpad/962/console-workload-zz_probe_962.rs`) built
  all 17 native session rows at Simd8, Simd4 and Scalar and rendered 64 blocks each. It recorded:
  - the 64-block digest;
  - `bank_shape`, transposes, `symmetry_counters` (the unit census) and `bank_symmetry_counters`;
  - collapse counters and transitions, route folds, scatter redirects and structural mono tracks;
  - every `unit_eligibility` row.

  All 51 lines are identical before and after.
- **Graph fixtures.** `graph_fixture`'s fingerprint and `--manifest` output are byte-identical on
  the base and the final tree. The builtins fixtures check reports ok (50 files).

### Gates

| command | result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | pass |
| `cargo test --locked -p graph -p graph-compiler` | pass: graph lib 108, rt10 1, rt1 1, rt9 8; graph-compiler lib 75, `graph_fixture` 1, `route_gain` 3, `scale` 2, `track_delay` 8, doc 6 |
| the same with `--features graph/test-support,builtins-compiler/test-support` | pass, same counts |
| `cargo test --locked -p builtins-compiler --features test-support` | pass: 58, 9, 3, 6, 1, 2 |
| `cargo test --locked -p host-core --all-features` | pass |
| `cargo test --locked --release -p console-workload` (CI's mode), and in debug | pass: 8, 4, 23, 3 both ways |
| `bash scripts/check-graph-policy.sh .` and `bash scripts/test-graph-policy.sh` | PASS, ok |
| `bash scripts/check-graph-determinism.sh` | PASS (100/100) |
| `bash scripts/check-builtins-fixtures.sh .` | ok (50 files) |
| `target/debug/graph_fixture --check` | fails, "graph fixture manifest mismatch": **identically on the base** (see deviations) |

### Deviations

- **Scope.** Step 2 names the compile path. The search also found six quadratic scans on the bind
  path (fixes 3-8), all in `crates/graph`. They are fixed here because the gate binds; before the
  fixes, bind took 983 s at 65,537 tracks against the compile's 10 s. If review holds that bind
  belongs in its own issue, `0d444f00` separates cleanly by file region. The fixes to
  `with_builtin_banks` and `effect_control_resource`, plus the gate without its bind half, are a
  compile-only slice.
- **Release test builds need `--config 'profile.release.panic="unwind"'`.** Without it,
  `cargo test --release -p graph-compiler --test ...` fails with E0463. The `graph_fixture` bin
  builds under the release `panic = "abort"`, and the test harness under unwind. So
  `effect-package`'s cdylib is built twice to one filename ("output filename collision"). This is
  not new, and no CI job builds graph-compiler's tests in release. Every release number here was
  taken with the override: LTO fat and one codegen unit as shipped, unwinding instead of aborting.
- **`graph_fixture --check` fails before and after.** The checked-in `fixtures/graph/MANIFEST.tsv`
  last changed at `5cf9709d` (#98). The base generates the same `--manifest` output as the final
  tree and fails the same way. No CI job runs `--check`, only the fresh-process determinism loop,
  which passes. This is left for its own issue.
- **Extra sizes.** The step-1 table adds 16,384 and 32,768 tracks, to show the growth rate.

## Sol attempt 1 verdict: PASS

Reviewer: Claude Opus 5.5 (Sol), 2026-09-27, `git diff 0109d841..1d3ae356`, built in the branch
worktree with `CARGO_INCREMENTAL=0` and a detached scratch worktree of `0109d841` for the "before"
arm. The timed console benchmark was not run.

### Byte identity (before and after)

- **The implementer's probes, re-run on both trees:** all 112 graph fingerprints (28 sessions at
  four widths) are identical, and they match the recorded `fp-final.txt`. All 51 console rows
  (digest, census, unit rows) are identical, and they match `cw-after.txt`.
- **`graph_fixture`:** the fingerprint and `--manifest` output are identical on both trees.
- **An independent randomized probe** (scratch, not committed) covered 800 seeds. Each seed
  generates a session with:
  - odd and even track counts from 1 to 37, and a 35% "console" shape (all tracks one strip, one
    post-matrix route each) so that route folds occur;
  - 1-2 native effects per rack from all eight, with bypass, maximum link, and compressor or gate
    sidechains from earlier tracks at every tap;
  - per-side builtin delays, mono channel mappings, sends at every tap, and submix chains;
  - per-track meters at every tap, through all three builtins entries (plain, console controls,
    and between render calls), plus effect console channels;
  - external observers on random stage nodes, aliases included.

  Each seed was compiled at Scalar, Simd4 and Simd8 and bound twice, once with processor feeds and
  once with a played-planes source set: 4,800 lines. Each line records the compile fingerprint as
  above, a six-block PCM digest, meter snapshot contents, an observer hash, the selected split
  fader, bank shape, transposes, symmetry, collapse counters, folds, redirects, dispatch and
  qualification counters, source-plane counts, and every `unit_eligibility` row.

  The 4,800 lines are **byte-identical** on both trees, and no line panicked. Coverage: 496 split
  pairs selected, 294 lines with route folds, 2,066 with scatter redirects, 918 with effect banks,
  3,354 metered, 2,594 observed, 64 collapsed, and 3,975 with source planes read in place.

### Each rewritten scan

Each lookup answers the scan's question exactly. The key types (`GraphNodeId`, `EffectNodeId`,
`BufferRef`, `EffectRack`) all derive `Ord` with `Eq`, so set and map membership equals `==`
membership.

- **`with_builtin_banks`:** binary search of a sorted borrow copy. The short-circuit order is
  unchanged.
- **`effect_control_resource`:** the same key triple as the scan's conjunction.
- **`has_valid_structural_layout`:** any-over-all-edges becomes any-over-each-member's-incoming-
  edges. This is the same boolean, because members are strictly increasing and every edge kind is
  kept.
- **`units_of`:** the same `(position, op)` pairs are pushed in op order and sorted the same way.
  The old key `bank + ops.len()` could collide only if an effect bank index reached `ops.len()`,
  which is impossible in a validated plan.
- **`chains_into` and `observed`:** `taps_by_op` of the same `program` and `spec`, built in each
  caller. An index above `u32::MAX` matched no tap before and matches none now.
- **`BorrowedPlanningMetadata`:** sets built from the same lists. `route` keeps the last entry,
  as `rev().find()` did.
- **`crossing_sources`:** the set of the scan's sources.
- **`ops_naming_buffers` plus `partition_point`:** each buffer's list is built in ascending op
  order over the same five naming roles, so "the first namer after the fader is at or past the
  matrix" is exactly "no op strictly between the pair names the buffer".

**The split-pass premise is not a property of plans.** It is `chains_into`'s own clause
(`crates/graph/src/runtime.rs:6886`, `readers[before].len() != 1 || readers[before][0] != after`),
evaluated on the same `readers` array the lookup uses (`:5293`). No plan can violate it.

- Multiple readers, sends, and sidechain reads (`op_dataflow` counts sidechain reads): `readers`
  has more than one entry, so the old loop failed every candidate at `chains_into` and the new one
  has no candidate.
- Duplicate reads of one producer: the same result, because the length check fails.
- Observers: these are not readers. Both paths test them with the identical `has_observer`
  clauses.
- Delayed consumers: both paths reject them in the same predicate, and staging slots are named in
  the interval index.

Every condition ahead of the ownership moves is side-effect-free. So skipping the non-candidate
runs cannot change which pair is chosen, or the state of `parts.bindings` for any other pair.
`run_units` partitions the ops, so the op-to-run map is single-valued.

### Render, realtime, unsafe

- Every hunk is at `lib.rs:1205-1362` or `runtime.rs:4550` and later. The `REALTIME_POLICY` regions
  are at `lib.rs:1969-1985` and `2580-2728`, and in `runtime.rs` they end at 3637, so no hunk
  overlaps one.
- Every changed function is reached only from compile or from `bind` / `preflight_sequential` /
  `build_sequential`.
- `check-realtime-policy.sh` passes (55 regions) and `test-realtime-policy.sh` is ok.
- The workspace denies `unsafe_code`, and the diff adds none.

### The scale gate

| test | release | debug | peak RSS |
|---|---:|---:|---:|
| `compiles_and_binds_65_537_tracks_with_builtins`, run alone | 17.0 s | 59.9 s | 1.31 GB |
| the whole `scale` binary | - | 60.3 s | 1.87 GB |

Release was built with the `panic="unwind"` override. These times reproduce the evidence.

CI's `test-debug-a` ran the existing scale test in 32.7 s, against about 30 s here, so the binary
should take about 66 s on CI. Recent `test-debug-a` jobs took 3.5-5 min of their 15-minute
budget, which leaves ample room. The missing wall-clock bound is acceptable under the brief's own
wording. The job timeout catches a regression of fixes 1, 3 or 4, whose debug cost would run to
tens of minutes; smaller regressions go undetected (finding 1).

**Growth is linear up to a log factor.** Debug, the gate's own session at 8,192 / 16,384 / 32,768
/ 65,537 tracks:

- compile: 3.59 / 7.67 / 16.34 / 35.46 s;
- bind: 1.38 / 3.13 / 6.90 / 14.64 s;
- so each doubling costs 2.12-2.27x.

Three shapes host-core actually prepares were also timed in debug from 1,024 to 8,192 tracks:

- EQ and compressor banks with console channels, a meter and a route per track, observers, and a
  source set, at Simd8;
- the same with a compressor per track sidechained to the previous track;
- the same at Scalar.

Compile and bind grew 2.1-2.3x per doubling in all three.

### Scope ruling

The widening to bind is **accepted, and the issue stays whole**. Step 3 requires the gate to bind
at 65,537 tracks, and bind took 983 s in release before fixes 3-8. So step 3 cannot pass without
them. They stay inside step 2's two crates, change no architecture, and are individually
equivalence-proved. Splitting them out would ship a gate that cannot run.

### Pre-existing items, confirmed not caused here

- **Release test builds.** `cargo test --release -p graph-compiler --test scale` fails with E0463
  and an `effect_package` output filename collision on `0109d841` too. The diff touches no
  manifest.
- **`graph_fixture --check`.** It exits 1 on both trees, with identical fingerprint and manifest
  output. The checked-in `MANIFEST.tsv` is stale since `5cf9709d`.

### Gates re-run on the branch

All pass:

- `cargo fmt --all --check`;
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`;
- `cargo test --locked -p graph -p graph-compiler`: 108 / 1 / 1 / 8 and 75 / 1 / 3 / 2 / 8 / 6;
- `-p builtins-compiler --features test-support`: 58 / 9 / 3 / 6 / 1 / 2;
- `-p host-core --all-features`;
- `-p console-workload` in debug: 8 / 4 / 23 / 3;
- `check-graph-policy.sh` and `test-graph-policy.sh`;
- `check-graph-determinism.sh` (100/100);
- `check-builtins-fixtures.sh` (50 files).

### Findings, most severe first

1. **Low. The gate's regression claim is wider than its coverage.** The doc comment at
   `crates/graph-compiler/tests/scale.rs:118` says a regression "shows up as this test's time".
   The session has no effects, one route, and Simd8 on CI.
   - Fix 2 (`effect_control_resource`), fix 7 (the Scalar interval scan) and fix 8 (the metadata
     sets on the route fold, with one route per track) are not exercised at scale.
   - A regression of fix 5 or 6 (16 s and 7 s in release) adds well under the job timeout.
   - Only fixes 1, 3 and 4 are guarded by the timeout backstop.

   This is not blocking, because step 3 named this session. Recommended follow-up: a second
   bounded shape (one route and one banked effect per track) or a doubling-ratio check.
2. **Low. `scatter_redirects` is still superlinear, and it is left unclassified.** The evidence
   shows 2.3 ms at 8,192 tracks and 47 ms at 65,537, about n^1.45. The likely cause is
   `scatter_target`'s per-op `run.contains` inside a window scan (`runtime.rs:6069-6072`). It is
   harmless at 65,537 tracks. AGENTS.md asks for diminishing-return work to be recorded as an
   issue, and "left alone" is not that record.
3. **Info, out of scope and pre-existing.** The randomized probe found compile-admitted plans that
   bind refuses with `graph.scheduler.layout`: 44 of 4,800 binds, 22 seed-width pairs, identical on
   `0109d841`. `has_valid_structural_layout` (`crates/graph/src/lib.rs:1233`) rejects an effect bank
   whose members sit at different dependency levels. Seed 412 at Simd8 reproduces it:
   - nine tracks, where seven carry `soft-clip` at dynamic slot 0 and one carries it at slot 1
     behind an EQ;
   - all eight are banked together, at levels 5,5,5,6,5,5,5,5;
   - the same session binds at Simd4.

   A host would fail prepare on a session the compiler accepted. This needs its own issue. It is
   not #962's.
4. **Info.** `units_of`'s key change from `bank + ops.len()` to `(is_builtin, bank)` changes
   behaviour only in a state no validated plan reaches, as proved above.
