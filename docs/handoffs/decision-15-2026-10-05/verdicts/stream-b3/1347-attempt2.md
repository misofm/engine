PASS

# #1347 attempt 2 -- adversarial verdict (with the #1346 follow-up 51d6ba7a9)

I reviewed `git diff 731c8be30 84eb9d485` (attempt 2) and re-checked `git diff e61b85c51^ e61b85c51`
(attempt 1) against it. I also reviewed 51d6ba7a9 (it has its own section below). The spec I read
is the one at 51d6ba7a9, with its Attempt 2 section. I also read AGENTS.md, decision 15, the
attempt-1 verdict and the STREAMS hot-file rows.

- Gates ran on an export of 51d6ba7a9 at /tmp/claude-1002/v1347b/tree. I git-initialised the
  export for the reachability step and built it with its own target dir.
- Mutations ran on a second export with a second target dir.
- Realtime-policy probes ran on a third export.
- I never built in, edited or read uncommitted files in /home/bl/misofm/wt-d15-b3.

All findings of the attempt-1 verdict are fixed: M1, m1, m2, n1, n2 and n3. n4 and n5 are root's
work and are carried. The refactor moved no rendered bit and no count. Every gate I ran passes. I
found no BLOCKER and no MAJOR.

## The five questions

1. **M1. Was deleting the test safe? Yes.** No gate-4 behavior is left undefended.
   - The deleted test refused its send in the staging loop. That is before the follow pass and
     before the room pass, so its VCA half never reached the code it claimed to defend.
   - Admission has one rollback site, in `admit_commands` at hosts/host-web/src/lib.rs:4660-4664.
     Every refusal kind runs it, so a refusal at any point exercises the same `vcas.rollback()`.
   - Cells are written in one place only, `write_strip_cells` (lib.rs:4703), and only after every
     check.
   - Re-run:
     - M5 (`write_strip_cells` moved before the room pass) turns six host-web tests red. They are
       exactly the six that the attempt-2 table names.
     - M7 (`vcas.rollback()` removed) turns two tests red:
       `a_long_vca_mute_ramp_refuses_at_the_first_coalesced_index` and
       `vca_records_are_addressed_and_shape_checked`.
   - With the send queue gone, a follow record cannot overfill anything. The only late refusal
     left is the effect room pass. Its all-or-nothing rule is held by M5's six tests, among them
     `a_mixed_batch_is_one_transaction_across_every_queue`.
   - Deferring a room-pass VCA test until #1345 makes bypass a cell is acceptable: the rollback it
     would defend is already held by two tests.

