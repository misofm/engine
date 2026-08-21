# Sol implementation brief — issue 044 conditioned time-domain launch parametric EQ recurrence

## Decision and attempt budget

**READY FOR PREIMPLEMENTATION COMPARISON ONLY.** One Terra attempt and at most one bounded Sol
correction are available. Issue 042 is stopped without overall PASS. Preserve its public EQ
surface, complete analytic/search evidence and reusable scalar/bank/graph/audit infrastructure,
but treat its direct-history delta recurrence and state bytes as rejected. No benchmark or timing
command is authorized; `timed_benchmark_invocations=0`.

## Frozen comparison boundary

Use the accepted endpoint-conditioned seven-word transfer as immutable design input. Compare only
bounded `f32` runtime recurrence/state representations of that same transfer. At minimum include:

- an explicitly scaled direct-history delta state;
- an algebraically conditioned internal-state recurrence derived from the same transfer; and
- a candidate with a mathematically specified underflow-to-positive-zero rule at state-update
  boundaries.

Production may not import the comparison/oracle crate. Do not add a fourth filter family, alter
RBJ design, use f64 runtime state, special-case the failing bell, or change public domains.

For each candidate run the exact 48 cases formed by four launch rates, six kinds and two frozen
edges `(10,-24,0.1,0.1)` and `(20000,24,18,1)`. Run both the full one-second impulse and the seeded
million-sample valid sequence for every case. Record DFT error, state/output range, first invalid or
underflow sample, recovery count, layout words and a deterministic aggregate hash. Expected decay
may become positive zero only through the candidate's explicit frozen rule; it must not increment
recovery or mask nonfinite/unstable state.

## Selection gate

Select nothing until one candidate passes all 96 sequences with zero recovery, only finite normal
or positive-zero stored/output values, <=0.05 dB impulse/DFT error where the independent reference
is >=-120 dB, bounded million-sample state and fixed scalar/W4/W8 feasibility. Sol then amends this
brief with exactly one recurrence, retained/state word order, scaling constants, rounding points,
underflow boundaries/counters, scalar temporary order, allowed FMA sites, identity warming,
anchor-switch conversion, reset and atomic restore contract. If no candidate passes, stop before
production.

## Post-selection implementation boundary

Replace only the rejected recurrence/state path in `miso-engine-parametric-eq` and its prepared
delta bank token. Retain descriptors, parameters, automation order, graph topology and latency/tail.
Storage must be truly W4/W8 sized and exactly accounted. Scalar and base SIMD use the frozen graph;
FMA uses only enumerated contractions. Restore validates both lanes and every section before commit.

Re-enable the preserved ignored Issue-042 impulse, seeded-design and million-sample regressions.
Then run the complete analytic grid, 1,104 searches, 10,000 seeded designs, all time-domain cases,
automation, reset/restore, recovery, bypass and isolation tests. Finally rerun the existing
nine-track graph integration, 100,000-render audit and target/instruction script without expanding
their scope.

## Stop conditions

Stop for any tolerance/domain/recovery waiver, f64 production lane, unbounded or dynamically sized
state, hidden recurrence change after selection, unenumerated contraction, invalid value treated as
ordinary decay, render allocation/feature detection, graph/control expansion, timing or a third
attempt. Preserve the first failure and rescope rather than tuning around it.
