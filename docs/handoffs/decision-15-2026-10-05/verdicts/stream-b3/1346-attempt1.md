PASS

# #1346 attempt 1 -- adversarial verdict

Reviewed `git diff 010ae8739 315637606` (code 62ad55156, 864dbf13a, ee72e3427; evidence 315637606) on
an export of 315637606 at /tmp/claude-1002/v1346/tree (git-initialised for the reachability step),
built with its own target dir. Mutations ran on a second export with a second target dir. Read the
spec at 315637606 (the implementer only added the Attempt record; no line of the body at 010ae8739
changed), AGENTS.md, decision 15, the #1312 spec and its batch-2 verdict, and STREAMS.md. No BLOCKER
or MAJOR finding.

## What was checked and holds

- **D1/D2 cells and drain** (crates/builtins-compiler/src/lib.rs:615-960). There are eight cells
  (trim L/R, polarity L/R, filter s0 L/R, s1 L/R) and one dirty word, and the cell index is the
  dirty bit. `InputReader::drain` reads each pending cell under the snapshot that the caller passes
  in. It pairs two `Applied` channel words into one `Both` record only when the words are equal
  (`paired_channels`). It applies trim, then polarity, then section 0, then section 1. It applies
  every value before it returns the first refusal (`first_error.and(apply(..))` evaluates
  `apply` eagerly). It adds the replaced count to the session counter once per drain.
- **D3 witness.** `witness.admit(&record)` runs on the rebuilt record before each apply. An equal
  pair is a `Both` record (`Preserve`). A lone channel, or unequal words, gives per-lane records
  (`Desymmetrize`). Mutations M1/M2/M14 confirm this.
- **D5 carry.** `drain_for_carry` is `drain_controls(LiveSnapshot::ALL, true)`, which counts a
  refused apply and continues. M16 (the carry applies nothing) turns the #1276 carry tests in
  successor_swap.rs red.
- **Block drain under the snapshot.** `begin_block` passes the block's snapshot. M5 (`ALL`) is
  red only in the new unit test. The scalar per-node processor drains under
  `block.live_snapshot`.
- **D6/D7 browser** (hosts/host-web/src/lib.rs:4677-4800, 5730-5775). The input band leaves the
  room pass and `in_flight`. It only checks that the strip holds its input cells
  (`UNSUPPORTED_KIND` otherwise), before the stamp. `write_strip_cells` now also writes `Input`
  records and the prepared builtin targets of a `BuiltinInput` owner marker, under the batch's one
  stamp, after every check. Before the stamp, staging validates the targets: `slot > 1` is
  refused, `validate_prepared_input_filter_target`, `validate_builtin_target_set` and the shadow
  base revision are all checked. The write pass tests the address inline. It is equivalent to
  `prepared_queue_address` for a builtin target because the marker's strip has a shadow. The
  `assert_eq!(written, target_count)`, the queue pushes, the EQ publishes, `commit_owner` and the
  shadow commit are all `expect`s or asserts after the stamp. The filter shadows are still committed
  (the second owner loop), so they stay the control-side authority. M11 (cell writes moved before
  the room pass) turns eight browser refusal tests red.
- **Acked-batch question: can an ack ever precede a drop? No.** All fallible checks come before
  the first cell write. `InputCells::write` cannot fail on a validated value. Its only panic is a
  section other than 0 or 1, and both hosts refuse that before the stamp. A replaced value is in
  the committed model and is counted (M7a/M7b red). A value above a block's snapshot stays dirty
  for a later block. The carry drain reads everything that the predecessor holds and continues past
  a refusal. If an invariant breaks after the stamp, the result is a panic (a trap in Wasm), never
  an ack with nothing written.
- **Render safety.** The two new `REALTIME_POLICY` regions (the bank's `drain_controls`;
  `paired_channels`, `read_input_cell` and `InputReader::drain`) contain no allocation, lock,
  `expect`, `unwrap` or panic. `check-realtime-policy.sh` reports 102 regions in 27 files.
  `audit capi` reports allocations, deallocations, locks, syscalls, file and network I/O, and panics
  as 0, and `total_violations` as 0. The builtins and graph trace audits and the probe mutations
  pass. On the shipped module, `check-web-audioworklet.sh` passes: the render call-graph closure is
  allocation-free and trap-free. The shipped module is `ffec707cbbe5317b...`, the same as the
  implementer reported.
