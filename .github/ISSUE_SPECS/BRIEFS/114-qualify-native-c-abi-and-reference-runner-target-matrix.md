# Sol implementation brief — issue 114 native C ABI/runner qualification

## Decision

**WAITING FOR ISSUES 073 AND 113.** After both pass, qualify their frozen bytes; do not repair them
inside this issue. Sol High implements and Sol XHigh verifies with one pass plus one bounded HOLD
correction. A second HOLD stops. Timed and benchmark counts remain zero.

Direct dependencies are **Native WAV/RF64 reference runner over frozen C ABI V1** (073) and **Close
C ABI control/event transport and transactional plan replacement** (113). PASS gates **End-to-end
release, performance, and listening qualification** (026). Issue 025 needs 113, not this matrix.

## Matrix

Pin all accepted product, header, lock, runner and fixture hashes. Run linked C11/C++17 static/shared
Linux consumers; compile/link and exact-object inspection on macOS x86_64/AArch64 and Windows
GNU/MSVC; Android AArch64 and iOS AArch64 compile boundaries; then the frozen four-rate RIFF and
representative RF64 runner corpus. Every row is exact PASS/FAIL/UNAVAILABLE with tool identity;
missing tooling alone permits unavailable.

Qualify commands/responses/events, replay/revision, source submit/seek, transactional replacement,
source epochs, retirement/destroy order, layout/version/symbols, malformed/one-short inputs,
atomic runner output and non-timed million-call realtime audits. Independent checkers must reject
false export matches, altered authorities, fabricated availability and timing evidence.

## Fence

Only a new qualification tool/fixtures/docs/checkers, minimal manifest/lock rows and issue evidence
may change. CAPI/source/protocol/session/graph/DSP/header/runner/accepted fixtures are immutable. Run
no benchmark, timing, tuning, browser/device workload or listening. Sol High stops at the complete
focused checkpoint for strict Sol XHigh verdict.
