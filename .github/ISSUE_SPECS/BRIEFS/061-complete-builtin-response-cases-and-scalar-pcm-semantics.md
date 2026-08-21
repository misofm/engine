# Sol implementation brief — issue 061 complete builtin response cases and scalar PCM semantics

## Decision and budget

**READY FOR TERRA ATTEMPT 1 after the Issue-060 rescope is committed.** One Terra attempt and one
bounded Sol correction/review are available; a second failure stops. Consume Issue 059's accepted
zero-recovery rule and stopped Issue 060 checkpoint `10f0235` only as technical input. Production,
graph, JSONL, audit, target and benchmark work is forbidden.

## Frozen response closure

Keep exactly 735 HPF, 735 LPF and 160 fixed-cascade rows, the exact launch rates
`44100,48000,88200,96000`, quanta `1,127,128,255,1024`, and the Issue-035 cutoff/probe construction.
Parse response case blocks as exact eight-field canonical records: ID, category, rate, quantum,
section, cutoff, probe and oracle. Require their coordinates to match both the ID-derived frozen
grid and CSV coordinate; reject ignored, reordered, missing, duplicate, noncanonical or extra
fields. Preserve the frozen 17-digit decimal convention.

For every CSV row require finite canonical fields, exact coordinate and `recovery_count=0`. Check
independent RBJ and cast-state tolerances for every applicable row. Check the one-second impulse
DFT and nonnegative final-4096 tail for analytic-only exact-cutoff and `0.49*rate` rows too; restrict
sustained fundamental/residual/total rules only to the already frozen coherent-bin rows. For one
`(rate,section,cutoff,probe)` coordinate, all five quantum rows must have bit-identical serialized
measurement fields and recovery. Stop for any changed tolerance/domain or production request.

## Frozen scalar PCM closure

Keep the current 32 non-graph paths. Validate the unsuffixed `matrix-ramp.f32le` explicitly rather
than merely owning its manifest path. Every identity/gain/polarity/mute/matrix corner/ramp/retarget
word comes from closed-form operation order; filter, partition, reset and L/R isolation use the
independent retained-`f32` recurrence.

Keep one `pcm/reset.f32le`: author an explicit deterministic sequence that processes a prepared
nonidentity chain, invokes `DiscontinuityKeepTargets`, processes the frozen probe, invokes
`FullToPrepared`, and processes it again. The checker independently derives every output word and
proves both calls occurred through focused fixture tests; no new case or payload is allowed.

## Gates and stop rules

Run focused response-case/CSV mutations and scalar PCM mutations, checked scratch and checked-in
corpus validation, fixture/reference tests, format, warning-denied package Clippy and diff checks.
Record exact hashes/counts and zero workload/timing. Stop for graph/JSONL work, a new corpus/file,
production changes or a second failed attempt.
