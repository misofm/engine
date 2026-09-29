# Pad gate/expander, transient shaper and soft-clip banks with inactive lanes

Slice P2e of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's M1 and amendment 6 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`). No open issue covers these
three effects.

## Problem

The gate/expander, transient shaper and soft-clip are on the console eligibility list (decision 12,
"Eligibility"), so each must bank at every track count. After P2a, each `bind_homogeneous_bank`
still declines a padded request:

- gate/expander at `crates/gate-expander/src/lib.rs:1036`;
- transient shaper at `crates/transient-shaper/src/lib.rs:816`;
- soft-clip at `crates/soft-clip/src/lib.rs:879`.

Two of them also need a fix before they can pad:

- **The gate** fills memberless lanes with all-zero parameters, which lie outside its declared
  domains (`gate-expander/src/lib.rs:1049`, `:308-315`).
- **Soft-clip** charges every lane in its D7 recovery (`soft-clip/src/lib.rs:1035-1048`).

## Smallest closable slice

Opt each of the three effects into padded requests under P2a's contract:
- a padded lane carries a clone of an active member's request, never zeros, is fed `+0.0`, and its
  output is discarded;
- D7 recovery and reports attribute active lanes only.

The gate's zero-filled memberless lanes are replaced by the clone rule. Work in three checkpoints,
one per effect. If the slice outgrows half a day, root may split it per effect without a rebrief.

Authorized paths: the bank binding, D7 path and lane bookkeeping of `crates/gate-expander/src/lib.rs`,
`crates/transient-shaper/src/lib.rs` and `crates/soft-clip/src/lib.rs`, their tests, and this spec.

## Dependencies

- *Let an effect bank bind a partial group with inactive lanes* (P2a, #1088).

This is batch C2. P2b-P2e edit disjoint effect crates, so after P2a merges they may land in
any order, one merge each.

## Objective gates

1. For each effect and every active count 1..W-1, a padded bank's active lanes are bit-identical
   to the same tracks rendered per node. This holds on random input, on random parameters and on
   each effect's fixtures, at Simd8 on x86-64 and at Simd4 through `scripts/run-aarch64-tests.sh`
   or the wasm gates.
2. The gate: no lane is ever prepared with a parameter outside its declared domain. A planted
   zero-filled lane turns the test red.
3. Coupling rule: active lanes' bits do not depend on the clone source.
4. D7: for each effect, a planted non-finite state in one active lane recovers and reports that
   lane alone. Soft-clip charges active lanes only.
5. `cargo test -p gate-expander -p transient-shaper -p soft-clip -p graph-compiler -p graph` pass,
   as do `scripts/check-effect-runtime-policy.sh`, `scripts/check-realtime-policy.sh` and the
   realtime audits. PR evidence: console digests are unchanged.

## Non-goals

- No kernel change.
- #894 (silent-block admission for the gate and transient shaper) stays a separate performance
  issue.
- The multiband compressor is not padded until #1069 closes (decision 12, "Eligibility"). Its
  padding is a later issue.

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
