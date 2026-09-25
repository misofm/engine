# Copy-removal cycle plan (2026-09-25)

## Issue numbers

| issue | draft slug | title |
|---|---|---|
| #914 | carry-fold-and-redirect-counters-in-the-console-meters-record | Carry fold and redirect counters in the console meters record |
| #915 | fuse-the-fold-epilogue-into-the-scatter-transpose | Fuse the fold epilogue into the scatter transpose |
| #916 | write-the-master-straight-into-the-host-planes | Write the master straight into the host planes |
| #917 | retain-the-played-transfer-block-through-the-render-and-expose-its-planes | Retain the played transfer block through the render and expose its planes |
| #918 | gather-banked-source-inputs-from-the-played-transfer-block | Gather banked source inputs from the played transfer block |
| #919 | decode-native-sources-straight-into-the-recycled-transfer-block | Decode native sources straight into the recycled transfer block |
| #920 | fold-in-place-routes-into-the-output-nodes-reduction | Fold in-place routes into the Output node's reduction |


Base: `main` at `f6adb7e7` (PR #913 merged on top of `ae40fe64`; #913 changed only
`scripts/run-console-benchmark.sh` and `artifacts/`, so every `file:line` verified on `ae40fe64`
still holds, and the `--pure-path` / `--pure-path-baseline` arms are now on `main` at
`run-console-benchmark.sh:126,281-282`). Every anchor in the drafts was verified against the
code, not carried over from the audit. Drafts (H1 is the future GitHub title):

| # | draft | one line | crates |
|---|---|---|---|
| T1 | `carry-fold-and-redirect-counters-in-the-console-meters-record.md` | `console_meters` states `bank_route_folds` / `bank_scatter_redirects` per arm, validator pins them equal | bench, console-workload, scripts |
| O3 | `fuse-the-fold-epilogue-into-the-scatter-transpose.md` | all-folded full banks route and accumulate each tile straight from the transpose; `staging_*` untouched | rack, graph |
| O9 | `write-the-master-straight-into-the-host-planes.md` | the Output node's storage for the block is the host's planes; the end-of-block copy and the fan-in-one arena copy go | engine (one accessor), graph |
| R1 | `fold-in-place-routes-into-the-output-nodes-reduction.md` | in-place plain routes whose sole reader is the Output op are retired into one fused mix-and-accumulate reduction (Output-only, all-or-nothing) | graph, console-workload test, floor ruling sentence |
| S2 | `decode-native-sources-straight-into-the-recycled-transfer-block.md` | the native worker decodes into the reserved `TransferBlock`; staging block and its copy go | source (producer, worker) |
| S1a | `retain-the-played-transfer-block-through-the-render-and-expose-its-planes.md` | the consumer keeps the played block until the next `begin_block`, exposes tail-zeroed planes; ring allocates `count + 1`, charged to overhead | source (consumer, ring) |
| S1b | `gather-banked-source-inputs-from-the-played-transfer-block.md` | banked tracks gather from the played block; the ring-to-arena copy goes for them | graph, source (trait impl) |

## Concurrency groups and merge order

Forced dependencies are exactly two: **S1a -> S1b** (S1b lifts S1a's `played_planes` onto the
graph trait) and **O9 -> R1** (R1's kernel writes the host planes and reads the arena through
shared borrows only; without O9 it would need an arena accessor). Everything else in the order
below is a convenient rebase, not a dependency:

- **Group A (graph render path):** O3, then O9, then R1. O3 and O9 both edit `ArenaMembers`'s
  master write sites in `crates/graph/src/runtime.rs`; doing O3 first and rebasing O9 onto it
  is the cheaper rebase, not a requirement. R1 follows O9 (forced).
- **Group B (source crate):** S2 and S1a concurrently -- both edit `crates/source/src/lib.rs`
  on disjoint sides (producer/worker vs consumer/ring shape). Whichever merges second rebases;
  the one shared region is `resource_report` and its pinned tests (S2 removes the worker
  staging bytes, S1a adds the retained block), reconciled once.
- **Group C (tooling):** T1, independent of everything. Merging it first is convenient so the
  batch-boundary benchmark records carry the counters; it is not a dependency of any brief.

S1b after O9 is also a convenience (both add a parameter to `Runtime::execute` and thread it
into `ArenaMembers`; the second rebases). Suggested merge order:
T1 -> O3 -> S2 / S1a (either order) -> O9 -> R1 -> S1b.

