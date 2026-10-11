PASS

# #1432 attempt 1 verdict: commit 194fe80d8 (parent c4ca6f2dd), branch codex/d15-stream-b2

Verifier: adversarial, read-only on the worktree. The commit was exported to /tmp/claude-1002/v1432/ and built there.
Evidence logs are in /tmp/claude-1002/v1432/ev/ (summary.txt, the per-mutation logs, probes.log, cross.log and wsp.log).

The implementation follows D1-D3 and Amendment 1. Each loom proof is the real module on loom atomics. The mutation table reproduces, with one benign discrepancy (see below). I found no BLOCKER and no MAJOR. Two MINOR guard gaps are confirmed by probe. Both can happen only through a caller defect, and #1502 must close or inherit them.

## BLOCKER

None.

## MAJOR

None.

## MINOR

- **m1. Nothing ties a `GatePin` to its gate, so the m2 guard can be disabled without notice.**
  - Location: `crates/engine/src/realtime/latest_cell.rs:170` (`GatePin::new(initial)`), `:195` (`publish(&mut self, gate, ..)`) and `:218` (`stamp`).
  - The spec says "one `GatePin` per gate ... at first the gate's initial revision", but the API leaves the match to the caller.
  - Probe `probe_mismatched_pin_disables_the_published_guard` (ev/probes.log): `RevisionGate::new(5)` with `GatePin::new(0)`.
    - `stamp(3)` passes the debug assert although 3 is at or below the gate's published revision.
    - Render at S = 5 applies the revision-3 value, then a second `stamp(3)` write rewrites that slot in place.
    - That is the torn-read case review m2 asked the guard to prevent.
  - A pin used with a different gate in `publish` also corrupts `pinned` without detection (review M2's class of failure).
  - Reachability: only a caller defect. #1502 owns the construction.
  - Fix, at no runtime cost: build the pin from the gate (`GatePin::new(&RevisionGate)`, a masked read at construction) or return both from one constructor. Add a debug identity check in `publish`. Do this in this issue's attempt 2, or make it a frozen decision in #1502's brief. Do not leave it as a documented-only rule; that is the "no interim shortcuts" principle.
- **m2. The rule "every stamped and written revision is published before a higher stamp" is unchecked, and the slot rule depends on it.**
  - Location: `latest_cell.rs:374-378`. `A` is "the newest value of an earlier, *hence published*, revision".
  - `stamp` (`:218-232`) asserts only that the revision is above `last_published`, so gaps pass.
  - Probe `probe_abandoned_stamp_overwrites_the_slot_render_holds` (ev/probes.log):
    1. Publish revisions 1 and 2. Render takes S = 2 and its pending set.
    2. Control writes revision 3 and never publishes it, then writes revision 4.
    3. The revision-4 write chooses revision 2's slot. Its `A` is the unpublished revision-3 slot, and the stale pin protects only revision 1.
    4. Render's read at S = 2 returns `Unchanged`, so revision 2 is skipped. Concurrently, this is a torn read.
  - Reachability: only a control path that writes cells and then abandons the commit. The validate-then-admit rule (#139/#140) forbids that, so this is not a live defect today.
  - Fix:
    - State the rule on `GatePin::stamp` and `CellWriter::write`.
    - Debug-assert it cheaply: the pin remembers an open stamped revision, and a new stamp must equal it until `publish`.
    - Or carry `last_published` in `LiveStamp` and define `A` as the newest value at or below it. The leak of an abandoned value is still the caller's contract.
  - #1312's brief must carry the rule.

## NIT

- n1. **`scripts/test-realtime-policy.sh:436-446, 1405-1415` is edited although the spec's path list leaves it out.**
  - This is a necessary consequence of the authorized floor raise, not scope creep. The fixture sits exactly on the floors, gate 2 requires the script green, and batch 1 (#1314) made the same edit.
  - The implementer disclosed it. The spec's authorized-path list missed it (a root brief gap).
- n2. **L4 (`:1192-1200`) does not assert P's resolution count.**
  - It checks only C's count (4). An assertion that P resolved 0 or 1 writes, consistent with D2, would make the model complete. The behaviour is correct.
- n3. **Nothing ties `CellReader::read`'s `bit` to the cell, or `pending`'s order to the snapshot (`:328`, `:483`).**
  - Both rules are documented, and loom shows the misorder red. #1312's stage binding owns them. Acceptable.
- n4. **One row of the mutation table is understated.**
  - The take-before-snapshot model mutation is also red in L4 (table: L1, L3, R2).
  - The "no in-place rule" row reproduces only with the forced variant below: dropping only the in-place lookup leaves every test green, because slot choice is deterministic. The claim itself holds.

## Judgments on the four questions

1. **Loom precision: confirmed.**
   - Gate word: written only by `fetch_or` (`:150`) and `swap` (`:205`).
   - Dirty word: written only by `fetch_or` (`:286`) and `swap` (`:303`), plus a `Relaxed` load.
   - L4 mailbox: written only by `compare_exchange` (`:1137`, `:1160`).
   - Slot words, revision and sequence: only plain `Relaxed` stores (`:424-427`), and no read-modify-write anywhere.
   - Initial values come from `new`. There is no `unsafe`. The models import loom atomics through the local `cfg(loom)` shim (`:48-51`) and run the production types.
   - Every red row below fails on a model assertion (torn words, "applied revision above S", a "C applied X at S" mismatch, the count), not by abort.
2. **`test-realtime-policy.sh`:** a necessary consequence (n1).
3. **API deviations:**
   - Deferral inside `read` is stricter than the prototype, which returned `newer` for the caller to act on. Good.
   - `LiveStamp`'s private fields close review M2's free-constructor concern. Good.
   - The order of `pending` after `snapshot` is documented and caught by the loom models. Acceptable.
   - The separate `GatePin::new(initial)` is m1.
   - #1502 can build on this soundly if it resolves m1 and carries m2.
   - Note for #1502 and #1277: a gate built at `initial = r` cannot take a stamp of `r`, so a candidate's retargets need a gate built below their publication revision. L4 shows the shape (gate 0, then write and publish 2).
4. **The aborting mutations (`unmasked`, `ceiling`):**
   - Re-run: in each, the model first panics on the real assertion ("S = 9223372036854775809", "S = 0", "the newest value at or below S applies"). Loom then aborts while unwinding, so the abort is teardown, not the cause of the red.
   - The unit tests catch both cleanly: `the_announcement_never_leaks_into_a_revision`, `a_read_applies_only_what_...` and `the_ceiling_is_...` plus five more.
   - Adequate.

## Mutation re-runs (each loom test run separately; unit = `latest_cell::tests`)

| Mutation | Loom red | Unit red |
|---|---|---|
| reader ignores S | L1 L2 L3 R2 L4 | a_read_applies_only_what_the_snapshot_covers... |
| snapshot is a load | L2 R2 L4 | the_pin_follows..., the_ceiling... |
| protect only A | L2 R2 L4 | none |
| forgets pin on an unannounced swap | L2 L4 | the_pin_follows... |
| dirty bit before the slot stores | L1 L2 L3 L4 | none |
| dirty take Relaxed | L1 L2 L3 R2 L4 | none |
| reader never defers | L1 L2 L3 R2 L4 | a_read_applies_only... |
| gate Relaxed | L1 L3 R2 L4 | none |
| take before snapshot (model) | L1 L3 R2 L4 | none |
| second write of a revision forced into another slot | R2 only | two_writes_in_one_revision_apply_the_second |
| no in-place lookup only (benign variant) | none | none |
| snapshot unmasked | L1 red; L3, R2, L4 abort after the assertion | the_announcement_never_leaks..., a_read_applies_only... |
| ceiling 2^63-2 | L2, R2, L4 abort after the assertion | six tests |
| superseded off by one | L1 L2 L3 R2 L4 | five tests |
| a nonzero take replaces the deferred mask | L1 only | none |
| tag stored first | none (as claimed) | none |

No test outside this new module exercises the new types, so no existing test already catches any of these.

## Test value (one sentence per new test)

- **L1** `spsc_loom_cells_one_block_never_tears`: red when a block applies one cell of a transaction and not the other (triple-buffer reader, take before snapshot, `Relaxed` gate, a take that replaces the deferred mask).
- **L2** `spsc_loom_cells_a_pinned_reader_survives_a_lapping_writer`: red when control overwrites the slot a pinned reader needs (snapshot as a load, protect only `A`, a forgotten pin).
- **L3** `spsc_loom_cells_a_dirty_bit_is_never_lost`: red when a value committed during a drain is lost between take and deferral (dirty bit before the stores, `Relaxed` take, no deferral).
- **R2** `spsc_loom_cells_an_in_place_rewrite_races_reads`: red when two writes of one revision occupy two slots; it is the only loom model that catches this.
- **L4** `spsc_loom_cells_a_claim_races_stamped_writes`: red when a just-claimed candidate's slot is overwritten or its value deferred wrongly across the claim (snapshot as a load, protect only `A`, a forgotten pin, no deferral).
- `peek_changes_nothing_and_a_read_counts_the_replaced_writes`: red when a peek consumes, or when the superseded count is wrong.
- `two_writes_in_one_revision_apply_the_second`: red when the in-place rule is lost (sequential catch).
- `a_lapped_reader_applies_the_last_commit_and_no_write_fails`: red when the count over a 9,999-write lap is wrong, or the ceiling is off.
- `a_read_applies_only_what_the_snapshot_covers_and_defers_the_rest`: red when a reader ignores S or does not defer.
- `the_all_snapshot_applies_the_newest_write`: red when the count is wrong under `LiveSnapshot::ALL`. It adds no unique catch beyond the count rows, but it is the only test of `ALL`.
- `the_pin_follows_the_announcement_and_survives_unannounced_publications`: red when the pin is dropped on an unannounced swap, or the snapshot is a load.
- `the_announcement_never_leaks_into_a_revision`: red when a snapshot does not mask `ANNOUNCED`.
- `the_ceiling_is_the_largest_revision_a_gate_holds`: red when the ceiling constant is wrong.
- `a_gate_above_the_ceiling_is_refused`: red when `RevisionGate::new` accepts a revision above the ceiling.
- `a_stamp_at_the_last_published_revision_panics`: red when `stamp`'s debug assert is removed.
- `a_publication_at_the_last_published_revision_panics`: red when `publish`'s debug assert is removed.
- `a_write_below_a_written_revision_panics`: red when `write`'s non-decreasing debug assert is removed.

## Gates run (on the exported commit, `CARGO_TARGET_DIR` under /tmp/claude-1002/v1432/target)

- Loom `spsc_loom` (release, `--cfg loom`): 13 passed (5 new), 25 s.
- `cargo test --locked -p engine --features realtime-audit`: 70, 4 and 1 passed.
- `cargo test --locked -p engine --lib latest_cell`: 12 passed.
- `bash scripts/check-realtime-policy.sh`: ok (96 regions in 27 files, which is the measured count).
- `bash scripts/test-realtime-policy.sh`: ok.
- `cargo fmt --all -- --check`: ok.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: ok.
- `RUSTDOCFLAGS=-Dwarnings cargo doc --locked --workspace --no-deps`: ok.
- `bash scripts/check-cross-targets.sh`: PASS.
  - The first run failed on root-filesystem inode exhaustion (environmental). It was rerun after root restored space.
- `bash scripts/check-workspace-policy.sh`: ok. It had the same environmental first failure and was rerun.
- The worklet chain was not re-run: the module is not referenced from the browser host.