- **live_lanes / slot guard (point 6).** `BuiltinStage` now forwards `live_lanes` (M9 is red in
  the new graph test). Input, fader, matrix and fused banks report the lanes that hold a reader. The
  guard in `BankChain::new` (`crates/rack/src/lib.rs:2081`,
  `live_lanes() == 0 || active_lanes != 0`) therefore gets non-zero masks for builtin banks and no
  longer short-circuits on the first disjunct. But a builtin slot is always active, so no reachable
  builtin configuration can make the guard fire. Also, no test pins the four implementations (m2).
  The guard is non-vacuous in principle and unreachable in practice, as the spec's Dependencies
  section says.
- **Re-pins (point 1), each arithmetically checked.**
  - `STRIP_PREPARATION_BYTES` 1088 -> 1136 (+48 = `InputReader` 88 - ring `Consumer` 40).
  - `resources.jsonl`: every processor-payload and largest-allocation field moves by exactly
    48 x tracks: 1 -> +48, 4 -> +192, 65537 -> +3,145,776. The allocation counts do not move.
    Only that row of `MANIFEST.tsv` changes.
  - Joined-corpus pin and `ACCEPTED_MANIFEST_SHA256`: the same new manifest digest.
  - `direct-route.resources.json` `builtin_bank_bytes` 6663 -> 7047 (+384 = 8 lanes x 48).
    `incremental_plan_bytes` and `session_plus_plan_bytes` move by the same +384, and no other
    field moves.
  - All of these pin the single owner of a corpus or an existing audit constant, so AGENTS.md's
    digest rule permits them. `check-builtins-fixtures.sh`, `check-graph-determinism.sh` and the
    release audit unit tests all pass on them.
- **Out-of-list edits (point 1): all forced, and none expands scope.**
  - `crates/graph/src/{lib,runtime}.rs`: required by the spec's Dependencies section.
  - `crates/graph-compiler/src/lib.rs`: four test literals for the removed `queue_capacity` field.
  - `crates/control-plane/src/compile.rs`: one doc comment that would otherwise be false.
  - host-core tests: `try_push` is gone.
  - tools/audit and fixtures: re-pins.
  - The realtime-policy floor (96 -> 102) and the self-test pad (14 -> 20): the STREAMS standing
    exception covers these, and "B's other cell slices (#1312, #1346, #1347)" are named there. The
    attempt record names the edit, as the exception requires.
  - The STREAMS hot-file rows do not list #1346 for `crates/graph/src/{lib,runtime}.rs`,
    `crates/graph-compiler/src/*` or `crates/host-core/src/prepare.rs` (see m5).
- **Point 2.** The rewritten dual arm of `a_symmetric_ride_renders_the_same_bits_collapsed_or_not`
  writes a `Both` trim and a left-only polarity write. The polarity write keeps the value that the
  lane already has and uses the same 512 window. It still declines all eight lanes (census
  assertion) and still renders the bits of the collapsed arm. Mutation M15 removes
  `mirror_trim_ramp` from `InputStage::process_mono`, which is the defect named in the test's
  doc, and the test goes red. So it still defends what it defended. The old split into two
  per-lane trim records cannot exist any more: two equal channel words are one `Both` record by D2.
- **Point 3.** HPF-only edits (selector 3) write the two section-0 cells. That is what the spec's
  "30 Both HPF edits (section 0)" says. A whole-pair edit would touch section 1 too and break the
  116 arithmetic. OK.
- **Point 4.** M4 (section 1 drained before section 0) stays green in BC, HC and WEB. The two
  sections hold independent state, so the order is a determinism convention with no observable
  effect. It is acceptable because it is recorded as such (attempt record item 9, open item c).
