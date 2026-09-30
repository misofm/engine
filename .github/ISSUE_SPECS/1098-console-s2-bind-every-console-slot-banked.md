# Bind every console slot banked for every track count

Slice S2 of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's H2, H5, M3, L3 and amendments 3,
9 and 10 in `docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

Decision 12 makes console slots always bank, on every target and for every track count, with
partial groups padded and no member threshold. After P1, P2a-P2e and S1a:
- the mechanism exists;
- every eligible effect accepts padded requests;
- a bypassed lane stays in its bank;
- console slots lower to the `Simd1` and `Simd2` racks, which after S1a no session can reach any
  other way.

What is missing is the policy and the guarantee. The planner still pads nothing, so a console
remainder renders per node.

## Smallest closable slice

1. **Policy.** The planner forms one bank group per (slot, pool class, dependency level) for every
   console slot, and pads each to W (H2, M3). Inserts keep today's rule: full groups bank and
   remainders render per node (decision 12, "Inserts bank opportunistically, as today").
2. **Guarantee.** On every production backend (Simd8 and Simd4), a console slot never renders per
   node. If a console group does not bind banked, compilation fails with a typed diagnostic (for
   example `console.slot.unbanked`) naming the slot, pool class and level. There is no silent
   fallback. S1a's eligibility refusal makes that unreachable for valid sessions; the diagnostic
   guards against regressions. The test-only `Scalar` oracle (#1059), which exists only for tests
   and `test-support`, is exempt: it is the per-node reference that gate 2 compares against.
3. **#971 under padding** (amendment 10). Once every console group binds, the stranded-mono
   demotion's "keep the move only if it binds more banks" objective
   (`crates/graph-compiler/src/banks.rs:243-356`) measures nothing for console slots. Decide one of
   the following, state why in this spec's evidence, and pin the decision with a test:
   - restate the objective, for example minimise planes x banks across the track's slots;
   - retire the demotion for console slots.

   The mono pool is worth about 36 % on the standing console row (M3), so the choice must not
   silently demote mono tracks.
4. **Record what grouping costs.** Differing insert counts split `post_insert` into one group per
   level, and misaligned cohorts pay a planar/AoSoA round trip at chain fusion (H2). Do not fix it
   here. An ALAP alignment of `post_insert` banks is a successor only if S4 measures a need.

Authorized paths: `crates/graph-compiler/src/banks.rs` and its planner call sites, the compile
diagnostics, their tests, and this spec.

## Owner decisions that bind this slice

- Console slots always bank, with no threshold. The owner accepted H5's cost: a one- or
  two-member remainder costs more padded than per node at W=8.
- Banking may couple lanes' cost, never their bits.
- Bypass is per lane, and a bypassed lane stays in its bank.
- The silent fast path stays bank-wide. A per-lane silence skip is a later issue if S4's
  sparse-activity row warrants it.

## Dependencies

This is batch C4, after batch C2 (P1, P2a-P2e) and batch C3 (S1r, S1a-S1d) have been pushed.

- *Keep a bypassed lane in its effect bank* (P1).
- *Pad parametric EQ banks with inactive lanes* (P2b).
- *Pad compressor banks with inactive lanes* (P2c).
- *Pad true-peak limiter banks with inactive lanes* (P2d).
- *Pad gate/expander, transient shaper and soft-clip banks with inactive lanes* (P2e).
- *Add the session console and per-track inserts to the session schema* (S1a).

## Objective gates

1. **Binding.** Every console slot binds banked at N in {1, 3, 5, 9, 10, 13}. The test is on the
   compiled plan: no console slot has a per-node node. It covers:
   - mixed bypass;
   - mixed insert counts, so `post_insert` splits by level;
   - mono and stereo pool classes;
   - each eligible effect as a slot.

   It runs at Simd8 on x86-64 and at Simd4 through `scripts/run-aarch64-tests.sh` or the wasm gates
   (L3). The group count equals the sum over (pool class, level) of `ceil(n / W)`.
2. **Class A.** A committed randomized differential renders these sessions banked, on a
   production backend, and per node, through the test-only `Scalar` oracle. Every track is
   bit-identical in both, with NaNs folded (decision 10). A planted
   whole-bank decision that is not bit-neutral per lane turns it red.
3. **Diagnostic.** A planted factory decline on a console group, on a production backend, fails
   the compile with the typed diagnostic and never yields a per-node plan. The `Scalar` oracle's
   exemption does not reach a production backend: a test compiles the same session on Simd8 with the
   planted decline and expects the refusal.
4. **#971.** The chosen rule is pinned by a test on a session with stranded mono tracks.
5. **Realtime.** `scripts/check-realtime-policy.sh`, the realtime audits and the callgraph gates
   pass, and render allocates nothing.
6. **PR evidence.** The 64-track console digests are unchanged. The ragged 9-track fixture's
   render is unchanged, and its plan now binds the remainder banked; explain every moved plan pin.

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