2. **m2. Is the superseded count still exact? Yes.**
   - `LiveValuesSuperseded::add` is private (crates/engine/src/realtime/latest_cell.rs:725). Its
     only caller is `CellReader::read_counted` (latest_cell.rs:624-641), which is inside the
     reader's `REALTIME_POLICY` region.
   - Before attempt 2 there were exactly four writers (git grep at 731c8be30):
     - builtins-compiler lib.rs:453 (fader), :508 (matrix) and :839 (input);
     - graph runtime.rs:912 (route).

     All four now go through `read_counted`, at builtins-compiler lib.rs:438, :494 and :752 and
     graph runtime.rs:906. No other production `CellReader::read` call remains.
   - **Exactly s-p-1 per cell read.** `read` is unchanged and computes
     `sequence - last_applied - 1`. `read_counted` adds that value once, and only for an `Applied`
     read. The callers' local sums and their `.add` calls are gone, so nothing counts twice.
   - The fader and input drains used to make one saturating add of a saturating sum. They now
     make one saturating add per applied read. For non-negative terms the result is the same.
   - **No read-modify-write on render.** The count is still a relaxed load and a store, and render
     is the only caller.
   - Measured; each mutation was applied alone and then reverted. The base run had 945 tests, all
     green:

     | # | Defect | Red |
     |---|---|---|
     | MM1 | the add in `read_counted` removed | 9 tests |
     | MM2 | route drain calls the uncounted `read` | GC `the_last_of_many_writes_...`, HC `many_send_edits_before_one_block_...`, WEB `many_send_edits_before_one_render_...` |
     | MM3 | fader drain uncounted | BC `a_drain_applies_the_newest_value_...`, HC `both_hosts_drain_two_fader_and_two_pan_edits_alike`, WEB `the_status_reports_the_sessions_superseded_count` |
     | MM4 | matrix drain uncounted | BC `a_matrix_drain_counts_the_replaced_writes_...`, HC `both_hosts_drain_...` |
     | MM5 | input drain uncounted | BC `many_input_edits_before_one_block_...`, WEB `many_input_edits_before_one_render_...` |
     | MM6 | count added twice | the same 9 tests as MM1 |

     Each of the four readers has its own catch.
   - **No rendered bit and no #1346, #1312 or #1399 count moved.** All of these pass:
     - the #1312, #1346 and #1399 count tests (`the_status_reports_...`, `both_hosts_drain_...`,
       `many_input_edits_...`);
     - the builtins fixture manifest digest (50 files);
     - `check-graph-determinism.sh`, `check-console-fixtures.sh` and the worklet's native parity;
     - the G5 native digests.

     The shipped module's sha256 changed, as the spec says (codegen only):
     `2502cd4bec95d27380aa851df07138ed1e89d0ae5f95bf5f29aa10f4e8ccb32e`, the value the attempt
     record gives.

3. **The new realtime regions are correct.**
   - crates/graph/src/lib.rs:959-991 holds `RouteControlRecord::new` and its accessors.
     :1012-1031 holds `from_words`. Both are reachable from `LiveRoute::drain`, and `to_words`
     (control only) stays outside them.
   - The gate reports 104 regions in 27 files, and the floors are 104 and 27.
   - In the self-test, pad 1 holds 22 regions and the other thirteen pads hold 3 each:
     22 + 13 x 3 = 61. With the 43 mutation regions that is 104, so the fixture sits exactly on
     both floors. `test-realtime-policy.sh` passes.
   - Probes on a third export:
     - `Vec::new()` in `from_words` gives "marked realtime forbidden-body predicate".
     - Removing `from_words`'s markers gives "expected at least one hundred four marked realtime
       regions".
     - An `assert!` in `new` passes. `assert!` is not on the gate's forbidden list. This is the
       gate's existing scope, not this slice's.

4. **The .d.ts text is correct in both copies.**
   - The two files are byte-identical at 84eb9d485, and `check-sdk-types.sh` passes.
   - The header paragraph now lists the fader, mute, pan and matrix cells, the input trim,
     polarity and filter cells, and the send gain, matrix and mute cells, for issues 1312, 1346 and
     1347. It says only effect records keep queues.
   - `Backpressure` now names only an effect queue.
   - I checked this against admission. The only `RESULT_BACKPRESSURE` room refusals are the EQ
     prepared-owner `command_wanted` overflow (lib.rs:5701-5711) and the effect room pass
     (:5779-5785). Both are effect lanes. See n2 for the one reason-8 path the text leaves out.

5. **51d6ba7a9.** See its own section below.

## Findings

### BLOCKER
None.

### MAJOR
None.

### MINOR
None.

### NIT

- **n1. Two sentences claim more than the code does.**
  - Where:
    - latest_cell.rs:723-724: "no other crate or control path can write the count".
    - The spec's Attempt 2 m2 paragraph: "enforced by the type system".
  - Why this is not quite true: `CellReader::new` and `read_counted` are `pub`. Code on any thread
    could build a reader and call `read_counted` with the session's count after a stamped write
    to its own cell.
  - Why it is still acceptable: the rule now rests on "only a cell's reader writes the count". It
    is no longer a free public `add`, and abuse would need a deliberately constructed
    cell-and-stamp sequence.
  - Fix: say "only a cell read writes the count".
