FAIL

# #1343 attempt 1: verdict (adversarial verifier, opus-xhigh)

Reviewed: `git diff 880e4b86d 3bb27fdc1` on `codex/d15-stream-b`. The commit was exported with
`git archive` to `/tmp/claude-1002/v1343/tree`, and builds ran with
`CARGO_TARGET_DIR=/tmp/claude-1002/v1343/target`. The worktree was not touched. I checked the work
against the spec (D1-D8, Amendment 1, the gates and the test-value lines), AGENTS.md, decision 15
(D15-9) and the owner principle.

The implementation is correct as written. The state machine, its memory orderings, the credits
and D8 are sound, and every gate is green. One MAJOR fails the attempt: gate 2's loom model does
not prove the mailbox's steady state. Two plausible ordering defects in the unsafe code create a
real data race, and every gate stays green under both of them.

## MAJOR

### M-1. The loom model never reaches cell reuse, so two ordering defects in the unsafe code pass every gate

- **Where:** `crates/engine/src/realtime/spsc.rs:794`, the claim CAS, `AcqRel`;
  `spsc.rs:645-646`, `try_reserve`'s `Acquire` load; the models at `spsc.rs:901` and `:934`.
- **The path that is not covered:** control publishes into a cell that render moved a payload out
  of at an earlier claim. Production does this on every second publication. Cell 0 starts
  `Active` with no payload. Neither model makes render claim twice and then has control write the
  cell that render emptied first. The furthest the models go is "claim A in cell 1, publish B
  into cell 0, claim B" (4 of 640 executions in the republish model; instrumented run).
- **Defects that pass every gate (I ran each one):**
  - MA: the claim CAS success ordering changed from `AcqRel` to `Acquire`. This drops the
    `Release` that the comment at `spsc.rs:789` gives as the reason the old `Active` cell can go
    back to the writer.
  - MB: `try_reserve`'s load changed from `Acquire` to `Relaxed`.

  Each one leaves render's `take()` and control's next `replace()` in the same `UnsafeCell`
  without a happens-before edge. That is a data race on a `PreparedRenderPlan`, which is undefined
  behaviour. Results: loom `spsc_loom` 3/3 green under MA and under MB. On x86 a CAS is a locked
  instruction whatever its ordering, so no native test can see the change. The native race
  becomes visible only on AArch64, the Android/iOS target.
- **Why this is MAJOR:** gate 2 says "render never reads a cell control is writing" under every
  interleaving, and the gate's test-value line says "Only loom's interleavings reach it". The
  model does not reach the path where that property matters most. AGENTS.md judges a generated
  test by what its generator reaches. The owner principle rules out an unproven unsafe hand-off
  in the core primitive.
- **Fix (verified):** add this model in `spsc.rs`'s `loom_tests`. Its waits use
  `loom::thread::yield_now`. A straight-line control thread does not work: loom's DPOR never
  schedules the claim between the publications, and a probe built that way reached B in 0 of 10
  executions.

  ```rust
  #[test]
  fn spsc_loom_plan_mailbox_reuses_the_cell_render_emptied() {
      loom::model(|| {
          let drops = Arc::new(AtomicUsize::new(0));
          let (mut writer, mut reader) = plan_mailbox::<Candidate>();
          let render = loom::thread::spawn(move || {
              let mut adopted = Vec::new();
              while adopted.len() < 2 {
                  if let Some(observed) = reader.observe() {
                      if let Some(value) = reader.claim(observed) {
                          adopted.push(value.id);
                      }
                  }
                  loom::thread::yield_now();
              }
              (adopted, reader)
          });
          for id in 1..=3 {
              loop {
                  if let Some(permit) = writer.try_reserve() {
                      assert!(permit.commit(candidate(id, &drops)).is_ok());
                      break;
                  }
                  loom::thread::yield_now();
              }
          }
          let (adopted, reader) = render.join().expect("render");
          assert_eq!(adopted, [1, 2]);
          drop((writer, reader));
          assert_eq!(drops.load(Ordering::Relaxed), 3);
      });
  }
  ```

  Results:
  - On 3bb27fdc1: green, and the whole `spsc_loom` filter still takes about 6 s.
  - Under MA and under MB: red, with loom's "Causality violation: Concurrent write accesses to
    `UnsafeCell`".
  - Under MC (publication CAS `Relaxed`): red, as are both existing models.

  The name above is a suggestion. The `spsc_loom_plan_mailbox_` prefix keeps the model inside the
  CI filter.

