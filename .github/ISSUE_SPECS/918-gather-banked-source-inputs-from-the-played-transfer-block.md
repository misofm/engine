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
