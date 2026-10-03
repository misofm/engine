# #1201 *Delay a submix strip's summed input*: Sol verdict, attempt 1

- Reviewed: `git diff ea951253 84d26a1d8` (branch `codex/batch-submix-k1`, worktree
  `/home/bl/misofm/wt-submix-k1`): 7 files, +573/-36.
- Binding: `AGENTS.md` and `.github/ISSUE_SPECS/1201-delay-a-submix-strips-summed-input.md`,
  including its Attempt 1 record and deviations.
- How I ran it: I did not touch the worktree, the branch or GitHub (I only read `gh issue view 1201`:
  OPEN, and the title matches the spec's H1).
  - I exported `84d26a1d8` with `git archive` to `/tmp/claude-1002/v1201/src` and built with my own
    `CARGO_TARGET_DIR`. Debug builds used `CARGO_PROFILE_DEV_DEBUG=0` to save disk.
  - I applied every mutation to the export only and reverted each one. Before the gates ran I
    `diff -r`'d the export against a fresh archive: IDENTICAL. I removed the export afterwards.
- My proposed test for MINOR-1 is in `submix-verdicts/1201-attempt1-verifier-scratch.rs`: green on
  the commit, red under mutation C.

## Verdict: PASS

There is no BLOCKER and no MAJOR.
- D1-D5 are all present, at the sites and in the shape the spec freezes.
- Every gate I could run passes when I re-run it.
- The behaviour change for a non-source, unbound node cannot move a production bit (proof below).
- The fold order is right, and per-lane delays work.

There are two MINOR findings and two NITs. Neither MINOR needs another attempt. MINOR-1 is a test
the root can adopt.

## Gates (re-run by me on `84d26a1d8`, x86-64-v3, native W8)

| Gate | Command | Result |
|---|---|---|
| 1, 3, 4 | `host-core --test submix_strip` (features `host-core/test-support,engine/realtime-audit`) | 10/10 ok |
| 2 | `graph --lib a_bus_delay_runs_on_the_sum_after_the_reduction` | ok |
| 5 | `cargo build --locked --release -p audit -p bench -p capi -p session-validator` | exit 0 |
| 5 | `check-graph-determinism.sh` | PASS (100/100) |
| 5 | `graph_fixture -- --check` | exit 0 |
| 5 | `check-console-fixtures.sh target/release/session_validator` | ok |
| 5 | `check-builtins-fixtures.sh . target/release/audit` | ok (50 files) |
| 5 | `trace-graph-audit.sh target/release/audit` | PASS (1,000,000 blocks). See note N1. |
| 5 | `graph-compiler/tests/track_delay.rs`, `host-core/tests/track_delay.rs` | unchanged files, all ok (inside the runs below) |
| 6 | test-debug-a workspace command (exact excludes and features from DESIGN section 7) | exit 0; 98 binaries, 1129 passed, 0 failed |
| 6 | `cargo test --locked --release -p audit -p bench -p console-workload` | exit 0; 10 binaries, 110 passed |
| 6 | `cargo fmt --all -- --check`; workspace clippy `--all-targets --all-features -D warnings` | both exit 0 |
| 6 | `check-`/`test-` policy pairs for graph, realtime and workspace | all exit 0 |
| extra | `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps -p graph -p graph-compiler -p host-core` (host-core with `control-provider`) | exit 0 |
| 6 | `run-aarch64-tests.sh debug` | **Not runnable here.** The host is x86_64, with no aarch64-linux target, no cross linker and no qemu-user. It must run in CI's `aarch64-debug` job at the batch push (N2). |

The commit touches only authorized paths.

## Adversarial checks

**Realtime.**
- The new `SumDelay` arm sits in `execute_op`, inside the `REALTIME_POLICY` region (`runtime.rs`
  2943-3239).
- It calls `output_planes` and then the same `TrackDelayLine::process` as `TrackDelay`, which is
  itself in a policy region (761-784). That is a ring swap, bounded by `block.len() / ring.len()`
  takes, with no allocation, lock, syscall or data-dependent loop.
- The ring is sized in `node_kind`, at bind, on the control thread.
- Gate 4 measures zero allocations and frees after block 0, with both the render audit and the
  thread-scoped counters.
- Every render path reaches the arm: no unit is inert unless it is a `SourceInput`
  (`unit_inert`, 2524), and the graph runtime has no silence-skip path.

**PDC.**
- `SumDelay` adds nothing to latency, timing or `inserted_delays`.
- Gate 1 asserts that `output_latency` and `inserted_delays` match the undelayed artifact. On this
  single-path session, declaring the delay as latency would move `output_latency` by 37.

**Fold order and per-lane delays.**
- A fold master's inputs are rewritten to its own output. Its `execute_op` reduction is then a
  no-op over the folded sum, and the `SumDelay` arm runs after it. So the order is sum, then delay,
  then strip (the bus `PostInputBuiltins` reads the delayed buffer).
- The fold's first contributor *stores* every block. The delayed words left in the master buffer are
  therefore never accumulated onto.
- `route_fold` declines on `parts.has_source(master)`. A delay entry is not a source, so a delayed
  bus keeps its fold. Gate 3 checks the count, 8 = 8.
- Different L/R delays on a folded bus are covered by gate 1. Its two-track bus is a fold master
  (Terra's fold-master mutation turned it red), and it uses 37 on the left and 0 on the right.
- **Mutation A** (mine): the arm swaps its lanes (`process(out_right, out_left)`). Gate 1 goes RED;
  gate 2 stays green because its delays are equal.

**Gate 2's mutation (Terra's ledger row 1201-2), reproduced.**
- **Mutation D**: the `SumDelay` line moves into the early-return path, before the reduction, and
  returns.
- Gate 2 goes RED with exactly the ledger message: `lane 0 sample 5: 0 != 5398.029`.
- Gate 3 also goes RED: in the fold-declined arm the reduction overwrites the delayed words.
- Gate 1 stays GREEN. Its bus is folded, and a fold master's sum is already in the buffer before the
  op runs. This is why gate 2 (unfolded, graph-level) and gate 3's declined arm are what pin D3.
  It is consistent with the spec's hazard note.

**The behaviour change.** A delay entry on a non-source, unbound node now delays that node's sum
instead of being dropped. I checked whether any production session can reach that path for a track
and change output bits. **No (refuted):**
1. A compiled plan puts every track `Input` in `required_bindings` (`compile.rs` 810-823; only
   submix Inputs are excluded).
2. Bind requires the supplied bindings plus the source claims to equal `required` exactly
   (`lib.rs` `bind_optional_source_set`, the coverage merge).
3. So a track `Input` is either a source claim, which lowers to `TrackDelay` (unchanged), or a
   supplied binding.
4. host-core's `prepare.rs` (~1160) filters `TrackStage::Input` out of its identity bindings and
   serves every track input through the source set. host-web and capi both prepare through
   host-core. The audit tools (`tools/audit/src/{graph,builtins_graph,fixture_builtins}.rs`) and
   `console-workload` bind track inputs to processors. That is `Bound`, which precedes the new arm,
   so it is unchanged (and the entry is unused, as before).
5. The new arm is therefore reachable for a track only through
   `GraphNodeBinding::identity(<track Input>)`, and no production or tool code does that.
6. Even there, a track `Input` has no graph inputs. `reduce_plane`'s `[]` arm fills `+0.0`, and a
   ring of `+0.0` emits `+0.0`, which is bit-identical to the old `Identity` fill.
7. What remains observable: one extra ring at bind (already charged by the compiler's estimate),
   and the evidence-only symmetry census when L != R.
- A host cannot bind a submix `Input` either, because it is not required and coverage would fail.
- `strips()` yields tracks first, in `model.tracks` order. A track-only session therefore emits the
  identical `track_delays` vector, and the gate-5 fixture and digest checks confirm it.

**Dropped bind-time assert.**
- I accept it. The spec's hazard names gate 1 as the catch for an unconsumed submix entry, and
  Terra's D1 tracks-only mutation turned gates 1 and 3 red.
- The remaining silent drop is a delayed track `Input` bound to a host processor. It predates this
  slice and is harness-only.

## Test value (one sentence each)

- **Gate 1, `a_bus_delay_shifts_its_summed_input_and_pdc_never_sees_it`**: red if a submix's delay
  is not lowered (Terra's D1 mutation), lands on the wrong lane (my mutation A), delays one
  contributor instead of the sum, or is charged to PDC. No other test delays a bus.
- **Gate 2, `a_bus_delay_runs_on_the_sum_after_the_reduction`**: red if the line runs before the
  unfolded D9 reduction (mutation D, reproduced). It is the only graph-level test of a delayed
  reduction, with a delay longer than the quantum.
- **Gate 3, `a_folded_bus_still_delays_its_sum`**: red if a delayed bus loses its fold, if a fold
  master skips its kind (Terra), or if the delay precedes the reduction on the declined arm
  (mutation D, mine).
- **Gate 4, `a_delayed_bus_renders_without_allocating`**: red if the bus line allocates on render
  (Terra's `Vec` mutation). #1200's gate 8 has no bus delay.
- **The refactors are faithful.** `assert_renders_without_allocating` and `graph_artifact` keep
  #1200's gate 8 and its PDC test with the same plan ID and the same assertions.

## Findings

### MINOR-1: no test composes delay lines, and line-index aliasing goes unnoticed

- **Mutation C** (mine): `track_delays[0].process(..)` in the `SumDelay` arm.
- It stays GREEN across every test in graph, graph-compiler and host-core (`--all-targets` with
  test-support).
- No committed session has a delayed track feeding a delayed bus, or two delayed buses. So aliasing
  between the `TrackDelay` and `SumDelay` arms, or between two buses, is uncaught. Two arms now
  allocate from one line vector in `node_kind`, so this is a plausible refactor defect.
- The code is correct today. My scratch test passes on the commit: positions L [23, 51] and R
  [8, 10], bit-equal to the undelayed impulses.
- **Fix:** adopt `submix-verdicts/1201-attempt1-verifier-scratch.rs` into `submix_strip.rs`. It
  sets t0 to 11/0 into a bus at 37/5, and t1 into a second bus at 13/0. Under mutation C it is RED
  (`left: [21, 25]`, `right: [23, 51]`).
- Test value: red if delay lines alias across arms or strips, or if a track's delay and its bus's
  delay fail to add.

### MINOR-2: the D5 witness arm has no test (an accepted deviation)

- **Mutation B** (mine): the `SumDelay` witness always answers `designed(true)`. Every test stays
  GREEN across graph, graph-compiler and host-core.
- This matches the recorded deviation. The witness feeds only `symmetry_counters` and
  `lane_eligibility`, which are evidence, and a bus is never collapsed. So no product bit depends
  on it.
- **Optional fix:** add a sibling unit test in `runtime.rs` tests, next to
  `the_node_witness_agrees_with_its_line`, without editing that test (the non-goal). It would
  assert `NodeKind::SumDelay { line: 0, channels_agree: l == r }.channel_symmetry().eligible() ==
  (l == r)`.
- Alternatively, record in the spec that the arm is evidence-only and deliberately untested.

### NIT-1: the schema doc's "everything below" over-reaches

- `docs/SESSION_SCHEMA_V1.md` says: "On a submix, `delay_samples` delays the summed input instead;
  everything below applies to it unchanged."
- The paragraph below also carries the trim, polarity and HPF/LPF live-command sentences (kinds
  10-12). Live controls address tracks only (canonical track order), so "everything below" can be
  read as making a bus's lanes live-addressable.
- **Fix:** say "the next paragraph's rules apply to it unchanged: not latency, charged to
  `graph_delay_bytes`, prepared-only".

### NIT-2: a stale estimate comment

- `crates/graph-compiler/src/estimate.rs:26` still says "every delayed track". It now covers every
  delayed strip.
- The file is outside this slice's authorized paths, so it belongs in the K1 follow-up ledger.

### Notes (no action)

- **N1.** `trace-graph-audit.sh` uses a fixed audit graph with no submix delay, so it does not
  execute `SumDelay`.
  - The arm's realtime evidence is gate 4 (exact zero allocations) and inspection: it calls the
    same `process` as `TrackDelay`, which the trace already covers through the `pdc_delay_block`
    kernel.
- **N2.** 4-lane coverage is unverified locally.
  - Gate 3 asserts `folds > 0`. At Simd4, eight tracks form two cohorts folding into one master.
  - The fold proof is width-agnostic, and the standing 64-track workloads fold at every width, so I
    expect it to hold. CI's `aarch64-debug` job at the batch push is the check.
