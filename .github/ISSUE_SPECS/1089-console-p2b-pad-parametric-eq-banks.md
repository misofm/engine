# Pad parametric EQ banks with inactive lanes

Slice P2b of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's M1 and amendment 6 in
`.github/ISSUE_SPECS/DRAFT-console-strip-VERIFY.md`, commit `03aceb94`).

## Problem

A console EQ slot must bank at every track count (decision 12, "Banking"). After *Let an effect
bank bind a partial group with inactive lanes* (P2a), the EQ's `bind_homogeneous_bank`
(`crates/parametric-eq/src/lib.rs:3200`) still declines any request whose active mask is not full.

- The EQ already pads internally with identity words (`:3123-3170`). It refuses non-native widths
  (`:3210`).
- Its whole-bank D7 recovery zeroes or resets every lane (`:2926-2930`), so a fault on one lane
  would be reported against padded lanes too.

## Smallest closable slice

Opt the EQ into padded requests under P2a's contract:
- a padded lane carries a clone of an active member's request, is fed `+0.0`, and its output is
  discarded;
- D7 recovery and reports attribute active lanes only.

Keep #1070's order: validate every active member first, then decide bind or decline.

Authorized paths: `crates/parametric-eq/src/lib.rs` (bank binding, the D7 path and the bank's
lane bookkeeping only), its tests, and this spec.

## Relation to #888 and #887

- This slice **supersedes #888's absent-member half**: a cohort of fewer than W members binding as
  one bank.
- **#888 is amended** to keep only its identity-slot half: an insert cohort whose member lacks the
  EQ slot. That is an insert-only optimisation, and no console slice depends on it. See the
  amendment appended to #888's spec.
- #887 (tiled gather and scatter for partial banks) is a performance follow-up, not a
  prerequisite. The rack already runs partial chains.

## Dependencies

- *Let an effect bank bind a partial group with inactive lanes* (P2a).

## Objective gates

1. For every active count 1..W-1, a padded EQ bank's active lanes are bit-identical to the same
   tracks rendered per node. This holds on random input, on random parameters and ramps, and on
   the EQ's fixtures, at Simd8 on x86-64 and at Simd4 through `scripts/run-aarch64-tests.sh` or the
   wasm gates.
2. Coupling rule: the active lanes' bits do not depend on which active member the padded lanes
   clone. A test varies the clone source.
3. A padded lane is never scattered, and its state stays finite on `+0.0` input.
4. D7: a planted non-finite state in one active lane recovers that lane and reports it alone.
   Padded lanes are neither reported nor charged.
5. `cargo test -p parametric-eq -p graph-compiler -p graph` pass, as do
   `scripts/check-parametric-eq-render-contract.sh`, `scripts/check-effect-runtime-policy.sh`,
   `scripts/check-realtime-policy.sh` and the realtime audits. PR evidence: console digests are
   unchanged.

## Non-goals

- No kernel arithmetic change on active lanes.
- No planner policy change: console slots start padding in S2, and inserts keep today's rule.

## Standing rules for the implementer

- Work only from this body, the umbrella issue and decision 12. Read the cited code first.
- Class A: every gate that says "bit-identical" is a hard stop, not a tolerance. NaNs fold to one
  value (decision 10).
- Render stays allocation-, lock- and syscall-free. Only `crates/lane` names `wide` or intrinsics.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value"). A
  one-time "no bit moved" comparison against the pre-change base is PR evidence, not a committed
  test.
