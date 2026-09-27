# Delete the builtins-less compile entry

Scoping study: `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md` (owner-directed removal, 2026-09-27; option (b) per the owner's second ruling below).

## Amendments (adversarial verification, 2026-09-27): rescoped

This issue is now slice (iii) only: **delete the builtins-less compile entry and its `Option`
arms**, after #962 (linear compile), #963 (tools and audits) and #964 (graph-compiler tests)
have removed every caller. Steps 2 and 3 of the slice below moved to #964 and #963.
- **Gate 1 is replaced** (the old pattern matched six unrelated functions):
  `rg -nP 'GraphCompiler::compile(?!_with_builtins)|GraphCompileRequest|PreparedGraphArtifact\b' crates tools hosts`
  finds nothing, and `rg 'PreparedGraphPlan::new' tools` finds nothing.
- Hand-built plans in `crates/graph`, `crates/source` and `crates/builtins-compiler` tests are
  graph constructions, not compiles, and are out of scope (crate cycle).
- **Flagged for the owner, out of scope:** `PreparedGraphPlan::new` plus `bind` stays public (the
  compilers need it), and `Backend::Scalar` still produces bankless with-builtins plans (host-core
  tests, the CI scalar wasm compile, scalar-only split-pair code).

## Rulings (coordinator, 2026-09-27)

The owner directed the complete removal of the builtins-less path ("I don't think we should be
benchmarking something that never gets used in the real world"). Option (b) is taken, by the owner's
second ruling ("We should remove everything related to a builtins-less compile because a
builtins-less compile is never needed in production"): the builtins-less compile is removed
entirely, including any test-only entry point, builtins become mandatory in the graph compiler,
and every test that compiled without builtins is ported to `compile_with_builtins`. The optimisation batch lands unchanged and this work deletes #937's
code afterwards. The wasm console arm is re-indexed. `sixty_four_track_gain_pan_only` is the
pure-audio-path target; its fused twin follows. #938 is re-based onto the gain/pan session. The
phase-profile harness is retargeted to `gain_pan_only`. Ported tools compile at
`Backend::current()`.

## Smallest closable slice (option (b), owner ruling 2026-09-27)

1. **Delete the builtins-less compile.** Remove `GraphCompiler::compile` (the entry that takes no
   prepared builtins) and every `Option` arm that exists only for it in
   `crates/graph-compiler/src/compile.rs` (the study cites `:449`, `:654`, `:679`); builtins become
   a required input of the one remaining compile entry. No `#[cfg(test)]` or feature-gated
   builtins-less entry may remain anywhere.
2. **Port every test that compiled without builtins** to `compile_with_builtins`, in
   `crates/graph-compiler` (the study's 47 functions: the B, P and G groups, `tests/track_delay.rs`,
   `tests/scale.rs`) and anywhere else the grep in gate 1 finds. Re-pin SHAs, tails and chain counts
   that move, each with a one-line reason; never delete a test to make the port easier. A test
   whose subject was the builtins-less path itself is deleted and listed.
3. **Port the tools** (`graph_fixture`, the #650 audit, the #006 compile benchmark) to
   `compile_with_builtins` at `Backend::current()`, and regenerate `fixtures/graph/v1/*` and its
   manifest in the same change, coordinated with #947.
4. **Docs.** Update every doc and ruling that describes a builtins-less compile as available.

## Objective gates

1. `rg -n 'GraphCompiler::compile\(|fn compile\(' crates tools hosts` finds only
   `compile_with_builtins` (or its renamed successor) and no builtins-less entry.
2. Every ported test passes; the list of deleted tests is in the evidence with a reason each.
3. Every console workload digest and unit census unchanged; the regenerated fixtures pass #947's gate.
4. fmt, clippy `-D warnings`, doc `-D warnings`, `cargo test --workspace` (or the CI debug and release
   test jobs' commands), the graph policy, determinism, realtime and wasm-gate scripts.

## Dependencies

#958; #947.

## Standing rules for the implementer

- No product behaviour changes; every console digest unchanged is a hard stop.
- Commit on `codex/<issue>-<slug>`. Do not run timed benchmarks.


## Attempt 1 evidence

Implementer: Claude Opus 5.5, 2026-09-27, branch `codex/959-delete-builtins-less-compile` from
`792a1e87`. Implementation commit `1fea862a`: 8 files, +156 / -179.

### What was deleted

In `crates/graph-compiler`:

- **`GraphCompiler::compile`**, the builtins-less entry: 8 lines (`compile.rs:136-143`).
- **`GraphCompileRequest`** (17 lines), **`GraphCompileFailure`** (4 lines) and
  **`PreparedGraphArtifact`** (11 lines), all public, in `lib.rs`. The
  `GraphCompileRequest::dispatch` doc (#99 F6) moved onto `GraphBuiltinsCompileRequest::dispatch`,
  which used to link to it.
- **The `Option` arms**, in `compile.rs`, each replaced by its `Some` body:
  - the `prepared_builtins: Option<&PreparedBuiltinsSession>` parameter, now a required
    `&PreparedBuiltinsSession`;
  - `:449`: the input-section symmetry witness `if let Some`;
  - `:654`: the scalar-owner charge `if let Some`;
  - `:679`: the builtin-bank charge `if let Some`, and its `else` arm, a zero
    `GraphBuiltinBankResourceEstimate::default()`;
  - `:816`: the `builtin_stages_bindable = prepared_builtins.is_some()` switch. The three builtin
    stages are now listed in `required_bindings` unconditionally, in one `matches!`;
  - `:220-223`: the tail lookup's `unwrap_or(TailSamples::Finite(0))` for a track with no builtin
    tail. It fired only for the builtins-less entry's empty map. `validate_for_session` requires
    one tail per session track, so it is now an `expect`, like its neighbour
    `expect("validated prepared effect")`.
- **`ids::failure`** (9 lines) moved into `compile.rs`, because only the pipeline uses it.

The pipeline `compile_with_builtin_tails(GraphCompileRequest, &BTreeMap<String, _>, Option<_>)` is
now `compile_graph(plan_id, effects, caps, dispatch, &PreparedBuiltinsSession)`. It is private and
builds the tail map itself, keyed by `&str` borrowed from the builtins. It returns two private
carriers local to `compile.rs`: `CompiledGraph` (plan, report, pool classes) and `GraphFailure`
(effects, diagnostics). No `#[cfg(test)]`, feature-gated or crate-private builtins-less entry
remains. `compile_with_builtins` gained a doc comment saying builtins are required.

### Docs updated

- `crates/graph-compiler/src/compile.rs`: the module doc, the pool-class contributor comment (was
  `:435-439`) and the `required_bindings` comment (was `:803-815`).
- `crates/graph-compiler/src/lib.rs`: the `GraphEvidence` doc ("never built by `compile`") now links
  `compile_with_builtins`.
- `crates/builtins-compiler/src/lib.rs:3988-3991` (`SessionPoolClasses`): the input-builtins term
  is always present.
- `crates/graph/src/program.rs` (`is_builtin_stage`), `crates/graph/src/program/tests.rs:212` and
  `crates/graph/src/lib.rs:6247`: these described `GraphCompiler::compile` as a live producer of
  unlisted builtin stages. They now say it was deleted by #959, and that only a hand-built plan
  can leave a stage unlisted until #958. These are doc comments only.
- `docs/rulings/effect-floor-accounting.md:537`: the retired-plumbing paragraph records that #959
  deleted the entry.

The remaining "builtins-less" mentions in `crates`, `tools` and `scripts` are history ("until
#964 …", "retired by #956"), and were left alone.

### Gates

All run with `CARGO_INCREMENTAL=0`, the worktree's own `target/`, and `set -o pipefail`.

| gate | command | result |
|---|---|---|
| grep 1 | `rg -nP 'GraphCompiler::compile(?!_with_builtins)\|GraphCompileRequest\|PreparedGraphArtifact\b' crates tools hosts` | no match (exit 1) |
| grep 2 | `rg 'PreparedGraphPlan::new' tools` | no match (exit 1) |
| also | `rg 'GraphCompileFailure\|compile_with_builtin_tails\|prepared_builtins: Option' crates tools hosts docs` (without `docs/handoffs`) | no match |
| fmt | `cargo fmt --all --check` | pass |
| clippy | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| doc | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | pass |
| graph-compiler dev | `cargo test --locked -p graph-compiler` | pass: lib 80, `graph_fixture` 2, `route_gain` 3, `scale` 2 (60.4 s), `track_delay` 8, doc 6 |
| graph-compiler release | `cargo test --locked --release -p graph-compiler --config 'profile.release.panic="unwind"'` | pass, same counts (`scale` 17.3 s) |
| CI feature set | `cargo test --locked -p graph-compiler --features graph/test-support,builtins-compiler/test-support` | pass, same counts |
| graph | `cargo test --locked -p graph` | pass: lib 109, rt10 1, rt1 1, rt9 1 |
| builtins-compiler | `cargo test --locked -p builtins-compiler --features test-support` | pass: 58, 9, 3, 6, 1, 2 |
| host-core | `cargo test --locked -p host-core --all-features` | pass (every binary, lib 86) |
| capi | `cargo test --locked -p capi` | pass (`resource_lifecycle` 32, doc 4) |
| console-workload | `cargo test --locked -p console-workload` (dev) | pass: 8, 4, 23, 3 |
| tools, CI release mode | `cargo test --locked --release -p audit -p bench -p console-workload` | pass: audit 49, bench 64 (1 ignored), console-workload 8/4/23/3 |
| other tools | `cargo test --locked -p native-pcm-runner -p parameter-metadata` | pass: 19, 1; 18, 5 |
| wasm32 | `cargo check --locked --target wasm32-unknown-unknown -p host-core -p host-web` | pass |
| fixtures | `target/debug/graph_fixture --check` | pass (exit 0) |
| determinism | `bash scripts/check-graph-determinism.sh` | PASS (100/100) |
| graph policy | `bash scripts/check-graph-policy.sh .`; `bash scripts/test-graph-policy.sh` | PASS; ok |
| realtime policy | `bash scripts/check-realtime-policy.sh`; `bash scripts/test-realtime-policy.sh` | ok (57 marked regions in 16 files); ok |
| workspace policy | `bash scripts/check-workspace-policy.sh` | ok |
| C ABI, SDK, wire | `git diff --stat 792a1e87..1fea862a` | untouched: no file under `crates/capi`, `crates/protocol`, `hosts` or any SDK changes |

**Console digests and unit census (hard stop): unchanged.** A scratch probe (`zz_probe_959.rs`) was
copied into `tools/console-workload/tests/` only while it ran, and never committed. It builds every
native session row (`native_session_rows()`, 17 rows) at Simd8, Simd4 and Scalar, renders 64 blocks,
and prints one line per row and width, 51 lines in all. Each line has:

- the output SHA-256;
- `bank_shape`, transposes, `symmetry_counters` (the unit census) and `bank_symmetry_counters`;
- the collapse counters and transitions, route folds, scatter redirects and structural mono tracks;
- every `unit_eligibility` row.

The command was `cargo test --locked --release -p console-workload --test zz_probe_959`. Results:

- the base (`792a1e87`, the change stashed) and the head (`1fea862a`) print the same 51 lines,
  byte for byte;
- both also equal `cw-after.txt`, #962's recorded probe output. For example,
  `sixty_four_track_gain_pan_ring` at Simd8 is `01e465a7…2dfdb4`, census `[129, 129]`.

**Graph fixtures: unchanged.** `graph_fixture`'s fingerprint (default mode) and its `--manifest`
output are the same bytes on the base and the head.

The timed benchmark was not run.

### Mutations (applied one at a time to the head, then reverted)

Run with `cargo test --locked -p graph-compiler --lib --test track_delay`.

| id | mutation | result |
|---|---|---|
| 959-1 | `PostFader` left out of `required_bindings` | RED: 66 lib tests fail, including `builtins_replace_only_the_three_internal_track_bindings` |
| 959-2 | builtin scalar-owner charge replaced by a zero estimate | RED: `live_scalar_owner_bytes_are_published_and_capped_before_binding` |
| 959-3 | input-section tail ignored (`Finite(0)` for every track) | RED: `builtins_replace_only_the_three_internal_track_bindings` |
| 959-4 | the input-section symmetry witness is not conjoined into `pool_classes` | **survives** graph-compiler's lib and `track_delay` tests, console-workload's dev suite, and the 51-line probe (identical) |

### Deviations and notes for the verifier

- **Overlap with #958.** #958's S1 first bullet is "`compile.rs:803-835` lists the three builtin
  stages unconditionally". That falls out of this issue's own requirement: with no
  `Option<&PreparedBuiltinsSession>`, the `builtin_stages_bindable` switch has no false case. So it
  is done here.
  - #958 keeps everything else in `crates/graph`: `program::lower`'s `bindable` parameter,
    `is_builtin_stage`, the #925 tests and `MUTATIONS.md`.
  - Only the doc comments above changed in `crates/graph`, and no graph code.
- **The tail fallback is now an `expect`.** This is the one arm where "delete the `None` case"
  adds a panic path. The invariant it relies on is checked by `validate_for_session`
  (`builtin.prepared.tail_set`), which runs before `compile_graph` in the only caller. Mutation
  959-3 shows the tail source is pinned.
- **Coverage gap, pre-existing (959-4).** No compile-level test and no console row has a track
  whose prepared input section is channel-asymmetric while its source is mono. So the input-builtins
  `DESIGNED` term, now unconditional, is not pinned in graph-compiler or console-workload. The
  builtins-less entry left this arm untested before too. Suggested follow-up: a graph-compiler test
  with a mono-source track whose input section differs L/R, asserting its pool class is `Stereo`.
  Not added here, because it is outside this slice.
- The private carriers `CompiledGraph` and `GraphFailure` exist because the pipeline has to hand
  the effects back on failure while the builtins stay borrowed. They are not an entry: nothing
  outside `compile.rs` can name them.
