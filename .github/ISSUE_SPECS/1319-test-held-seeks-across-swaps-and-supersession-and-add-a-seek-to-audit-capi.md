# Test held seeks across swaps and supersession, and add a seek to audit capi

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-12, D15-9, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

An acknowledged `seek_at` is never dropped by a plan swap or by a candidate that a newer structural
transaction supersedes: committed tests prove that a held seek moves with its source's consumer
across a swap, survives a supersession while it is still queued, and carries on from a candidate
render already took. The release C ABI audit applies one anchored seek inside its audited render
scope, so the held-seek render path is under the zero-allocation, zero-syscall gate.

D15-12's other tooling items (the `source.id.invalid` diagnostic, the #1293 D4 trap, the off-grid
lateness assertion, test-only untimed reads) are #1350; see Non-goals.

## Context

- The #1275 verifier's probe P5 passed on the candidate but was never committed (the decision-15
  adversary records it: `docs/handoffs/decision-15-2026-10-05/PLAN-2026-10-05-adversary-round1.md`,
  "MINOR — untested ack path"; the plan item is `PLAN-2026-10-05-agreed.md:191`). The probe:
  `seek_at(fixture-source, 2, 0, 1024)` before the transaction; the predecessor renders blocks 4-5 and holds the seek; the transaction
  swaps at block 6; the carried consumer applies the seek at block 8; bit-identical to the reference
  over 16 blocks. The #1275 ack argument relies on it: a seek the predecessor already popped
  survives only as `held_seek` inside the moved consumer.
- No committed test holds a seek across a swap. The host-core helper asserts
  `seek_before >= SWAP_BLOCK` (`crates/host-core/tests/successor_swap.rs:635`). The C ABI test
  `an_added_c_abi_source_starts_at_its_anchored_render_sample` (`crates/capi/src/runtime/tests.rs:1236`)
  seeks only after the commit. The carry moves the whole consumer with `mem::swap`
  (`crates/source/src/lib.rs:1880`), so `held_seek` (`:1018`) travels with it today; nothing pins
  that.
