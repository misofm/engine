# Sol implementation brief — issue 114 native C ABI/runner qualification

## Decision

**COMPLETE / SOL XHIGH PASS / READY TO CLOSE AFTER UPSTREAM AND CI SYNCHRONIZATION.** Issues 116
and 121 were qualified as one frozen candidate without product repair. Sol High completed the
qualification-only pass and sole bounded correction; Sol XHigh returned strict terminal PASS.
Timing and benchmark invocation counts remained exactly zero.

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

## Final evidence and verdict

Pass 1 received the sole bounded **HOLD** because the checksum-bound evidence was not yet
semantically sealed: artifact/symbol/audit/count/gate records could be changed together with the
manifest, corrected consumer exits were not tied to final binaries/libraries, `readelf` was claimed
without validation, and strict gate results were not recorded.

The sole correction changed qualification evidence/checkers only and performed no product build,
consumer, runner, CAPI-audit or realtime-workload rerun. The final technical checkpoint is commit
`7a7b3c1862cacf1387471f64209a0994261e7262`, tree
`d00b08ed41641f11ce08a9edd8d828cd3dcf2430`. Evidence manifest
`6719a5027046695becd74696564d59392c8572faa4c6f6e003e5de943f1fac42` is independently enforced by
semantic checker `6c36bdc4188cf24003c6d36e99ffa9c1c20d71ecdfafdc7897cbeaf4725c51fc` and final checker
`5c9c50a662b500ef0ef05b50dd2de0ff51c892bac0edf3bb224d4ba2f2ccfe69`; correlated mutation suite
`e67d09a79e372133ff995f2054251d762427b7c609e9ac4607006e94c2d0d759` proves that recomputing the
manifest cannot bless altered artifact, symbol, audit, counter, consumer, raw-stage, gate or matrix
evidence.

The final matrix is exactly ten Linux `PASS` rows and seven candid `UNAVAILABLE` rows. The semantic
seal binds both 14-symbol GNU `nm` definition sets with zero prefix imports, corrected C11/C++17
static/shared exits and source/header/library/binary identities, raw logs and audits, armed syscall
trace, all execution/prohibited counters and 22 strict gates. The unsupported independent
`readelf` claim was removed.

One-shot counters are one product build, one consumer-fixture correction with initial exit 13,
four corrected consumer passes, one CAPI/protocol test invocation, one frozen runner-corpus
invocation, one 100,000-call C audit and one 1,000,000-block render/swap audit. Benchmark, timing,
playback, listening, browser and device invocations are zero. Sol XHigh returned strict terminal
**PASS** with the product/header/runner/accepted-fixture fence unchanged. Issue 114 is complete and
ready to close after upstream evidence, green required CI and GitHub synchronization; then Issue
026 is unblocked.
