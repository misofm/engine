# Gather banked source inputs from the played transfer block

## Product outcome

Every block, `SourceSet::copy_track_input` copies both channels of every claimed source into the
track's arena input buffer, and the track's first bank then reads that buffer only to transpose
it into AoSoA scratch. For a track whose input is consumed solely by a bank gather, let the
gather read the played `TransferBlock`'s planes directly and skip the arena copy. Class A: the
gather transposes the same words from a different address; no rendered bit moves. This is the
render-side half of the audit's "a PCM sample is written three times" finding.

## Root evidence

- `crates/graph/src/lib.rs:2226-2231`: for every `(claim, buffer)` in `source_input_buffers`
  (built at `:2119-2134` from the source claims and `program.node_buffer`), the executor calls
  `source_set.copy_track_input(claim, left, right)` into `runtime.buffer_mut(buffer)`
  (`runtime.rs:1830`, `lease.write_stereo`).
- `crates/graph/src/runtime.rs:1940-1965`: a bank unit's member whose `bank_gather_source`
  (`:2216`) resolves takes its input buffer index into `bank_inputs[lane]`; the chain gathers
  through `ArenaMembers::plane` (`:1300-1305`, `lease.read_stereo(self.inputs[lane])`), and
  `tile_gather` / `gather_lane` (`crates/rack/src/lib.rs:263`, `:180`) read those slices.
- `execute_op` (`runtime.rs:2332`) returns early for `NodeKind::SourceInput` (`:2353`); a
  `TrackDelay` writes its line **in place** over the source buffer (`:2343-2350`).
- `crates/graph/src/program.rs:207` `is_dedicated`: `TrackStage::Input` is not dedicated, so a
  single sole reader may be lowered in place over the source buffer (`:691-700`), which writes
  it.
- Observers on the Input stage read `lease.read_stereo(op.output)` (`runtime.rs:2602`).
- The consumer-side contract this issue relies on is "Retain the played transfer block through
  the render and expose its planes": `played_planes(claim)` returns quantum-long read-only
  slices, tail-zeroed, `None` on an underrun; the block is the consumer's until the next
  `begin_block`.
- `GraphPreparedSourceSetDriver` (`lib.rs:1624`) is the seam; the graph's own test drivers
  (`lib.rs:2550`, `:3367`) implement it and must keep compiling through a defaulted method.

## Smallest closable slice

Authorized paths: `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`,
`crates/source/src/lib.rs` (only the driver's `played_planes` implementation moving onto the
graph trait), `crates/graph/tests/` (a new allocation-and-identity test beside
`rt9_resident_bank_input_alloc.rs`), `crates/graph/tests/MUTATIONS.md`, `crates/host-core/tests/`
(one end-to-end oracle with a real ring), and this spec.

1. Trait: `GraphPreparedSourceSetDriver::played_planes(&self, claim_index: usize) ->
   Option<(&[f32], &[f32])>`, default `None`; `GraphPreparedSourceSet` forwards it after the
   same claim-index check `copy_track_input` makes.
