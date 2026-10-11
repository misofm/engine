PASS

# #1432 follow-ups verdict: commit 203ec4194 (parent 194fe80d8), branch codex/d15-stream-b2

Verifier: adversarial, read-only on the worktree. The commit was exported to /tmp/claude-1002/v1432fu/tree and built there.
Evidence: /tmp/claude-1002/v1432fu/ev/ (mutations.txt and mut/*.log, probes.log, loom.log, loom_debug_asserts.log, test.log, crp/trp/fmt/clippy/doc/wsp/cross logs).

The commit closes attempt 1's m1 and m2 and both NITs. The new definition of `A` is correct, and it is stronger than the old one. Each claimed mutation row reproduces. I found no BLOCKER and no MAJOR.

## BLOCKER

None.

## MAJOR

None.

## MINOR

- **m1. Spec D1 now has two definitions of `A`.**
  - `.github/ISSUE_SPECS/1432-...md:100-101` (Write, step 1) still says "`A` = the greatest mirrored revision `< stamp.revision` (the newest published value)".
  - The amended `stamp` bullet (D1, `GatePin`) and the code (`latest_cell.rs:406`) say "the newest value at or below `published`".
  - Under the rule the two are the same. But #1312, #1277 and #1502 read D1 as the contract.
  - Fix: change step 1 to "at or below `stamp.published`", one line.
- **m2. A stamp kept across its own publication passes every check, and the in-place rule then rewrites a published slot.**
  - Location: `latest_cell.rs:91` (`LiveStamp` is `Copy`), `:398-402` (the in-place lookup) and `:241` (all checks run at stamp time only).
  - Probe `probe_stale_stamp_targets_the_slot_render_reads` (ev/probes.log):
    1. `stamp(3)`, write X, `publish(3)`.
    2. Render snapshots S = 3.
    3. A second write with the same stamp targets slot 0, which is the slot render reads at S = 3. No debug assert fires.
  - Sequential result: S = 3 applies a second value of revision 3. If the two run at the same time, render reads torn words.
  - This is the precondition that review m2 named ("a stamp at or below a published revision"). The current guard checks it only when the stamp is made.
  - It is not new: the in-place lookup is unchanged since attempt 1. It needs a caller defect, and no caller exists yet.
  - Fix at no runtime cost: make the stamp borrow the pin, `LiveStamp<'p>` with a `PhantomData<&'p mut GatePin>`. Then `publish(&mut self)` cannot run while a stamp is alive. All current users (L1-L4, R2, the fixture) already drop the stamp before `publish`.
  - Do it in #1502 or #1312, or carry it as a frozen rule in their briefs. A rule that is only documented is not enough (owner principle).
- **m3. #1312 does not carry the new rule yet, and the rule adds a constraint.**
  - The follow-ups say "#1312's brief must carry the rule" (spec `:412`), but `1312-*.md` D7 is unchanged.
  - The new debug assert makes this pattern panic: stamp `r`, a later check fails, and the next transaction stamps `r+1`. Nothing was written, so the pattern is harmless.
  - #1312 D7 must state one of two things: control takes the stamp only after every fallible check, or a refused transaction stamps no higher revision before it publishes. Root owns this sync.

## NIT

- n1. **The gate-identity check (`latest_cell.rs:219-223`) cannot detect a wrong gate that holds the same revision.**
  - #1502 D1 builds every gate at 0, so a first publication to the wrong fresh gate passes.
  - Probe `probe_identity_check_misses_same_revision_gates`: green, which shows the gap.
  - The spec text "catches a pin used with another gate" overstates this. #1502's handle (gate and pin in one value) and D5's pointer-identity check give the real protection.
  - Suggest "catches a pin used with another gate whose revision differs".
- n2. **No test was deleted.** The test list goes from 17 to 22: 5 new, 2 rewritten, none removed.
  - The commit and spec do not claim a deletion, so the brief's "one unit test deleted as redundant" is wrong.
  - No existing test is now superseded. `a_stamp_at_the_last_published_revision_panics` and `a_second_revision_is_not_stamped_...` defend different asserts.

## Judgments on the five questions

1. **The new `A` is correct, and the loom proofs and the slot-reuse argument still hold.**
   - Design Lemma 2 ("Protected") needs `N` to be `A` when `S_R` is the newest published revision `rev(w_j)`.
   - The stamp is taken between `w_j` and the write, so `stamp.published = rev(w_j)`. The new `A` is then "newest at or below `rev(w_j)`", which is `N` by definition. The old formula needed the extra premise "every write below the in-flight revision is published" to get there.
   - When control obeys the rule, the two formulas select the same slot. All five loom models obey the rule: I re-ran them with `-C debug-assertions=on`, and 13 passed with every new assert live in every interleaving. So each loom proof carries over unchanged.
   - When control breaks the rule in a release build:
     - The gate still holds `published`, so `S` is in {`published`, pin} (Lemma 1). The selected slot is `A` or `B`, so render never reads a slot that is being written.
     - The in-place target is above `published`, so render never reads it.
     - The only remaining effect is that an abandoned value leaks after a later publication. That value is visible and stable, and the leak is the caller's contract.
   - Mutation `published - 1` makes all five loom models and two unit tests red. So the loom models defend the new field directly, not only through the unit test.
   - Effect on #1502, #1503, #1504 and #1312: no guarantee changes. C1-C6, "never overwrites the newest published value nor the value render's open snapshot needs" (#1312 D11) and #1277's `ALL`-equals-adoption-S argument do not depend on how `A` is computed. The only stale stamp case is m2, and it is not new.
2. **The edit to spec D1 is within the follow-up's authority.**
   - Both shapes, `pair` (one constructor) and "carry `last_published` in `LiveStamp`, `A` at or below it", are fixes that the attempt-1 verdict named.
   - The change is local to #1432's own API. No other spec or code uses `RevisionGate::new`, `GatePin::new` or `stamp(&self)` (I checked with grep). Only the handoff prototype does, and that is historical.
   - It guarantees more than before, and it is marked inline as a follow-up change. No root ruling is needed. Root should still see m1 and m3.
3. **Test value and mutations:** each claimed row reproduces (table below). Each new test goes red on its own defect and no other test does. The only overlap is `stamp does not record open`, which correctly makes both open-revision tests red.
4. **Deleted test:** nothing was deleted (n2).
5. **"A gate built at `r` cannot take a stamp of `r`": #1502 is compatible, and no root ruling is needed.**
   - `stamp` requires a revision above `last_published`, and the pin starts at `initial`.
   - This limit existed in attempt 1. The follow-up only documents it.
   - #1502 D1 builds every plan's gate at 0 at preparation:
     - A candidate's revision `r` is at least 1, so #1277 retargets stamped at `r` are accepted.
     - D5 publishes the initial revision through the handle (pin 0 to `r`, `open` is `None`).
     - Live commits stamp above the last revision published on the newest epoch's gate.
   - `pair()` fits D1's handle: `Arc::new(gate)` plus the pin, both in `PlanRevisionGate`.
   - Revision 0 is never stamped, because cells start unwritten. #1502 only has to keep "built at 0". If it builds a gate at the plan's revision, it breaks #1277.

## Mutation re-runs (unit = `latest_cell::tests`, debug; each loom test run separately, release)

| Mutation | Unit red | Loom red |
|---|---|---|
| `pair` starts the pin at 0 | a_pair_starts_the_pin_at_the_gates_revision | none |
| identity assert removed | a_pin_publishes_only_to_its_own_gate | none |
| `stamp` open assert removed | a_second_revision_is_not_stamped_before_the_first_is_published | none |
| `publish` open assert removed | a_publication_below_the_stamped_revision_panics | none |
| `stamp` does not record `open` | the two tests above | none |
| `publish` does not clear `open` (extra) | 7 tests | none |
| `A` back to "below the stamp's revision" | an_abandoned_revision_never_shields_the_slot_render_holds | none (as claimed) |
| `published` captured one low (extra) | a_read_applies_only_..., an_abandoned_... | L1 L2 L3 R2 L4 |
| P's resolved count over by one (model) | none | L4 (`:1299`) |
| protect only `A`, rerun | none | L2 R2 L4 |
| forget the pin on an unannounced swap, rerun | the_pin_follows_... | L2 L4 |

After the runs, the file was restored and checked with cmp against the committed content.

## Test value (one sentence per new or rewritten test)

- `a_pair_starts_the_pin_at_the_gates_revision`: red when `pair` builds a pin that does not match its gate's revision (attempt-1 m1 probe). No other test builds a nonzero pair whose pin it then checks.
- `a_pin_publishes_only_to_its_own_gate`: red when the debug identity check is lost, so that a pin used with another gate corrupts `pinned` without notice.
- `a_second_revision_is_not_stamped_before_the_first_is_published`: red when `stamp` accepts a second revision while one is open (attempt-1 m2, the abandoned revision).
- `a_publication_below_the_stamped_revision_panics`: red when `publish` accepts a revision below the open one, which would abandon the stamped writes.
- `an_abandoned_revision_never_shields_the_slot_render_holds`: red when `A` again follows an unpublished revision, so that a rule violation in a release build overwrites the slot render holds. No loom model reaches this.
- `a_write_below_a_written_revision_panics` (rewritten): still red only when `write`'s non-decreasing assert is removed. The new path (an old stamp kept across a later publication) is the only one that does not trip `stamp` first.
- `the_all_snapshot_applies_the_newest_write` (rewritten): same value as in attempt 1. The change only makes it obey the rule.
- L4's new P assertion: red when P's resolution count is wrong (model mutation red at `:1299`).

## Gates run (exported commit, `CARGO_TARGET_DIR` under /tmp/claude-1002/v1432fu/target)

- CI loom command (`--cfg loom`, release, `spsc_loom`): 13 passed, 24.7 s.
- The same with `-C debug-assertions=on` (extra): 13 passed.
- `cargo test --locked -p engine --features realtime-audit`: 75, 4 and 1 passed.
- `check-realtime-policy.sh`: ok (96 regions in 27 files).
- `test-realtime-policy.sh`: ok.
- `cargo fmt --all -- --check`: ok.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: ok.
- `RUSTDOCFLAGS=-Dwarnings cargo doc --locked --workspace --no-deps`: ok.
- `check-workspace-policy.sh`: ok.
- `check-cross-targets.sh`: PASS.
- The worklet chain was not re-run: the browser host does not use this module.
