# Revert the alias lowering of unlisted builtin stages

Scoping study: `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md` (owner-directed removal, 2026-09-27; option (b) per the owner's second ruling below).

## Amendments (adversarial verification, 2026-09-27; override conflicting text)

1. **Authorized paths:** `crates/graph-compiler/src/compile.rs`, `crates/graph/src/program.rs`,
   `crates/graph/src/program/tests.rs`, `crates/graph/src/lib.rs` (the `lower` call only), the
   #925 tests in `crates/graph` and `crates/graph-compiler`, `crates/graph/tests/MUTATIONS.md`,
   and this spec.
2. Simulating the revert turns red exactly: the two #925 tests, the B test and 2 of the 4 P tests.
   It moves no fixture byte. Evidence: `docs/handoffs/builtins-less-removal-2026-09-27/VERIFY.md`.
3. **Order:** land after #964 (the graph-compiler test port), so the same tests are not churned
   twice.

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

## Smallest closable slice

- **S1. Revert #925's alias arm.**
  - `compile.rs:803-835` lists the three builtin stages unconditionally.
  - `program::lower` loses `bindable` (`program.rs:544-581`, `:649-664`) and `is_builtin_stage`.
  - Delete the #925 tests (study §3.2 and §3.1), and drop the `builtins_bound` argument at its 28
    call sites.
  - The graph-compiler B test and the P tests' builtins-less arms go back to the identity-bound
    shape or are deleted.
  - Gate: every with-builtins program is unchanged. The predicate already reduces to
    `is_alias_candidate` there.

## Dependencies

#957.

## Standing rules for the implementer

- Class A for every with-builtins plan: every program, digest and unit census unchanged is a hard stop.
- Commit on `codex/<issue>-<slug>`. Do not run timed benchmarks.



## Attempt 1 evidence

Implementer: Claude Opus 5.5, 2026-09-27, branch `codex/958-revert-alias-lowering` on #959's
branch head `37bf68bd`. Implementation commit `9327bf97`: 5 files, +64 / -722.

### What was removed

The `compile.rs` bullet of S1 was already done by #959, which deleted the
`builtin_stages_bindable` switch: the three builtin stages are listed unconditionally. What remained
was the graph side.

- **`crates/graph/src/program.rs`:**
  - `is_builtin_stage`, with its doc;
  - the `bindable` parameter of `lower`, `lower_with` and the `#[cfg(test)]`
    `lower_with_per_bank_windows`;
  - the `listed` interning;
  - the `!listed[index]` clause. The elision predicate is `is_alias_candidate(id)` again, as
    before #925.
  - Docs: the module doc's argument list, the `bindable` paragraph of `lower`'s doc and the
    `is_alias_candidate` doc. That doc is restored to its pre-#925 sentence, plus a three-line
    history note. The window-hold paragraph now says "#925 elided", in the past tense.
- **`crates/graph/src/lib.rs`:**
  - `lower_from_current_fields` no longer passes `&self.required_bindings`. That is the `lower`
    call.
  - Its doc and the comment in `lowered` no longer cite `program::is_builtin_stage`. The `lowered`
    comment is restored to its pre-#925 wording.
- **`crates/graph-compiler/src/compile.rs`:** a comment only. The `required_bindings` comment now
  says #958 reverted the elision and that `program::lower` no longer reads this set.

### Tests deleted or reshaped

| test | fate | reason |
|---|---|---|
| graph `tests::identity_bound_builtin_stages_alias_without_moving_a_bit` and its corpus helpers (`splitmix`, `hostile_sample`, `HostileSource`, `PlaneRecorder`; 347 lines) | deleted | #925 gate 1. It compared the unlisted (aliased) arm with the listed arm, and the unlisted arm no longer exists. The helpers had no other user. `runtime.rs` has its own `splitmix` and `hostile_sample`. |
| graph `program::tests::unlisted_builtin_stages_lower_as_aliases` | deleted | #925 gate 2: it pinned the elision itself. |
| graph `program::tests::lowering_preserves_dataflow_and_bounds_the_arena_on_random_graphs` | reshaped: partial-listing arm deleted | This is the study's "random-graph arm". The arm lowered each corpus graph with a seeded subset of builtin stages listed. The subset had its own seed, so the main corpus is unchanged. |
| `builtins_bound` helper, and its argument at 27 call sites; the `transparent` helper; the `bindable` parameter of `evaluate_spec` and `assert_program_matches_spec` | removed | These are the mechanical call-site edits. `crates/graph/src/program/tests.rs` is now **byte-identical to its pre-#925 content** (`git diff ba3b3a25^ HEAD -- crates/graph/src/program/tests.rs` is empty). Only #925 and #959 had touched it since. |
| graph `tests::…` E9 fixture (the rack-boundary tap test) | comment only | It still lists and identity-binds `PostInputBuiltins` and `PostMatrix`, which is harmless and matches a compiled plan. The comment no longer says an unlisted stage is an alias. |
| graph-compiler B (`accepted_session_compiles_binds_and_renders_direct_route`) and P tests (`builtins_replace_only_the_three_internal_track_bindings`, `the_merged_span_hold_costs_the_input_slots`, and the other two) | no change | #964 had already deleted their builtins-less arms. All 80 lib tests are green. |

`crates/graph/tests/MUTATIONS.md`: the #925 section is marked **Retired (issue #958)**, in the same
form as #957's retirements. Its rows stay as history.

### Identical-output probe (class A)

**Method.** A scratch copy of #959's probe (`zz_probe_958.rs`) was copied into
`tools/console-workload/tests/` only while it ran, and never committed. It builds every native
session row (`native_session_rows()`, 17 rows) at `Simd8`, `Simd4` and `Scalar` and renders 64
blocks. For each of the 51 row/width pairs it prints:

- the output SHA-256;
- `bank_shape`, transposes and `symmetry_counters` (the unit census);
- bank symmetry, collapse counters and transitions, route folds, scatter redirects and mono tracks;
- every `unit_eligibility` row.

Scratch instrumentation, also never committed, wrapped `lower_from_current_fields` on both trees.
It appended one line per lowering: plan id, node, op and tap counts, arena buffers, and a hash of
the whole `ExecutionProgram` `Debug` text. The base run also logged every `lower_with` call that
had an unlisted builtin stage (`ALIAS925`). Base is `37bf68bd`, head is `9327bf97`'s tree, and the
command is `cargo test --locked --release -p console-workload --test zz_probe_958`.

Base and head are byte-identical:

| output | lines | result |
|---|---:|---|
| console rows: digest, census, eligibility | 51 | identical. It also equals #959's recorded `cw-after-959.txt`. |
| programs lowered while building those rows | 136 | identical: every op, input, tap, buffer and `node_op` entry |
| `graph_fixture` fingerprint (default mode) | 1 | identical |
| `graph_fixture --manifest` | 8 | identical. It also equals #959's `manifest-after.txt`. |
| programs `graph_fixture` lowered | 2 | identical |

**The predicate already reduced to `is_alias_candidate` on every with-builtins plan (base
census).** With `ALIAS925` logging on the base, the with-builtins suites logged **zero** unlisted
builtin stages. The suites were:

- `host-core --all-features`;
- `builtins-compiler --features test-support`;
- `graph-compiler`, `console-workload`, `source` and `capi`: 550 tests in all;
- the release probe and `graph_fixture`.

The only hits were hand-built plans in `crates/graph`'s own tests:

- the two #925 tests and the corpus's partial-listing arm, all deleted;
- three builtins-agnostic tests that build plans with no builtin stage listed:
  `a_plan_without_observers_makes_zero_observe_calls`,
  `fifty_random_dag_sessions_render_deterministic_nonsilent_pcm` and
  `permanent_observer_skip_moves_no_observed_or_rendered_bit`.

Those three now lower their builtin stages as identity ops again, the pre-#925 shape, and stay
green. They are hand-built graph constructions, not with-builtins plans (study §3.1, "keep").

### Gates

All run with `CARGO_INCREMENTAL=0`, the worktree's own `target/`, and `set -o pipefail`, on the
committed tree with no scratch file present.

| gate | command | result |
|---|---|---|
| fmt | `cargo fmt --all --check` | pass |
| clippy | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| doc | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | pass |
| graph | `cargo test --locked -p graph` | pass: lib 107 (109 − the two #925 tests), rt10 1, rt1 1, rt9 1, doc 0 |
| graph, test-support | `cargo test --locked -p graph --features test-support` | pass: lib 107, rt10 1, rt1 1, rt9 8 |
| graph-compiler | `cargo test --locked -p graph-compiler` | pass: lib 80, `graph_fixture` 2, `route_gain` 3, `scale` 2, `track_delay` 8, doc 6 |
| builtins-compiler | `cargo test --locked -p builtins-compiler --features test-support` | pass: 58, 9, 3, 6, 1, 2 |
| host-core | `cargo test --locked -p host-core --all-features` | pass: every binary, lib 86 |
| console-workload | `cargo test --locked -p console-workload` | pass: 8, 4, 23, 3 |
| also | `cargo test --locked -p capi -p source` | pass: 106 tests |
| fixtures | `target/debug/graph_fixture --check` | pass (exit 0); fingerprint and manifest equal the base |
| graph policy | `bash scripts/check-graph-policy.sh .`; `bash scripts/test-graph-policy.sh` | PASS; ok |
| determinism | `bash scripts/check-graph-determinism.sh` | PASS (100/100) |
| also | `bash scripts/check-realtime-policy.sh`; `bash scripts/check-workspace-policy.sh` | ok (57 marked regions in 16 files); ok |
| residue | `grep -rn 'is_builtin_stage\|builtins_bound\|bindable: &' crates tools hosts` | no match |

The timed benchmark was not run.

### Deviations and notes for the verifier

- **`crates/graph/src/lib.rs` beyond the `lower` call.** Three comment-only edits sit next to it.
  Each would otherwise cite the deleted `program::is_builtin_stage` or describe an elision that no
  longer happens:
  - the doc of `lower_from_current_fields`, the function that makes the call;
  - the comment in `lowered`, its caller;
  - the E9 fixture's comment.

  No code outside the `lower` call and the deleted #925 test changed.
- **`crates/graph-compiler/src/lib.rs` is untouched.** Its remaining #925 mentions are history in
  the P tests' docs, for example "the identity post-input copy level that #925 removes" at
  `:7503`. The brief's "B and P tests" bullet was already done by #964.
- **No new mutation rows.** The change is a pure removal. The class-A claim is carried by the
  probe's byte identity and the base census above, not by a new gate.

## Sol attempt 1 verdict: PASS

Reviewer: Claude Opus 5.5 (Sol, adversarial verifier), 2026-09-27. Tree: branch
`codex/958-revert-alias-lowering` at `403afba9` (implementation `9327bf97`, base `37bf68bd`).
Nothing was pushed. The timed benchmark was not run. The scratch probes were never committed.

### Findings, by severity

No blocking findings.

1. **Low (doc precision).** `crates/graph/src/program.rs:189` says whether a node keeps its op is
   "decided by its kind alone". `crates/graph/src/lib.rs:1398` says it is "decided by node kind
   alone". The predicate at `program.rs:622-628` is the node's kind *and* the condition of one main
   input, no sidechain and no PDC delay: a delayed rack boundary keeps its op
   (`a_delayed_stage_boundary_is_not_elided`). The intended point is that `required_bindings` no
   longer decides. That is right, but the wording overstates it. Doc-only. Fix it the next time
   either file is touched.
2. **Low (stale tense, outside the authorized paths).** `crates/graph-compiler/src/lib.rs:7503`
   still says "the identity post-input copy level that #925 removes". That change is now reverted,
   and the plan it applied to no longer exists. Leave it for a later doc sweep.
3. **Info.** `scripts/run-console-benchmark.sh:127-133` describes the #925 alias arm as part of the
   `--plumbing-floor` arm pair. That text records a retired arm pair (#956's area). It makes no
   claim about how lowering works now. No action is needed for #958.
4. **Info.** The E9 fixture keeps the two identity bindings #925 added (`crates/graph/src/lib.rs`,
   the rack-boundary tap test). They are harmless, and the comment is now correct. Lowering no
   longer reads the listing. A builtin stage that keeps its op binds as `NodeKind::Identity`
   whether it is unlisted or identity-bound (`runtime.rs:4346`).

### 1. Class A: every with-builtins program is unchanged

**Structural argument.**

- The only non-test constructor of `PreparedGraphPlanParts` is `compile.rs:812`.
- There, `required_bindings` is every schedule node of kind `Input`, `PostInputBuiltins`,
  `PostFader`, `PostMatrix` or `Output` (`compile.rs:796-810`).
- `lower` refuses a schedule that is not exactly the spec's nodes (`ScheduleMismatch`). So every
  builtin-stage node of every compiled plan is listed.
- No non-test code mutates `required_bindings`. The only mutations are at `lib.rs:4077` and
  `:6087` and `runtime.rs:11155-11159` and `:13700-13702`, and all four are tests. Builtin-bank
  attachment and bind only read the field.
- So on every with-builtins plan the old predicate,
  `is_alias_candidate || (is_builtin_stage && !listed)`, equals `is_alias_candidate`.
- Split pairs, mono collapse, observers and meters act at bind time or at run time, and never feed
  lowering. Sends from builtin taps and routed sidechains add *outgoing* edges to a builtin stage.
  Elision reads only a node's incoming edges.

**Randomized probe (my own).** The probe generated 600 seeded sessions:

- 1 to 11 tracks and 0 to 2 chained submixes;
- 0 to 2 launch effects per rack, with a shared program on half the seeds so that banks form;
- random bypass, mono or stereo channel mapping, per-lane input delay, trim and polarity, mute,
  and pan or a 2x2 matrix;
- a main route from a random tap, plus 0 to 2 extra sends from builtin and other taps;
- routed compressor sidechains from random taps of earlier tracks;
- random meters on all seven taps.

Each session was compiled at `Simd8`, `Simd4` and `Scalar` through both
`prepare_session_builtins` and `prepare_selected_session_builtins_between_render_calls`. Each plan
was bound, its mono collapse was armed, and 8 blocks were rendered.

That gave 3,600 artifacts with no rejects: 1,002 had sidechains, 2,514 had PDC delays and 2,568
had track delays. For each artifact the probe recorded:

- an assertion that every builtin-stage node is listed (64,710 checked, 0 unlisted);
- a SHA-256 of the lowered program's `Debug` text;
- a SHA-256 of the host planes and the meter snapshots;
- a SHA-256 of the unit eligibility.

For the base run, `37bf68bd`'s graph and graph-compiler sources were swapped into the same tree.
Base and head output are byte-identical (`cmp`).

**Console rows (my own probe).** The probe rendered the 17 native rows at `Simd8`, `Simd4` and
`Scalar` for 64 blocks. For each it recorded:

- the digest and the meters;
- the bank shape and transposes;
- the symmetry counters, and the collapse counters and transitions;
- the route folds and scatter redirects;
- the mono tracks and the unit eligibility.

All 51 lines are identical between base and head.

**`graph_fixture`.** The default fingerprint and `--manifest` are identical between base and
head, and the worktree's build prints the same. `--check` exits 0.

**Census.** A scratch copy of head logged, at every `lower_from_current_fields` call, how many
builtin-stage nodes the plan left out of `required_bindings`. The suites were:

- builtins-compiler with `test-support`;
- graph-compiler, except `tests/scale.rs`, whose run the instrumentation's linear lookup made
  quadratic (scale passes in the gate run below);
- console-workload, source and capi;
- host-core with all features.

All 541 tests were green. Across 23 test binaries there were 3,942 lowerings, and none had an
unlisted builtin stage.

### 2. Revert fidelity

- **`program/tests.rs`.** `git diff ba3b3a25^ HEAD -- crates/graph/src/program/tests.rs` is empty.
  Only #925 (`ba3b3a25`, `0973b805`) and #959 (`1fea862a`) have touched the file since. #959's edit
  was one doc line on a #925 test that is now deleted. So the revert loses nothing that is not
  #925's.
- **`program.rs`.** Its code equals the pre-#925 code, and only docs differ:
  - the module doc now lists `banks`, which the pre-#925 doc left out;
  - there is a #925 history note;
  - the #169/#931 window-hold paragraph from `f70c8302` stays, and it is still true.
- **Call sites.** `builtins_bound` occurred 28 times at base: the definition and 27 calls, one of
  them inside the deleted random-graph arm.
- **Deleted tests.** Gate 1 with its four helpers, gate 2 and the partial-listing arm tested only
  the contrast between an unlisted and a listed stage. After the revert both arms lower to the same
  program, so that contrast no longer exists. The helpers had no other user, and the tree compiles.
- **The three tests that change shape.** They predate #925, and #925 did not edit them. Each
  derives its expected counts from the lowered program (`ops.len()`, `node_op` and `taps`), so when
  the identity ops come back both sides of each check move together.
  - `aliased > 0` still holds, through the rack boundaries.
  - `four_track_builtin_plan` lists its bank members, so its 9-op pins do not move.

  They pass for the right reason.
- **Tests VERIFY §3 predicted red.** #936 gate 2 was already ported onto a `PostSimd1` alias by
  #957 (`runtime.rs:14418-14429`). #964 had already reshaped the B and P tests.

### 3. Completeness

- **Code.** `grep -rn 'is_builtin_stage\|builtins_bound\|bindable'` finds no residue. The only
  matches are an unrelated `bindable` local in `random_dag_plan` (`lib.rs:4945`), prose, and the
  retired ledger rows. No caller passes `required_bindings` to `lower`.
- **Docs and rulings.** `docs/rulings/effect-floor-accounting.md:531-539` and the graph-compiler
  test docs mention #925 only as history.
- **The three `lib.rs` edits beyond the call.**
  - The `lowered` comment is restored byte for byte to its pre-#925 text.
  - The `lower_from_current_fields` doc and the E9 comment are doc-only and correct, apart from
    finding 1.
- **`compile.rs`.** The change is a comment only, and it is accurate.
- **Scope.** The diff touches 6 files. All are inside Amendment 1, and the `lib.rs` comment edits
  are disclosed.

### 4. `MUTATIONS.md`

The #925 section carries a retirement note in the same form as #957's. Rows 925-1 to 925-5 all
mutate code that is now deleted: `is_builtin_stage`, `listed`, the bindable seam, and the
builtins-less compile. `builtins_replace_only_the_three_internal_track_bindings` still pins the
with-builtins op list. Nothing live is lost.

### 5. Gates

Run in the worktree with `CARGO_INCREMENTAL=0` and its own `target/`, which was deleted afterwards.

| gate | result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | pass |
| `cargo test --locked -p graph` | lib 107, rt10 1, rt1 1, rt9 1, doc 0 |
| `cargo test --locked -p graph --features test-support` | lib 107, rt10 1, rt1 1, rt9 8 |
| `cargo test --locked -p graph-compiler` | 80, 2, 3, 2, 8, doc 6 |
| `cargo test --locked -p builtins-compiler --features test-support` | 58, 9, 3, 6, 1, 2 |
| `cargo test --locked -p host-core --all-features` | 18 binaries green (lib 86) |
| `cargo test --locked -p console-workload` | 8, 4, 23, 3 |
| `target/debug/graph_fixture --check` | exit 0 |
| `bash scripts/check-graph-policy.sh .`; `bash scripts/test-graph-policy.sh` | PASS; ok |
| `bash scripts/check-graph-determinism.sh` | PASS (100/100) |
| `bash scripts/check-realtime-policy.sh`; `bash scripts/check-workspace-policy.sh` | ok; ok |