- **n2. The `Backpressure` doc misses one path.**
  - Where: sdk/src/browser/shipped-host.d.ts:343 and its twin.
  - The path: reason 8 is also returned with `RESULT_INTERNAL` when the revision gate refuses a
    stamp (hosts/host-web/src/lib.rs:4717-4722).
  - The old text did not cover this path either, and it is practically unreachable.
  - The VCA kinds (16 and 17) are named in neither list. They write member cells, so they never
    get backpressure.
  - Root may want one clause on each in the batch's typings pass.
- **n3. Carried for root, unchanged:**
  - n4 of attempt 1: the STREAMS hot-file rows for builtins-compiler `lib.rs`, host-core
    `prepare.rs` and the `.d.ts` pair do not name #1347.
  - n5 of attempt 1: spec D2 still reads `write(record)`.
- **n4. Carried, older than this slice.** M13 (dropping `ready.vcas.commit()` on success) is
  still green on every test I ran. The attempt-1 verdict showed the same at e61b85c51^. It is not
  #1347's defect. Root should file it as a bounded follow-up so it does not stay only in verdict
  notes. The spec does not record it.

## Test value

Attempt 2 adds no test and rewrites no test. It deletes `a_vca_batch_with_a_late_refusal_is_refused_whole`.
That deletion is correct: by M5 and M7 above, the test had no catch of its own. The attempt-1 test-value sentences for the
surviving #1347 tests still hold. These mutations reproduce them on this tree:
- MM2 is the route-count catch of graph-compiler `the_last_of_many_writes_...`, host-core
  `many_send_edits_...` and host-web `many_send_edits_...`.
- M5 is the catch of `a_refused_send_batch_writes_nothing_and_keeps_the_mirror`.

## Gates run (export of 51d6ba7a9; all exit 0)

- **lint job, every step:**
  - `cargo fetch --locked`, fmt, clippy (`--workspace --all-targets --all-features -D warnings`)
    and `cargo doc -D warnings`.
  - Workspace policy and its mutations, then session policy and its mutations.
  - Env vocabulary, then bench policy and its mutations.
  - **test-support CI policy and its mutations.**
  - The scalar-oracle self-test.
  - Script reachability and its mutations, on the git-initialised copy.
  - Console benchmark fixture, then **the builtins fixture manifest mutation tests**.
  - Bench preconditions.
  - Host-core policy and its mutations, then protocol policy and its mutations.
  - Realtime policy (104 regions in 27 files) and its mutations, the audit-leak check and the
    artifact-evidence-leak check, each with its self-test.
  - The trace validator, then lane policy and its mutations, then the unfused seal and its
    self-test.
  - **check-rack-policy**, builtins policy and graph policy, each with its mutations.
  - Effect-runtime policy and fixtures, each with its mutations.
  - Conformance boundaries, then the PEQ render seal.
  - The release-shape self-test, then the npm publish modes.
  - The stem-store gate, then stem identity.
  - Both sub-v3 refusal probes, then the AVX2+FMA probe.
- **test-release job:**
  - lane, math and wasm-gates (G5 included).
  - filter_liveness, tail_contract, ramp_endpoint (8 crates) and `designer_total --ignored`.
  - The M3 FMA cfg check.
  - **The loom leg, `spsc_loom`: 13 passed.**
  - Not run: M1 and F1 exhaustive. No math changed, so the math closure leaves them out.
- **audit-native subset:**
  - The release build of audit, bench, capi and session-validator.
  - `cargo test --release -p audit -p bench -p console-workload`.
  - **`audit capi`**: allocations, deallocations, locks and syscalls 0; `total_violations` 0.
  - `trace-builtins-audit.sh`, `trace-builtins-graph-audit.sh` and `trace-graph-audit.sh`.
  - The builtins, builtins-graph and realtime probe mutations.
  - **`check-capi-abi.sh` and its self-test**, then the libcapi scalar-oracle check.
  - `check-graph-determinism.sh`.
  - **`check-builtins-fixtures.sh . target/release/audit`: the manifest digest, 50 files.**
  - `check-console-fixtures.sh`.
