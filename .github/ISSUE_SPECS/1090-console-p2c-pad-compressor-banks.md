# Pad compressor banks with inactive lanes

Slice P2c of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's M1, M2 and amendment 6 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

A console compressor slot must bank at every track count (decision 12, "Banking"). After P2a, the
compressor's `bind_homogeneous_bank` (`crates/compressor/src/lib.rs:756`) still declines a padded
request.

- It declines a sidechain (`:786-795`), which console slots never carry (decision 12,
  "Sidechain"). It refuses non-native widths (`:800-802`).
- Its whole-bank D7 recovery resets every lane.
- Its whole-bank fast paths couple cost, not bits:
  - the idle-lane guard (`docs/rulings/compressor-idle-lane-guard-console-under-resolved.md`: "one
    automated track drags every other lane of its bank");
  - silent admission (`block_is_positive_zero` over `frames * lanes`,
    `crates/effect-runtime/src/bank.rs:137`).

## Smallest closable slice

Opt the compressor into padded requests under P2a's contract:
- a padded lane carries a clone of an active member's request, is fed `+0.0`, and its output is
  discarded;
- D7 recovery and reports attribute active lanes only.

A padded lane's detector and gain smoother stay at their rest state on `+0.0` input, so a silent
padded lane never keeps an otherwise silent bank out of silent admission.

Authorized paths:
- `crates/compressor/src/lib.rs` (bank binding, the D7 path and lane bookkeeping only);
- `crates/compressor/src/kernel.rs`, only if a padded lane needs a guard that costs nothing on
  active lanes;
- their tests, and this spec.

## Relation to #889

- This slice **supersedes #889's absent-member half**: a cohort of fewer than W compressor tracks
  binding as one bank.
- **#889 is amended** to keep only its identity-slot half: an insert cohort whose member lacks the
  compressor slot. It is insert-only and optional, and no console slice depends on it. See the
  amendment appended to #889's spec.

## Dependencies

- *Let an effect bank bind a partial group with inactive lanes* (P2a, #1088).

This is batch C2. P2b-P2e edit disjoint effect crates, so after P2a merges they may land in
any order, one merge each.
- It reuses the test shape of *Pad parametric EQ banks with inactive lanes* (P2b) but does not
  depend on it.

## Objective gates

1. For every active count 1..W-1, a padded compressor bank's active lanes are bit-identical to the
   same tracks rendered per node. This holds on random input, on the compressor's fixtures, and on
   blocks where active lanes ramp or are cut mid-ramp (the #1069 shape), at Simd8 on x86-64 and at
   Simd4 through `scripts/run-aarch64-tests.sh` or the wasm gates.
2. Coupling rule: active lanes' bits do not depend on the clone source, and every link mode
   combines L and R of one lane only.
3. A bank whose active lanes are all silent still takes silent admission. A padded lane that
   defeated it would turn this test red.
4. D7: a planted non-finite state in one active lane recovers and reports that lane alone.
   A bypassed lane counts as active: a bypassed lane fed a tripping value (for example `1e30`
   behind enough legal gain) leaves every enabled bank-mate's bits unchanged (P1 verdict, M2).
5. `cargo test -p compressor -p graph-compiler -p graph` pass, as do
   `scripts/check-effect-runtime-policy.sh`, `scripts/check-realtime-policy.sh` and the realtime
   audits. PR evidence: console digests are unchanged.

## Non-goals

- No kernel arithmetic change on active lanes.
- No ramp-path change.
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
