# Move a source consumer into a successor graph plan

Slice 2 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

A graph plan can be bound with a **vacant** source entry, and at the swap block it takes the
predecessor's `PcmSourceConsumer` for that entry: the ring, its queued blocks, its generation, its
read position and its pending generation-change flag. The successor's next block then reads the same
audio the predecessor would have read. This is the render half of source continuity; *Prepare a
successor plan whose unchanged sources keep playing* (slice 3) builds the host-core half on it.

## Context

- `SourceGraphSourceSetDriver` (`crates/source/src/lib.rs:1407`) owns `Box<[GraphSourceEntry]>`
  (`:1403`, one `PcmSourceConsumer` each, `:955`) and a `pending_generation_change` flag, and drives
  them through `begin_block` (`:1494`), `copy_track_input` and `played_planes` (`:1572`). The graph
  sees it as `Box<dyn GraphPreparedSourceSetDriver>` (`crates/graph/src/lib.rs:2109`, `Send` only)
  inside `GraphPreparedSourceSet` (`:2165`), owned by `GraphExecutor` (`:2628`) in `source_set`.
- `prepare_graph_source_set` (`source/src/lib.rs:1609`) seals consumers into a set and computes its
  resource report.
- Hand-built graph plans with a source set already appear in the source crate's tests
  (`one_four_channel_source_fans_out_to_three_inputs_in_the_sequential_executor`, about `:2346`).
- Every host passes `plan_id: 1` (`crates/host-core/src/prepare.rs:1095`), so `plan_id` cannot tell
  two plans apart.
- *Hand the outgoing plan to its successor at the swap block* (#1270) adds
  `PreparedPlanExecutor::adopt_predecessor`, `as_any_mut` and `CarryOutcome`.

## Decisions frozen for this slice

- **D1. Vacant entries.** `GraphSourceEntry` holds `Option<PcmSourceConsumer>`. A new constructor,
  `SourceGraphSource::vacant(channel_count)`, makes an entry with no ring. A vacant entry renders
  `+0.0` on every claim and reports the block as an underrun in its observation validity. The set's
  resource report charges no PCM payload and no ring overhead for it and counts it separately.
- **D2. Graph identity.** Each bound `GraphExecutor` takes a process-unique, nonzero `u64` from a
  global atomic counter at bind (off the render thread). `graph::plan_identity(&mut
  PreparedRenderPlan) -> Option<u64>` reads it through the engine's `as_any_mut` (add a hidden
  `PreparedRenderPlan::executor_any_mut` accessor in `engine`).
- **D3. Carry program.** `graph::GraphCarryProgram { predecessor: u64, sources: Box<[(u32, u32)]> }`
  (successor source index, predecessor source index), installed before publication by
  `graph::install_carry_program(&mut PreparedRenderPlan, GraphCarryProgram) -> Result<(),
  GraphCarryInstallError>`. It is refused if an index is out of range or a successor index is not
  vacant. Stored in the `GraphExecutor`; its bytes are charged to the plan's report; a plan without
  one charges nothing.
- **D4. Render-side move.** `GraphExecutor::adopt_predecessor`:
  1. downcasts the predecessor (`as_any_mut`); another type returns `PredecessorMismatch`
     (amended by the phase-1 follow-ups: the successor installed a program, so it asked for state
     and the predecessor had none to give; `NotRequested` would skip the mismatch counter);
  2. compares the predecessor's identity with the program's; a mismatch returns
     `PredecessorMismatch` and moves nothing;
  3. for each pair, calls a new `GraphPreparedSourceSetDriver::adopt_sources(&mut self, predecessor:
     &mut dyn GraphPreparedSourceSetDriver, moves: &[(u32, u32)]) -> bool` (default `false`; add
     `as_any_mut` to that trait too). `SourceGraphSourceSetDriver` implements it with
     `core::mem::swap` of the two `Option<PcmSourceConsumer>`s, and ORs the predecessor's
     `pending_generation_change` into its own. The predecessor's entry becomes vacant;
  4. returns `Carried`.

  No allocation, no free, no lock, bounded by the number of moves.

## Deliverables

1. D1 in `crates/source/src/lib.rs`; D2-D4 in `crates/graph` and `crates/engine` (accessor only).
2. Tests in the source crate (gates 1-4).

## Authorized paths

- `crates/source/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/engine/src/realtime/plan.rs` (the accessor only)

## Non-goals

- No host-core preparation, no producers, no C ABI (slices 3-4).
- No DSP state other than sources.

## Objective gates

1. **Gap-free acceptance.** In the source crate's tests, build plan A with one source feeding two
   track inputs (the shape of the existing fan-out test) and plan B with the same claims but a vacant
   entry. Feed the ring a signal with no exact-zero sample. Render A for 4 blocks through
   `engine::realtime::plan_exchange`, install B's program (predecessor = A's identity), publish B,
   render 4 more. All 8 blocks are bit-identical to A rendered for 8 blocks. Block 5 reports
   `carry == Carried`.
