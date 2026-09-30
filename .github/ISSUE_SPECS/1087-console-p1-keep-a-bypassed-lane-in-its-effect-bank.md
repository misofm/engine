# Keep a bypassed lane in its effect bank

Slice P1 of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's finding H1 and amendment 1 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

A console slot's bypass must keep the track in its bank (decision 12, "Bypass"). Today it splits
the cohort:

- `EffectProgramKey` includes `bypass` (`crates/effect-contract/src/lib.rs:984-999`), so a
  bypassed track and an enabled track never share a bank. The key's own doc (`:950-983`) records
  why it is still there: every effect's bank reads one `metadata.bypass` for the whole bank, and
  `parametric-eq` does not run the wet path when bypassed (`crates/parametric-eq/src/lib.rs:3666-3668`).
- The session `bypass` becomes the prepared bypass (`crates/effect-compiler/src/prepare.rs:356`).
- A per-lane mechanism already exists outside the effects:
  - `EffectControlRecord::Bypass` (`crates/effect-contract/src/live.rs:89-95`);
  - `BypassShunt` (`live.rs:761-883`), which runs the wet path, preserves latency and selects whole
    blocks per lane;
  - the rack's live bank stage (`ConsoleEffectBankStage`, `crates/rack/src/lib.rs:911-1030`), which
    holds one control lane per bank lane and one AoSoA shunt.
- The gap: the shunt is built only when some lane has a live control channel
  (`rack/src/lib.rs:996-999`), and live lanes are seeded from the prepared bypass
  (`effect-compiler/src/prepare.rs:1331-1333`), which has already split the cohort.

## Smallest closable slice

For every bankable native effect, not only console slots (the console does not exist until S1a,
and the rule is the same wherever the effect sits):

1. A session `bypass` lowers to prepared `bypass = false` plus the lane's initial `BypassShunt`
   state.
2. The shunt is built for a bank whenever any lane is bypassed, whether or not a live control
   channel is attached.
3. `bypass` leaves the grouping identity, so mixed-bypass cohorts bind one bank. PDC is unchanged:
   route timings come from `PreparedEffectMetadata.latency` only.

A lane that ends up per node may keep today's prepared-bypass path, which skips the wet path. It
may also use the shunt. Its bits must be identical either way; choose one and say why.

Authorized paths:
- `crates/effect-contract/src/lib.rs` (the key) and `src/live.rs` (shunt construction only);
- `crates/effect-compiler/src/prepare.rs`;
- `crates/graph-compiler/src/banks.rs`;
- `crates/rack/src/lib.rs`;
- their tests, and this spec.

## Owner decisions that bind this slice

- A bypassed lane runs the wet path. That cost is accepted.
- Latency is always paid. A bypassed slot keeps its fixed latency on every lane, and a bypassed
  lane's impulse lands on the same sample as an enabled lane's.
- Banking may couple lanes' cost, never their bits. The shunt's per-lane select is a bitwise select,
  never an arithmetic identity: `fma(0, wet, dry)` turns `-0.0` into `+0.0`.

## Dependencies

None beyond decision 12. This is the first slice of batch C2, which starts after batch C1 (B0 and
S0's baseline) has been pushed, so that S0 timed the engine without this change.

P1 and P2a both edit `effect-contract/src/lib.rs` and `graph-compiler/src/banks.rs`, so P1 merges
before P2a starts.

This changes insert cohorts as well as console slots. On the app's current shape, bypassed tracks
stop forming their own cohort. S0 and S4's app-shape row measures that change.

## Objective gates

1. A graph-compiler test: a cohort of W tracks with any mix of bypassed and enabled lanes binds
   one bank, at Simd8 on x86-64 and at Simd4 through `scripts/run-aarch64-tests.sh` or the wasm
   gates.
2. A differential test: each bypassed lane's output is bit-identical to today's per-node
   prepared-bypass render. It covers random input, a `-0.0` input run and a latent slot's first
   block. The latent slot is the true-peak limiter at `rate/100 + 6` samples. NaNs fold to one
   value (decision 10). A planted `fma(0, wet, dry)` select turns it red.
3. Toggling bypass live on a banked lane still works and still preserves latency. The existing
   live-bypass tests pass unchanged.
4. PR evidence, not a committed test: console digests unchanged for sessions with uniform bypass.
5. `scripts/check-realtime-policy.sh`, `scripts/check-effect-contract.sh`,
   `scripts/check-rack-policy.sh`, the realtime audits and the callgraph gates pass. Render
   allocates nothing, measured with `bench_support::alloc`'s counters after warm-up.

## Non-goals

- No change to effect kernels.
- #892 (feed the dry line without a copy and swap when nothing is bypassed) stays its own
  performance issue. It matters more after P1, because a latent limiter bank with any bypassed lane
  now feeds a shunt.
- No bank-wide skip when every lane is bypassed; it is out of scope under decision 12.

## Standing rules for the implementer

- Work only from this body, the umbrella issue and decision 12. Read the cited code first.
- Class A: every gate that says "bit-identical" is a hard stop, not a tolerance.
- Render stays allocation-, lock- and syscall-free. Only `crates/lane` names `wide` or intrinsics.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value").