2. Bind-time mode per claim, decided in `GraphExecutor::new` from the lowered program: a claim
   is `InPlace` iff its buffer is (a) never any op's `output` other than the `SourceInput` op
   itself (so no in-place consumer and no `TrackDelay`), (b) read only by ops that are bank
   members whose `bank_gather_source` names it, (c) not observed at the Input stage, and (d) the
   driver exists. Everything else is `Copy` (today's path, unchanged). Store
   `source_plane_of_buffer: Box<[u32]>` (claim index, `u32::MAX` for none) in the runtime.
3. Render: the executor's copy loop skips `InPlace` claims. `execute` receives
   `Option<&dyn GraphSourcePlanes>` (the source set) and `ArenaMembers` carries it plus the
   table; `plane(lane)` returns the played planes for an `InPlace` buffer, or, on `None`
   (underrun / end of region), `lease.read_stereo(0)` -- the arena's silence buffer is index 0
   (`ARENA_BASE = 1`, `runtime.rs:49`; there is no `ARENA_SILENCE_BUFFER` constant in graph) --
   exactly the zeros `copy_channel` would have written. One table lookup and one predictable
   branch per lane per block; no other lease read site changes. Both `gather` (`rack:2534-2548`)
   and `gather_tiled` read through `BankMembers::plane`, so the in-place read serves partial
   banks as well as full ones.
4. Keep a test-only switch `test_only_set_source_in_place_declined(bool)` (bind-time, exported
   `#[doc(hidden)]` beside `test_only_set_route_fold_declined` in `lib.rs:35`) so an oracle can
   bind the same plan in `Copy` mode.

## Non-goals

No change for unbanked consumers (routes, dynamic-rack effects, submixes: they keep the copy or
their in-place lowering), for delayed tracks, for observed Input stages, for the producer side,
or for the web submit copy. No arena or `disjoint.rs` change.

## Objective gates

1. New graph test with a fake driver: for random input over 8 blocks including one underrun
   block, one short block (tail zeros), and a mono mapping (`left_channel == right_channel`),
   the master and every meter window of a `W8 x 8`, a `W4 x 4` and a partial `W4 x 6` banked
   plan bound `InPlace` are bit-identical to the same plan bound with the switch set (`Copy`), and the `Copy` arm's
   output is bit-identical to the pre-change executor. Assert the mode table: every banked
   claim `InPlace`; a claim with a `TrackDelay`, an in-place route consumer, or an Input-stage
   observer is `Copy`.
2. New end-to-end test (host-core) through a real `PcmSourceRing` and the production driver,
   fed through `SourceControlSet::submit` (`crates/host-core/src/source.rs`) rather than the
   native worker: render N blocks with the test submitting fixture chunks, compare the output
   digest against the `Copy`-mode digest; withhold one block's submission to produce the
   underrun, and submit one after a seek (generation change).
3. Allocation: a new `crates/graph/tests/rt10_source_in_place_alloc.rs` (pattern of `rt9`)
   proves zero allocations across 1000 rendered blocks in `InPlace` mode with the fake driver.
4. `MUTATIONS.md` rows: read the wrong channel plane (red on gate 1), skip the silence fallback
   (red on the underrun block), mark a `TrackDelay` claim `InPlace` (red: the delay line's write
   lands in the arena and the gather reads the raw block), keep the copy loop for `InPlace`
   claims (no red -- record that the mode counter in gate 1 is what distinguishes it).
5. `cargo test -p graph` (with and without `--features test-support`), `-p source`, `-p
   host-core --all-features`, `-p graph-compiler`, `-p console-workload`;
   `scripts/check-graph-determinism.sh`, `scripts/check-graph-policy.sh`,
   `scripts/check-realtime-policy.sh`.

## Console benchmark rows

None. The console rows bind `FrozenGraphSource` processors, not the ring; their per-track
"source write" is the fixture's own copy and stays. Evidence for this issue is the bit-identity
and allocation gates; a runner-based timing would be dominated by the runner's own I/O (#895)
and is not asked for.

## Dependencies

"Retain the played transfer block through the render and expose its planes" (must merge first;
this issue lifts its `played_planes` onto the graph trait). Merge after "Write the master
straight into the host planes" because both add a parameter to `Runtime::execute` and thread it
into `ArenaMembers`; this issue rebases.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate above that says "bit-identical" is a hard stop, not a tolerance.
- The owner's copy rule: a block-sized copy on the render path exists only with a written justification that no in-place or direct-write form exists. This issue removes the ring-to-arena copy for banked tracks; the `Copy` mode that remains for delayed, observed and unbanked-consumer tracks is justified in the mode table's doc comment, per clause, and nowhere else.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). `crates/graph` stays free of `unsafe`.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named above before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. No benchmark row is listed.
- The AudioWorklet artifact pin and browser qualification are repinned once at the batch boundary; do not repin here.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (§1.2 step 2, §1.3, PR #879) and tracker #349.

## What the implementer will hit

- `bank_gather_source` (`runtime.rs:2216`) admits only the "dedication copy" shape
  (`bank_gather_source_admits_only_the_dedication_copy`, `:8224`); read it before writing clause
  (b), because the member op that gathers is the first slot's member, not the `SourceInput` op.
- The resident-input path (`identity[index].resident_input`, `acquire_resident_input` in rack)
  hands a *predecessor chain's* scratch to a bank instead of gathering; it never reads a source
  buffer, so it is unaffected, but `rt9_resident_bank_input_alloc.rs` is the allocation-test
  pattern to copy.
- The console-workload fixtures have no source set; `bank_shape`, `bank_route_folds` and the
  chain-shape tests must not move.
- Text-pinning tests split the source on the exact spellings `pub(crate) fn execute(`,
  `pub(crate) fn observe_unit(` and `    fn render(` (`runtime.rs:6088-6095`, `:6180-6192`);
  keep those spellings when adding the source-planes parameter.
- `GraphExecutor::render`'s error paths (`lib.rs:2233-2280`) run `complete_pending`; the source
  set's played block is released at the next `begin_block` regardless of an error, which the
  hold rule already guarantees.

## Attempt 1 evidence

Implementer: Terra. Branch `codex/918-gather-from-played-transfer-block`, based on `63eeebf0` (the
verified #916 tip `135df65a` with the verified #917 branch merged). Commits: `b939e27b`
(implementation and gate 1), `3857a7fb` (gates 2 and 3), `7f027946` (a ninth gate-1 shape and the
set's plane refusals), `b6476b37` (MUTATIONS rows), and this record. Nothing was pushed.

### Design

1. **Trait.** `GraphPreparedSourceSetDriver::played_planes(&self, claim_index) -> Option<(&[f32],
   &[f32])>` defaults to `None`, beside a new `provides_played_planes(&self) -> bool` defaulting to
   `false` (deviation 1). A crate-private `GraphSourcePlanes` trait is what the runtime sees;
   `GraphPreparedSourceSet` implements it with the claim-index check `copy_track_input` makes, and
   refuses (`None`) a plane that is not one quantum. `SourceGraphSourceSetDriver` implements both
   methods: #917's inherent `pub(crate) played_planes` (and its `allow(dead_code)`) moved into the
   trait impl unchanged, inside a `REALTIME_POLICY` region.
2. **Bind-time mode per claim** (`runtime.rs` `source_plane_table`, called by `build_sequential`,
   which `GraphExecutor::new` calls). It runs on the finished units, after every scatter redirect has
   repointed its member, with `op_dataflow`'s readers and `op_slot`. A claim is **in place** iff:
   (a) its input op's unit is a plain `NodeKind::SourceInput` (not `TrackDelay`); (b) every reader of
   the input op's value is a first-slot member of a bank unit that reads that buffer and nothing else
   and writes nothing before its gather (`gathers_only`: `BankMember`, no staged input, no sidechain,
   `inputs == [buffer]`); (c) the input op carries no observer (its own or an elided alias's); (d)
   the driver lends its planes. Everything else is on the copy. The result is
   `Runtime::source_plane_of_buffer: Box<[u32]>` (claim index, `NO_SOURCE_CLAIM = u32::MAX`),
   sized to the highest in-place slot + 1 and empty when no claim is in place, plus a lane mask
   `UnitIdentity::source_lanes: u8` marking exactly the first-slot lanes of (b) (deviation 3). The
   per-clause copy justification is that function's doc comment and nowhere else.
3. **Render.** `GraphExecutor::new` keeps only copy claims in `source_input_buffers`, so the copy
   loop is the old loop over a shorter list. After it, `render` takes
   `sources = source_set.as_ref().map(|set| set as &dyn GraphSourcePlanes)` and passes it to
   `Runtime::execute(index, first_sample, host, sources)`, the new parameter after #916's `host`
   (the pinned spellings `pub(crate) fn execute(`, `pub(crate) fn observe_unit(` and `    fn render(`
   are unchanged). A bank unit hands `SourceGather { planes, of_buffer, lanes }` to `ArenaMembers`.
   `ArenaMembers::plane(lane)` tests the lane's mask bit; on a marked lane with a tabled buffer it
   returns the played planes, or `lease.read_stereo(ARENA_SILENCE_BUFFER)` on `None`; every other
   lane reads `lease.read_stereo(inputs[lane])` as before. `gather`, `gather_tiled`, `gather_mono`
   and `gather_mono_tiled` all read through `plane`, so full, partial and collapsed banks are served.
   No other lease read site changed.
4. **Test seams.** `test_only_set_source_in_place_declined(bool)` (bind-time, exported
   `#[doc(hidden)]` beside `test_only_set_route_fold_declined`) binds every claim on the copy.
   `test_only_source_plane_counts()` / `test_only_source_plane_reset()` count `[claims copied,
   gathers served a played block, gathers served silence]`: the brief's mode counter.

**Two facts measured on `63eeebf0` that decide how (a) and (b) had to be read.**

- *Physical slots are reused.* In every gate-1 shape, every input's arena slot is later the output
  of another op: the track's route, and in `W4 x 6` the second cohort's `PostInputBuiltins` member.
  Read on the physical slot, "(a) never any op's output other than the `SourceInput` op" binds no
  claim in place in any shape. So the clauses are read on the input's *value* (`op_dataflow`'s
  readers of the input op), and because the table is keyed by slot, a lookup must be restricted to
  the gathers of that value: the lane mask. Without it, a later bank that gathers the recoloured slot
  is served the played block (MUTATIONS 918-5, red on shape 7).
- *The redirected single-slot member.* In the brief's own shapes the route takes the input's retired
  slot and `apply_scatter_redirects` repoints the member's output at it, so `member.output ==
  inputs[0]` and `bank_gather_source` returns `None`: the member runs its `[own output]` no-op
  reduction and the chain gathers the input slot through the member's output. A (b) that named only
  `bank_gather_source` would bind none of the brief's `W8 x 8`, `W4 x 4`, `W4 x 6` claims in place.
  `gathers_only` covers both routes into the gather, and also a member that runs in place over its
  input (deviation 2).

### Lifetime argument

- **What is read, and when.** A marked lane reads the claim's planes in its bank unit's
  `chain.run`, inside `render`'s unit loop, after `begin_block` played the block and after the copy
  loop would have copied it (it now skips the claim), and before `render` returns.
- **No release point can fire in between.** The played block's data stops being readable only at
  `PcmSourceConsumer::end_block`, reached from the next `begin_block`, from `prepare_seek`, from the
  driver's zero-mapping path in `begin_block`, or by drop (#917). Each takes the consumer `&mut`,
  reachable only through `&mut GraphPreparedSourceSet`. In `render` the last `&mut` use of the set
  is the copy loop; `sources` is a shared borrow of the set that lives across the whole unit loop,
  so the borrow checker rejects any `&mut` use of the set until the loop ends, and every error path
  (`invalidate_observers_after_failure`, `host.silence`, `complete_pending`) touches only the runtime
  and the host planes. `prepare_source_seek` is a `&mut self` method of the executor, which the
  `&mut self` render excludes. So the planes cannot move while any unit can reach them, and the
  next `begin_block` is the next render's.
- **Underrun and end of region.** `played_planes` returns `None` (no played block), and the gather
  reads the arena's buffer 0, which no lease can write (`disjoint.rs` I1), so it is `+0.0` for the
  arena's life, exactly the `destination.fill(0.0)` `copy_channel` writes. A **short block** is
  lent as #917's `play` left it: the played frames, then `+0.0` zeroed in place to the quantum,
  the words `copy_channel` writes. A **mono mapping** lends one channel's slice twice (two shared
  borrows). Nothing falls back to the copy at render: the mode is fixed at bind.
- **Claims that keep the copy.** A delayed track (`TrackDelay` runs its line in place over the copied
  words, and a gather must read the aligned block there), an observed input stage (the observer
  reads the copied words from the arena), and any input with a reader that is not a qualifying bank
  gather: a route or other in-place consumer, a compensation-delayed (staged) edge, a sidechain, a
  submix, the Output, a dynamic-rack or bound processor. Their copy loop is the pre-change loop, and
  their gathers read the arena as before.

### Tests

New:
- graph `runtime::tests::a_banked_source_gathers_the_played_block_bit_for_bit_with_the_copy` (gate
  1). A fake `PlayedSource` driver with the production contract renders eight blocks (an underrun, a
  short block, a `-0.0` in every channel, every third claim a mono mapping), in place and declined,
  with every claim's arena slot poisoned before each block. Nine shapes: the brief's `W8 x 8`,
  `W4 x 4` (16 frames, whole tiles) and partial `W4 x 6`; the `W4 x 6` with a delayed, a metered and
  a directly routed track; `W8 x 8` with redirects declined (the literal `bank_gather_source`
  gather); `W4 x 6` with in-place `PostMatrix` members and folded routes; two
  `PostInputBuiltins -> bound PostFader -> PostMatrix` shapes whose second bank gathers each input's
  recoloured slot; and `W4 x 6` with a compensation delay on one track's edge. Per shape it asserts
  the mode table, that the declined arm's digest equals the pre-change digest below, every block's
  master and every meter window bit for bit, the bound `[route folds, scatter redirects]`, every
  meter published every block, and the mode counters.
- graph `tests::a_source_set_lends_only_quantum_planes_of_its_own_claims` (the set's refusals).
- graph `tests/rt10_source_in_place_alloc.rs`
  `an_in_place_source_gather_renders_the_copy_bits_and_allocates_nothing` (gate 3): 1000 in-place
  blocks, zero allocations and frees in render scope (with a positive control), and the same plan
  bound with a driver that does not lend renders the same bits; the driver's own call counts are
  `[0 copies, 4000 lends]` against `[4000, 0]`. It uses only the public API, so it runs with and
  without `test-support`.
- host-core `tests/source_in_place.rs`
  `a_ring_fed_banked_session_gathers_in_place_with_the_copy_bits` (gate 2): the compiled
  `parametric-eq-bank-console`, its mono-mapped twin (its bank collapses on all 12 blocks, so the
  one-plane gather is exercised) and `parametric-eq-nine-track`, fed through
  `SourceControlSet::submit` into the real `PcmSourceRing` and rendered through the production
  driver: whole blocks, a withheld block (underrun) whose chunk then arrives late and is discarded
  as stale (a generation's chunks must be contiguous), a seek to generation 2, two whole chunks and
  the region's short last chunk, and two blocks past the end. Every block's master equals the copy
  arm's; every claim of all three sessions is in place (`[0, claims x 9, claims x 3]` against
  `[claims x 12, 0, 0]`).

Changed: `rt9_resident_entry_has_one_guarded_production_caller_and_control` expects the render-loop
statement with the new `sources` argument; the 15 hand-built `ArenaMembers` in `runtime.rs` tests
carry `sources: SourceGather::NONE`; the five test `UnitIdentity` literals carry `source_lanes: 0`;
`render_arena_oracle` passes `None` to `execute`. No assertion changed meaning.

**Pre-change digests.** Recorded by the gate-1 fixture itself, compiled against `63eeebf0`'s
`graph/src/lib.rs` and `runtime.rs` (swapped in, then restored), rendering the copy path that was
the only path then: `0x7da82488c5b78876` (`W8 x 8`, and again with redirects declined),
`0xc9daced780fae77f` (`W4 x 4`), `0xf3175c3fc88e6167` (`W4 x 6`, and again with in-place
`PostMatrix` members), `0x25f5d76410a6b66a` (mixed), `0x6c186a5b3585e2a6` and `0x0be859e11a6e5267`
(the fader shapes), `0xb0c7a1d1e6bd959f` (compensated).

### Gates

| gate | command | result |
|---|---|---|
| 1 | gate-1 test | pass: 9 shapes, mode tables as above, copy arm = pre-change digests, in-place arm = copy arm bit for bit |
| 2 | `cargo test -p host-core --test source_in_place` | pass: 3 sessions, every block equal, all claims in place |
| 3 | `cargo test -p graph --test rt10_source_in_place_alloc` (also under `--features test-support`) | pass: 1000 blocks, `(allocations, deallocations) == (0, 0)` |
| 4 | mutation sweep | `crates/graph/tests/MUTATIONS.md`, "Issue #918", rows 918-1 to 918-12 (below) |
| 5 | `cargo test -p graph` / `--features test-support` | lib 101, rt1 1, rt9 1, rt10 1 / lib 101, rt1 1, rt9 8, rt10 1; 0 failed |
| 5 | `cargo test -p source --all-features` | 64 + 1; 0 failed |
| 5 | `cargo test -p host-core --all-features` | 18 suites, 225 passed, 0 failed (2 pre-existing ignores) |
| 5 | `cargo test -p graph-compiler -p console-workload` | 73 + 1 + 3 + 1 + 8 + 6; `automation` 4, `chain_shape` 22, `placement` 3; 0 failed |
| 5 | `cargo test -p capi` | lib 32, `resource_lifecycle` 4; 0 failed (no accounting pin moved) |
| 5 | `cargo test -p builtins-compiler --features test-support` | lib 58, `allocation_tracker` 9, 3, 6, 1, 2; 0 failed |
| 5 | `bash scripts/check-graph-determinism.sh` | PASS (100/100); evidence sha256 `e5d45be6...a8face`, the value #916 recorded before and after its change |
| 5 | `bash scripts/check-graph-policy.sh` / `check-realtime-policy.sh` | PASS / ok (53 marked regions in 15 files; +2: the set's `GraphSourcePlanes` impl and the driver's `played_planes`) |
| std | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean; exit 0 |
| extra | `RUSTFLAGS="-C target-feature=+simd128" cargo check --target wasm32-unknown-unknown -p graph -p source` | compiles; nothing run under wasm |

### Mutations

Full rows in `crates/graph/tests/MUTATIONS.md`. The brief's four:

- **Wrong channel plane (918-1):** red on gates 1, 2 and 3. The production driver's own swap (918-8)
  is red on gate 2 and on two #917 source tests.
- **Skip the silence fallback (918-2):** red on gate 1 at the underrun block (the poison word
  surfaces). Green on gates 2 and 3, whose never-written slots still hold the arena's initial `+0.0`;
  that is why gate 1 poisons.
- **A `TrackDelay` claim in place (918-3):** red on gate 1's mode table and on 4 of 8 host-core
  `track_delay` tests; with the mode assertion removed (918-3b) red on the bits at block 0.
- **Keep the copy loop for in-place claims (918-4):** no bit goes red, as predicted; red only on the
  mode counters of all three gates.

Beyond the brief, red: the table read by slot alone (918-5), an observed input in place (918-6), no
reader clause (918-7), lending by default (918-9, red on #916's gate-3 `Source(true)` arm, whose
driver never lends), both effective-read clauses of `gathers_only` dropped (918-10b), the set's
checks dropped (918-11), no planes handed to the unit loop (918-12). Equivalent and disclosed:
918-10 and 918-10c each drop one of `gathers_only`'s two guards of the same fact.

### Memory

`Runtime` grows by one `Box<[u32]>` (16 bytes on 64-bit: `size_of::<Runtime>()` measured 456), and
the field is mirrored in both runtime layout witnesses, so every charged delta is unchanged (the
capi `resource_lifecycle` and host-core cap tests pass unchanged). `UnitIdentity`'s byte sits in
existing padding: measured 32 bytes, as before by layout (27 bytes of fields before, 28 now).
The table is one bind-time allocation of `4 x (highest in-place slot + 1)` bytes per executor,
independent of stem length, and, like the runtime's other bind-time tables (`bank_inputs`,
`bank_outputs`), not charged to a resource row.

### Deviations

1. **`provides_played_planes`.** A second, defaulted trait method. A defaulted `played_planes` alone
   cannot tell "never lends" from "underrun on every block": a driver that did not override it would
   be bound in place and render silence. Mutation 918-9 shows exactly that on #916's `FailingSource`.
   The graph's own test drivers keep compiling through both defaults.
2. **Clause (b) is `gathers_only`, not `bank_gather_source` alone**, and (a) is read on the input's
   value, not its physical slot (the two measured facts above). One consequence against the brief's
   parenthetical "(so no in-place consumer)": a *bank member* running in place over its input (shape
   6) is bound in place, because its reduction is the `[own output]` no-op and its gather is its only
   read of the input; an in-place route, the brief's example, keeps the copy (shape 4).
3. **The lane mask `UnitIdentity::source_lanes`**, not in the brief, makes the slot-keyed table exact
   under recolouring. It fits `UnitIdentity`'s padding and costs one bit test per lane per block.
4. **Where the mode is decided.** In `build_sequential` (called from `GraphExecutor::new`), because it
   needs the finished units, `op_slot` and the dataflow readers.
5. **`ARENA_SILENCE_BUFFER`**, the constant `engine::realtime` exports (value `0`), is named rather
   than the literal `0`.
6. **Gate 1 has nine shapes**, the brief's three plus six that reach each clause; **gate 2** adds the
   mono twin and the nine-track session; its withheld chunk is submitted late (and discarded as stale)
   because the ring refuses a non-contiguous chunk within a generation.

### Not done, and risks

- `cargo run -p audit -- source` was not run: the brief does not name it, and its plan has no bank,
  so its claim binds on the copy and cannot reach the new path.
- Browser: only the wasm32 check above. The AudioWorklet artifact hash changes and is repinned at
  the batch boundary; the browser resource rows are not expected to move (the charged deltas are
  unchanged), but that was not measured against a built module.
- Documentation outside this issue's paths that describes the ring-to-arena copy was not edited.
- Disk was nearly full during this attempt; I removed my own worktree's `target/debug/incremental`
  and relinkable test executables between runs and built with `CARGO_INCREMENTAL=0`.
- Anchor drift: every cited anchor had moved with #915/#916 (for example the copy loop is now at
  `lib.rs:2321`, `bank_gather_source` at `runtime.rs:2742`, `ArenaMembers::plane` at `runtime.rs:1526`,
  the rack's `gather` at `rack/src/lib.rs:2630`); each cited function and behaviour was as described.