2. **Vacant without carry.** B published without a program renders `+0.0` and flags the underrun;
   B with a wrong predecessor identity reports `PredecessorMismatch` and also renders `+0.0`.
3. **Generation change carried.** With two directly owned plans and the synchronous form
   (`adopt_predecessor_plan`; `prepare_source_seek` needs `&mut` access to the plan), a seek prepared
   on A's consumer just before the swap is reported as a generation change in B's first block.
4. **Install refusals.** Out-of-range and non-vacant indices refuse at install, off the render
   thread.
5. Commands:
   - `cargo test --locked -p source -p graph --features graph/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - the umbrella's inherited gates.

## Test value

- Gate 1: a consumer that is copied rather than moved, or a move that happens after the first
  render, turns it red.
- Gate 2: a mismatch that still moves (handing a ring to a plan it was not built for) turns it red.
- Gate 3: a carry that drops the pending generation flag makes observers trust a block across a
  seek; it turns red.

## Dependencies

- *Hand the outgoing plan to its successor at the swap block* (#1270).

## Attempt record

### Attempt 1 (implementer, on `22c9bd5a1`)

**Implementation.**

- D1 (`crates/source/src/lib.rs`): `GraphSourceEntry` holds `Option<PcmSourceConsumer>` plus the
  channel count the mappings were validated against. `SourceGraphSource::vacant(channel_count)`
  carries no ring and no `SourceResourceReport`, so the set charges it no PCM payload and no ring
  overhead. A vacant entry sets `source_underrun` in `begin_block`, writes `+0.0` in
  `copy_track_input`, lends no planes, refuses seek preparation and reports zero telemetry. The
  set counts vacant sources apart through a new `GraphPreparedSourceSet::vacant_source_count`.
- D2 (`crates/graph/src/lib.rs`, `crates/engine/src/realtime/plan.rs`): `GraphExecutor` takes a
  nonzero identity from a global `AtomicU64` at bind. `graph::plan_identity` reads it through the
  new hidden `PreparedRenderPlan::executor_any_mut`.
- D3: `GraphCarryProgram`, `GraphCarryInstallError` and `graph::install_carry_program`. Install
  refuses a non-graph plan, a plan without a source set, an out-of-range successor index, an
  occupied successor index, and a repeated successor or predecessor index. The program is stored in
  the executor. `graph::carry_program_retained_bytes` (zero without a program) reports its move-table
  bytes; this attempt reads "charged to the plan's report" as that accessor, because the program is
  installed after bind, when the graph estimate has already been admitted. Slice 3 adds it to the
  host's plan charge.
- D4: `GraphExecutor::adopt_predecessor` follows D4 steps 1-4. On the driver trait it adds
  `as_any_mut`, `source_vacancy`, `vacant_source_count` and `adopt_sources`, each with a default.
  `SourceGraphSourceSetDriver::adopt_sources` checks every move first, all or nothing: both indices
  in range, successor vacant, predecessor occupied, equal channel counts and quanta. It then
  `mem::swap`s each pair and ORs in the predecessor's `pending_generation_change`. When a move
  fails it moves nothing, and the executor reports `PredecessorMismatch`. So a predecessor index
  that is out of range, which install cannot check, is refused at the swap block.

**Gates.**

| Gate | Test | Result |
| --- | --- | --- |
| 1 gap-free | `source` `tests::carry::a_carried_consumer_continues_the_predecessor_audio_gap_free` | green: 8 blocks bit-identical to the unswapped plan, block 5 `Carried`, no underrun or generation flag on any block |
| 2 vacant | `...::a_vacant_source_without_a_matching_carry_renders_silence_and_an_underrun` | green: no program gives `NotRequested`, a wrong identity gives `PredecessorMismatch`, both render `+0.0` with the underrun flag; an out-of-range predecessor index also gives `PredecessorMismatch` |
| 3 generation | `...::a_seek_prepared_before_the_swap_is_reported_in_the_successors_first_block` | green: B's first block plays the sought PCM and flags the generation change; its second block does not |
| 4 install | `...::install_refuses_out_of_range_occupied_and_repeated_sources` | green; also identities are distinct and nonzero, and the retained bytes are 0, then 8 |
| D1 report | `...::a_vacant_source_charges_no_ring_and_is_counted_apart` | green |

**Mutations** (each applied, run red, reverted, run green):

- M1, skip the swap in `adopt_sources` (it returns `true` with nothing moved): gates 1 and 3 red.
- M2, drop the identity comparison in `GraphExecutor::adopt_predecessor`: gate 2 red.
- M3, drop the `pending_generation_change` OR: gate 3 red.
- M4, accept an occupied successor index at install: gate 4 red.
- M5, drop the duplicate-index check at install: gate 4 red.
- M6, a vacant entry that does not flag the underrun: gate 2 red.

**Allocation proof.** The coordinator authorized `crates/graph/tests/rt11_swap_carry_alloc.rs`
for the allocation proof. It is its own test binary, holding only this test, so the audited
allocator it links cannot abort unrelated graph tests. No `Cargo.toml` entry is needed: CI's
workspace debug step builds every target with `graph/test-support,engine/realtime-audit`, and the
test passes under those features too. `the_swap_block_carry_allocates_and_frees_nothing` moves a
heap-owning slot through `plan_exchange`, renders four warm blocks first, then reads
`bench_support::alloc`'s thread-scoped counters around the swap block. It asserts `Applied` and
`Carried`, that the carried value is rendered, and that `(allocations, deallocations) == (0, 0)`
exactly. Mutations (each run red, reverted, run green): the slot cloned instead of swapped gave
`(1, 0)`, and a `Box` in `GraphExecutor::adopt_predecessor` gave `(1, 1)`.

**Commands.**

- `cargo test --locked -p source -p graph --features graph/test-support`: green (graph lib 90, source lib 24, all integration binaries).
- `cargo test --locked -p engine -p host-core`: green (29 result lines, 0 failed).
- `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: green.
- `check-workspace-policy.sh`, `test-workspace-policy.sh`, `check-realtime-policy.sh`, `test-realtime-policy.sh` and `check-capi-abi.sh`: green.
- `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`: 0 allocations, 0 deallocations, 0 syscalls.
- `bash scripts/trace-graph-audit.sh target/release/audit`: PASS (1000000 blocks).
- `bash scripts/check-cross-targets.sh`: exit 0, with only the known #1018 iOS `memset_pattern16` expected failures.
- Worklet chain into fresh directories: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts` and `test-web-audioworklet.sh` all green. `sourceTotalBytes` is 3294 of 3648, which is +8 B per source entry for the vacancy-aware entry.
- **ARTIFACT CHANGED**: the shipped module is `92415199cfb8bf6e7a08926acce85bf1737d1e4619eb658bb3fda47fb6b5d335`. Per `docs/RELEASE.md` ("Between releases"), nothing re-pins between releases, so the pin file is untouched.

### Verdicts

| Attempt | Verdict file | Result | Summary |
|---|---|---|---|
| 1 (`5488fb2f5`, `1a911e31f`) | `1271-attempt1.md` | PASS | Real-ring carry gap-free and allocation-free (verifier probe `(0, 0)`); 7 MINOR (all-or-none, repeated predecessor, vacant copy path, non-graph predecessor, channel/quantum guards untested; 5 and 6 handed to #1272), 3 NIT. |

### Phase-1 follow-ups

Closed (batch follow-ups commit on `codex/seamless-swap`; tests in `crates/source/src/lib.rs`
`tests::carry`, each red under its mutation and green after the revert):
- MINOR 1: `a_refused_second_move_moves_nothing` (two real rings, program `[(0, 0), (1, 5)]`).
  M6 (check-and-swap one move at a time): red.
- MINOR 2: `install_refuses_a_repeated_predecessor_index` (`[(0, 0), (1, 0)]` on two vacancies).
  M5a (install checks only the successor index): red.
- MINOR 3 and NIT 1: `a_vacant_claim_writes_positive_zero_over_dirty_buffers_and_refuses_seek`
  (driver-level, NaN and -1.0 buffers). M7 (vacant branch's `fill(0.0)` deleted): red; M13
  (`can_prepare_source_seek` true for a vacancy): red.
- MINOR 4: D4.1 amended above; `GraphExecutor::adopt_predecessor` returns `PredecessorMismatch`
  for a non-graph predecessor. `a_program_facing_a_plain_predecessor_reports_a_mismatch` uses a
  predecessor whose executor is not a graph executor (a plan with no executor at all stops in
  `PreparedRenderPlan::carry_from` first, #1270 NIT-4). M9 (the old `NotRequested`): red.
- MINOR 7: `the_swap_block_refuses_other_channel_counts_and_quanta` (one-channel consumer into a
  two-channel vacancy through `adopt_predecessor_plan`; the quantum guard at the driver, since two
  plans of one envelope cannot disagree). M10 (channel guard deleted): red; M11 (quantum guard
  deleted): red.

MINOR 5 (real-driver allocation proof) and MINOR 6 (charging the carry program) were handed to
#1272 and closed there. Stays open: NIT 2 (the pending generation change is per set; over-reports,
safe, as D4 step 3 specifies) and NIT 3 (install refuses a plan without a source set; later slices
that carry non-source state into such a plan must relax it).
