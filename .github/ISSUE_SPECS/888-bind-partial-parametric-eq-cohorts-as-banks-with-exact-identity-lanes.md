# Bind partial parametric EQ cohorts as banks with exact identity lanes

## Product outcome

The graph compiler banks only cohorts that are full and whose every member carries every slot; anything else renders scalar per node, so a five-track session on an eight-lane host banks no EQ at all. Let the EQ accept identity lanes so a partial cohort binds as one bank. Banking must not move a bit.

## Root evidence

- `crates/graph-compiler/src/banks.rs:214-222`: `if !group.is_full() { continue; }` and the `active_slots` all-lanes check; the comment cites the closed #96 F7 "no per-lane bypass mask".
- `crates/effect-runtime/src/bank.rs:261` `BankKernel` already declares `identity_coef()` (`:272`); `HomogeneousBank` (`:245-371`) has no production users.
- `crates/parametric-eq/src/lib.rs:2390` `bind_homogeneous_bank` is the EQ's bank entry; the rack already carries an `active_lanes` mask and `PreparedSlot::lane_active`.

## Smallest closable slice

Authorized paths: `crates/graph-compiler/src/banks.rs`, `crates/parametric-eq/src/lib.rs` (bank binding only), `crates/effect-runtime/src/bank.rs` (only if a shared identity-lane helper is needed), their tests, and this spec.

Allow a cohort with fewer than W members, or with members lacking the EQ slot, to bind as a bank whose missing lanes carry the EQ's exact identity coefficients and whose scatter skips them (relying on "Tiled gather and scatter for partial banks"). Scalar per-node rendering remains for cohorts the planner still rejects.

## Non-goals

No other effect (compressor is a separate issue). No kernel change. No change to the identity coefficient definition.

## Objective gates

1. New graph-compiler test: sessions with 1..W-1 EQ tracks bind one bank with identity lanes (assert on the plan).
2. New end-to-end test: for those sessions, rendered PCM is bit-identical to the same session rendered with banking disabled (scalar per node), on random input and on the EQ fixtures.
3. `scripts/check-graph-determinism.sh`, `scripts/check-parametric-eq-render-contract.sh`, `scripts/check-effect-runtime-policy.sh`, `tools/realtime-policy` pass; `cargo test -p graph-compiler -p parametric-eq -p graph` green.

## Dependencies

"Tiled gather and scatter for partial banks".

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`tools/realtime-policy` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.

## Amendment (2026-09-29, console strip, owner decision 12)

Decision 12 (`docs/rulings/engine-footprint-2026-09-29.md`; umbrella *Console strip: session-level
console effects with per-track inserts*) splits this issue's two halves.

- **Superseded: absent-member lanes.** A cohort of fewer than W EQ members binding as one bank
  moves to two slices:
  - *Pad parametric EQ banks with inactive lanes* (P2b) owns the mechanism, on the contract of *Let
    an effect bank bind a partial group with inactive lanes* (P2a). A padded lane clones an active
    member's request (not identity coefficients), is fed `+0.0`, and has its output discarded. D7
    is masked to active lanes.
  - *Bind every console slot banked for every track count* (S2) owns the policy for console slots.
- **Kept: identity slots.** An insert cohort whose member lacks the EQ slot binds with an identity
  slot on that lane. Console slots never need this, because every track has every slot. It is an
  insert-only optimisation, and no console slice depends on it.