- **Point 5.** `ReadyOwnership::live_values_superseded` under `#[cfg(test)]`
  (hosts/host-web/src/lib.rs:1531-1532) is a test observation seam. It is a clone of the real
  per-plan count that render already adds to, and it has no product behaviour. Reporting the count
  is #1399's deliverable, so it is not an interim shortcut. The #1312 verdict's m3 (the browser
  count is per plan, not per session, and is not charged) still stands for #1399.
- **#1509 wording.** The added prose uses "producer" only for the existing
  `TrackControlProducer` type and `Consumer` for the removed type. There is no new role vocabulary
  (but see n4).

## Findings

### MINOR

- **m1. Gate 5 was rewritten on effect-bypass queues, not on observation records as the gate
  says.** The four rewritten browser tests are `idle_render_skips_admission_counter_clear_...`
  (hosts/host-web/src/tests.rs:2763), `prepared_builtin_input_filter_owner_is_atomic_...`
  (:3741), `prepared_eq_owner_refusals_...` (:4052) and `a_refused_solo_submission_...` (:7731),
  with the new helpers `stage_effect_bypass` (:2745) and `solo_host_with_insert`. They now fill
  effect queues with `COMMAND_EFFECT_BYPASS`. #1345 (stream B, next) makes bypass a cell
  (#1345 D1; "Observe stays a FIFO", D2), so #1345 must rewrite these four tests again. The attempt
  record names the choice but does not say why `COMMAND_OBSERVE_SUBSCRIBE` was not used. Note:
  #1345 D2 sizes the observation queue by tap count, so an observation-based rewrite would also
  need adjustment then. Fix: record in #1345's superseded-tests gate that these four browser tests
  now ride the bypass band, or rewrite them on observe records now.
- **m2. No test pins the four builtin `live_lanes` implementations.** Mutation M10 makes the
  input, fader, matrix and fused `live_lanes` return 0 (crates/builtins-compiler/src/lib.rs:1292,
  1530, 1614, 2078). It survives BC, HC, WEB and GRAPH. Only the `BuiltinStage` forward is
  defended, and only with a fake bank (crates/graph/src/runtime.rs:8905-8939). Fix: add a
  builtins-compiler assertion that a prepared banked plan's input, fader and matrix banks report
  exactly the lanes that hold a reader (and 0 with `input_lane: false` for the input bank).
- **m3. `equal_words_written_per_channel_in_one_block_keep_the_collapse` has no unique catch**
  (crates/host-core/tests/input_liveness_live_controls.rs:614). Its stated mutation (M1) also turns
  red five existing host-core tests and the BC gate-3 test. M14 (never pair) turns red the same
  set. I found no plausible defect that only this test catches: the separate-but-equal case is
  already case 2 of `the_input_witness_folds_from_the_final_cell_contents`, and the end-to-end
  collapse dispatch is already covered by `a_both_lane_record_drained_while_collapsed_keeps_the_collapse`.
  AGENTS.md test-value rule: name a unique defect, or delete the test.
- **m5. Hot-file rows to ratify.** These edits are forced, but the STREAMS table does not list
  #1346 for:
  - `crates/graph/src/{lib,runtime}.rs`: a trait method with a default, the forward and one test.
  - `crates/graph-compiler/src/*`: four test literals.
  - `crates/host-core/src/prepare.rs`: the authorized path, but the row lists only B #1312 and
    #1344.
  Root should add #1346 to these rows, or ratify the edits. The later slices in those rows rebase
  trivially.

### NIT

- **n1.** crates/builtins-compiler/src/lib.rs:14510-14524: `let prepared = ...; let _ = prepared;`
  is dead code in `many_input_edits_before_one_block_...`.
- **n2.** The doc of `the_input_drains_apply_every_value_past_a_refused_one`
  (lib.rs:14209-14213) names a red mutation that stays green. Mutation M8b replaces the carry
  arm's count-and-continue with returning the error, and the test passes, because the drain still
  applies every value. The carry arm defends only the refused count. The block arm is what catches
  M8 (the drain stops at the first refusal). Fix the doc.
- **n3.** hosts/host-web/src/lib.rs:5685-5694: `command_wanted` still accumulates for the input
  band's markers and refuses with `BACKPRESSURE` on `u32` overflow. That is a leftover of the queue
  semantics, unreachable, and meaningless for a cell band.
- **n4.** lib.rs:983 "shared by the producer and the reader": spell out `TrackControlProducer` so
  that the prose does not read as a role (#1509).
- **n5.** `BuiltinBankProcessor::drain_controls` now skips a lane's whole drain when
  `live.get_mut(lane)` is `None`. The old code still applied the records. This is unreachable
  (`live` is `[_; MAXIMUM_BANK_LANES]`), but a silent skip of acknowledged values is the wrong
  fallback shape. Use a `debug_assert!` or index directly.

## Test value (one sentence per new or rewritten test)

- `input_cells_drain_in_the_canonical_order_whatever_the_write_order` (BC, gate 2): goes red when
  polarity drains before trim (M3), which moves bits when the two windows differ. Nothing else
  catches it.
- `the_input_witness_folds_from_the_final_cell_contents` (BC, gate 3): goes red on a fixed
  `Desymmetrize` (M1), a fixed `Preserve` (M2) or no pairing (M14). It is the only BC test of the
  equal words written separately to each channel in one block.
- `many_input_edits_before_one_block_apply_the_last_and_count_each_replaced_value` (BC): goes red
  when the replaced count is dropped (M7a) or counted for the left trim cell only (M7b), at crate
  level.
- `the_input_drain_reads_under_the_blocks_snapshot` (BC): goes red when `begin_block` reads `ALL`
  (M5). It is the only red test across BC, HC and WEB.
- `the_input_drains_apply_every_value_past_a_refused_one` (BC, rewritten): goes red when the drain
  stops applying after a refusal (M8, block arm) or the carry applies nothing (M16).
- `a_block_applies_the_last_of_many_input_edits` (BC integration, rewritten): goes red when either
  the banked or the scalar drain is skipped or applies an older walk value (M6), through full
  prepared graphs, with walks longer than the old ring.
- host-web `many_input_edits_before_one_render_apply_the_last_and_are_counted` (gate 1): goes red
  when the count is lost or one-sided (M7a/M7b), the browser skips the input record's cell write
  (M12) or skips the prepared filter target's cell write (M13). It is the only browser test of the
  count.
- graph `a_builtin_stage_forwards_its_banks_live_lanes`: goes red when `BuiltinStage` drops the
  forward (M9). Nothing else catches it.
- host-core `equal_words_written_per_channel_in_one_block_keep_the_collapse`: no unique defect
  found (m3).
- `a_symmetric_ride_renders_the_same_bits_collapsed_or_not` (rewritten): still goes red when
  `mirror_trim_ramp` is dropped (M15).
- `a_both_lane_trim_command_addresses_both_lanes` (rewritten): goes red when the browser skips the
  input write (M12). The implementer's `Both`-as-`Left` lowering mutation was not re-run.
- The rewritten browser refusal tests (`a_refused_solo_...`, `prepared_builtin_input_filter_owner_is_atomic_...`):
  go red when cells are written before the room pass (M11). `idle_render_...` and
  `prepared_eq_owner_refusals_...` keep their queue-credit claims on the effect band (see m1).

## Mutations re-run (second export, one at a time, restored with `git checkout` and verified clean)

Baseline: BC, HC, WEB and GRAPH all green.

| # | Defect | Red |
|---|---|---|
| M1 | fixed `Desymmetrize` admit | BC witness-fold, per-node; HC: 6 tests incl. equal_words, symmetric_ride, census |
| M2 | fixed `Preserve` admit | BC witness-fold, per-node; HC one_channel_record_after_swap, symmetric_ride, census |
| M3 | polarity before trim | BC canonical-order only |
| M4 | section 1 before 0 | **none** (independent state; accepted, point 4) |
| M5 | `begin_block` reads `ALL` | BC snapshot test only |
| M6 | `begin_block` drains nothing | 7 BC, 17 HC, 3 WEB |
| M7a | count not added | BC many-edits, WEB gate 1 |
| M7b | count left trim cell only | BC many-edits, WEB gate 1 |
| M8 | drain stops at first refusal | BC drains-past-refusal (block arm) |
| M8b | carry arm returns the error | **none** (n2) |
| M9 | `BuiltinStage` drops the forward | graph forward test |
| M10 | four builtin `live_lanes` return 0 | **none** (m2) |
| M11 | browser cell writes before room pass | 8 WEB refusal tests |
| M12 | browser skips input record write | WEB both-lanes, trim reaches render plane, gate 1 |
| M13 | browser skips filter target write | WEB gate 1, filter-owner atomic |
| M14 | equal words never paired | 3 BC, 6 HC |
| M15 | `mirror_trim_ramp` removed from process_mono | HC symmetric_ride, both_lane_record_drained_while_collapsed |
| M16 | carry drain applies nothing | BC drains-past-refusal; HC 3 successor_swap carry tests |

Logs kept: /tmp/claude-1002/v1346/mutlogs/ and /tmp/claude-1002/v1346/logs/.

## Gates run (export of 315637606)

- **qualification `lint` job, every step:** all pass.
  - fmt, clippy (`--workspace --all-targets --all-features -D warnings`), doc (`-D warnings`).
  - workspace policy and its mutations; session policy; env vocabulary; bench policy.
  - test-support CI coverage and its mutations (12 packages).
  - scalar-oracle self-test; script reachability (120 files) and its mutations.
  - console benchmark fixture; builtins-fixtures mutation tests; bench preconditions.
  - host-core and protocol policies.
  - realtime policy (102 regions, 27 files), its self-test, the audit-leak and artifact-evidence
    gates; realtime trace validator.
  - lane policy; unfused seal; check-rack-policy, builtins-policy and graph-policy, each with its
    mutations; effect-runtime policy and fixtures.
  - conformance boundaries; PEQ render-contract seal; release-shape self-test; npm publish modes;
    stem store; stem-identity drift.
  - the two sub-v3 refusal probes and the AVX2+FMA probe.
- **qualification `test-release` job:** all pass.
  - lane/math/wasm-gates release; filter_liveness; tail_contract; ramp_endpoint (8 crates);
    designer_total `--ignored`.
  - M3 FMA cfg; the loom SPSC leg.
  - The math-closure steps are not routed: no file under crates/math or crates/lane changed.
- **Spec gates:** all pass.
  - builtins-compiler `--features test-support`: 87 passed.
  - builtins `--features test-support,lane/test-support`: 155.
  - host-web `--features test-support`: 196.
  - host-core `--features control-provider,test-support`: 299, the #1276 carry tests (gate 4)
    included.
  - graph `--features test-support`: 100; graph-compiler: 170.
  - `cargo build --locked --release -p audit -p capi && audit capi`: all violation counts 0.
- **Also run:**
  - test-debug-a, the whole set with its features: 131 result lines, 1555 passed, 0 failed; its
    doctests pass.
  - audit-native: release build; audit, bench and console-workload release tests; C ABI audit
    record; builtins and builtins-graph traces; graph trace; builtins probe mutations;
    check-capi-abi and its self-test; scalar-oracle `--native libcapi.so`; graph determinism
    (100 processes); check-builtins-fixtures; check-console-fixtures. All pass.
  - check-cross-targets: pass.
- **Worklet chain:** all pass.
  - `build-web-audioworklet.sh --named-twin` (module `ffec707cbbe5317b...`).
  - `check-web-audioworklet.sh --without-metadata-regeneration`.
  - `check-browser-expected-resources.py --artifacts` (graphSessionPlusPlanBytes 30098 of 35648).
  - `check-scalar-oracle-absent.py --wasm`.
  - hermetic `test-web-audioworklet.sh` with a private TMPDIR, left empty.
  - V8 spill gate and its self-test.
- **Not run:** the Playwright `browser` legs (no browsers on this box); test-debug-b, wasm-guests
  and the SDK job (this slice does not change their inputs except through the module, and the
  artifact gates above already cover the module). Root should run the browser legs at the batch
  boundary.