- A seek on the newest committed session goes to the pending candidate's ring
  (`SessionState::seek_at`, `crates/control-plane/src/control.rs:1671-1686`, via
  `newest_providers_mut`). A persisting source's producer moves to the successor at commit
  (`adopt_persisting`, `crates/host-core/src/source.rs:286`; called at `control.rs:1148-1155`).
  Today a second structural transaction while a candidate waits is refused
  (`control.rs:1095-1097`). *Supersede an unadopted candidate plan by compare-and-swap* (#1310)
  replaces it by withdraw-then-prepare (#1310 D1, on *Let the control thread withdraw an
  unadopted candidate plan*, #1343, and *Prepare a successor across a withdrawn candidate plan*,
  #1344): a withdrawn candidate A is succeeded by B prepared against the running plan P0, with A's
  own sources donated; a candidate render has taken is the base of B, as today. There is no
  re-targeted carry.
- #1310's gate 2 already covers a held seek on a source the withdrawn candidate *added* (the
  donated ring). Nothing covers a seek queued on a source that *persists* from P0 through A into
  B: its ring stays P0's, its producer moves twice (into A's set at T1, into B's at T2), and its
  command slot holds the acked seek through the withdrawal.
- `audit capi` (`tools/audit/src/capi.rs`) commits one structural transaction before call
  `CALLS_BEFORE_TRANSACTION = 1` (`:55`, `apply_structural_transaction` at `:181-213`) and never
  calls `seek_at`. CI runs it at `.github/workflows/qualification.yml:711-719` and validates the
  JSON at `:720-760` (`pcm_digest` is format-checked only).

## Decisions frozen for this slice

- **D1. P5 as a C ABI test** in `crates/capi/src/runtime/tests.rs`, named
  `a_held_anchored_seek_crosses_a_plan_swap`, built from the helpers of
  `an_added_source_starts_at_its_anchor_at` (`:1113-1233`). At 48 kHz, quantum 128, with a
  `source_ring_frames` of at least 8 quanta: submit generation 1 of the fixture source for blocks
  0-7; render blocks 0-3; `seek_at(fixture-source, 2, 0, 1024)`; submit generation 2 from frame 0;
  render blocks 4-5 (the seek is held; queued generation 1 plays); commit a structural transaction
  that keeps the source (add a track and its source); render the swap at block 6 and on to block 15.
  The reference is a fresh session of the committed document fed generation 1 for blocks 0-7 and
  generation 2 from frame 0 at block 8 on. Bit-identical over all 16 blocks (every block plays, as
  `assert_bit_identical` requires), and `test_plan_carry_counts` shows one carried swap.
- **D2. Withdrawn candidate, persisting source.** Same file,
  `a_queued_seek_on_a_persisting_source_survives_supersession`: the fixture source plays for blocks
  0-3. With no render in between: T1 (structural, keeps the source, changes another track's insert
  quality); `seek_at(fixture-source, 2, 0, A)` with `A` four blocks ahead (queued in the ring's
  command slot, not popped); T2 (structural, keeps the source, changes a third track's insert)
  supersedes T1's withdrawn candidate. Then submit generation 2 from frame 0 (through the
  producer that is now B's) and render to `A + 4` blocks. The source switches at `A`,
  bit-identical to a twin that submits T1∪T2 as one transaction at the same point and seeks the
  same way. #1310's test-support withdrawal counter shows one `Withdrawn` outcome.
- **D3. Taken candidate.** `a_held_seek_carries_from_a_taken_candidate`: T1 adds source `s2`;
  `seek_at(s2, 2, 0, A)` and submit generation 2 from frame 0; render one block, so render claims
  T1's candidate A and A's consumer pops and holds the seek. T2 (structural, keeps `s2`) commits.
  Its service step (*Add miso_engine_v1_service for bounded control work between edits*, #1348
  D2) has promoted A, so B is prepared against A as the base (#1310 D1, the `Taken` outcome's
  path) and carries A's consumer with its held seek. Render to `A + 4` blocks: `s2` enters at
  `A`, bit-identical to a fresh session of T2's document fed `s2` from frame 0 at `A`. #1310's
  withdrawal counters both stay 0. The racy `Taken` branch inside a withdrawal is #1310 gate 5's.
- **D4. Audit.** In `audit capi`, right after the structural transaction (control work, outside
  every render scope): `miso_engine_v1_source_seek_at(session, "fixture-source", 2, A, A)` with
  `A = 4 * QUANTUM_FRAMES`, then submit one generation-2 quantum from frame `A`. Assert both return
  `RESULT_OK`. Call 4's audited render applies the held seek. The existing assertions (`replacements
  == 1`, `(carried, carry_mismatches) == (1, 0)`, `snapshot.total() == 0`) stay. The JSON schema is
  unchanged; `pcm_digest` moves, which CI checks only for format.

## Deliverables

1. D1 now; D2 and D3 after #1310 has landed (they exercise its withdrawal paths).
2. D4.

## Authorized paths

- `crates/capi/src/runtime/tests.rs`
- `tools/audit/src/capi.rs`

## Non-goals

- The `source.id.invalid` diagnostic, the #1293 D4 trap (a typed held preparation), the off-grid
  lateness assertion and test-only untimed reads: *Tighten the seek entry points: source.id.invalid,
  a typed held preparation, timed reads only* (#1350).

## Objective gates

1. D1, D2, D3 pass: `cargo test --locked -p capi`.
2. **P5 is red on the defect it names.** Record in the PR: with the carry patched to reset the
   carried consumer's `held_seek` (or to rebuild the consumer), D1 fails at block 8.
3. Record in the PR: D2 is red with the withdrawal path patched to rebuild a persisting ring (or
   clear its command slot) for B; D3 is red with B prepared against A's carried base instead of A
   (so `s2`, which A added, is fresh in B).
4. **Audit.** `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   prints `"total_violations":0` and exits 0; `cargo test --locked --release -p audit`.
5. `bash scripts/check-workspace-policy.sh`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`.

## Test value

- D1: red if a swap resets, drops or fails to carry a held seek; no committed test seeks before a
  swap (`successor_swap.rs:635` forbids it).
- D2: red if supersession rebuilds a persisting source's ring or loses its queued seek while A is
  withdrawn; #1310's gates seek only on a donated source.
- D3: red if the control plane treats a candidate render took as withdrawn and prepares B across
  it, so B gets a fresh ring for `s2` and the held, acked seek is lost.
- D4: red (as a realtime violation) if applying a held seek on the render thread allocates, frees,
  locks or makes a syscall; no audited render applies one today.

## Dependencies

- *Supersede an unadopted candidate plan by compare-and-swap* (#1310), for D2 and D3 and its
  withdrawal-outcome counters.
- *Add miso_engine_v1_service for bounded control work between edits* (#1348), for D3's promotion
  before T2.

## Amendment (controls coordinator, batch 15 check, 2026-10-15)

Checked against `228fca26c`.

**Moved anchors.** Each fact in Context still holds; only the lines moved.

- The decision-15 handoff files are unchanged: the adversary's "MINOR — untested ack path" is
  `docs/handoffs/decision-15-2026-10-05/PLAN-2026-10-05-adversary-round1.md:460`, and the plan
  item is `PLAN-2026-10-05-agreed.md:191`.
- The host-core helper's assertion is `crates/host-core/tests/successor_swap.rs:638`
  (`anchored_swap_run`, `:631`).
- In `crates/capi/src/runtime/tests.rs`: `an_added_source_starts_at_its_anchor_at` is
  `:1333-1455`, `an_added_c_abi_source_starts_at_its_anchored_render_sample` `:1456`,
  `assert_bit_identical` `:1513`. `test_plan_carry_counts` is `crates/capi/src/ffi.rs:1256`.
- The carry's `mem::swap` is `crates/source/src/lib.rs:1931` (`adopt_sources`, `:1902-1938`).
  `held_seek` is still `:1018`.
- `SessionState::seek_at` is `crates/control-plane/src/control.rs:3028-3041`; it runs the
  #1348 service step first, then calls `newest_providers_mut` (`:2578`). `adopt_persisting` is
  still `crates/host-core/src/source.rs:286`; the control plane calls it at `control.rs:1846-1849`.
- A second structural transaction while a candidate waits is still refused, now at
  `control.rs:1742-1744` (`pending_providers` is a `Vec`, but this check keeps it at one).
- `tools/audit/src/capi.rs`: `CALLS_BEFORE_TRANSACTION` is `:52`, `apply_structural_transaction`
  `:178-211`, `submit_source_chunk` `:213-251`, the run loop `:480-560`. The audit also commits a
  live edit every 64 calls (`LIVE_EDIT_PERIOD`, `:39`, #1258). CI runs it at
  `.github/workflows/qualification.yml:777-785` and validates the JSON from `:786`.

**Code notes for the implementer.**

- D2 and D3: decision 17 and its slices make more values live (trim, polarity, input filters,
  sends, VCA). A value edit can take the live path and publish no candidate. So T1 and T2 must be
  structural edits (insert quality, or adding or removing an insert or a track). Assert edit-path
  3 (rebuild) in each response, so that a later live flip turns the test red, not vacuous.
- D4: `submit_source_chunk` hard-codes generation 1 (`:220`). Give it a generation argument. The
  structural transaction already submits the generation-1 block at frame 128 before the seek, so
  the order stays right. The fixture source has 48000 frames, so frame `A = 512` is in its region.
- D4 runs the seek and the submit outside every render scope, like the transaction. The
  seek call's service step is control work there too.

**Dependencies on GitHub.**

- #1348 is closed. D3's promotion runs in `SessionState::service` (`control.rs:1486`).
- #1343 is closed: the engine's withdrawal primitive exists
  (`PlanPublisher::withdraw`, `crates/engine/src/realtime/plan_exchange.rs:552`). The control
  plane does not use it yet.
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310) is open and is not in the
  batch-15/16 list. #1310 depends on *Prepare a successor across a withdrawn candidate plan*
  (#1344), also open and not in the list. D2, D3 and the withdrawal counters need both.

**Decisions.**

- D1 and D4 hold against the code and decision 17.
- D2 and D3 hold as designs, but they cannot run before #1310.
- **NEEDS ROOT RULING (R4, scope against #1310).** Deliverable 1 says D2 and D3 come "after #1310
  has landed", and #1310 is outside the batch-15/16 list. Proposed resolution: split D2 and D3
  (with gate 3's D2/D3 records) into a successor issue that depends on #1310. #1319 then closes on
  D1 and D4 in batch 15. The alternative is to keep #1319 open across batches, which the
  "smallest closable slice" rule in AGENTS.md argues against.

**Gates and commands.** Every named command, package and file exists. Gate 4's
`cargo test --locked --release -p audit` is also run by CI (`qualification.yml:775-776`).

**Realtime-path edits.** None under `crates/engine/src/realtime`. D4 adds a held seek to the
audited render, so a defect on that path shows up as an audit violation, not as an edit here.
