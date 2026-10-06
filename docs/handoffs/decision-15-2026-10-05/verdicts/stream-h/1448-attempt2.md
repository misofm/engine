PASS

# #1448 attempt 2: adversarial verdict

Reviewed: `c037a87fe` (code) and `2e3681cb1` (evidence record), parent `a24d17768`, branch
`codex/d15-stream-h`. Whole #1448 change: `git diff b55968d0d 2e3681cb1 -- hosts/host-web/src/lib.rs
hosts/host-web/src/tests.rs .github/ISSUE_SPECS/1448-*`. Spec:
`.github/ISSUE_SPECS/1448-bound-the-browser-meter-poll-by-each-queue-s-count-at-entry.md` at
`2e3681cb1`. Exports under `/tmp/claude-1002/v1448b/` (deleted after the verdict). Evidence:
`/home/bl/misofm/submix-verdicts/evidence/1448-attempt2-verify/`.

MAJOR-1 is fixed. The pass loop now has D2's shape, and every mention of the roots in the window
fits the #1440 and #1441 conditions. MINOR-1 and the NITs are fixed, and the corrected record is
true. I re-ran every gate. Each one passes and agrees with the record. I found two NITs and no other
issue.

## BLOCKER

None.

## MAJOR

None.

## MINOR

None.

## NIT

- **NIT-1. `hosts/host-web/src/lib.rs:1637`.** The `meter_remaining` doc says "One count per track
  meter". The meters include the submix strips (the new test has 5 meters: 3 tracks and 2 submixes).
  This is the same error that attempt 2 corrected on `METER_QUEUE_DEPTH` (`:1169`, "track or
  submix"). The `meter_pending` doc above it (`:1634`, "per track") has the same wording, but it is
  older than #1448.
- **NIT-2. `hosts/host-web/src/lib.rs:3446`.** "This function indexes with `get`/`get_mut`, never
  `[]`" is not true word for word. `ready.meter_header.reserved[0]` (`:3582`) and `reserved[0]` and
  `reserved[1]` (`:3774-3775`) use `[]` with constant indexes into a `[u64; 2]`. The compiler checks
  these at compile time, so they add no runtime check. The conclusion of the comment is correct, and
  I measured it: the head twin has one `panic_bounds_check` call in `poll_meters`, with `Location`
  `crates/engine/src/realtime/spsc.rs:449:37` (`slots[self.local]` in the inlined `try_pop`). The
  base has the same call. Better words: "no runtime-checked `[]` index".

## MAJOR-1: the D8b conditions, item by item (head `lib.rs:3467-3594`)

- **Condition 1.** `<Q>` = `ready.meters` and `<K>` = `ready.meter_remaining`, both `<fields>`. The
  root is the `ready` binding of `let Some(ready) = self.ready.as_mut() else`. Nothing rebinds or
  assigns `ready` between the set loop and the pass loop. `<path>` = `.consumer`.
- **Condition 2 (`:3467-3469`).**
  `for (meter, remaining) in ready.meters.iter().zip(&mut ready.meter_remaining) { *remaining = meter.consumer.available_at_entry().min(METER_QUEUE_DEPTH); }`.
  The body is one statement, and `<cap>` is an identifier. It is the last set-loop statement before
  the pass loop, so the pairing is correct.
- **Condition 3.** The set loop and `for _ in 0..passes` are statements of the same function-body
  block, and the pass loop is inside `for _ in 0..passes`. No loop encloses the set loop.
- **Condition 4 (`:3479-3484`).** `ready.meters.iter_mut().zip(&mut ready.meter_pending).zip(&mut ready.meter_remaining)`.
  It has two `.zip(&mut <fields>)` steps, and the last one is `.zip(&mut <K>)`. There is no other
  adapter. Term for term, it is the set loop's iterator, with `iter` read as `iter_mut`. The pattern
  `((meter, pending), remaining)` nests as the iterator does: `<r'>` = `meter` and `<c'>` =
  `remaining`. Attempt 1's `.zip(<fields>.iter_mut())` is gone.
- **Condition 5.** The first statement is `if pending.is_some() || *remaining == 0 { continue; }`.
  The second is `*remaining -= 1;`. The comment between them is not a statement. The third,
  `if let Ok(snapshot) = meter.consumer.try_pop() { .. }`, holds the only pop, with receiver
  `meter.consumer`. The pass body names `remaining` only in the guard and the decrement, and it
  names `meter` only in the receiver.
- **Condition 6 (window `:3470-3594`).** Each mention of `ready` is on the closed list:
  - item 4: `for remaining in ready.meter_remaining.iter()`, with `*remaining` only as the
    argument of `saturating_add`;
  - item 2: `ready.meters.iter_mut()` and `&mut ready.meter_remaining` in the pass header;
  - item 1: `&mut ready.meter_pending` in the header, and every `ready.meter_pending`,
    `ready.meter_loss_count`, `ready.meter_snapshot_generation`, `ready.meter_generation` and
    `ready.meter_header` path in the checks;
  - item 3: `ready.meters.get(index)` in the `.all(..)` closure;
  - item 5: `ready.reset_meter_delivery(false);` stands directly in the reset branch's block. The
    rest of that block is `ready.meter_loss_count = 1;`, `self.meter_activation_sample = ..;` and
    `return 0;`, with no loop, pop, closure or macro.

  The window has no macro call. `self.meter_activation_sample` is not a root.
- **#1441 B1b-D1.** The pass loop is `<fields>.iter_mut()` followed by two
  `.zip(&mut fields)` steps, which is in the list. `for _ in 0..passes` is `0..<bound>` with an
  identifier bound. **#1418 and #1426 (Amendment 1)** do not judge the form's own loops or a
  per-queue pop again. They depend on B1a-D8b, which holds.
- **#1443 "#1448 guard", G-D1.** Line 3430 (before the doc comment) and line 3780 (after the
  closing brace) are blank at head. A marker can replace each one, and no line moves.

**Codegen of the fix.** I built the `a24d17768` twin, and a twin of `a24d17768` that has only the
three loop-spelling edits. Both give shipped module `cbec2e672ae1f05ac0a4f02540e65f9cbb0863b99c7cea7eaad284932c59be50`
and named twin `cdd780bd…82f8`. On the `a24d17768` twin against the head twin,
`mtr-closure-offset-cmp.py` reports no differing function (24/24, 80/80, 25/25). The record and the
commit message are correct.

## MINOR-1 and the NITs of attempt 1

- The digests are corrected. All seven of my gate-3 digests are the same as the attempt-2 table.
- The render-export sentence is corrected. `miso_engine_web_v1_render` (`ffi.rs:3949-3958`) reads
  only `host.status()` and calls `reject_output_quantum` or `render_next`.
- The cause of the closure-count drift is given, and the full script output is recorded. My run is
  byte-identical to the implementer's `closure-cmp.txt`.
- The `METER_QUEUE_DEPTH` doc now says "strip" (see NIT-1 for the field doc). The record says two
  `continue`s and marks attempt 1's "three" as superseded. It names `wasm-gates` G5/G6 and says why
  they were not run. I confirmed that neither `wasm-gates` nor `wasm-gate-guest` depends on
  `host-web` (`cargo tree`: 0 matches). The bounds-check comment is corrected (see NIT-2).
- Attempt 1's record is kept as written, and each corrected claim has a "superseded" mark.

## Whole-change review

- **Spec fit.** D1 to D4 and D6 are implemented as written. The resource charge goes into
  `meter_delivery_bytes` (and so into `bridge_metadata_bytes` and `bridge_retained_bytes`) and into
  the largest-allocation row. Its arithmetic is checked, and it uses the `web.resource.arithmetic`
  diagnostic. There is no marker and no floor change (ruling (C)). The changes stay inside the
  authorized paths.
- **Realtime.** `poll_meters` gets no allocation, lock, syscall or panic path.
  `meter_remaining` is allocated in `compile_ready`. `available_at_entry` is one Acquire load. The
  decrement cannot underflow because the guard comes first. All browser `render-allocations` rows
  are 0.
- **The pass bound.** A pass that continues clears a slot. The next pass refills it (and spends a
  count) or ends at the empty-slot check. So a pass cut by `1 + Σ remaining` would pop nothing and
  would end the loop with the same state. Gate 3 confirms this: the parent and the change are
  identical poll for poll.
- **Digest rule.** No committed test pins a digest or a byte count. The gate-3 digests are PR
  evidence. No test greps source.

## Acked-batch question (meter queues): can an ack come before a drop?

No. Nothing in the change acknowledges a window. The render producer drops the newest window when a
queue is full, and it counts each drop. `poll_meters` counts each candidate that it drops in
`meter_loss_count` before it publishes, and the next published header reports it. The bound only
stops a queue from popping past its count at entry. A window that arrives during the call stays in
the queue for the next poll. If the decrement comes before a pop that fails (this cannot occur with
one consumer), the poll uses one count but loses no data.

## Test value

- `meter_gap_on_one_meter_leaves_no_backlog_at_one_block_windows` (`tests.rs:5766`): it is red when
  the poll's passes are bounded by the minimum over the meters (mutant (min)). That design keeps a
  permanent one-window lag after a loss on one meter at one-block windows, and no other test catches
  it. On mutant (min), the full suite has 1 red test: this one, at `tests.rs:5799`, block 11,
  left 0, right 1. PASS on test value.
- `bus_meter_host_with` (`tests.rs:5704`) is a helper. `bus_meter_host` calls it with `(2, 64)`, and
  no existing caller changed.

## Gates run (exports; `CARGO_TARGET_DIR=/tmp/claude-1002/v1448b/target`)

1. `cargo test --locked -p host-web --features test-support --no-fail-fast` at head: lib 188 passed,
   0 failed, 2 ignored. Integration targets: 2 + 1 + 1 passed.
2. Mutants, which I wrote again from the spec, with the full suite:
   - (a), the two `continue`s become `return 0`: 3 red tests. These are the new test (`:5799`,
     block 11), `meter_lease_reacquisition_waits_for_a_clean_boundary` (`:5364`) and
     `meter_reacquisition_rejects_a_full_stale_queue_then_recovers` (`:5400`).
   - (min), mode 1: 1 red test, the new test only (`:5799`, block 11).
   - Parent `lib.rs` (`b55968d0d`) with the head `tests.rs`: 188 passed.
3. D7 driver `vconfirm.rs` (sha256 `46d3d801…aeeaa45c`, the same file as attempt 1): parent and
   change are identical (`diff -r`) on all 7 runs. Polls, loss counts and digests are the same as the
   attempt-2 table. (min) and (a) each differ from the change on all 7 runs.
4. Worklet chain (`build-web-audioworklet.sh --named-twin`):
   - Head shipped module `1dc7b08d…bd21`, named twin `d4a9691e…271e`. Base twin (head tree with
     `b55968d0d`'s `lib.rs`) `57bb4c3d…96a1`. All three are the same as the record, and the
     artifact changed, as the spec expects.
   - These passed: `strip-wasm-names.py --self-test` and `check`;
     `check-web-audioworklet.sh --without-metadata-regeneration` (`meter_poll`: closure 9, traps 2,
     the only trap owner is `poll_meters`); `check-browser-expected-resources.py --artifacts`;
     `check-scalar-oracle-absent.py --wasm`; the V8 spill gate (self-test and module, Node 22.23.2);
     and `test-web-audioworklet.sh` with a private TMPDIR (rc 0, nothing left behind).
   - Browser legs (`npm run qualify -- --artifacts … --sdk-root … --browser <b> --check-matrix
     --self-test-mutations`, `npm ci` in `sdk` and in `qualification`, private pulseaudio null sink,
     no `sdk/dist`): chromium 151.0.7922.34, firefox 153.0 and webkit 26.5 all pass. Each
     `render-allocations` row is 0.
   - Closure comparison, base against head: identical to the record. render 24/24 and
     `command_submit` 80/80 are offset-only +8. In `meter_poll` 25/25, `poll_meters` has a BODY
     change, and two functions are offset-only +8.
5. `cargo fmt --all -- --check`, `cargo clippy --locked -p host-web --all-targets --features
   test-support -- -D warnings`, `cargo clippy --locked --workspace --all-targets --all-features --
   -D warnings`, `bash scripts/check-realtime-policy.sh` ("89 marked regions in 25 files") and
   `bash scripts/check-workspace-policy.sh`: all exit 0.

Not run: AArch64 (CI only) and `wasm-gates` G5/G6 (they do not depend on `host-web`; the batch CI
runs them).
