# Sol implementation brief — issue 063 complete builtin meter, diagnostic, and resource semantics

## Decision and budget

**READY FOR TERRA ATTEMPT 1 after the Issue-060 rescope is committed.** One Terra attempt and one
bounded Sol correction/review are available; a second failure stops. The interrupted Issue-060 2B
diff is design evidence only and must not be assumed compiling or correct. No production, graph
PCM, final corruption, audit, target or benchmark work is allowed.

## Frozen typed rows

Parse JSONL with exact canonical key sets and encodings, not substring presence. Require exactly
seven graph meter snapshots and fifteen window/drop records. The independent meter model freezes
sample sanitation, per-lane peak/energy/RMS, clip/sanitation interval and cumulative counts,
hold/decay, sequence, reset generation, discontinuity, queue capacity, wrap, drain/drop and
overflow behavior. Graph snapshot values may be supplied by Issue 062's independently modeled tap
samples when available; this issue owns meter recurrence and serialization, not graph PCM.

Require the exact thirteen diagnostic case/code/path/error tuples already present in the frozen
file, strictly sorted with no extras. Require the exact nine resource coordinates and prove each
retained total with checked processor-plus-meter arithmetic, the exact maximum single allocation
and exact retained allocation count under the pinned native fixture ABI. Hardcoded observed totals
without a named derivation are not sufficient.

## Gates and stop rules

Add focused manifest-valid tuple removals and field mutations for each owned JSONL class. Run valid
scratch/check-in validation, focused fixture/reference tests, format, warning-denied package
Clippy and diff checks. Record counts, hashes and derivation. Stop for response/PCM/PDC work, new
formats or files, final cross-format mutation work, production changes, timing or a second failure.
