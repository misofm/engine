FAIL

# #1448 attempt 1: adversarial verdict

Reviewed: `98d4a1534` (code) and `33cb616f3` (evidence record), parent `b55968d0d`, branch
`codex/d15-stream-h`. Range `git diff b55968d0d 33cb616f3`. Spec:
`.github/ISSUE_SPECS/1448-bound-the-browser-meter-poll-by-each-queue-s-count-at-entry.md` at
`33cb616f3`. Exports: `/tmp/claude-1002/v1448/{tree,base,mut}` (deleted after the verdict).
Evidence: `/home/bl/misofm/submix-verdicts/evidence/1448-attempt1/`.

The behaviour is right. Every gate I re-ran passes, and gates 2 and 3 reproduce. The attempt fails on
one MAJOR: the pass loop is not written in the shape that D2 fixes and that the drain rule accepts.
The fix is two tokens.

## BLOCKER

None.

## MAJOR

**MAJOR-1. The pass loop's `.zip` steps do not have D2's shape. When marked, the drain rule as
specified refuses them.** `hosts/host-web/src/lib.rs:3481-3482` (head):

```rust
                .zip(ready.meter_pending.iter_mut())
                .zip(ready.meter_remaining.iter_mut())
```

D2 (spec lines 108-114) writes `.zip(&mut ready.meter_pending)` and `.zip(&mut ready.meter_remaining)`.
It also says "The names are illustrative; the shape is not" (line 139), and it calls this form "the
form the tool accepts (B1a-D8b)". The root ruling (C) asks the verifier to check that the code would
pass the drain rule when marked. By the letter of the rule, it would not:

- B1a-D8b condition 4 (`.github/ISSUE_SPECS/1440-…md:168-169`): the pass loop has "`.zip(&mut <fields>)`
  steps, one of which is `.zip(&mut <K>)`". Head has `.zip(<K>.iter_mut())`. The `iter_mut` = `&mut`
  equivalence is stated only for the set loop (condition 2, `:156`).
- B1a-D8b condition 6, item 2 (`:197-198`): in the window, `<K>` may appear only as "`&mut <K>` as the
  argument of condition 4's `.zip(&mut <K>)` step". The refusal list (`:216-217`) names
  "`<K>` … in any other position (… `.iter_mut()`, …)" explicitly. `ready.meter_remaining.iter_mut()`
  at `:3482` is in the window, so it is a refused mention.
- B1b-D1 (`.github/ISSUE_SPECS/1441-…md:52-53, 59-63`): the pass loop of a D8b form must be a finite
  form with `.zip(<literal | fields | &fields | &mut fields>)` steps. `.zip(<fields>.iter_mut())` is
  not in that list. This applies to both `.zip` steps.
- The accepted test case `per_queue_meter_poll` (`1440-…md:269-271`) and #1418's description of this
  pass loop (`1418-…md:670`) both use `.zip(&mut …)`.

