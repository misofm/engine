# Read plain-strip sources in place from the played transfer block

Drafted from `DRAFTS/PLAN.md`, Part 2. Depends on drafts A and B; the row it can move is the one
draft D adds (`sixty_four_track_plumbing_ring`); `sixty_four_track_plumbing_only` is not refed
(coordinator ruling).

## Product outcome

A ring-fed session copies every source claim's played planes into the claim's arena buffer at
the top of each block (`copy_track_input`), and #918 let a **bank's gather** read the played
planes instead. After drafts A and B, an unbanked track's whole chain is that copy followed by the
fused Output kernel reading the buffer once. Extend #918's in-place table to a claim whose
buffer's only reader is the fused Output kernel, and let that kernel read the played planes for
such inputs. Class A: the words the kernel reads are the words the copy would have written
(#918's contract: "the words are the ones `copy_track_input` would have written for the claim").
The bench's equivalent of the removed pass is 4,224 cycles of copy plus 64 dispatches per block
(table C); the production pass is `copy_track_input` per claim.

## Root evidence

- `crates/graph/src/lib.rs:2200-2231`: `lent_claims` are offered when the driver
  `provides_played_planes()` (`:1653`), `build_sequential` decides which are bound in place, and
  the copy loop (`:2321-2331`) is filtered by `runtime.source_in_place(claim, buffer)`
  (`runtime.rs:2289`).
- `runtime.rs:5324` `source_plane_table`: builds `source_plane_of_buffer` (`:1989`; sentinel
  `NO_SOURCE_CLAIM`, `:1451`) for **bank gathers only**; `SourceGather` (`:1456`) and
  `ArenaMembers::plane` (`:1526`) read `sources.played_planes(claim)` (`GraphSourcePlanes`,
  `lib.rs:1674`) for a lane whose `source_lanes` bit is set.
- `runtime.rs:2374-2416` `execute` passes `sources` to the Bank arm only; the Op arm (the
  Output op after draft B) never sees them.
- `tools/console-workload/src/lib.rs:1513-1575` `FrozenGraphSource` is a `GraphRuntimeProcessor`
  bound to each `Input` node; `process` copies its frozen block into the arena. The row binds no
  source set, so no claim exists to be lent.
- #918's gates and `test_only_set_source_in_place_declined` (`lib.rs:2211`) are the model.

## Smallest closable slice

Authorized paths: `crates/graph/src/runtime.rs` (`source_plane_table`, the Output op's fused
kernel input selection, `execute`'s Op arm), `crates/graph/src/lib.rs` (none expected), their tests,
`crates/graph/tests/MUTATIONS.md`, and this spec.

1. `source_plane_table`: also admit a lent claim whose buffer is an input of the fused Output op
   (draft B's `route_table` inputs) and is read by nothing else (`op_dataflow` readers), setting
   `source_plane_of_buffer[buffer] = claim`.
2. `execute`: pass `sources` to the Op arm when the unit is `output_unit`; the fused kernel forms
   each input's planes as `sources.played_planes(claim)` when
   `source_plane_of_buffer[buffer] != NO_SOURCE_CLAIM`, else `lease.read`. **Underrun rule:** on
   `played_planes(claim) == None` the kernel reads the arena's silence buffer
   (`ARENA_SILENCE_BUFFER`), exactly as `ArenaMembers::plane` (`runtime.rs:1526`) does for a bank
   lane, which is the `+0.0` block `copy_track_input` would have written.
3. `source_in_place` answers true for such a claim, so the copy loop skips it.

## Non-goals

No change to #918's bank path, to the ring (`crates/source`), to the bench's `FrozenGraphSource`,
or to a claim with any other reader (a send, an observer tap, a delayed edge): those keep the copy.

## Objective gates

1. Graph test with a driver that offers played planes: a 64-claim plan with in-place routes into
   the Output, hostile input, frames `{1, 7, 16, 128}`: host planes bit-identical to the same plan
   with `test_only_set_source_in_place_declined(true)` over 8 blocks;
   `test_only_source_plane_counts()` (`runtime.rs:4567`, `[u64; 3]`) shows 64 in-place reads and
   0 copies per block with the table on and the reverse with it off; `output_route_folds() == 64`
   both ways.
2. Declining shapes: a claim with a send tap reader, a claim under a delayed edge, a claim read by
   a submix: copied (count 1 each), output identical.
3. Underrun: a driver that answers `None` for one claim on block 3 renders that block bit-identical
   to the declined arm (whose `copy_track_input` wrote `+0.0`), with the silence buffer read and no
   copy counted.
4. Red mutations: read `lease.read` for an admitted claim after skipping its copy (stale words:
   gate 1 fails on block 2); admit a claim with two readers (gate 2); read the claim's stale arena
   buffer instead of the silence buffer on `None` (gate 3).
5. `cargo test -p graph` both ways, `-p source`, `-p host-core`; `scripts/check-graph-determinism.sh`,
   `check-graph-policy.sh`, `check-realtime-policy.sh`.

## Console benchmark rows

`sixty_four_track_plumbing_ring` only (draft D's row: the same session fed through a
`FrozenSourceDriver` that lends played planes; its `source set` phase is the copy loop this issue
removes). `sixty_four_track_plumbing_only` binds `FrozenGraphSource` processors, not a source set,
so neither #918 nor this issue can show on it, and by ruling it is not refed. "In place" has no
meaning for a bound processor: `GraphRuntimeProcessor::process` is write-only into the block it
is handed, so the frozen processor's copy *is* its contract.

## Dependencies

After "Lower identity-bound track stages as aliases" and "Fuse in-place routes into the Output
reduction in pairs" (forced: the kernel that reads the played planes is B's). After #917/#918
(merged). "Add a driver-fed plumbing row to the console benchmark" must be merged for the
batch-boundary measurement to show it.

## Standing rules for the implementer

- Work only from this body. Read the cited functions and #918's spec first; do not survey the
  workspace.
- Class A means the change moves no rendered bit. Every gate above that says "bit-identical" is a
  hard stop.
- The owner's copy rule: this issue removes a block-sized copy per claim; do not introduce one.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh`
  is mandatory). `crates/graph` stays free of `unsafe`.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features
  -- -D warnings`, and the focused tests named above before every checkpoint. Commit on a
  `codex/<issue>-<slug>` branch from synchronized `main`.
- Do not quote a projected saving.
- The AudioWorklet artifact pin and browser qualification are repinned once at the batch boundary.
- Source of these findings: `.github/ISSUE_SPECS/DRAFTS/PLAN.md`,
  `.github/ISSUE_SPECS/918-gather-banked-source-inputs-from-the-played-transfer-block.md`.

## What the implementer will hit

- The played block is released at the next `begin_block` (#917); the Output op runs last in the
  block, inside the borrow `render` forms at `lib.rs:2337-2338`, so the lifetime argument #918
  made for the bank gather holds unchanged.
- `source_plane_table` sets a per-lane bit in `UnitIdentity::source_lanes` (`runtime.rs:1888`)
  for banks; the Output op has up to 64 inputs, so its selection is by
  `source_plane_of_buffer` lookup per input, not by a bit field.
- The seam `test_only_set_source_in_place_declined` already covers both admission sites once
  `lent_claims` is empty; gate 1's declined arm needs no second seam.

## Attempt 1 evidence

Implementer: Terra. Branch `codex/927-plain-strip-sources-in-place`, based on `169a2486` (the
verified #926 tip with the verified #925 branch merged and the plumbing row's count pin retargeted).
Local commits only: `50b6a656` (implementation, gates 1-3, the kernel test), `a9ebe6ea` (the rt10
allocation test), `135d551e` (a bound-stage declining shape), `7896ac7e` (gate 2's shape order),
and the evidence commit (this section and `crates/graph/tests/MUTATIONS.md` rows 927-1 to 927-13).
Nothing pushed; no PR; the AudioWorklet pin untouched. Files: `crates/graph/src/runtime.rs`,
`crates/graph/src/lib.rs` (comments only), `crates/graph/tests/rt10_source_in_place_alloc.rs`,
`crates/graph/tests/MUTATIONS.md`, this spec.

### Design

1. **Bind: which claims the Output reads in place** (`runtime.rs` `source_plane_table`, called by
   `build_sequential` on the finished units, as for #918). #926's `OutputRouteFold` now also
   records `producers`: the retired route feeding each Output input, in edge order. Beside #918's
   bank rule (b), a lent claim is admitted by the new clause (b') when its input op's
   `op_dataflow` readers are exactly `[route]` and `route` is `producers[i]` for some Output input
   `i`, subject to #918's (a), (c), (d) and a new clause (e) (below). The function returns
   `SourcePlanes { of_buffer, output }`: `of_buffer` is `Runtime::source_plane_of_buffer` with the
   admitted claim's entry written (so `source_in_place` answers true and the copy loop, filtered at
   bind in `GraphExecutor::new` exactly as for #918, skips the claim); `output` is a new runtime
   table `Runtime::output_sources: Box<[u32]>`, one entry per Output input, in the same edge order
   as `output_routes`: the claim that input reads in place, or `NO_SOURCE_CLAIM`. Empty when no
   input is read in place.
2. **Render.** `Runtime::execute`'s Op arm hands the Output unit (and only it)
   `OutputSources { planes: sources, claims: output_sources }`; every other op gets
   `OutputSources::NONE`. `execute_op` passes it to `route_reduce`, which refuses before any write
   a claim table that is neither empty nor one entry per input, and passes it with each pair's
   first input position to `route_pair`. `route_pair` forms each input's two planes once per input
   per block through `OutputSources::input(lease, position, buffer)`: for a claimed input, the
   source set's `played_planes(claim)`, or on `None` `lease.read_stereo(ARENA_SILENCE_BUFFER)`;
   for every other input `lease.read_stereo(buffer)`, which is what `route_pair` read before. The
   loops (`route_run`, `route_tail`) are unchanged and read only those slices. One table read and
   one predictable branch per input per block, one `played_planes` call per claimed input; nothing
   is copied or allocated.
3. **Counters.** `test_only_source_plane_counts()` keeps its `[u64; 3]` shape: `[claims copied,
   reads served a played block, reads served silence]`, where a read is now a bank gather's lane or
   a claimed Output input (per input per block).

### The eligibility rule for the Output reader

A lent claim is read in place by the fused Output reduction iff:

- **(a)** its input op's unit is a plain `NodeKind::SourceInput` (not a `TrackDelay`, whose line runs
  in place over the copied words);
- **(c)** that op carries no observer, its own or an elided alias's (taps after the input op fold
  their observers into it), since an observer reads the copied words from the arena;
- **(d)** the driver lends (`provides_played_planes`);
- **(b')** its readers are exactly one op, the route #926's fold retired for Output input `i`. The
  fold admitted that route only as a plain route running in place over its one undelayed input
  (this claim's value, so input `i` is this buffer), read by the Output op alone, unobserved (the
  aliases after it included), with nothing between it and the Output naming the buffer. Retired, it
  never runs; the fused reduction is the buffer's only read. A second reader of the claim beside
  its route cannot occur (two readers keep the route from running in place and the fold declines
  outright, `program.rs` `reads_of[owner] == 1`); any other reader -- a submix, a bound stage, a
  staged (delayed) edge, a route that runs, or two such -- reads the arena and keeps the copy;
- **(e)** no op scheduled before the input op names its buffer (`op_names_buffer` over
  `program.ops[..op]`). The copy fills the buffer before the first unit, while the input op sits at
  its own place in the schedule; the colouring may hand its slot to a value that dies before the
  input op, whose producer then overwrites the copied words, and the Output reads those. The compiler
  puts every input first in level 0 (TrackStage nodes sort first), so on a compiled plan only an
  earlier input can have used the slot, and only one with no reader at all (it frees its slot at
  once); a hand-built plan can schedule an input anywhere (gate 2's `LateInput`).

The Output never looks a buffer up in `source_plane_of_buffer`: it selects each input's claim by
**position** (deviation 1).

### Lifetime argument

- **What is read, and when.** A claimed input's planes are read inside the Output unit's
  `route_reduce`, reached from `GraphExecutor::render`'s unit loop through
  `runtime.execute(unit, time.absolute_sample, host.reborrow(), sources)`, after `begin_block`
  played the block and after the copy loop (which skips the claim), and before `render` returns.
  The slices live only for that call.
- **No release point can fire in between.** The played block stops being readable only at
  `PcmSourceConsumer::end_block`, reached from the next `begin_block`, from `prepare_seek`, from the
  driver's zero-mapping path in `begin_block`, or by drop (#917); each needs the consumer `&mut`,
  reachable only through `&mut GraphPreparedSourceSet`. `render` destructures the executor into
  disjoint fields; its last `&mut` use of the set is the copy loop, and `sources =
  source_set.as_ref().map(|set| set as &dyn GraphSourcePlanes)` is a shared borrow held across the
  whole unit loop, so the borrow checker rejects any `&mut` use of the set until the loop ends. The
  error paths touch only the runtime and the host planes. `prepare_source_seek` is `&mut self` on
  the executor, excluded by `render`'s `&mut self`. The Output unit is inside that loop, so #918's
  argument holds unchanged: the planes cannot move while the kernel reads them, and the next
  `begin_block` is the next render's.
- **No aliasing.** The kernel holds `&ArenaLease` (the runtime's), the lent planes (the source
  set's), and `&mut` host planes (the caller's `output`): three disjoint owners, which the borrow
  checker proves at the `execute` call site. The kernel writes only the host planes.

### Underrun rule

`played_planes(claim) == None` (underrun, end of region, no mapping; also the set's refusal of a
plane that is not one quantum) reads `lease.read_stereo(ARENA_SILENCE_BUFFER)`, buffer 0, which no
lease can write (`disjoint.rs` I1): `+0.0` for the arena's life, exactly what `copy_track_input`
(`copy_channel`'s `destination.fill(0.0)`) writes for an unplayed quantum, and exactly what
`ArenaMembers::plane` serves a bank lane. A short block is lent as #917's `play` left it (played
frames, then `+0.0` zeroed in place), the words `copy_channel` writes; a mono mapping lends one
slice twice. Nothing falls back to the copy at render: the mode is fixed at bind.

### Why the in-place words are the copy's (class A)

Under the copy, the claim's buffer holds the copied words from before the first unit through the
input op (e), and from the input op to the Output op the colouring gives the slot to nothing else
(the value's last reader is the Output op); the input op is a no-op (a), the one reader is the
retired route, which does not run (b'), no observer reads it (c). So the fused reduction's load of
input `i` under the copy is the copied words, and in place it is the same words from the played
block (or the silence buffer's `+0.0` where the copy wrote `+0.0`). `route_run`'s arithmetic and
order are untouched: same operands, same operations, same order. Gate 1 and the kernel test show it
bit for bit, with the slots poisoned so any read of a skipped slot shows.

### Tests

New, `crates/graph/src/runtime.rs` tests (fixture: `RingSource`, a fake driver with the production
contract per claim; `RingShape`; 64 tracks `Input -> Route -> Output`; hostile words and 2x2s; every
fifth claim mono; the script `RING_SCRIPT`: whole blocks, claim 9 alone underrunning at block 3, a
short block, a block where nothing plays; every claim's slot poisoned before each block; host planes
at stride `frames + 3`; two Output meters):

- `runtime::tests::a_plain_strip_source_is_read_in_place_by_the_fused_output_with_the_copy_bits`
  (gates 1 and 3): frames `{1, 7, 16, 128}`, in place against `test_only_set_source_in_place_declined`;
  host storage (padding included) and meter windows bit for bit; `output_route_folds() == 64` both
  ways; every claim in place / every claim copied; per block `[0, 64, 0]` / `[64, 0, 0]`; block 3
  `[0, 63, 1]` (the underrun: silence read, no copy); block 6 `[0, 0, 64]`.
- `runtime::tests::a_claim_with_another_reader_keeps_the_copy_and_the_copy_bits` (gate 2 and every
  other clause), the same assertions per shape: the brief's `SendTap` (fold 65; track 5's path runs
  through its bound identity `PostFader` so its route stays plain), `DelayedEdge` (a compensation
  delay on `Input -> PostFader`), `SubmixReader` (`Input -> submix -> route`), each copying claim 5
  alone (`[1, 63, 0]` per whole block); then `BoundStage` (a processor in place over the claim's
  slot), `TrackDelayed` (a), `ObservedInput` and `ObservedAlias` (c), `LateInput` (e), each copying
  claim 5 alone; `DeadClaim` (deviation 1: a 65th claim with no reader, which #918 binds in place,
  whose freed slot becomes an Output input's buffer), copying claim 5 and reading 63;
  `RouteEdgeDelayed` and `FoldDeclined`, where #926's fold declines, so no claim is read in place
  (64 copies). The two colouring shapes are asserted to build their hazard on the lowered program.
- `runtime::tests::a_route_reduction_reads_each_lent_input_as_the_copys_words` (kernel):
  `route_reduce` at `f32`, `Simd4`, `Simd8`; frames `{1, 3, 7, 8, 13, 16, 33, 64}`; fan-in
  `{2, 3, 5, 9, 64}`; two in three inputs lent under reversed claim indices, some of those
  underrunning, the lent slots poisoned; bit for bit against the same reduction over an arena
  holding the copy's words; the counts; a claim table of the wrong length refused before any write.
- `rt10_source_in_place_alloc::an_in_place_output_read_renders_the_copy_bits_and_allocates_nothing`:
  a bankless four-claim plan, 1000 blocks (one in seven an underrun), `(0, 0)` allocations and frees
  in render scope (positive control first), the not-lending plan's bits, driver calls `[0, 4000]`
  against `[4000, 0]`. Both rt10 tests now take rt9's allocator-mode mutex (the mode is process
  wide).

**Pre-change digests.** `RING_PRE_CHANGE` pins, per shape, FNV-1a over the declined arm's four
per-quantum digests. They were recorded by compiling this fixture into `169a2486`'s own
`runtime.rs` (the base file, with the fixture block appended, the one new-field read replaced by an
empty list, and a printing test; then restored), where the copy was the only path of a bankless
plan; on the base both arms rendered the same bits. The declined arm here reproduces all twelve,
so the copy path did not move.

Changed: the two `execute_op` test call sites and the kernel test's `route_reduce` calls take
`OutputSources::NONE`. No assertion changed meaning.

### Gates

| gate | command | result |
|---|---|---|
| 1, 3 | gate-1 test | pass: 4 quanta, bits, folds 64 both ways, counts as above |
| 2 | gate-2 test | pass: 11 shapes x 4 quanta, each declined arm = pre-change digest |
| kernel | kernel test | pass at three widths |
| 4 | mutation sweep | `MUTATIONS.md` "Issue #927", rows 927-1 to 927-13 (below) |
| 5 | `cargo test --locked -p graph` / `--features test-support` | lib 109, rt1 1, rt10 2, rt9 1 / lib 109, rt1 1, rt10 2, rt9 8; 0 failed |
| 5 | `cargo test --locked -p source --all-features` | 72 (1 ignored) + 1; 0 failed |
| 5 | `cargo test --locked -p host-core --all-features` | lib 86 and every suite (`builtin_batch_endpoint` 20, `source_in_place` 1, `track_delay` 8, ...); 0 failed, pre-existing ignores only |
| 5 | `bash scripts/check-graph-determinism.sh` | PASS (100/100); evidence sha256 `e5d45be6...a8face`, #916's, #918's and #926's value |
| 5 | `check-graph-policy.sh` / `check-realtime-policy.sh` / `check-lane-policy.sh` | PASS / ok (53 marked regions in 15 files; no new region: the new render code sits in #926's) / ok |
| extra | `cargo test --locked -p console-workload` | automation 4, `chain_shape` 24, placement 3, `plumbing_profile` 2 ignored; 0 failed |
| extra | `cargo test --locked -p graph-compiler` | 73 + 1 + 3 + 1 + 8 + 6; 0 failed |
| extra | `cargo test --locked -p capi` | lib 32, `resource_lifecycle` 4; 0 failed (no accounting pin moved) |
| extra | `cargo test --locked -p builtins-compiler --features test-support` | 58, `allocation_tracker` 9, 3, 6, 1, 2; 0 failed |
| extra | simd128 AudioWorklet module built with `build-web-audioworklet.sh`'s cargo invocation and flags (digest `3ce8a08a...c085`, not pinned), then the callgraph script's `--kernel-shape --kernel-pattern '4wide6f32x[48]' --kernel-min 11` and `--callgraph miso_engine_web_v1_render` | exit 0 / exit 0: `f32x4_arith` 11,683, kernels 15 (both #926's values), `route_reduce<f32x4>` 44 vector / 0 scalar (#926's); render closure 8 |
| extra | `cargo check --locked -p graph -p source --target wasm32-unknown-unknown` | compiles (`FrameLane` = `f32`) |
| std | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean; exit 0 |

### Mutations

Full rows in `crates/graph/tests/MUTATIONS.md`, "Issue #927". The brief's three:

- **927-1, read `lease.read` for an admitted claim after skipping its copy:** RED on gate 1, gate
  2, the kernel test and rt10. Gate 1 fails at block 0, not the brief's block 2, because the slots
  are poisoned before every block.
- **927-2, admit a claim with two readers:** as literally written (the `[reader]` pattern relaxed
  to any retired-route reader) it is **GREEN and equivalent**: a retired route is in place over the
  claim's buffer, which the lowering allows only for a sole reader, so no claim has one among two
  readers. The intent, made non-equivalent: **927-2b** (every plain claim that (b) declines skips
  the copy) is RED on gate 2 at `SendTap` (mode table; bits-only, the host planes), and **927-2c**
  (admission keyed on the buffer rather than the reader) is RED at `SubmixReader`'s mode table and,
  bits-only, on `BoundStage`'s host planes.
- **927-3, read the stale arena buffer instead of the silence buffer on `None`:** RED on gate 1 at
  block 3 (the poison), gate 2 and the kernel test; GREEN on rt10, whose never-written slots hold
  the arena's initial `+0.0` (why the gates poison).

Beyond the brief, red: (e) dropped (927-4, `LateInput`), the brief's literal buffer-keyed lookup
(927-5, `DeadClaim`, the evidence for deviation 1), (a) and (c) dropped (927-6, 927-7), no planes
handed to the Output (927-8), the input position read as the claim (927-10: GREEN on gate 1, whose
inputs come in claim order, RED on gate 2 and the kernel test, which lends under reversed indices
for that reason), swapped planes (927-11), the fold's producers reversed (927-12), a `Vec` in the
pair (927-13: RED on rt10's allocation audit and the realtime policy, GREEN on the bit gates). And
927-9 (the copy loop keeps every claim) moves no bit and is RED only on the counters, as #918's
918-4 was.

### Memory

`Runtime` grows by one `Box<[u32]>` (`output_sources`, 16 bytes on 64-bit), mirrored in both runtime
layout witnesses (`RuntimeWithoutSplitPairTable`, `RuntimeWithoutObservationActivation`), so every
charged delta is unchanged: the capi `resource_lifecycle` and host-core suites pass unchanged. The
table is one bind-time allocation of `4 x` (Output fan-in) bytes, only when some input is read in
place, not charged to a resource row, like `output_routes` and `source_plane_of_buffer`.
`OutputRouteFold` (bind-time only) gains a `Vec<usize>` of the same length as its routes.

### Deviations

1. **The Output selects each input's claim by position (`Runtime::output_sources`), not by
   `source_plane_of_buffer[buffer]`.** The brief's item 2 keys the kernel's selection on the
   buffer. That is inexact for the same reason #918 needed its lane mask (918-5): the table is keyed
   by physical slot, and a slot the table names can be a different value when the Output reads it.
   Here the table names claims #918 binds in place because nothing reads them; the colouring frees
   such a claim's slot at once, and a later value the Output reads can take it (gate 2's
   `DeadClaim`, whose hazard the test asserts is built). Keyed by buffer, that input is served the
   dead claim's block (927-5, RED). The table entry is still written, and is only what makes the
   copy loop skip the claim (item 3). The cost is one boxed slice; the brief's "not a bit field"
   holds (a `u32` per input, like `output_routes`' `[f32; 4]` per input).
2. **Clause (e), not in the brief.** Without it, a hand-built plan that schedules an input after an
   op that uses its slot renders differently in place than on the copy (927-4, RED on
   `LateInput`). The copy path's own behaviour there (the earlier op overwrites the copied words)
   is pre-existing, and so is #918's bank path's exposure to it: #918's (b) has no such clause.
   Not changed here (non-goal); reported for a successor.
3. **`OutputRouteFold::producers`**, a field of #926's bind-time struct, so (b') names the route that
   feeds each input rather than re-deriving it.
4. **Gate 2's "send tap reader" needs the track's own path to run through a bound stage.** A send
   beside the track's route gives the input two readers, so the route copies and #926's fold
   declines outright (every claim then keeps the copy). `SendTap` routes track 5 through its bound
   identity `PostFader`, which keeps the fold and makes claim 5 the one copy. Likewise "a delayed
   edge" is modelled as a delay on `Input -> PostFader` (`DelayedEdge`, one copy); a delay on the
   route's own edge declines the fold outright (`RouteEdgeDelayed`, 64 copies).
5. **Gates beyond the brief:** the kernel test, eight declining shapes beyond the brief's three,
   the pre-change digests, and the rt10 allocation test (`crates/graph/tests/`, the graph crate's
   tests). `lib.rs` changed in comments only.
6. **Mutation 927-2 as literally written is an equivalent mutant**, recorded as such with its
   non-equivalent forms 927-2b and 927-2c.

### Not done, and risks

- **No production-driver end-to-end test.** `crates/host-core/tests` is outside the paths, and no
  host-core session reaches this path: host-core always compiles with builtins, where every input is
  read by `PostInputBuiltins` (a bank member or an op), never by a retired route (checked with a
  temporary bind probe over `cargo test -p host-core --all-features`: no bind admitted an Output
  read). The builtins-less plan this issue moves is the console workloads' and any embedder's that
  calls `GraphCompiler::compile`; the ring-fed row #928 adds (`origin/codex/928-driver-fed-plumbing-row`,
  not on this branch) is its first production-shaped consumer and is where the batch-boundary
  benchmark can see it. `RingSource` mirrors the production driver's per-claim contract
  (`crates/source/src/lib.rs` `played_planes` / `copy_channel`).
- **Not run:** the `plumbing_profile` harness (its bound-feed row binds `FrozenGraphSource`
  processors and no source set, so nothing here can show on it), `cargo run -p audit -- source`
  (not named by the brief), browser qualification and the pin (batch boundary), the paired console
  benchmark. No performance claim is made.
- **A pre-existing class-A hole in #918's reader-less rule, found here and not fixed.** Two claims
  can share a slot when an earlier one has no reader: the colouring frees its slot at the input op
  and the next input op takes it. On the copy path both are then copied into that slot in claim
  order, so a later-indexed reader-less claim overwrites the live one; #918 binds the reader-less
  claim in place (not copied), so its in-place arm keeps the live claim's words and the declined
  arm does not. Measured with a scratch variant of `DeadClaim` whose dead claim sorts first (not
  committed): (e) correctly keeps claim 0 on the copy, and the arms still differ at block 0
  (`1277984491` in place against `1277984273` declined). The difference is #918's rule alone (the
  base tree's in-place arm also skips the reader-less claim's copy; reasoned from its code, not
  re-measured there), and the copy path is the arm that is wrong. The committed `DeadClaim` schedules its dead claim last among the inputs, so no live claim
  shares its slot. A compiled plan reaches this only with an input no op reads. For a successor:
  keep every claim's slot reserved from the top of the block, or refuse a reader-less claim at
  bind.
- **Stale prose outside the paths:** `crates/source/src/lib.rs`'s `played_planes` doc still says a
  banked gather is the in-place reader (non-goal: no change to `crates/source`).
- **Anchor drift** (base `169a2486`): the brief's `lib.rs` anchors predate #925/#926. `lent_claims`
  is `lib.rs:2362` (cited `:2200-2231`), `provides_played_planes` `:1815` (`:1653`), the copy loop
  `:2497` (`:2321-2331`), the `sources` borrow `:2515` (`:2337-2338`), `GraphSourcePlanes` `:1833`
  (`:1674`); `test_only_set_source_in_place_declined` is defined at `runtime.rs:4899` and exported
  at `lib.rs:36` (cited `lib.rs:2211`). In `runtime.rs`: `source_plane_table` `:5730` (`:5324`),
  `source_plane_of_buffer` `:2263` (`:1989`), `NO_SOURCE_CLAIM` `:1725` (`:1451`), `SourceGather`
  `:1730` (`:1456`), `ArenaMembers::plane` `:1800` (`:1526`), `execute` `:2708` (`:2374-2416`),
  `source_in_place` `:2596` (`:2289`), `test_only_source_plane_counts` `:4921` (`:4567`),
  `UnitIdentity::source_lanes` `:2162` (`:1888`). Each cited function and behaviour was as
  described. The brief's `DRAFTS/PLAN.md` is `docs/handoffs/plumbing-floor-2026-09-26/PLAN.md` on
  this tree.
