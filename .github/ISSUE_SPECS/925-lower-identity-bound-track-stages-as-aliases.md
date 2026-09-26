# Lower identity-bound track stages as aliases

**Ruled** (coordinator, final): option 1, keyed on `required_bindings` at compile time, no new
struct field; see "Decision" below. Drafted from the measured breakdown in `DRAFTS/PLAN.md`,
Part 2, table A.

## Product outcome

On a plan compiled without builtins (`GraphCompiler::compile`), every track's
`PostInputBuiltins`, `PostFader` and `PostMatrix` stage is a required binding that the host
acknowledges with `GraphNodeBinding::identity`, and each is lowered as a plain identity op.
`PostInputBuiltins` is dedicated storage by node kind, so its op copies its input; `PostFader`
reads a dedicated buffer, so its op copies again; `PostMatrix` is in place and does nothing but
dispatch. On `sixty_four_track_plumbing_only` that is 192 of the block's 321 units and 20,600 of
its 37,500 cycles (55 %): three copies of every track's block per block, and 192 dispatches of
about 57 cycles net that compute nothing. Lower the three stages of a builtins-less plan as aliases,
exactly as the three rack boundaries already are: no op, no buffer, no unit. The track's route
then runs in place over the Input's own buffer. Class A: an identity copy moves no bit and an
in-place identity computes nothing, so the rendered words are the same words.

## Root evidence

- `crates/graph-compiler/src/compile.rs:803-816`: `required_bindings` of the builtins-less
  `compile` (`:139`) lists `Input | PostInputBuiltins | PostFader | PostMatrix` and `Output`.
- `crates/graph/src/program.rs:186-194` `is_alias_candidate`: only
  `PostSimd1 | PostDynamic | PostSimd2PreFader`; `:610-617` elides an alias candidate with one
  main input, no sidechain and no delay, and `:619-633` resolves reads through the alias chain.
  Its comment at `:180-186` ("The other four stages ... are all bindable and keep their ops") is
  what this issue changes.
- `program.rs:226-231` `is_dedicated`: `PostInputBuiltins` by kind, `Output` by kind; `:716-720`
  in place iff sole undelayed reader **and** neither end is dedicated. So the `PostInputBuiltins`
  identity op copies (its output is dedicated), the `PostFader` identity op copies (its input is
  dedicated), the `PostMatrix` identity op is in place.
- `crates/graph/src/runtime.rs:2873` `execute_op`: an identity op with one input reaches
  `reduce_plane` (`:371`), whose `[single]` arm `copy_from_slice`s unless `single == out`; the
  `NodeKind::Identity` arm (`:2940-2952`) then does nothing without a split pair.
- `crates/graph/src/lib.rs:1195` `lower_from_current_fields` lowers from the plan's fields only;
  `:1212-1222` `lowered` refuses a required binding whose node the lowering elided.
- `tools/console-workload/src/lib.rs:1620` binds every non-Input required node to the identity;
  `crates/host-core/src/prepare.rs:1376` does the same for a with-builtins artifact's
  `external_binding_nodes()` (where the three stages are bank members or split pairs, never
  identities).
- Measured (`DRAFTS/PLAN.md` table A): identity-copy units 128 x 129 cycles = 16,534; identity-
  alias units 64 x 63 = 4,060; the copy itself is 66 cycles per unit (table C), the rest dispatch.

## Decision: option 1, keyed on `required_bindings` (ruled)

**The mechanism (no new field).** `compile` (`crates/graph-compiler/src/compile.rs:139`) drops
`PostInputBuiltins | PostFader | PostMatrix` from `required_bindings` (`:803-816`) when
`prepared_builtins.is_none()` (`:148`); `compile_with_builtins` (`:68`) is untouched.
`program::lower` (`crates/graph/src/program.rs:507`) takes the bindable set as `&[GraphNodeId]`
-- already `self.required_bindings` at the one call site, `lower_from_current_fields`
(`crates/graph/src/lib.rs:1195`) -- and the alias predicate becomes
`is_alias_candidate(id) || (is_builtin_stage(id) && !bindable.contains(id))`, where
`is_builtin_stage` is the three stages. A hand-built plan that lists `PostFader` keeps its op (the
test at `lib.rs:5728` still binds `Scale` there), and `lowered()`'s elided-binding refusal
(`:1212-1222`) stays meaningful: a listed stage is never elided, an unlisted one never bound.

