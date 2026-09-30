# Bound and charge the bypass shunts, and keep the multiband's prepared bypass

Slice P1b of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`). It closes the conditions in Sol's P1
verdict (`.github/ISSUE_SPECS/1087-console-p1-keep-a-bypassed-lane-in-its-effect-bank.md`, "Sol
verdict, attempt 1", M1, M2, L2 and L3, commit `e0e65ccb` on `codex/1087-per-lane-bypass`). Batch
C2 is not pushed until this slice closes.

## Problem

P1 (#1087) lowers a session `bypass` to prepared `bypass = false` plus a `BypassShunt`. That left
four residuals:

1. **An unbounded staging window (M1).** Every session-bypassed instance that renders per node is
   now a `graph::runtime::ConsoleEffect` (`crates/graph/src/runtime.rs:864-879`). Its constructor
   sizes a staging window from `automation_capacity`, whether or not the lane has a live control
   channel.
   - One track with a bypassed EQ and `maximum_automation_spans_per_block: u32::MAX` bound at base.
     At P1 it aborts bind with `memory allocation of 171798691800 bytes failed`
     (`u32::MAX` x 40 B).
   - `baa03f09` fixed the rack twin (`ConsoleEffectBankStage`). The graph twin is unchanged, and no
     test covers it.
2. **Uncharged memory (M1).** `effect_control_resource` (`crates/graph-compiler/src/estimate.rs:151`)
   charges neither the staging windows nor any `BypassShunt`: its dry blocks, and a latent slot's
   delay line (31 KiB for a limiter slot at W = 8). At 128 spans, Sol measured:

   | Plan | Retained bytes added | Estimate added |
   |---|---|---|
   | One per-node bypassed EQ | 6,186 B | 72 B |
   | One 8-lane bypassed EQ slot | 7,808 B | 576 B |

   Live consoles already had this gap. P1 extends it to console-free hosts such as the C ABI.
3. **Multiband D7 coupling has no owner (M2).** A bypassed lane now shares a bank with enabled
   lanes. One bypassed multiband lane fed about `6e29` trips the bank's whole-bank D7 recovery and
   silences its enabled bank-mates. This is legal input, but far beyond real audio. P2b, P2c and
   P2e make D7 per lane for the EQ, compressor, gate, transient shaper and soft-clip. Nothing does
   so for the multiband: #1069 is its ramp-cut defect and does not mention D7.
4. **No committed allocation witness (L2, L3).** The zero-allocation evidence for a session bypass
   is a scratch probe. `bypass_cohorts` never sees a `-0.0` reach a bypassed slot from an enabled
   upstream stage, so an arithmetic `fma(0, wet, dry)` restore passes it.

## Smallest closable slice

1. **Bound the window.** A channel-less per-node lane (no live control channel) holds no staging
   window, as `baa03f09` did for the rack. A lane with a channel keeps today's window.
2. **Charge the memory.** `effect_control_resource`, or its caller, charges every staging window
   and every `BypassShunt` that bind will allocate, in both the per-node and the banked form. The
   estimate must not undercount the retained bytes it describes.
3. **Keep the multiband's prepared bypass.** A session `bypass` on `miso.multiband-compressor`
   keeps today's prepared bypass and is not lowered to a shunt. Its mixed-bypass cohorts therefore
   decline a bank, exactly as before P1. Name the list separately from `NEVER_BANKED_EFFECTS`,
   because the multiband still banks uniform cohorts. The slice that makes the multiband
   console-eligible, after #1069, owns its per-lane D7 and lifts this exclusion.
4. **Witnesses.**
   - A committed test renders mixed session bypass, both banked and per node, at Simd8, Simd4 and
     Scalar under `bench_support::alloc`'s counters, and asserts zero allocator calls after warm-up.
   - `bypass_cohorts` gains a session-level case in which an enabled upstream stage puts `-0.0`
     on a bypassed slot's input (for example, a compressor with negative makeup ahead of the slot).

Authorized paths:
- `crates/graph/src/runtime.rs` (`ConsoleEffect` construction only);
- `crates/graph-compiler/src/estimate.rs` and its caller in `crates/effect-compiler/src/prepare.rs`;
- `crates/effect-compiler/src/prepare.rs` (the prepared-bypass list);
- `crates/effect-contract/src/live.rs`, only for a `BypassShunt` size helper;
- their tests, `bypass_cohorts`, and this spec.

## Dependencies

- P1 (#1087). Branch from P1's head and merge right after P1, before P2a.

## Objective gates

1. One track with a session-bypassed EQ and `maximum_automation_spans_per_block: u32::MAX`
   binds, and so does an 8-lane bank with one bypassed lane. A test for each. Reverting step 1
   aborts the first.
2. A test asserts that a session-bypassed plan's estimate is at least its measured retained bytes,
   for one per-node bypassed EQ and one 8-lane bypassed limiter slot at 128 spans. It fails if the
   shunt charge is removed.
3. An 8-track multiband cohort with a mixed bypass mask renders exactly as it does at P1's base
   (`6fdf5db2`): the same plan shape (no bank) and the same bits. One bypassed lane fed `6e29` leaves
   its enabled neighbours' bits unchanged.
4. The allocation test and the `-0.0` case pass. A planted `fma(0, wet, dry)` restore turns the
   `-0.0` case red.
5. `scripts/check-realtime-policy.sh`, `scripts/check-effect-contract.sh`,
   `scripts/check-rack-policy.sh`, the realtime audits and the callgraph gates pass. Every other
   shipped plan's digests are unchanged.

## Non-goals

- No per-lane D7 for the multiband.
- No change to the estimate of anything but staging windows and shunts.
- #892 (dry-line copy and swap) stays its own performance issue.

## Standing rules for the implementer

- Work only from this body, the umbrella issue, P1's spec and verdict, and decision 12. Read the
  cited code first.
- Class A: every gate that says "bit-identical", "the same bits" or "unchanged" is a hard stop,
  not a tolerance.
- Render stays allocation-, lock- and syscall-free. Only `crates/lane` names `wide` or intrinsics.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value").