Effect: the guard commit "H #1448's guard, landed by J after C2" (#1443, section "#1448 guard",
G-D3 at `:285`) must stop when the tool refuses, and it may not change the code. A strict B1a port
refuses this pass loop, so the defect appears only after #1448 has closed, in another stream's
batch. Making the meter poll markable is the reason this issue exists (Problem: "The shape also
cannot be marked"). Under the owner's no-shortcuts principle, the shape must match now.

Fix (verified on a scratch export, `specform-fix.diff` in the evidence folder): use
`.zip(&mut ready.meter_pending).zip(&mut ready.meter_remaining)` in the pass loop. For symmetry with D2, also use
`.zip(&mut ready.meter_remaining)` in the set loop (`:3466`). The set loop's current spelling is
allowed by condition 2. With the fix, `cargo clippy --locked -p host-web --all-targets --features
test-support -- -D warnings` and `cargo fmt --all -- --check` are clean, and `--lib` gives 188
passed, 0 failed. The iterators are the same types, so behaviour does not change.

Note: today's awk gate refuses the marked `poll_meters` in both spellings
(`marked realtime unbounded try_pop drain` at `:3490`), because D8b is not implemented yet. So this
finding comes from the specified rule, not from a run of the tool. Both blank lines that G-D1 will
replace with markers are present at head.

## MINOR

**MINOR-1. The evidence record has errors** (spec file, Attempt record):

- `:411-413`: the digests of the three injection runs are wrong after the ellipsis. The prefixes,
  poll counts, window counts and loss counts are correct. The real digests (same at parent and
  change) are:
  - meter blocks 1: `925c6370f84b5bc1cc1a4c001443675b5af2213ea9a2368f4f268e135af1bbc3`
    (the record has `…71820`, which is the end of the meter-blocks-12 digest);
  - meter blocks 2: `560fdeac49d00623364777cb97d302654f8bcee9bf18378e362b5e219a689e64`
    (the record has `…e689e64`);
  - meter blocks 12: `37454e939901bea7ea27af23b6c2e3f1ca063d9b2cd7760761898c8f25d71820`
    (the record has `…f1ca063`, which is from the middle of the digest).
- `:445-447`: "which the spec's expected list did not name" is false. Gate 4 (`:315`) puts
  `miso_engine_web_v1_render` first in the render list. Because it is on the list, the record
  does not have to name a field. Also, "(it reads a `ReadyOwnership` field offset)" is not shown,
  and it is probably wrong. The export reads only `host.status()`, then calls `reject_output_quantum`
  or `render_next`. Its moved offsets (1608 and 1624, then 1616 and 1632, `render-export-deltas.txt`) are host
  fields placed after the inline `Option<ReadyOwnership>`. They move because that field grew by 8.
- `:447-449`: the closure counts (24 and 80, where the spec has 25 and 81) are given without a cause.
  The cause is spec-anchor drift, not a gap. On the fifth review's `6d28a80ec` twins, the same script
  counts 25/81/26. Between those twins and `b55968d0d`, `__rdl_dealloc` leaves each closure, and the
  dlmalloc functions are now monomorphised in `host_web` (#1333's allocator work), so each closure
  has one less function on both sides (`closure-drift-6d28a80ec-twin-vs-b55968d0d.txt`). The
  head/base sets agree.
- `:444`: gate 4 asks for the script's full output. The record shortens the names and leaves out the
  BODY diff lines. The full output is in `closure-cmp.txt` in the evidence folder.

## NIT

- `hosts/host-web/src/lib.rs:1169`: "Depth of every track meter queue". The same depth builds the
  submix meter queues too (the new test has 5 meters: 3 tracks and 2 submixes). Say "strip".
- Spec `:388`: "the three `continue`s become `return 0`". The gap/discontinuity block and the
  invalid-span block have two `continue` statements. My two-site mutant gives the same red tests.
- The record does not list `wasm-gates` G5/G6 (gate 4, "No PCM moved"). `tools/wasm-gates` and
  `tools/wasm-gate-guest` do not depend on `host-web`, so this change cannot move them. CI runs them
  on the batch.
- Outside this issue: at base and at head, the call-graph gate reports `traps=2` with
  `panic_bounds_check` owned by `poll_meters`. The function's comment says that no bounds check is
  present. The new code adds no trap (the counts are equal at base and head).

## Spec-anchor questions from the brief: my decisions

- `miso_engine_web_v1_render` offset-only +8: it is on the spec's list (`:315`), so this is not a
  deviation. At `6d28a80ec`, the fifth review's twins also show it as offset-only +8. It needs no
  field name. The record's sentence is wrong (MINOR-1).
- 24/80 vs 25/81: spec-anchor drift from `__rdl_dealloc` leaving the closures. It is in the base,
  not in this change. Not a gap.
- Boot/teardown functions missing from the `meter_poll` closure: these functions are not in the
  `meter_poll` call graph, so the script cannot list them. The spec's sentence describes the whole
  module. A whole-module comparison (`module-cmp.txt`) shows BODY changes only in `poll_meters`,
  `compile_ready`, `ffi::boot_staged`, `project_buffers`, the `ReadyOwnership` drop glue, and the
  renamed collect/`into_boxed_slice` instances (a `Vec<u32>` instance became `Vec<usize>`; on wasm32
  these are identical and were merged). These are exactly the spec's list. Every other changed
  function is offset-only. All deltas are +8, except `boot_with_spectrum_config` [8, 16], which is
  a boot function outside the three closures. Not a gap.

## Acked-batch question (meter queues): can an ack come before a drop?

No. The render thread produces with drop-newest and counts each drop in `cumulative_dropped_snapshots`.
`poll_meters` counts each dropped candidate in `meter_loss_count` before it publishes, and the next
published header reports it. The new bound only stops a queue from popping past its count at entry.
A window that arrives during the call stays queued for the next poll. Nothing is dropped and
nothing is published early. A decrement before a pop that fails (impossible for a single consumer)
uses a count and loses no data. The observation `acknowledge` path is unchanged.

## Test value

- `meter_gap_on_one_meter_leaves_no_backlog_at_one_block_windows`: it goes red when the poll's passes
  are bounded by the minimum, over the meters, of count plus held slot (mutant (min)). That design
  keeps a permanent one-window lag at one-block windows. I ran the full `host-web` suite (lib and the
  4 integration targets) on mutant (min). This test is the only failure, at `tests.rs:5799`
  ("block 11: one window published", left 0, right 1). It is also red on next-poll recovery
  (mutant (a)), but `meter_lease_reacquisition_waits_for_a_clean_boundary` (`:5364`) and
  `meter_reacquisition_rejects_a_full_stale_queue_then_recovers` (`:5400`) catch that defect too.
  It is green on the parent's `lib.rs`. It is not a regression reproducer, so that is correct. PASS
  on test value.
- `bus_meter_host_with`: a test helper. `bus_meter_host` delegates to it with (2, 64), and no
  existing caller changed.

## Gates run (on exports, `CARGO_TARGET_DIR` under `/tmp/claude-1002/v1448/target`)

1. `cargo test --locked -p host-web --features test-support` at head: lib 188 passed, 2 ignored
   (release-only budgets); integration targets 2 + 1 + 1 passed. All 17 meter tests are green and
   unchanged.
2. Mutations (full suite, `--no-fail-fast`):
   - (a) the two `continue`s become `return 0`: 3 red (the new test at `:5799` block 11, `:5364`, `:5400`).
   - (min) mode 1: 1 red, the new test only (`:5799`, block 11).
   - Parent `lib.rs` with head `tests.rs`: 188 passed.
3. D7 driver (the implementer's `vconfirm.rs`, sha256 `46d3d801…aeeaa45c`; I checked it against D7
   and against `mtr-review4-probes.rs`): parent and change are byte-identical on all 7 runs. The
   polls, windows and loss counts match the record. Discrimination: (min) and (a) both differ from
   the change on all 7 runs. Gain-reduction words are non-zero in all 3 injection runs.
4. Worklet chain, head build `build-web-audioworklet.sh --named-twin`: the shipped module
   `d101134022c27641b7a4ec66cea211fa6971fd565bfa02317b4996be7398e4e5` is the same as the record's.
   The base module is `1c8782cd…` (ARTIFACT CHANGED, as expected). Passed:
   - `strip-wasm-names.py --self-test` and `check`;
   - `check-web-audioworklet.sh --without-metadata-regeneration` (the `meter_poll` call graph is
     the same as base: closure 9, traps 2);
   - `check-browser-expected-resources.py --artifacts` (every row is in budget, and no pin moved);
   - `check-scalar-oracle-absent.py --wasm`;
   - the V8 spill gate (self-test and module, Node 22.23.2);
   - `test-web-audioworklet.sh` with a private TMPDIR (exit 0, nothing left behind);
   - chromium leg `npm run qualify -- --artifacts … --sdk-root … --browser chromium --check-matrix
     --self-test-mutations` (node_modules installed with `npm ci` as CI does, Chromium
     151.0.7922.34). Every `render-allocations` row is 0, and all qualification gates passed.

   Closure comparison (`mtr-closure-offset-cmp.py`, base and head twins): the same output as the
   record. render 24/24 and command_submit 80/80 are offset-only +8. In `meter_poll` 25/25,
   `poll_meters` is BODY and two functions are offset-only +8. No function is on one side only.
   Whole module: see above. I did not run firefox or webkit (the implementer did), AArch64 (CI only)
   or wasm-gates G5/G6 (they do not depend on `host-web`).
5. `cargo fmt --all -- --check`, `cargo clippy --locked -p host-web --all-targets --features
   test-support -- -D warnings`, `cargo clippy --locked --workspace --all-targets --all-features --
   -D warnings`, `bash scripts/check-realtime-policy.sh` ("89 marked regions in 25 files") and
   `bash scripts/check-workspace-policy.sh`: all exit 0.

## What attempt 2 needs

- MAJOR-1: change the pass loop to `.zip(&mut ready.meter_pending).zip(&mut ready.meter_remaining)`
  (and use `&mut ready.meter_remaining` in the set loop too). Re-run gates 1-5 on the new commit,
  so that the evidence is for the code that lands. The iterators are the same types, so gate 3 is
  expected to give the same digests. The module digest can change, so record the new one.
- MINOR-1: correct the record: the three digests, the render-export sentence, the cause of the
  closure-count drift, and the full script output.