**Why this and not the others (recorded for the implementer).**
- A `builtins_attached` field on `PreparedGraphPlanParts` (`lib.rs:1562`) breaks 24 constructors
  in 11 files outside the authorized paths; the bindable set is already a field of the plan.
- Every host compiles through `compile_with_builtins` (`crates/host-core/src/prepare.rs:1195`),
  where the three stages are compiler-owned bindings that keep their ops; nothing changes for
  any with-builtins plan. Attaching builtins later is a new compile and a plan swap, never a
  retarget of a builtins-less stage, so the D2 liveness ruling is untouched.
- Option 2 (bind-time lowering keyed on identity bindings, `processor: None` at `lib.rs:1814`)
  would make `program()` and the bound program differ, against #99 F2's derive-once-and-gate
  contract.
- The no-ruling alternative (keep the three ops, un-dedicate an identity-bound
  `PostInputBuiltins`) removes the two copies (about 8,400 cycles) and leaves 192 dead dispatches
  (about 11,000).
- Consequence: **a builtins-less plan has no bindable builtin stages**; a host cannot supply its
  own fader or matrix processor there (no host does today).

## Smallest closable slice

Authorized paths: `crates/graph-compiler/src/compile.rs` (`required_bindings` only),
`crates/graph/src/program.rs` (`lower`'s parameter, the alias predicate, the comment at
`:180-186`), `crates/graph/src/lib.rs` (`lower_from_current_fields`, `lowered`'s comment), their
tests, `crates/graph/src/program/tests.rs`, `tools/console-workload/tests/chain_shape.rs` (one new
test), `tools/console-workload/src/lib.rs:869-873` (comment only),
`docs/rulings/effect-floor-accounting.md:500-502` (one clause), and this spec.

1. The `required_bindings` change in `compile.rs` (`:803-816`, conditional on
   `prepared_builtins.is_none()`).
2. `lower(spec, schedule, levels, delays, bank_members, bindable)` with the predicate above.
   Nothing else in `lower` changes: the existing alias machinery (elision `:610-617`, `reads_of`
   resolution `:619-633`, taps for observers) does the rest.
3. Correct the three statements this issue falsifies or makes true: `program.rs:180-186` ("The
   other four stages ... keep their ops" -- false after this issue: they keep their ops only when
   listed as bindable); `docs/rulings/effect-floor-accounting.md:500-502` and
   `tools/console-workload/src/lib.rs:869-873` (both claim "every `TrackStage` lowers to an elided
   alias" on the plumbing row -- false today, true after this issue; say so with the date).
   #885's recorded clarification "a `PostMatrix` node is never elided" becomes builtins-plans-only;
   note it in the spec's evidence, not in #885's body.
4. Tests, below.

## Non-goals

No change to a with-builtins plan (every standing console row but the plumbing row), to
`is_dedicated` for bank members or effects, to the Output node's dedication (#916), or to the
route fold. Option 2 and the no-ruling alternative are not implemented.

## Objective gates

1. Graph test: a three-track builtins-less plan with hostile input (signed zeros, subnormals,
   magnitudes `2^-24..2^25`, exact `-0.0` on one route coefficient) renders bit-identical output
   over 16 blocks with the three stages unlisted and with them listed (the old shape; bound to
   the identity); the lowered program with them unlisted has exactly `2 x tracks + 1` ops and no
   dedicated buffer but the Output's.
2. Graph test: with builtins attached the three stages still lower as ops (pin the op count of an
   existing with-builtins fixture); a plan whose `required_bindings` omits a builtin stage lowers
   it as an alias, and one that lists it keeps the op (the `lib.rs:5728` `Scale`-on-`PostFader`
   test is the standing instance and must not move).
3. console-workload: `sixty_four_track_plumbing_only` digest over 64 blocks unchanged from the
   base commit's (pin the base digest in-test, as the copy-removal records pin
   `38ebb48908b62b9770ac7df1a8f6f2427bfd15eb849429219f8705a3926084c3`); a new test pins the row's
   `unit_eligibility().len() == 129` (rows are per unit, `lib.rs:2623`) and its bank shape at
   `[0, 0]`; `the_plumbing_row_binds_no_strip_at_all`,
   `every_standing_workload_folds_one_route_per_track` and the mono census `[65, 129]` unchanged.
4. Red mutations in `crates/graph/tests/MUTATIONS.md`: elide the three stages unconditionally
   (gate 2's listed case and every with-builtins bind fail); keep `PostInputBuiltins` dedicated
   while aliasing it (gate 1's op count fails); alias only `PostMatrix` (gate 1's op count fails).
5. `cargo test -p graph` (with and without `test-support`), `-p graph-compiler`,
   `-p console-workload`, `-p host-core`; `scripts/check-graph-determinism.sh`,
   `check-graph-policy.sh`, `check-realtime-policy.sh`.

## Console benchmark rows

Can move: `sixty_four_track_plumbing_only` only (removes the identity-copy and identity-alias
phases, 55 % of the block). Every with-builtins row must not move a bit or a unit.

## Dependencies

None. Convenient before "Fuse in-place routes into the Output reduction in pairs" (fewer units
for its in-between scan) and required before "Read plain-strip sources in place" (the route must
be in place over the Input's buffer).

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes,
  loads, stores, copies or branches. Every gate above that says "bit-identical" is a hard stop.
- The owner's copy rule: a block-sized copy on the render path exists only with a written
  justification that no in-place or direct-write form exists. This issue removes 128 such copies.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh`
  is mandatory). `crates/graph` stays free of `unsafe`.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features
  -- -D warnings`, and the focused tests named above before every checkpoint. Commit on a
  `codex/<issue>-<slug>` branch from synchronized `main`.
- Do not quote a projected saving. The cycle's paired console benchmark runs once at the batch
  boundary.
- The AudioWorklet artifact pin and browser qualification are repinned once at the batch boundary.
- Source of these findings: `.github/ISSUE_SPECS/DRAFTS/PLAN.md` (this cycle's measurement),
  `docs/rulings/effect-floor-accounting.md` ("Plumbing inventory").

## What the implementer will hit

- `lowered()`'s comment (`lib.rs:1213-1217`) says the compiler never asks for a binding on an
  alias candidate; after this issue an unlisted builtin stage is an alias candidate too, so the
  comment and the test at `lib.rs:5661-5670` (an elided binding is refused) need the bindable set
  in their fixtures.
- The plumbing row's `unit_eligibility()` census shrinks from 321 to 129 rows; grep
  `tools/console-workload/tests/` for any pinned unit count on that row before running gate 3.
- `program/tests.rs` builds `lower` directly; every call site gains the flag.
- The floor ruling's sentence "pays sixty-four individually dispatched route ops and an unfolded
  reduction" (`docs/rulings/effect-floor-accounting.md:527-529`) is still true after this issue;
  only draft B changes it.

## Attempt 1 evidence

Terra, branch `codex/925-alias-identity-bound-stages` from `3c93469d`. Local commits, none
pushed: `ba3b3a25` (the change), `6b36b0ff` and `f70c8302` (graph-compiler tests, under the scope
amendment below), `0973b805` (gates 1 to 3 and the corrections), `b1d9a5eb` (mutation rows), and
the commit that adds this record.

### Design

- **Where `required_bindings` is trimmed.** `compile_with_builtin_tails`
  (`crates/graph-compiler/src/compile.rs`, the `required_bindings` filter after the `spec` is
  assembled): `Input` and the `Output` are always listed. `PostInputBuiltins | PostFader |
  PostMatrix` are listed iff `prepared_builtins.is_some()`, which is true on the
  `compile_with_builtins` path and false on `compile`. Nothing else in the compiler changes. The
  compiler's own estimate and `buffer_assignments` never read `required_bindings`, and neither
  does the canonical text or SHA.
- **The predicate.** `program::lower(spec, schedule, levels, delays, banks, bindable)`, and
  likewise `lower_with` and the test-only `lower_with_per_bank_windows`. `bindable` is interned
  once into `listed: Vec<bool>` by `node_index`, so there is no per-node linear `contains`, and
  unknown ids are ignored. The elision predicate is
  `(is_alias_candidate(id) || (is_builtin_stage(id) && !listed[index])) && main_in.len() == 1 &&
  side_in.is_none() && edge_delay.is_none()`. The new `const fn is_builtin_stage` names the three
  stages. Nothing else in `lower` changes: elision, `reads_of` resolution through the alias
  chain, taps and colouring are the existing machinery.
- **The seam.** `PreparedGraphPlan::lower_from_current_fields` passes `&self.required_bindings`.
  So `program()`, `attach_builtin_banks`'s re-derivation and bind-time `lowered()` read the same
  field. #99 F2 (derive once, gate on every compile) holds.
- **What a with-builtins plan still does.** Exactly what it did. `compile_with_builtins` lists all
  three stages of every track, so `listed` is true for every builtin stage and the predicate
  reduces to `is_alias_candidate`, the pre-#925 lowering. Builtin bank members are listed by
  construction (`attach_builtin_banks` refuses a member that is not in `required_bindings`), so
  a bank member is never elided. Measured, not only argued: every standing console workload was
  rendered for 64 blocks on `3c93469d` (separate target directory) and on this branch. For all 15
  with-builtins rows, digest, `[chains, slots]`, transposes, unit count and symmetry census are
  identical. The plumbing row keeps its digest and changes only in units and census, as below.
- **Hand-built plans.** A plan that lists a builtin stage keeps its op. The E9 test binds
  `Scale(0.375)` to a listed `PostFader` and is unchanged in its assertions. `lowered()`'s
  elided-binding refusal still means what it says: a listed stage is never elided, and an
  unlisted one is never bound.
- **Consequence.** A builtins-less plan has no bindable builtin stage, so a host cannot supply its
  own fader or matrix processor there. No host does: every host compiles through
  `compile_with_builtins`.
- **#885's clarification** ("a `PostMatrix` node is never elided") now holds for plans that list
  their `PostMatrix` stages, which is every with-builtins plan. A builtins-less plan's
  `PostMatrix` is elided, but such a plan has no builtin banks, so the alias branch of
  `served_by_the_resident_lane` stays unreachable in practice, as #885 recorded.
- **Render path.** `runtime.rs` is untouched and no render-path code changed. The plan simply has
  fewer ops. `crates/graph` adds no `unsafe` and does not name `wide` (`check-graph-policy.sh`).

### Unit census of `sixty_four_track_plumbing_only`

| | before (`3c93469d`) | after |
|---|---|---|
| units | 321: 64 bound, 128 identity-copy, 64 identity-alias, 64 route, 1 output | 129: 64 bound, 64 route, 1 output |
| `symmetry_counters` | `[257, 321]` | `[65, 129]` |
| `[chains, slots]` / transposes | `[0, 0]` / 0 | `[0, 0]` / 0 |
| digest, 64 blocks | `57535244ba953d82f6c9c19428dc83a8ac412018c66acc167818e1917283f800` | same |

The chain per track is now `Input (bound) -> Route (in place over the input's buffer) -> Output`.

### Scope amendment (Sol)

Two tests in the `mod tests` region of `crates/graph-compiler/src/lib.rs`, outside the authorized
paths, pinned the old builtins-less shape. Sol authorized that region for three tests:

1. `accepted_session_compiles_binds_and_renders_direct_route` now expects
   `required_bindings.len() == 2` and the three stages absent.
2. `banking_a_dynamic_rack_costs_no_arena_buffers` is renamed
   `the_merged_span_hold_costs_the_input_slots_with_and_without_builtins`. It pinned banked arena
   == per-node arena == 193 on the builtins-less compile of `console-sixty-four-track.json`.
   **Finding: #169's arena-neutrality claim ("the window hold costs nothing") never held on the
   path every host compiles through.** It was measured only on the builtins-less plan, where the
   identity post-input copy level that #925 removes retired every `Input` slot outside any bank
   window. The test now pins both arms (ruling: option (i)):
   - **Builtins-less: 192 banked, 129 per node, both at most the old 193.** The EQ banks read the
     inputs directly, each EQ bank and its compressor bank form a chainable cohort pair, and the
     eight pairs' spans overlap into one. The input slots the EQ ops free are held inside it:
     64 inputs + 64 EQ + 64 compressor outputs. The hold is required, because a merged chain runs
     a cohort's compressor before the later EQ banks read their inputs. Per node there is no
     window: 64 dedicated EQ + 64 compressor outputs + the output.
   - **With builtins: 256 banked, 193 per node, identical at `3c93469d`.** This is 63 stereo
     buffers, about 63 KiB at 128 frames: memory, not copies, and unchanged by #925. The merged
     `builtins -> EQ -> compressor -> fader -> matrix` span holds the inputs: 64 inputs + 64
     post-input + 64 EQ + 64 compressor outputs. Per node, the post-input banks' one-level windows
     release them: 64 + 8 + 56 + 64 + 1.
   - Narrowing the merged-span hold is **#931**. The stale `program::lower` passage ("pins the
     193", "(193 -> 257)") now cites these pins and says the 257 was measured before #925.
3. `builtins_replace_only_the_three_internal_track_bindings` pins the with-builtins op list
   `[Input, PostInputBuiltins, PostFader, PostMatrix, Route, Output]` and, for the same session
   without builtins, `[Input, Route, Output]`.

### Tests

- Gate 1, `graph` `tests::identity_bound_builtin_stages_alias_without_moving_a_bit`:
  - Three tracks `Input -> seven stages -> Route -> Output`, 16 blocks of 16 frames.
  - Hostile input: signed zeros, subnormals, magnitudes `2^-24 .. 2^25`, fresh every block.
  - Hostile route gains and 2x2s, with track 1's `lr` exactly `-0.0`.
  - Unlisted arm against the same plan with the three stages listed and bound to
    `GraphNodeBinding::identity`.
  - Asserted bit-identical: the host planes, and every window (first sample and both planes) of a
    `PlaneRecorder` observer on each of track 1's three stages.
  - Unlisted program: `2 x tracks + 1 = 7` ops, 18 taps, 4 buffers, every route in place, and the
    output the only dedicated op. Listed program: `5 x tracks + 1 = 16` ops.
- Gate 2:
  - `graph` `program::tests::unlisted_builtin_stages_lower_as_aliases`:
    - Nothing listed: `[Input, Route, Output]`, 6 taps, route in place, 2 buffers, output the only
      dedicated op.
    - Each stage listed alone keeps exactly that op. Listing `PostInputBuiltins` brings back its
      dedicated buffer and the route's copy (3 buffers).
    - A PDC edge keeps an unlisted stage's op.
  - The standing instance: `tests::aliased_identity_stages_do_not_change_audio` (E9, `Scale` on a
    listed `PostFader`). Its fixture now lists `PostInputBuiltins` and `PostMatrix`, bound to the
    identity, and every assertion is unchanged.
  - With builtins: graph-compiler `builtins_replace_only_the_three_internal_track_bindings`
    (6 ops).
- `graph` `program::tests::lowering_preserves_dataflow_and_bounds_the_arena_on_random_graphs`
  gains an arm:
  - Each of the 300 seeded graphs is lowered again with a separately seeded subset of builtin
    stages listed, so the original corpus is unchanged.
  - The symbolic interpreter treats an unlisted builtin stage as transparent.
  - Asserted: same dataflow, every listed stage keeps its op, no other node moves, and every node
    is an op or an alias.
  - Every pre-existing `lower` call in `program/tests.rs` passes `builtins_bound(&spec)`: all
    builtin stages listed, which is the pre-#925 lowering.
- Gate 3: `console-workload` `the_plumbing_row_is_input_route_output_and_renders_the_base_bits`.
  - It pins the base digest over 64 blocks, 129 units (none banked, one lane each), census
    `[65, 129]`, `[0, 0]` and 0 transposes.
  - Unchanged and green: `the_plumbing_row_binds_no_strip_at_all`,
    `every_standing_workload_folds_one_route_per_track`, and
    `the_mono_row_pairs_unit_rows_are_pinned` (census `[65, 129]`).

### Gates

| gate | result |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | PASS, no warnings |
| `cargo test -p graph` | 106 passed (lib 103) |
| `cargo test -p graph --features test-support` | 113 passed (lib 103, rt9 8) |
| `cargo test -p graph-compiler` | 92 passed (lib 73) |
| `cargo test -p console-workload` | 30 passed, 2 ignored (the profile harness) |
| `cargo test -p builtins-compiler --features test-support` | 79 passed |
| `cargo test -p host-core --all-features` | 225 passed, 2 ignored |
| `cargo test -p bench` | 62 passed |
| `bash scripts/check-graph-determinism.sh` | PASS (100/100) |
| `bash scripts/check-graph-policy.sh` | PASS |
| `bash scripts/check-realtime-policy.sh` | PASS (53 marked regions in 15 files) |

Also green earlier on this branch: `-p audit`, `-p source`.

### Mutations

`crates/graph/tests/MUTATIONS.md`, section "Issue #925". Each row was applied alone to
`0973b805` and run over `graph --lib`, `graph-compiler --lib` and `console-workload --test
chain_shape`:

| # | mutation | result |
|---|---|---|
| 925-1 | elide the three stages unconditionally | RED: gate 2's listed case, and every with-builtins bind (graph 31, graph-compiler 23, chain_shape 22 of 23) |
| 925-2 | keep the dedicated `PostInputBuiltins` out of the predicate while the other two alias | RED: gate 1 op count (10 against 7), gate 2, gate 3, two graph-compiler pins |
| 925-3 | alias only `PostMatrix` | RED: gate 1 op count (13 against 7), gate 2, gate 3, two graph-compiler pins |
| 925-4 | `lower_from_current_fields` passes an empty bindable set | RED: graph 21, graph-compiler 23, chain_shape 22 |
| 925-5 | the builtins-less compile keeps listing the stages | RED: three graph-compiler tests and gate 3. graph stays green (no compiler there). |

### Profile (descriptive only; no claim)

`phase_profile` of `tools/console-workload/tests/plumbing_profile.rs`, `--release`,
`taskset -c 31`, 4,000 blocks per repeat, three repeats (ranges). Other implementers were building
in parallel: loadavg 2.5 before and 4.7 to 5.4 after. `kernel_replicas` was not rerun: it times
standalone replicas that #925 does not touch.

| phase (cycles/block) | before (`3c93469d`, 3.697 GHz) | after (`b1d9a5eb`, 3.702 GHz) |
|---|---:|---:|
| probes off, p50 | 11,061-11,071 ns (40,896-40,933 cycles) | 4,880-4,889 ns (18,067-18,100 cycles) |
| units | 64 bound, 128 identity-copy, 64 identity-alias, 64 route, 1 output | 64 bound, 64 route, 1 output |
| enter | 103 | 103-106 |
| source set + loop entry | 155-165 | 137-141 |
| bound units (64) | 9,648-9,726 | 7,333-7,351 |
| identity-copy units (128) | 17,757-17,788 | -- |
| identity-alias units (64) | 3,780-3,830 | -- |
| route units (64) | 8,570-8,626 | 8,014-8,038 |
| output unit (1) | 3,556-3,564 | 3,426-3,436 |

The paired console benchmark runs once at the batch boundary, and this table is not its number.

### Deviations and notes

- **Anchor drift.** `crates/graph/src/lib.rs` sat about 153 lines lower on `3c93469d` than the
  spec cites: `lower_from_current_fields` `:1348` (spec `:1195`), `lowered` `:1365`
  (`:1212-1222`), `PreparedGraphPlanParts` `:1715` (`:1562`), `GraphNodeBinding::identity`
  `:1964` (`:1814`). Every other anchor matched.
- The spec states the predicate as `bindable.contains(id)`. The implementation interns the set
  once. It is the same predicate, and the lowering is not quadratic in the track count.
- The floor ruling (`docs/rulings/effect-floor-accounting.md`, "Plumbing inventory") and the
  workload comment (`tools/console-workload/src/lib.rs`, the builtins-less compile path) keep
  their sentence and add a dated correction: false until #925, true after it.
  `program::is_alias_candidate`'s doc is rewritten. The floor ruling's "sixty-four individually
  dispatched route ops and an unfolded reduction" is still true and was not touched.
- **Build incident.** To measure `3c93469d`, a base worktree was once built into this branch's
  `target/`. Cargo keys workspace crates by workspace-relative path, so the base `graph` rlib
  replaced this branch's, and one graph-compiler run linked it. That run was discarded after
  `cargo clean -p graph -p graph-compiler`, and every number above comes from a clean build. The
  later base comparison used a separate target directory, since removed.
- The partial-listing arm of the random-graph test does not assert the arena bound
  `buffers <= ops`. That bound was written for the fully listed program, and fewer ops with PDC
  staging are not covered by its argument. The arm asserts dataflow and the op/alias structure.