## MINOR

### m-1. The deleted carry test did catch a defect that no other test catches

The attempt record says `carry::hand_over_runs_only_on_the_applied_block` (it was at
`crates/engine/src/realtime/mod.rs`, in the carry module before `:968`) caught nothing unique. It
also says `successor_continues_the_predecessor_state_gap_free` "already pins one hand-over per
applied block". That is true only for an owner's first swap.

- **Mutation:** the hand-over runs only on an owner's first swap (`plan_exchange.rs:540`,
  `carry_from` behind `if self.carried == 0 && self.carry_mismatched == 0`).
- **Result:** engine 42+4+1, host-core `successor_swap` 32 and graph `rt11_swap_carry_alloc` 1 all
  stay green.
- **Why the record missed it:** the implementer's U1-U7 are mailbox defects, not carry defects.
  The deferral half of the old test is superseded. Its other half is not: two carrying swaps on
  one owner, each with exactly one hook call and a gap-free output against the reference.
- **Rewrite (verified):** keep that half and replace the deferral with a withdrawal:
  - block 1: publish B (applied, carried, 1 call);
  - block 3: reclaim, publish C, withdraw it before the render (`None`, 0 calls);
  - block 5: republish C (applied, carried, 1 call);
  - then assert `blocks == reference` and `carried_count() == 2`.

  This rewrite goes red under the mutation and green on 3bb27fdc1. Restore it in attempt 2.

### m-2. D2's generation counter can be tested now; no test fails when it is removed

The attempt record says "no test here can turn red on its removal". That is not correct.

- **Mutation:** `MailboxWord::advanced` returns `self` (`spsc.rs:521`).
- **Result:** loom 3/3 and engine 42/42 stay green.
- **Test that catches it:** a sequential mailbox test of D4's literal rule ("the CAS fails only
  when control withdrew the candidate after render's load"):
  1. publish;
  2. `observe()`;
  3. withdraw (`Withdrawn`);
  4. republish the same value;
  5. `claim(stale observation)` must be `None`;
  6. a fresh `observe` + `claim` returns the value.

  This test is red under the mutation and green on 3bb27fdc1. It also catches a single
  transition that forgets `.advanced()`. Without the generation the ABA is harmless today, but
  #1311 depends on it, and D2 is frozen in this slice. A 15-line test belongs with it now, not
  later.

## NIT