- **Inserts bank opportunistically, as today** (decision 12). Whether a partial insert cohort
  should bind padded at all is a measured question: at W=8, a one- or two-member padded bank costs
  more than per-node rendering (Sol's H5 in `docs/handoffs/console-strip-2026-09-29/VERIFY.md`).
  The kept half ships only with a measured per-width rule, read against the console benchmark's
  remainder rows after *Measure the console strip against its baseline* (S4).
- **Dependencies** become P2b, whose mask and lane bookkeeping this reuses. #887 is optional.
- **Gates** 1 and 2 apply to the kept half: sessions whose EQ insert cohort has a member lacking
  the slot.

## Decision and evidence record (2026-10-24, backlog sprint, attempt 1; closes #888 and #889)

Narrowed to the planner policy, one PR for #888 and #889 (root scope note).

- **Absent-member lanes: delivered as policy.** The mechanism shipped in P2a-P2e, #887 and #1893.
  `banks::pads(group, width)` is now `console OR active_count() >= min_padded_insert_members(width)`.
  The planner constant is W=4: 2 and W=8: 3 (root decision). It comes from the H5 arithmetic
  (`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, H5): a padded eight-lane bank of three
  slots costs about 9.3 µs and a per-node track about 4.2 µs, so padding wins from three members
  at eight lanes. At four lanes, H5's "break-even probably near one member" gives two.
  **Reopen** when a measured per-node row exists at either width. There is no benchmark row: no
  per-node isolate exists, and production never runs a tuning mode.
  `stranded_mono_tracks` keeps `!pads`, so an insert remainder below k still strands (#971).
  Decision 12's Banking bullet gains one sentence: a partial insert cohort pads from k members.
  Class A: padding never moves an active lane's bits (coupling rule).
- **Identity slots (a member lacks a slot of its leader): retired, not delivered.** That needs a
  per-lane bypass in every bank kernel, which #96 F7 refused, or a per-slot save/restore copy
  (no-unnecessary-copies rule). `rack::BankChain` runs every slot over the whole resident block,
  and the contract admits prefix masks only (`effect.bank.mask_not_prefix`). The two 2026-09-27
  comments on #888 rule out identity coefficients as well: they are not exact on `-0.0`/inf, and
  a whole-bank D7 reaches bank-mates. This needs a new issue with its own design.
- **Silent skip (root question).** When the misaligned fixture's tail padded, the `skips > 0`
  check of `misaligned_lane_sets_decline_the_merge` failed. Cause, read from per-chain counters:
  the padded insert bank's padded lanes report at rest (rest mask `0b11110000`). Its active
  lanes' compressor and limiter are not at rest within the fixture's 12 blocks, the same as every
  full cohort's (rest `0`). Before #888 the per-node tail fed a builtin-only chain, which gathered
  exact zeros and skipped 3 blocks. That chain is now fed by the padded bank, which renders.
  There is no padding defect: the skip moved with the fixture's shape. The test now uses `W - 1`
  bare tracks, so the stripped remainder is one track, below k at both widths, and the skip is
  counted again. One more fact: the scatter-redirect count equals the per-node tail's size
  (`(64 - bare) % W`: checked with tails of 1 and 2), not the bare count.
- **Tests (keep the claim; shrink below k where the claim is per-node, pooling or replan):**
  - `bank_padding`: extended. Counts {1, k-1, k, W-1, W+3}, with k written out in the test. k
    members pad and k-1 do not, and every plan renders the Scalar oracle's bits.
  - `a_mono_track_that_fills_a_cohort_is_not_pooled_as_stereo`: shrunk. Only `ch00` keeps the
    limiter (a remainder of 1), so the per-program rule still moves a track and #971's does not.
  - `the_mono_remainder_stays_when_moving_it_binds_no_more_banks`: shrunk to one mono track.
  - `a_move_that_binds_as_many_banks_as_it_loses_is_not_kept`: one mono track and `W + k - 1`
    compressors. The trial's remainder of k-1 binds nothing; the move gains 2 and loses 2.
  - `the_rule_not_the_bank_gain_check_picks_the_moved_tracks`: P's stereo remainder is k-1, which
    the moved track lifts to k; T keeps one mono limiter.
  - The two replan tests (#1001): the stereo remainder is k-1, so the trial never binds it, and
    the move lifts it to k.
  - `a_console_track_is_never_pooled_as_stereo_to_fill_a_bank`: the stereo inserts are k-1.
  - `rack_placement_changes_the_bank_but_never_the_samples`: the counts are restated, because
    the claim is the bank count. Ten tracks leave a remainder of 2, which pads at W=4 and stays
    per node at W=8; the expected banks, the per-node count and the zero-plane bytes are derived
    from the threshold.
  - The randomized per-node-bits oracle now also asserts that the sweep reaches a padded insert
    bank.
