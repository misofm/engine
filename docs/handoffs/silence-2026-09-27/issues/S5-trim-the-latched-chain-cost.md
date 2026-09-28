# Trim what a latched chain still costs

Draft, slice S5 of the silence architecture issue (A0). Class A. Evidence:
`docs/handoffs/silence-2026-09-27/DESIGN.md` section 6 ("Decomposing the latched 4.97 µs").

## Product outcome

After S4 and #965, a latched bank still pays for its limiter's cursor bookkeeping (measured 0.40 µs
per idle block over eight banks: 36 `u64` divisions per bank), its slots' drain calls (about 53 ns per
chain), its member-op loop (about 53 ns per chain), and its rest checks (six virtual calls, the input
section's eight-vector OR). The idle row's target is about 1 µs per block on the production feed;
this slice removes the four items above:

1. **Deferred limiter advance.** While latched, `advance_silent_rest` adds `frames` to a pending
   counter (one per channel, since a collapsed block advances only the left channel) instead of
   advancing; every reader of the phase or cursor words materialises the pending count first, with
   the same modular arithmetic: the first unlatched block, `snapshot_track_state_payload`,
   `restore_track_state_payload`, `reset`, `desymmetrize_channels` (`copy_state_from`,
   `crates/true-peak-limiter/src/lib.rs:2359-2361`), `channels_agree`, the channel-symmetry witness
   reads, response and observation reads, and every test-only oracle. So every observable state
   word is bit-identical at every point it can be observed.
2. **Cached rest.** A slot's `silent_skippable` answer is cached by the chain while sealed and
   re-asked on a block whose `begin_block` reported an admission (`begin_block` returns whether it
   admitted anything; the default returns `false`), and invalidated on `desymmetrize`, a collapse
   transition, restore, reset and (trivially) a plan swap.
3. **Drain mask.** At bind, the chain records which slots' `begin_block` can admit anything (a
   console stage, a builtin bank with a control lane); a latched block calls only those.
4. **Skip the member loop.** `graph::Runtime::execute` asks the chain whether it will skip before
   running the bank's member ops, when every member's reduction is the dedication copy
   (`bank_gather_source` is `Some`, `crates/graph/src/runtime.rs:3043`); members that do real
   reductions still run.

## Non-goals

No new skip condition, no change to what qualifies, no effect claim changes beyond the limiter's
deferral.

## Authorized paths

`crates/rack/src/lib.rs`, `crates/graph/src/runtime.rs`, `crates/true-peak-limiter/src/lib.rs`, the
`begin_block` signatures in `crates/rack/src/lib.rs` and `crates/graph/src/lib.rs` and their
implementors (`crates/builtins-compiler/src/lib.rs`, rack's console stage), tests, `MUTATIONS.md`,
this spec.

## Objective gates

1. S4's bit-identity corpus, unchanged, plus a limiter-state gate: after every latched run of 1-5,000
   blocks, the limiter's state payload (all lanes) equals the declined arm's.
2. Red mutations: apply the pending advance twice; forget it on a snapshot; forget it on
   `desymmetrize`; defer the right channel on a collapsed block; cache the rest answer across an
   admitting block or a `desymmetrize`; mask out a slot that has a control lane.
3. Worst case and realtime gates as S4.
4. The descriptive in-process idle A/B (same runtime) is recorded; no projection is quoted.

## Console benchmark rows

`sixty_four_track_idle` and the sparse rows. Every digest unchanged.

## Dependencies

S4.

## Standing rules for the implementer

As S4.
