# Lower identity-bound track stages as aliases

**Owner ruling required** (a lowering shape; see "Decision" below). Drafted from the measured
breakdown in `DRAFTS/PLAN.md`, Part 2, table A.

## Product outcome

On a plan compiled without builtins (`GraphCompiler::compile`), every track's
`PostInputBuiltins`, `PostFader` and `PostMatrix` stage is a required binding that the host
acknowledges with `GraphNodeBinding::identity`, and each is lowered as a plain identity op.
`PostInputBuiltins` is dedicated storage by node kind, so its op copies its input; `PostFader`
reads a dedicated buffer, so its op copies again; `PostMatrix` is in place and does nothing but
dispatch. On `sixty_four_track_plumbing_only` that is 192 of the block's 321 units and 20,600 of
its 37,500 cycles (55 %): three copies of every track's block per block, and 192 dispatches of
about 65 cycles that compute nothing. Lower the three stages of a builtins-less plan as aliases,
exactly as the three rack boundaries already are: no op, no buffer, no unit. The track's route
then runs in place over the Input's own buffer. Class A: an identity copy moves no bit and an
in-place identity computes nothing, so the rendered words are the same words.

## Root evidence

- `crates/graph-compiler/src/compile.rs:803-816`: `required_bindings` of the builtins-less
  `compile` (`:139`) lists `Input | PostInputBuiltins | PostFader | PostMatrix` and `Output`.
- `crates/graph/src/program.rs:186-194` `is_alias_candidate`: only
  `PostSimd1 | PostDynamic | PostSimd2PreFader`; `:630-637` elides an alias candidate with one
  main input, no sidechain and no delay, and `:640-655` resolves reads through the alias chain.
- `program.rs:226-231` `is_dedicated`: `PostInputBuiltins` by kind, `Output` by kind; `:691-700`
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

## Decision: two mechanisms, one to be ruled on

**Option 1 (compile-time, pure fields -- recommended).** `PreparedGraphPlanParts` (`lib.rs:1562`)
gains `builtins_attached: bool`; `compile_with_builtins` (`compile.rs:68`) sets it, `compile`
(`:139`) clears it and drops the three stages from `required_bindings`. `program::lower`
(`program.rs:507`) takes the flag and, when it is clear, treats
`PostInputBuiltins | PostFader | PostMatrix` as alias candidates under the same three conditions
as `is_alias_candidate`. `lowered()`'s elided-binding refusal stays as it is and becomes the
guard: a plan with builtins attached still lowers the three as ops, and a red mutation that
elides them unconditionally fails every with-builtins bind. Consequence to rule on: **a
builtins-less plan has no bindable builtin stages**; a host cannot supply its own fader or matrix
processor there (no host does today).

**Option 2 (bind-time).** Keep `required_bindings`; `bind` (`lib.rs:1267`) passes the set of
nodes bound through `GraphNodeBinding::identity` (`processor: None`, `:1794`) into a bind-time
`lower`, which treats an identity-bound builtin stage as an alias candidate; `lowered()` accepts an
identity binding on an elided node. Keeps the door open for host processors on builtin stages at
the price of `program()` (the pre-bind description) and the bound program legitimately differing.

**Alternative without a ruling (smaller).** Keep the three ops; make `is_dedicated` false for a
`PostInputBuiltins` of a builtins-less plan (same flag as option 1) so all three run in place.
Removes the two copies (about 8,400 cycles), keeps the 192 dispatches (about 12,200).

## Smallest closable slice (option 1)

Authorized paths: `crates/graph-compiler/src/compile.rs` (`required_bindings`, the parts flag),
`crates/graph/src/program.rs` (`lower`'s parameter and the alias predicate), `crates/graph/src/lib.rs`
(`PreparedGraphPlanParts`, `lower_from_current_fields`), their tests, `crates/graph/src/program/tests.rs`,
`tools/console-workload/tests/chain_shape.rs` (one new test), and this spec.

1. The parts flag and the `required_bindings` change in `compile.rs`.
2. `lower(spec, schedule, levels, delays, bank_members, builtins_attached)`; the alias predicate
   `is_alias_candidate(id) || (!builtins_attached && is_builtin_stage(id))`, where
   `is_builtin_stage` is the three stages. Nothing else in `lower` changes: the existing alias
   machinery (elision, `reads_of` resolution, taps for observers) does the rest.
3. Tests, below.

## Non-goals

No change to a with-builtins plan (every standing console row but the plumbing row), to
`is_dedicated` for bank members or effects, to the Output node's dedication (#916), or to the
route fold. Option 2 and the alternative are not implemented unless the ruling picks them.

## Objective gates

1. Graph test: a three-track builtins-less plan with hostile input (signed zeros, subnormals,
   magnitudes `2^-24..2^25`, exact `-0.0` on one route coefficient) renders bit-identical output
   over 16 blocks with the flag clear and with a test-only override that lowers the old shape; the
   lowered program with the flag clear has exactly `2 x tracks + 1` ops and no dedicated buffer
   but the Output's.
2. Graph test: with builtins attached the three stages still lower as ops (pin the op count of an
   existing with-builtins fixture), and a plan that lists a builtin stage in `required_bindings`
   while the flag is clear is refused by `lowered()` (the existing elided-binding refusal).
3. console-workload: `sixty_four_track_plumbing_only` digest over 64 blocks unchanged from the
   base commit's (compare against the seam-declined arm in-test as `the_folded_master_is_the_
   reductions_own_bits` does); a new test pins the row's unit census at `129` and its bank shape
   at `[0, 0]`; `the_plumbing_row_binds_no_strip_at_all`,
   `every_standing_workload_folds_one_route_per_track` and the mono census `[65, 129]` unchanged.
4. Red mutations in `crates/graph/tests/MUTATIONS.md`: elide the three stages unconditionally
   (gate 2 fails); keep `PostInputBuiltins` dedicated while aliasing it (the lowering must reject
   or gate 1's op count fails); alias only `PostMatrix` (gate 1's op count fails).
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
  alias candidate; after this issue that is true only when the flag is clear, so the comment and
  the test at `lib.rs:5661-5670` (an elided binding is refused) need the flag in their fixtures.
- The plumbing row's `unit_eligibility()` census shrinks from 321 to 129 rows; grep
  `tools/console-workload/tests/` for any pinned unit count on that row before running gate 3.
- `program/tests.rs` builds `lower` directly; every call site gains the flag.
- The floor ruling's sentence "pays sixty-four individually dispatched route ops and an unfolded
  reduction" (`docs/rulings/effect-floor-accounting.md:527-529`) is still true after this issue;
  only draft B changes it.