- **`check-cross-targets.sh`:** PASS.
- **Worklet chain:**
  - `build-web-audioworklet.sh --named-twin`. Shipped module `2502cd4bec95...`; named twin
    `b5043e8591...`.
  - `strip-wasm-names.py` self-test and check.
  - `check-web-audioworklet.sh --without-metadata-regeneration`, then
    `check-browser-expected-resources.py --artifacts`.
  - `check-scalar-oracle-absent.py --wasm`.
  - `test-web-audioworklet.sh`, with a private TMPDIR and no leftovers.
  - The V8 spill gate and its self-test.
- **sdk job (source-bundle mode):**
  - `npm ci`.
  - `check-sdk-generated.sh`, `check-sdk-deletions.py` and `check-sdk-types.sh`.
  - **`check-sdk-headless.sh`: 362 tests, 362 pass, 0 fail.**
  - `sdk-package.sh check`.
  - Afterwards I removed `sdk/dist`. It was not present when I checked.
- **Spec gates:**
  - The mutation baseline ran `cargo test -p engine -p builtins-compiler -p graph -p
    graph-compiler -p host-core -p host-web -p control-plane` with the CI test-support features
    plus `host-core/control-provider`: 945 passed, 0 failed.
- **Not run:** the Playwright browser legs (no browsers on this machine).

## 51d6ba7a9 -- delete the SDK paired-filter queue eval (#1346 follow-up): PASS

- **The deletion is right.** The eval booted with `commandQueueRecords: 2`. It expected a
  `both`-channel `inputFilters` record to be refused `backpressure` with `admitted == 0`, and then
  admitted after one queue drain. #1346 made the input filter band a set of cells, so both of
  those claims are now false. `check-sdk-headless.sh` was red on it and is now green, 362 of 362.
- **What the eval caught is now caught elsewhere.**
  - Its writer-side claim (a refused both-lane record is never acknowledged as one half, stays
    pending and does not escalate) is still defended for effect queues by
    sdk/test/writer-evals.mjs:262, "one both-lane record costs two queue slots: 32 fit, 33 do
    not", and :306, the oversized both-lane gesture with `commandQueueRecords: 2`.
  - The eval never looked at cells or bits, so it never defended SDK lowering.
- **Is "the pair stays whole" still defended in Rust? Yes.** Measured:
  - **MP1.** Host-web writes only the first prepared-filter target of a pair. This turns red host-web
    `prepared_builtin_input_filter_owner_is_atomic_across_mixed_batches`, and only that test.
  - **M5.** Cells written before the room pass turns red that test and five others. A pair cannot
    be written by a refused batch.
  - **MP2.** Render never reads the right channel's filter cells (half of the pair applied). This
    turns six tests red:
    - builtins-compiler `input_cells_drain_in_the_canonical_order_whatever_the_write_order`;
    - builtins-compiler `many_input_edits_before_one_block_...`;
    - builtins-compiler `the_per_node_live_control_input_processor_drains_and_folds_its_witness`;
    - host-web `many_input_edits_before_one_render_...`;
    - the two `a_collapsed_chain_renders_the_forced_dual_bits_*` tests.
  - **NIT.** The #1346 spec note names builtins-compiler
    `the_input_witness_folds_from_the_final_cell_contents` as a holder. MP2 left that test green.
    It defends the `Both` fold, not whole application. The tests that actually catch a half-applied
    pair are the MP2 list. The note can name them when root syncs the body. The verdict does not
    depend on this.
- No other SDK eval asserts an input or send queue. The remaining `commandQueueRecords` evals
  run on EQ parameter records, which are an effect lane.

## Evidence kept
/tmp/claude-1002/v1347b/logs/: lint-summary.txt, rest-summary.txt, mutations.txt, one log per
mutation, and the per-gate logs.
