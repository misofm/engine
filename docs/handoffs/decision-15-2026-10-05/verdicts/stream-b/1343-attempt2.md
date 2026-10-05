PASS

# #1343 attempt 2: verdict (adversarial verifier, opus-xhigh)

Reviewed:
- attempt 2: `git diff 084754c4b fb961e801`;
- the whole slice: `git diff 880e4b86d fb961e801` on `crates/engine`, `tools/audit`, `scripts/trace-*`,
  `crates/control-plane`, `crates/host-core`, `crates/graph` and `crates/source`.

The commit was exported with `git archive` to `/tmp/claude-1002/v1343b/tree`. Builds ran with
`CARGO_TARGET_DIR` under `/tmp/claude-1002/v1343b/target`. I did not touch the worktree.
`646d6247d` (root's Amendment 2, spec text only) is not part of this review. Its new D3 wording
agrees with the code: the mailbox returns `MailboxInvariantBroken`, and `commit` and `republish`
treat it as unreachable.

Attempt 2 changes only tests. It adds three tests, all in `cfg(test)` or `cfg(all(test, loom))`
modules, and updates the attempt record. No production code changed.
- M-1 is fixed. The new loom model reaches cell reuse, and it goes red on MA and on MB.
- m-2 is fixed.
- m-1's test is restored as the attempt-1 verdict said. But its catch is not unique: two existing
  capi tests catch the same mutant. That makes the attempt-1 finding a false positive (MINOR m-1
  below).

There is no BLOCKER and no MAJOR.

## Attempt-1 findings

- **M-1: fixed.** `spsc_loom_plan_mailbox_reuses_the_cell_render_emptied` (`spsc.rs:996`)
  - Control publishes three candidates and render adopts two. Control's third publication goes
    into cell 1, the cell that render moved candidate 1 out of. The write is legal only after the
    claim of candidate 2 makes cell 1 `Empty`, so the model reaches the reuse path.
  - MA and MB turn only this model red (loom "Causality violation: Concurrent write accesses to
    `UnsafeCell`").
  - MC turns all three mailbox models red.
  - Reverted, the code passes 4/4 in about 4 s.
- **m-1: restored as directed, but see MINOR m-1.** The mutant "the hand-over runs only on the
  first swap" turns `carry::hand_over_runs_only_on_the_applied_block` (`mod.rs:985`) red.
- **m-2: fixed.** `stale_claim_after_withdraw_and_republish_fails_on_the_generation`
  (`spsc.rs:1260`)
  - It is red with the generation removed (engine 43/44).
  - Loom stays 4/4 green under the same mutant, so this test is the only catch.
- **n-1: resolved** by root's Amendment 2.

## MINOR

### m-1. The restored carry test has no catch of its own in the workspace

The attempt-1 finding came from an incomplete search: it ran engine, host-core and graph, but not
capi.

- **Where:**
  - the test, `crates/engine/src/realtime/mod.rs:981-1024`;
  - the attempt-2 record's test-value line for it ("no other test catches it").
- **CARRY1** (`enter_block`'s `carry_from` gated on `self.carried == 0 && self.carry_mismatched
  == 0`):
  - engine: only this test goes red (43/44);
  - host-core `successor_swap`: 32 green;
  - graph `rt11_swap_carry_alloc`: 1 green;
  - `cargo test -p capi --lib`: **2 red**. Both run in gate 5 and in CI:
    - `runtime::tests::control_calls_inside_a_plan_swapping_render_call_keep_replacement_live`
      (`crates/capi/src/runtime/tests.rs:1505`). It fails at `:1589` on
      `owner().carried_count() == round`, over four carrying swaps on one owner.
    - `runtime::live_tests::a_prepared_bypass_change_rebuilds_and_renders_the_committed_model`
      (`crates/capi/src/runtime/live_tests.rs:2722`). It fails at `:2790` on the rendered output.
- **DOUBLE** (my probe: the hand-over runs twice on every swap after the first). The test goes red,
  and so does capi's `tests.rs:1589`: a real hand-over moves state, so the second call changes the
  outcome.
- **Other defects:** I found no other plausible defect that only this test catches. Its withdrawal
  half gives a carry defect nothing new to reach, because a withdrawal never touches the active
  plan.
- **What this means for attempt 1:** its deletion was correct in effect, although its stated
  reason was wrong.
- **Why MINOR, not MAJOR:**
  - the product code is correct;
  - the test is correct and green;
  - the defect is one redundant test, restored on the verdict's instruction.
  - The batch is not pushed, so if the follow-ups delete it, the deletion still lands "in the
    same PR" (AGENTS.md).
- **Fix (follow-ups commit):**
  1. Delete `hand_over_runs_only_on_the_applied_block`.
  2. Correct the attempt-2 record: the hand-over-per-swap pin is capi's two tests.

  If root wants an engine-local pin anyway, that is root's decision, and the record must say that
  the catch is shared.

## NIT

- **n-1. The stale-claim test's reach.**
  - It goes red only when no transition on the withdraw-and-republish path advances the
    generation (G: `advanced` returns `self`).
  - When a single transition forgets `.advanced()`, everything stays green:
    - withdrawal only (GW): engine 44/44;
    - publication only (GP): engine 44/44.
  - That is correct. Every cycle that brings the cell states back to the same values contains a
    publication and also a withdrawal or a claim. So one advancing transition per cycle already
    stops the ABA.
  - So the attempt-1 verdict's claim that this test "also catches a single transition that forgets
    `.advanced()`" is wrong. Attempt 2's record line ("a transition that does not advance the
    generation") is slightly too broad. This is a wording fix only.
- **n-2. The loom gate builds with `--release`, so the mailbox's `debug_assert`s do not run in
  the loom models.** Two are affected:
  - withdraw's `Err` branch expects `Active` (`spsc.rs:692`);
  - commit expects an empty cell (`:724`).

  The models assert the outcomes instead ("`Taken` means render adopted `[1]`"), so the property
  is covered. No action.
- **n-3.** The restored test discards the reclaimed epoch (`let _ = retirer.try_reclaim()`). The
  old test asserted `PlanEpoch(0)` there. This is moot if m-1 deletes the test.

## A fresh look at the unsafe mailbox (`spsc.rs:462-806`)

No finding. The checks:

- **Payload safety needs exactly two happens-before edges. Each now has a test that fails without
  it.**
  1. The publication CAS `Release` (`:727-732`) pairs with render's `Acquire` `observe` (`:766`)
     and claim (`:794`). So the payload write happens before render's move out. MC turns all
     three mailbox models red; so did attempt 1's M1 and M3.
  2. The claim CAS `Release` (`:794`) pairs with `try_reserve`'s `Acquire` (`:646`). So render's
     move out of a cell happens before control's next write into it. MA and MB turn only the new
     model red.
- **Orderings that are stronger than needed (not defects):**
  - Withdraw's `Acquire` load and its `AcqRel` CAS. Control takes back a payload that it wrote
    itself, and render never touches a `Full` cell. Probe MD (withdraw CAS `Relaxed`/`Relaxed`)
    stays green 4/4 under loom, as expected.
  - The claim's `Acquire`. It repeats `observe`'s `Acquire`: the CAS succeeds only on the exact
    word that `observe` loaded, and the generation makes that word unique.
- **Withdraw's `Err` branch is sound.** Render changes the word only by claiming the `Full` cell,
  and the writer cannot publish during `withdraw` (`&mut self`). So the cell is `Active`
  (`Taken`), never `Empty`. `published` is overwritten only by a commit, and a commit needs no
  cell to be `Full`, so the last publication was claimed.
- **The generation.**
  - It adds 16 per transition, so it never carries into the four state bits.
  - It has 60 bits before it wraps.
  - Every cycle contains an advancing transition (n-1).
- **`MailboxObservation`.**
  - It is `Copy` and not bound to one mailbox.
  - It is `pub(crate)`, and only the same reader's `observe` makes one.
  - If an observation is used again after its claim, the generation makes the claim fail.
- **`Send` and `Sync`.**
  - `unsafe impl<T: Send> Sync for Mailbox<T>` is sound: values move between threads and are
    never shared.
  - Both endpoints are `Send` and `!Sync`. A `MailboxPermit` holds `&mut MailboxWriter`, so the
    word cannot change between reservation and commit (I5).
- **The render path.**
  - Each block does one load, at most one CAS and one bounded move. It sits inside a
    `REALTIME_POLICY` region.
  - The retirement slot is reserved before the claim, and dropping an unused `PushPermit` does
    nothing.
  - On a lost claim, render drops and frees nothing.
- **Teardown.** A candidate that is still `Full` drops with the last mailbox endpoint, as the old
  queue did. That happens at teardown, not on the render path.
- **Acked-batch question: can an ack ever precede a drop? No.** Attempt 2 does not change this,
  and attempt 1's analysis stands:
  - a won CAS either adopts the candidate in the same block or returns it whole;
  - a refused publication returns the plan;
  - the ack follows the infallible `commit`.

## Production-change check

`git diff 084754c4b fb961e801` touches only these places:
- the `loom_tests` module (`cfg(all(test, loom))`) and the `tests` module (`cfg(test)`) in
  `spsc.rs`;
- `tests::carry` in `mod.rs` (`cfg(test)`);
- the spec.

The parent `084754c4b` (#1309 follow-ups) touches only `crates/capi/src/runtime/error.rs` and the
comment in `scripts/check-host-core-policy.sh`, which are outside this slice. So the non-test
engine code is the same as in attempt 1. The browser-compiled module does not change either: the
attempt-1 verifier's build gave sha256 `87c03c64...28cf`. I did not run the worklet chain again,
for that reason.

## Test-value sentences

- **`spsc_loom_plan_mailbox_reuses_the_cell_render_emptied`:** a claim CAS without `Release`
  (MA), or a `try_reserve` load without `Acquire` (MB). Either one lets control's write into a
  cell race render's move out of it. Only this model goes red. The other loom models and every
  native test stay green, and on x86 no native test can see the race.
- **`stale_claim_after_withdraw_and_republish_fails_on_the_generation`:** a mailbox word whose
  transitions do not advance the generation. Then a claim built from a load taken before a
  withdraw and republish succeeds. Only this test goes red: engine 43/44, loom 4/4 green.
- **`carry::hand_over_runs_only_on_the_applied_block`:** no catch of its own in the workspace.
  The hand-over-only-on-the-first-swap mutant also turns two capi tests red (m-1). The follow-ups
  delete it.

## Mutations run again (export of fb961e801; each reverted to green)

| Mutant | Change | Result |
| --- | --- | --- |
| MA | claim CAS `AcqRel` -> `Acquire` | loom: only the reuse model red |
| MB | `try_reserve` load `Acquire` -> `Relaxed` | loom: only the reuse model red |
| MC | publication CAS -> `Relaxed`/`Relaxed` | loom: all 3 mailbox models red |
| MD (probe) | withdraw CAS -> `Relaxed`/`Relaxed` | loom 4/4 green (not a defect, see above) |
| G | `advanced` returns `self` | engine: only the stale-claim test red; loom 4/4 green |
| GW (probe) | withdraw does not advance | engine 44/44 green (n-1) |
| GP (probe) | publication does not advance | engine 44/44 green (n-1) |
| CARRY1 | hand-over only on the first swap | engine: only the restored test red; host-core 32 and graph 1 green; **capi 2 red** (m-1) |
| DOUBLE (probe) | double hand-over on later swaps | engine: the restored test red; **capi `tests.rs:1589` red** |

The logs are in `/tmp/claude-1002/v1343b/logs/`.

## Gates run (export of fb961e801)

1. **Engine tests.** `cargo test --locked -p engine --features realtime-audit`: 44 + 4 + 1 pass.
2. **Loom.** `RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p
   engine --lib spsc_loom`: 4 pass.
3. **Realtime.**
   - `cargo build --locked --release -p audit -p capi`, then `audit capi`: allocations,
     deallocations, locks, syscalls and `total_violations` all 0.
   - `trace-realtime-audit.sh` ok: 2 accepted, 999997 refused, 999997 prior-plan renders while
     refused, 1 withdrawal, 1 republished adoption; 0 allocations, deallocations, locks and
     syscalls.
   - `trace-graph-audit.sh` PASS: 2/1/1/1/1.
   - `trace-builtins-graph-audit.sh` PASS: 2 applied, 999996/999996, 1/1.
   - `check-realtime-policy.sh` ok (90 regions in 25 files); `test-realtime-policy.sh` ok.
4. **Workspace.**
   - Tests: `-p capi` 73 + 2 + 11; `-p host-core --features control-provider,test-support --test
     successor_swap` 32; `-p graph --features test-support --test rt11_swap_carry_alloc` 1;
     `-p source` 37 + 1 + 1; `-p control-plane --features test-support` ok; `-p audit
     --release` 33.
   - `check-capi-abi.sh` ok (shared and static).
   - `cargo fmt --all -- --check` ok.
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` ok.
   - `check-workspace-policy.sh` ok.
   - `check-cross-targets.sh` PASS.

## For the follow-ups commit

1. Delete `hand_over_runs_only_on_the_applied_block` (m-1), unless root rules to keep an
   engine-local pin with a shared-catch record.
2. Correct the attempt-2 record's test-value lines (m-1, n-1).
