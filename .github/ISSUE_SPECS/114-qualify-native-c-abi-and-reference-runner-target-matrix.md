# 114 Qualify native C ABI and reference runner target matrix

## Outcome and readiness

Qualify the completed native C ABI control/render product and WAV/RF64 reference runner across the
launch native target boundary without changing product bytes or rerunning descriptive benchmarks.

**STATELESS SOL XHIGH BRIEF / WAITING FOR ISSUES 115 AND 113.** Sol High implements; Sol XHigh
briefs and verifies. One pass plus one bounded HOLD correction is the complete budget. A second HOLD
stops. Benchmark/timing counts are zero and must remain zero.

Read-only boundary inspection found remote Issue 114 unallocated. Root must create/synchronize it
under this exact title after the docs checkpoint is upstream. This record makes no GitHub change.

## Dependencies by exact title

- **Close portable native PCM runner publication and seal the reference tool** (Issue 115)
- **Close C ABI control/event transport and transactional plan replacement** (Issue 113)

This issue gates **End-to-end release, performance, and listening qualification** (Issue 026). It
does not gate Issue 025, which consumes Issue 113 directly.

## Qualification-only matrix

Freeze the accepted #115/#113 candidate before implementation and pin the installed header, static/
shared libraries, runner binary/source, Cargo locks, fixture manifests and protocol/session corpora.
No product correction is permitted here.

The exact matrix is:

- Linux x86_64: build and run C11 plus C++17 consumers against installed static and shared
  libraries; enumerate the exact exported V1 symbol set; verify ABI version, struct sizes/offsets,
  reserved-zero behavior, command/response/event buffers, source submit/seek, replacement boundary,
  retirement and destroy order;
- macOS x86_64 and AArch64 plus Windows GNU/MSVC: compile/link equivalent consumers and inspect
  exact symbols/imports without claiming a runtime where the environment lacks it;
- Android AArch64 and iOS AArch64 simulator/device compile boundaries: build the C ABI libraries,
  header consumer and reference-runner library portion without executing device code; and
- native runner: execute the frozen four-rate RIFF plus representative RF64 corpus and compare exact
  block-planar size/SHA-256, diagnostics and atomic no-clobber behavior to its independent manifest.

Each row records `PASS`, `FAIL`, or `UNAVAILABLE` with exact toolchain/target identity; unavailable
is allowed only for a missing installed target/tool and may not hide a compile or product failure.
Cross-compiled object inspection must distinguish exports from imports/internal references and use
synthetic parser mutations.

## Adversarial evidence

Run non-timed representative source lifetime, repeated command/event/replay, source-changing and
source-preserving replacement, full-retirement deferral/reclamation, destroy-order, malformed C
input, one-short buffer/limit and one-million render/swap audit rows. Audit counts are functional,
not performance measurements. Prove no allocation/free, lock, syscall, I/O, log, callback or render-
thread destruction in armed render intervals. The runner's accepted-output lifecycle must remain
atomic/no-clobber on success and every injected failure.

The checker rejects changed product/fixture authorities, missing/extra symbols, target omissions,
false-positive object references, fabricated unavailable rows, stale artifacts and any timing or
benchmark entry. Shell/parser mutations must prove those rejects.

## Allowed paths and gates

Allowed changes are a new `tools/miso-engine-capi-qualification/**` package if needed,
`fixtures/capi-qualification/v1/**`, `docs/C_ABI_V1_QUALIFICATION.md`, new exact target/qualification
checker and mutation scripts, minimal manifests/lock rows, and this issue's evidence/routing docs.
Existing CAPI/source/protocol/session/graph/DSP code, installed headers, Issue-115 runner code and
all accepted fixtures are read-only.

Gates are locked compile/test for the qualification tool, warning-denied Clippy/rustdoc, installed
header/library and exact target matrix, fixture/manifest validation, realtime/policy mutations,
shell syntax and clean/static/diff scans. No benchmark target, timer, tuning, listening, browser or
device workload is allowed. Sol High hands off one immutable qualification checkpoint; Sol XHigh
returns strict PASS or the sole bounded HOLD. Overall PASS requires exact candid matrix evidence and
then unblocks Issue 026.
