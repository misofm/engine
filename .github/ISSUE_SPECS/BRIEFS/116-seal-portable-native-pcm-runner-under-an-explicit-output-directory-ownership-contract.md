# Sol implementation brief — issue 116 explicit-directory portable runner seal

## Decision

**COMPLETE / SOL XHIGH PASS / READY TO CLOSE.** Stopped Issue 115's held-handle checkpoint was
completed under one explicit and honest precondition: the caller exclusively owns the output
directory for the complete invocation. The result makes no identity-conditional unlink claim
against a concurrent same-privilege directory mutator. Sol High implemented; Sol XHigh briefed,
issued one bounded HOLD and verified its exact correction. The sole fresh functional seal passed
without retry or edit. Benchmark/timing/real-workload/playback/listening counts are zero.

Accepted dependencies are **Stable C ABI and host-fed planar PCM render** (022) and **Close native-
source seek submission qualification and seal backpressure fix** (112). Stopped **Close portable
native PCM runner publication and seal the reference tool** (115), checkpoint `99f081f`, contributes
technical bytes only. Accepted 116 plus **Close C ABI control/event transport and transactional plan
replacement** (113) gates Issue 114.

## Smallest closable correction

Document the existing-directory exclusive-ownership precondition: no other actor mutates any entry
in the output directory from invocation entry through return. Under that boundary, finish the
shared bounded publication state machine and explicit Linux/Android, Apple and Windows adapters.
Keep create-new partials, exact held-handle verification, atomic no-replace publication, exact
post-publication identity, owned cleanup, final-sentinel preservation and one accepted final. An
unsupported adapter fails before output/source/engine work.

Linux/Android may retain `linkat(AT_EMPTY_PATH)` or use a proved `renameat2` no-replace design;
Apple uses a documented exclusive `renamex_np` or held-handle `linkat` strategy; Windows uses
retained `HANDLE` identity and `FileLinkInfo` with replacement disabled. No generic pathname
hard-link fallback, retry, lock, CLI/API/resource change or concurrency-safety overclaim.

Tests cover every nonconcurrent phase-boundary substitution and collision kind, wrong identity,
write/flush/sync/length/digest/publication/cleanup/final-check failures, unsupported preflight,
second-call no-clobber and exact success. Fakes must state rather than conceal the excluded
interstitial same-privilege mutation. Static mutations reject any restoration of the impossible
Issue-115 guarantee.

## Frozen input and fence

Technical checkpoint/tree: `99f081f327bd250343a80f928aa099c994ca8e59` /
`8f883e6dbbb31a4dd7350239c81a40466560e7e2`. Freeze runner manifest
`bd5cd87f0c2bcd0ae5e7faf5532b1869b39e72f296dee81b2f135345e728a8e1`, CLI
`89b2acb0f56c6e249bbbd7bcb965c7c0545fa3c6078772b5026362ec6b888dd0`, fixture manifest
`8d251ad6b1eca8c95e24b8b4e2959e397d8ec954502307351f1c7fb3c01a9634`, C header
`e7ba468361e0255cb465828c5dd317f1e5293213662c7bf9a5225cb2afaba4e7` and lock
`c89b195f0d31ad21852d0a931023c70e1eb4a0caa534bfd6e1692c1e1178fd52` unless a minimal binding
change is explicitly proved. CAPI/source/protocol/session/graph/DSP, decode/feed/render, fixtures,
digests, CLI and diagnostics are immutable.

Edit only runner publication/tests, its exact contract doc/checkers/mutations, unavoidable minimal
binding manifest/lock rows, and issue routing/evidence. No product, fixture or `src/main.rs` change.

## Evidence sequence

Run complete runner tests and fixture oracle, C ABI check, locked host check, strict
Clippy/rustdoc/fmt, policies/mutations/static scans and exact Linux/Windows/Apple library compile
rows. Sol High stops. Strict Sol XHigh focused PASS permits one clean checkpoint and one fresh full
functional runner seal. First failure stops; no retry, benchmark, timing or real workload. Only the
clean seal can yield overall PASS and unblock Issue 114.

## Final verdict and immutable evidence

Implementation commit/tree: `45f8f5af8bdd578b5ccb27fdb787f7a663c39818` /
`7e0a7b7d48362c9b9eaa15b1cfce7180c935c5b5`. Final hashes are runner library
`a1395b95f9cab07ec516fd0da2583d7cb3e0083613ed86120cd51b93f4ed805a`, contract
`ac80fc2112d3a060a0061eeb3e3db8f4c97aa1bfe475f38f98e49b830a09efc6`, checker
`596f99c66e2a5398bbcf534cfd850b8e5786f55ccbfef6376f7b9111d25a2f42`, and mutations
`1e436ad55eabe45425a9bf0563065b05ca426d216dd680dc850a74b3c08cb3ad`.

The focused HOLD corrected known-unowned fake cleanup and replaced a marker mutation with an actual
production final-guard broadening that the checker rejects. Sol XHigh then returned strict focused
PASS. The sole fresh full functional nonbenchmark seal ran once on the clean immutable candidate:
18/18 library tests passed with zero binary/fail/ignored/measured; all five 8,192-byte fixture
digests matched the frozen manifest; generator, C ABI, locked check, strict Clippy/rustdoc,
portability/original policies and mutations, workspace/realtime policies and mutations, shell,
static/frozen/clean scans, and exact Linux/Windows/Apple library compile rows all passed.

Runner manifest `bd5cd87...a8e1`, CLI `89b2acb0...8dd0`, fixture manifest
`8d251ad6...9634`, C header `e7ba4683...4e7`, and lock `c89b195f...fd52` remained frozen. All
benchmark/timing/real-workload/playback/listening counters are zero. Issue 116 has overall **SOL
XHIGH PASS**, is **COMPLETE / READY TO CLOSE**, and unblocks Issue 114 with accepted Issue 113. No
further runner or seal invocation is authorized. Root retains upstream evidence synchronization and
remote closure.