- **n-1. Open item 1, D3's typed error versus the panic at the exchange layer.** I judge this
  correct, not a defect. Root should align D3's wording.
  - D1-D4 are delivered in `spsc.rs`. There, the failed publication CAS is a typed
    `MailboxInvariantBroken<T>`: the value comes back whole, and the CAS is never retried.
  - At the exchange layer, `PlanReplacementReservation::commit` (`plan_exchange.rs:460`) and
    `republish` (`:412`) panic on it through `publish_into` (`:327`). The failure is unreachable
    by construction:
    - the permit holds `&mut MailboxWriter`, and the writer is unique and not `Clone`;
    - render writes the word only while a cell is `Full`, and no cell was `Full` at reservation;
    - a strong `compare_exchange` has no spurious failure;
    - a stale claim fails on the generation.
  - The control plane calls `reservation.commit()` after the protocol commit
    (`crates/control-plane/src/control.rs:1161`, the #1273 D2 design). If the error came back as
    `capi.plan.exchange`, the session would stay committed with no plan published. That split
    state is worse than an abort under `panic = "abort"`.
  - `control.rs:1431-1481` already uses `unreachable!` for proven-unreachable states after
    admission.
  - This matches D5's "republish, infallibly" and #1310 D4. A fallible signature would only make
    the error look recoverable, which it is not.
  - Recommendation: root amends D3 to say that the mailbox returns the typed error and the
    exchange treats it as unreachable.
- **n-2. Open item 2, republish's preconditions.** Sound. The preconditions are: the same
  exchange (`Arc::ptr_eq` on the credit counter), and `candidate.epoch + 1 == next_epoch`. #1310
  D4 refuses while A is withdrawn. B's reservation consumes no epoch until it commits, and the
  borrow checker forces B's reservation to drop before A is republished. So A is still the newest
  epoch, and the `Empty` cell exists. #1398 sizes retirement capacity to 3, so A's credit and B's
  reservation can coexist.
- **n-3. Literal counts in the builtins-graph audit.** `tools/audit/src/builtins_graph.rs:347` and
  the lines near it print `withdrawals`, `republished_adoptions`, `swaps_applied` and A/C's render
  counts as literals. Asserts back each of them, and this matches the file's existing style. The
  realtime and graph audits count. Optional: count here too.
- **n-4. Overlap among the new tests.** `second_publication_lands_in_the_cell_the_claim_left_empty`
  has no catch of its own among the new tests: U4 also turns 3 other new tests red. No
  pre-existing test catches U4, and the spec asks for this case by name, so it stays.
- **n-5. A spurious credit refusal (existing, not introduced here).** `RetirementCredit::try_take`
  (`plan_exchange.rs:80`) is one CAS. A credit that the retirer returns at the same time can cause
  a spurious control-side `RetirementFull`. The old `try_take_retirement_credit` behaves the same
  way. It is control-side only.
- **n-6. A minor error in the attempt record.** It says M2 turns only the republish model red. I
  saw both models red (the "one CAS per block" assert).

## Focus points

- **Memory ordering and soundness.**
  - The ownership rules hold:
    - render touches a cell's payload only after its CAS wins `Full -> Active`;
    - control writes only an `Empty` cell (`try_reserve`, then `commit`) and takes back only after
      its own `Full -> Empty` CAS wins;
    - nothing touches an `Active` cell.
  - The orderings are correct. The `Release` publication CAS pairs with render's `Acquire`
    `observe` and claim. The claim's `Release` pairs with `try_reserve`'s `Acquire`. The second
    pair is the edge that M-1 shows no test defends.
  - A candidate is adopted at most once, and is never both adopted and withdrawn: the two sides
    race on one word with a generation, and exactly one CAS wins.
  - Render makes at most one CAS per block (`claim`, with no retry, `spsc.rs:794`).
  - Render does no allocation, lock or syscall: the `audit capi` record, all three traces and
    `check-realtime-policy.sh` pass, and the mailbox reader sits inside a `REALTIME_POLICY`
    region.
  - `unsafe impl<T: Send> Sync for Mailbox<T>` is sound. `T: Send` is enough because values move
    between threads and are never shared, and only the `&mut` endpoint methods touch the cells.
- **Credits (D6).** Render moves a credit and never drops one.
  - On adoption the credit goes into the `RetiredPlan` and returns at `try_reclaim` after the pop.
  - Withdrawal moves the credit into the `UnadoptedCandidate`. Dropping it returns the credit,
    and so does `into_plan` (U3b confirms this).
  - Cancelling or dropping a reservation returns its credit through a partial move.
  - A claimed candidate always finds retirement room:
    1. The credit counter changes only by RMWs, so control's credit CAS synchronizes with every
       earlier return.
    2. Each return comes after its pop's cursor store.
    3. Render's `Acquire` `observe` follows control's `Release` publication.
    4. So render sees at most `capacity - 1` occupied slots.

    `enter_block` also reserves the slot before the claim CAS (`plan_exchange.rs:521-528`). If
    the room were ever missing, render would claim nothing and the candidate would stay
    withdrawable.
- **D8.** The counts are exact and the release build passes. I re-ran all three trace scripts on
  the release `audit` binary:
  - `realtime` (1,000,000 blocks): 2 accepted, 999997 refused, 999997 prior-plan renders while
    refused, 1 withdrawal, 1 republished adoption;
  - `graph`: 2/1/1/1/1, 2 destroyed off render;
  - `builtins-graph`: A 1, B 999998, C 1; 2 applied; 999996/999996; 1/1; retirement owner 2,
    control owner 1; 5 traced intervals.

  All have 0 allocations, deallocations, locks, syscalls and violations. Each round proves what
  D8 says:
  - the refused round refuses `RetirementFull` on the control side before every block, while
    render renders the active plan;
  - in the withdraw round, the render after the withdrawal applies nothing, and the next block
    after the republish adopts with epoch 2.

  Mutation A3 (republish publishes nothing; release build) aborts all three audits
  (`realtime.rs:147`, `graph.rs:169`, `builtins_graph.rs:385`).
- **Acked-batch question (D7): can an ack ever precede a drop? No.**
  - A candidate leaves the mailbox only through a won CAS. A claim adopts it in the same block,
    because the retirement slot is reserved before the claim. A withdrawal returns it whole.
  - A refused publication returns the plan.
  - The control plane acks after `reservation.commit()`, which cannot fail. If the invariant were
    broken, it would abort; it would never ack first.
  - At teardown, a candidate still in the mailbox is dropped with the exchange. The old queue did
    the same.

## Test-value sentences (each from my own mutation runs)

- `withdrawn_candidate_returns_whole_and_republishes_with_its_epoch`: a withdrawal that moves the
  payload out but leaves the cell `Full` (U7). Only this test goes red.
- `withdrawal_after_render_claimed_reports_taken`: a `Taken` withdrawal that keeps the publication
  record, so a second withdrawal reports `Taken` instead of `Nothing` (U9). Only this test goes
  red.
- `withdrawal_with_nothing_published_reports_nothing`: `into_plan` leaks the candidate's credit
  (U3b). Only this test goes red.
- `second_publication_lands_in_the_cell_the_claim_left_empty`: a publication into the `Active`
  cell (U4). No pre-existing test catches it; see n-4 for the overlap with new tests.
- `publication_without_a_credit_fails_full`: a refused reservation that forgets the last
  publication, so a withdrawal reports `Nothing` where render took the candidate (P3). Only this
  test goes red. The pre-existing reservation tests also catch U5 (a reservation without a
  credit).
- Rewritten `replacement_reservation_freezes_failure_precedence_and_serial_order`: a reservation
  admitted while the mailbox holds an unclaimed candidate (U6). This test and the existing
  concurrent stress test go red; the rewrite is forced by the one-candidate mailbox.
- `spsc_loom_plan_mailbox_claim_races_withdrawal`: a claim by plain store (M1), `Full` marked
  before the payload write (M3), or a `Relaxed` publication (MC). Each gives a loom causality
  violation. The generator reaches both outcomes of the claim/withdraw race.
- `spsc_loom_plan_mailbox_withdraw_republish_and_next_publication`: a claim that retries (M2,
  "one CAS per block"), plus M1, M3 and MC. Its reach is limited; see M-1.

## Gates run (export of 3bb27fdc1)

- Loom: `RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine
  --lib spsc_loom` gives 3 passed.
- `cargo build --locked --release -p audit -p capi`, then `audit capi`: allocations, deallocations,
  locks, syscalls and `total_violations` all 0.
- Trace scripts: `trace-realtime-audit.sh` ok, `trace-graph-audit.sh` PASS,
  `trace-builtins-graph-audit.sh` PASS, all with D8's counts (above).
- Policy scripts: `check-realtime-policy.sh` ok (90 regions in 25 files; the script is unchanged),
  `test-realtime-policy.sh` ok, `check-workspace-policy.sh` ok.
- `cargo fmt --all -- --check` ok.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` ok. This was
  re-run on the final commit, after the last test deletion.
- Tests:
  - `-p engine --features realtime-audit`: 42+4+1;
  - `-p capi`: 72+2+11;
  - `-p host-core --features control-provider,test-support --test successor_swap`: 32;
  - `-p graph --features test-support --test rt11_swap_carry_alloc`: 1;
  - `-p source`: 37+1+1;
  - `-p control-plane --features test-support`: ok;
  - `-p audit --release`: 33.

  All pass.
- `check-cross-targets.sh` PASS. `check-capi-abi.sh` ok (shared and static).
- Worklet chain (browser-compiled `engine` changed):
  - `build-web-audioworklet.sh --named-twin`: module sha256 `87c03c64...28cf`;
  - `check-web-audioworklet.sh --without-metadata-regeneration`;
  - `check-browser-expected-resources.py --artifacts`;
  - `check-scalar-oracle-absent.py`;
  - `test-web-audioworklet.sh`.

  All pass.
- Mutations re-done:
  - loom M1, M2, M3, MC: red;
  - loom MA, MB: green (M-1); red on the reuse model;
  - generation removal: green (m-2); red on the sequential test;
  - carry first-swap-only: green (m-1); red on the rewrite;
  - unit U2, U4, U5, U6, U7, U8, U9, U3b, P1, P3 (results above);
  - release audit A3: all three abort.

## For attempt 2

Add M-1's loom model (required). Restore a deferral-free version of the deleted carry test (m-1).
Add the generation test (m-2). Record each one red on its mutation and green on revert in the
Attempt record. No change to the production code is needed.
