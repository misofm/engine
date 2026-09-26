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