Files two issues both edit: `runtime.rs` (O3, O9, R1, S1b), `crates/graph/src/lib.rs` (O9, R1,
S1b), `crates/source/src/lib.rs` (S2, S1a, S1b's trait impl). None of the seven touches
`crates/lane`, `disjoint.rs` (O9 forms its group inputs with `lease.read` and needs no new
accessor), `spsc.rs`, the C ABI or the web ABI.

At the batch boundary, once: repin `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`
(every engine brief moves the worklet binary), run the browser qualification, and sweep `docs/`
for the statements the batch falsifies (any description of the executor's end-of-block copy;
the audit's O3/O9 rows; R1 itself edits the floor ruling's plumbing sentence).

## Benchmark plan

Register a paired arm `--copy-removal` / `--copy-removal-baseline` in
`scripts/run-console-benchmark.sh` beside the `--pure-path` arms (`:126`, `:281-282`), with an
arm note in the same shape. Baseline = `f6adb7e7` plus the arm registration plus T1 (tooling
only; the counters must exist on both arms); candidate = the merged batch. One invocation per
arm, one warmup, two measured rounds, no retry, on a controlled host if one is available (the
pure-path pair had to be captured under `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1`; a 170 us row
cannot resolve a saving of a few microseconds on that host).

Read the record honestly:

- **Expected to move (O3):** every banked row -- `sixty_four_track_console`, `_eq_only`,
  `_compressor_only`, `_builtins_only`, `_dispatch_only`, `_idle`, `_gain_pan_only`, the three
  mono rows, both `console_meters` arms, the hoist/observation/automation arms, the 128-track
  stretch. The lightest banked rows (`gain_pan_only`, `dispatch_only`, `idle`) should show it
  most in ratio terms; `nine_track_ragged_strip` moves only through its full first cohort.
- **Expected to move (R1):** `sixty_four_track_plumbing_only` only, and it is the one change in
  the cycle that addresses that row's per-track dispatch and store passes. Every other row banks
  and folds through the chain epilogue, where R1 never fires (`output_route_folds` must read 0
  on them).
- **Expected to move a little, everywhere (O9):** one `2·Q` copy per block off every row.
- **Cannot show anything:** S1a, S1b, S2. The console rows bind `FrozenGraphSource` processors
  (`tools/console-workload/src/lib.rs:1499`), not the source ring, and S2 is on the decode
  thread. Their evidence is bit-identity and allocation gates; a runner-based timing is
  dominated by the runner's own per-sample I/O until #895 lands, so none is asked for.
- **The metered pair:** with T1 the record shows `meters_on_bank_route_folds == 64`, which is
  the fact PR #913 could not state. Whether `meters_on` then reads below its pre-#885 self is a
  separate question that only a controlled host answers; the #881 metered live-console row is
  still the right place for a per-block metered number and is still open (amended to carry the
  same counters).
- Every `output_sha256` on every row, leg and arm must equal the baseline arm's; a differing
  digest is a class-A failure of the batch, not noise.

## Coordinator decisions on the owner questions

1. **Ring hold (S1a):** keep `count + 1`, charged to `overhead_bytes`. Written into the brief.
2. **Failure fill (O9):** zero-fill, scoped to executor-level errors; envelope rejections made
   before any unit runs leave the planes untouched. Written into the brief and its gate 3.
3. **Web `source_reserve`/`source_commit` ABI:** not this cycle. Note for the owner: the
   browser feed still copies JS -> wasm staging (`miso_engine_web_v1_source_submit`,
   `hosts/host-web/src/ffi.rs:5347`) -> `TransferBlock` (`submit_planes`) -> arena (until S1b);
   removing the middle copy is a `miso_engine_web_v1_*` addition with a worklet shim change,
   artifact repin and browser qualification.
4. **#881 amendment:** applied (one sentence in
   `.github/ISSUE_SPECS/881-add-a-metered-live-console-row-to-the-console-benchmark.md`).
5. **Unbanked route fold:** accepted in the reviewer's Output-only, all-or-nothing scope as R1,
   after O9.

## Considered and not briefed

**"Treat a strip with no processing as a trivial bank so route and master sum happen eight
lanes at a time."** Declined. A bank chain with no slots would still pay a planar-to-AoSoA
gather transpose and a scatter/fold transpose to reach the epilogue; the route's `mix2x2` is
already vectorised eight frames wide per track (`mix2x2_block::<FrameLane>`), so banking moves
the same lane-ops from frame-major to track-major and adds two transposes per track. That is
more passes, not fewer, and fails the class-A "fewer passes" clause on its face. R1 is the
form that removes passes: it fuses the route into the reduction with no transpose at all.

**Multi-quantum host submit (audit S1 proper).** Not briefed: it is a web feed change with its
own ruling history (`docs/rulings/bulk-source-submit-null.md` kept it open) and no render-path
copy; it belongs with decision 3.

**Dedicated read-only "external buffers" in `disjoint.rs`** as the mechanism for O9/S1b. Not
used: it would add a compare or an indirection to every lease access to save one copy, or move
`unsafe` reasoning about host lifetimes into the arena. Both briefs instead thread a borrowed
destination/source through the few call sites that name the master or a claimed source buffer,
keeping `crates/graph` free of `unsafe` and adding no branch to the arena's hot accessors.

## Audit facts found stale on the base

- Every line anchor in O3/O9/S1/S2 moved (e.g. `fold_plane`/`fold_cohort` 1218-1305 -> 1345-1428;
  `tile_scatter` 281-300 -> 290-308; the host copy `lib.rs:2283-2285` -> 2281-2283;
  `reduce_plane` 275-289 -> 298-313; `foldable_lane` 4826 -> 5120; `scatter_target` 4494 ->
  4761; `observe_unit` 1915 -> 2050; `submit_planes` copy 849-855 -> 846-852). The drafts cite
  the new positions.
- O1 ("metering disables the fold and the redirect") is delivered by #885/#886; O5 by #898
  (group-of-eight with a carried running sum, a ruling in its record); O6 by #900. `fold_cohort`
  now takes the carry through `ordered_accumulate_block`, which O3's fused pass replaces on the
  all-folded tiled path.
- O9's "single-track and single-submix sessions pay two copies" is overstated: a sole single
  undelayed producer of the Output node is lowered in place (`program.rs:691-700`), so those
  sessions pay one copy today unless the producer is dedicated storage or has another reader.
  O9 makes the Output node dedicated, which turns the in-place case into a direct producer-to-host
  copy: still one.
- §1.3's "ring -> arena, 1 copy" is true of ring-fed sessions only; the console benchmark never
  takes it (frozen bound sources). The audit did not say so, and it matters for reading this
  cycle's benchmark.
- The route_fold ledger's remark that "the colouring gives the session output the physical slot
  of track zero's input buffer" (`runtime.rs:5205-5208`) is still true on `main` and is what O9's
  dedication of the Output node changes; O9's brief has the implementer correct the comment.
