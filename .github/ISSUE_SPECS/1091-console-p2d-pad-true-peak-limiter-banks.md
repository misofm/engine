# Pad true-peak limiter banks with inactive lanes

Slice P2d of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's M1, L1, L4 and amendment 6 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`). No open issue covers the
limiter.

## Problem

The limiter is the standing `post_insert` slot of the console fixtures, and it must bank at every
track count (decision 12, "Banking"). After P2a, its `bind_homogeneous_bank`
(`crates/true-peak-limiter/src/lib.rs:4133`) still declines a padded request.

- It takes its fast body only when every lane shares window shape and phase (`:1424-1427`,
  `:716-728`). A padded lane configured any other way would drag the whole bank onto the slow body.
- Its whole-bank D7 recovery resets every lane (`:3527-3533`).
- Its latency is fixed per rate at `rate/100 + 6` samples (`:234-241`). `lookahead` has no
  automation rate, so per-track knobs never split a bank (L1).

## Smallest closable slice

Opt the limiter into padded requests under P2a's contract:
- a padded lane carries a clone of an active member's request, is fed `+0.0`, and its output is
  discarded;
- D7 recovery and reports attribute active lanes only.

Because the clone copies the member's window shape and phase, a padded bank takes the same body a
full bank of the same members would take.

Authorized paths: `crates/true-peak-limiter/src/lib.rs` (bank binding, the D7 path and lane
bookkeeping only), its tests, and this spec. Coordinate with the open limiter kernel issues
(#988-#992); this slice changes no kernel.

## Dependencies

- *Let an effect bank bind a partial group with inactive lanes* (P2a, #1088).

This is batch C2. P2b-P2e edit disjoint effect crates, so after P2a merges they may land in
any order, one merge each.

## Objective gates

1. For every active count 1..W-1, a padded limiter bank's active lanes are bit-identical to the
   same tracks rendered per node. This holds on random input, on the limiter's fixtures and
   `link_mode: maximum`, and from the first block (latency line filling), at Simd8 on x86-64 and at
   Simd4 through `scripts/run-aarch64-tests.sh` or the wasm gates.
2. A padded bank of members that share window shape and phase takes the fast body. A test observes
   the body choice, and a padded lane built from zeros or defaults turns it red.
3. Coupling rule: active lanes' bits do not depend on the clone source.
4. D7: a planted non-finite state in one active lane recovers and reports that lane alone.
5. `cargo test -p true-peak-limiter -p graph-compiler -p graph` pass, as do
   `scripts/check-effect-runtime-policy.sh`, `scripts/check-realtime-policy.sh` and the realtime
   audits. PR evidence: console digests are unchanged.

## Non-goals

- No kernel change.
- No change to the latency formula.
- No planner policy change.

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
