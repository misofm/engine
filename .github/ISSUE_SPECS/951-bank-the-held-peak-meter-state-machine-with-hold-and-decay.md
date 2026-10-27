# Bank the held-peak meter state machine with hold and decay

## Product outcome

#950 banks the full meter pass for meters with hold 0 and decay off, the only configuration any host binds today. A meter with a peak hold time or a dB-per-second decay still runs its held-peak state machine one lane at a time. Under the owner's rule ("We shouldn't leave scalar arithmetic where vector arithmetic is possible"), that is a scheduled defect, not a permanent path: the held-peak update is a per-lane, sample-serial state machine (hold countdown, then decay) that can run across a bank's lanes exactly like #950's seeded energy sum, with each meter's own state as the seed.

## Smallest closable slice

To be briefed after #950 lands, reusing its seed-and-accept design: a lane-parallel, sample-serial held-peak pass for hold > 0 and decay on, bit-identical to the scalar state machine on every published word (`held_peak`, and every word it feeds), with the scalar path as the oracle and a wasm differential.

## Dependencies

#949, #950.

Filed from #950's coordinator ruling 2.

## Brief — 2026-10-27 (Sol implementation, attempt 1)

The owner selected three Sol implementation streams and CI-conscious batching. This stream owns
only #951; no local build/test/formatter runs before root approves this brief. Every subsequent
check uses the shared serial wrapper at
`/home/bl/misofm/engine-work-evidence/easy3-20261027/run-serial.sh`, in this worktree,
with the wrapper's own target directory, jobs=2 and test threads=1. No push, PR or GitHub mutation
is authorized in this stream; root synchronizes the issue and delivers the batch.

Smallest closable product slice: bank every held-peak sample update for a final resident meter
with a positive hold and/or enabled decay, using each lane's own post-preamble state as the seed.
Keep the existing hold=0/decay=0 maximum merge and existing energy seed search unchanged. A separate
held seed query and held pass let a held-only meter seed without stealing another meter's energy
seed. The first compatible held state per lane is computed once; every consumer verifies its own
state and configuration before accepting and otherwise uses its correct scalar fallback. Blocks
crossing a window still take the existing scalar split; discontinuities seed from cleared state.

The scalar recurrence remains the definition: sanitize the sample, take its magnitude; if it is
at least the held value, replace held and reload the u32 hold counter; otherwise decrement a
positive counter; otherwise multiply by the prepared per-sample decay and flush non-normal
results to +0 when decay is enabled. Off means an exact identity multiplier. The counter is the
full u32 domain: two f32 vectors carry its high/low 16-bit digits, with exact borrow/decrement;
casting the whole counter to f32 would lose countdown frames above 2^24. Every lane has independent
state, hold length and decay. No cross-lane reductions, allocations, new latency, signal change,
host/protocol change, effect change or lane-trait expansion is needed. Snapshot/cumulative count
accounting remains the owning meter's existing commit. Meter state/config/result metadata is inline.

Exact planned paths:

- this numbered spec;
- `crates/lane/src/kernels/builtins.rs` (held state/config vocabulary and bank kernel);
- `crates/builtins/src/lib.rs` (held seed, validated accept, recurrence state commit);
- `crates/graph/src/lib.rs` and `crates/graph/src/runtime.rs` (necessary production seed/result
  transport and resident pass wiring, narrowly within the existing meter mechanism);
- `crates/builtins-compiler/src/lib.rs` (observer wiring and existing integration test);
- `crates/builtins/tests/meter.rs` (extend the existing owning scalar/banked differential);
- `tools/wasm-gate-corpus/src/lib.rs` (extend existing meter differential with independent held
  oracle, including high counter limbs and per-lane parameters);
- `tools/wasm-gate-guest/src/lib.rs` and `scripts/run-wasm-gates.sh` only if needed to extend the
  existing lowering probe/check to the held recurrence. No new report field or pinned digest.

Objective gates: existing meter differential must publish bit-identical full snapshots for all
metric selections, hold-only/decay-only/combined ballistics, window splits, resets/discontinuities,
negative zero/non-finites and lifetime counters. Update its existing commit-count assertions so
held configurations must actually bank; wrong held state/config seeds must decline. The existing
compiler integration test must prove production bank commits with positive hold/decay at padded
track counts and no render allocations/frees. Extend the existing cross-target kernel differential
for per-lane state/config and countdown boundaries (including u32::MAX), check wasm simd128 and
scalar-oracle identity, and inspect vector lowering using the existing guest probe. Red evidence
must show that the pre-change implementation fails the new bank-commit assertion and that a held
recurrence mutation fails the owner differential; restore before the green checkpoint. Reuse
existing native allocation, source sanitization and resource ownership gates instead of adding
redundant tests. Run focused owning tests, proportional lint/format, and wasm differential under
the shared serial wrapper. Broad merged-tree gates and independent Astra review belong to root.

No benchmark or listening exercise is needed: this is a bit-preserving meter-only arithmetic
change, not audio DSP or a claimed performance result. Extended target matrices or new harness
work are qualification successors if the representative existing gates cannot run locally.
After the coherent green tranche, commit exact paths locally and pause for root's status/commit
audit before further implementation. Maximum five attempts, each with one adversarial verdict.
