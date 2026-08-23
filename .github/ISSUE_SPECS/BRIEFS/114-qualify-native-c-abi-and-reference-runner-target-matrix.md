# Sol implementation brief — issue 114 native C ABI/runner qualification

## Decision

**STATELESS SOL XHIGH BRIEF / READY FOR SOL HIGH PASS 1.** Issues 116 and 121 are closed with strict
PASS. Qualify their joined frozen bytes; do not repair them inside this issue. Sol High implements
and Sol XHigh verifies with one pass plus one bounded HOLD correction. A second material HOLD stops.
Timing and benchmark invocation counts remain exactly zero.

Direct dependencies are **Seal portable native PCM runner under an explicit output-directory
ownership contract** (116) and **Close CAPI-owned render events and primitive replacement resource
authority** (121). Stopped Issue 115 is technical input only through accepted 116; stopped Issues
113, 117, 118, 119 and 120 are technical input only through accepted 121. PASS gates
**End-to-end release, performance, and listening qualification** (026). Issue 025 needs accepted
121, not this matrix.

The aggregate candidate is clean `main` commit `feb039765271ca62b0c905004689b88ad92df65b`, tree
`e3e11c343c6f6a5b5b380abe03c0431c6fe81579`. Bind accepted Issue 116 commit/tree
`45f8f5af8bdd578b5ccb27fdb787f7a663c39818` /
`7e0a7b7d48362c9b9eaa15b1cfce7180c935c5b5` and Issue 121 commit/tree
`a9a975d8f679707701cc60ad102c817eb54c3082` /
`16728c5ea434dde1a75bdd4500568db8c283a2ca`. The joined C header is
`83880c2fd7b5bc835425a5a64cae19c8a0bba17f49b4802b4033a8e7dfeac37c`; the lock is
`c89b195f0d31ad21852d0a931023c70e1eb4a0caa534bfd6e1692c1e1178fd52`. Existing `target/`
libraries are stale/unqualified and must not be consumed.

## Matrix

Pin all accepted product, header, lock, runner and fixture hashes. Run linked C11/C++17 static/shared
Linux consumers; compile/link and exact-object inspection on macOS x86_64/AArch64 and Windows
GNU/MSVC; Android AArch64 and iOS AArch64 compile boundaries; then the frozen four-rate RIFF and
representative RF64 runner corpus. Every row is exact PASS/FAIL/UNAVAILABLE with tool identity;
missing tooling alone permits unavailable.

Readiness inspection found Rust/Cargo 1.97.1 on Linux x86_64; Rust targets for Linux x86_64,
Windows GNU, macOS x86_64/AArch64, iOS AArch64 and Android AArch64 are installed. GCC/G++ 13.3.0,
GNU binutils 2.42, Python 3.12.3 and Bash 5.2.21 support the host rows. Windows MSVC target/tools,
MinGW C/C++ tools, Apple SDK/xcrun/object tools, iOS simulator target, Android NDK Clang and LLVM
object tools are absent. Freeze availability before building: later-present tools get exact
path/version; otherwise dependent rows are candid `UNAVAILABLE`. Only Linux runtime is available.
Missing tools may never conceal a compile/product failure.

Qualify commands/responses/events, replay/revision, source submit/seek, transactional replacement,
source epochs, retirement/destroy order, layout/version/symbols, malformed/one-short inputs,
atomic runner output and non-timed million-call realtime audits. Independent checkers must reject
false export matches, altered authorities, fabricated availability and timing evidence.

Produce static/shared libraries once in a fresh qualification-owned staging directory from the
pinned main tree, hash them before use and reject every pre-existing artifact. Issue 116 forbids
another seal/retry, but its frozen runner corpus may execute once as this issue's qualification row.

## Fence

Only a new qualification tool/fixtures/docs/checkers, minimal manifest/lock rows and issue evidence
may change. CAPI/source/protocol/session/graph/DSP/header, the runner accepted by Issue 116 and its
fixtures are immutable. Run
no benchmark, timing, tuning, browser/device workload or listening. Sol High stops at the complete
focused checkpoint for strict Sol XHigh verdict.
