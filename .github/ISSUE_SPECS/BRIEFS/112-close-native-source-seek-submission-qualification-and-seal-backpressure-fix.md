# Sol implementation brief — issue 112 seek submission qualification

## Decision

**SOL XHIGH READINESS PASS / READY FOR SOL HIGH PASS 1.** Sol High implements one test-only pass;
Sol XHigh adversarially verifies. One bounded pre-seal HOLD correction is the maximum second pass.
A second HOLD or failure of the sole broad seal is terminal STOP. No product, workload, benchmark,
timing, fuzz, playback or listening work is authorized.

Root must create/synchronize remote Issue 112 under the exact title `112 Close native-source seek
submission qualification and seal backpressure fix`; read-only inspection found the number
available. Accepted Issue 043, **Exact lock-free native source sanitation telemetry handoff**, is
the product dependency. Stopped Issue 101, **Close native-source seek backpressure without worker
termination**, contributes only frozen checkpoint `02bec81` / source SHA
`d8fd1762702a5b75a2943b8b99a45724e67afb7a82e5f63bff4f8bcb3f8aa98a`. Route
`043 -> 101 (stopped) -> 112 -> 073`.

## Sole correction

The coalescing test's final `sync_worker` is not a PCM-ready barrier: snapshot publication precedes
decode/submission. Convert that test to the existing test-support audit gate. Arm `AuditHold` while
generation 4 is still blocked behind the occupied provider seek slot; then perform the existing
generation-2 zero read. The first held acknowledgement proves frame20 was submitted. Read and assert
the exact generation-4 `[+0, 21, 22, 23]` block. `release_and_wait` then acknowledges only after the
next successful submission; read and assert contiguous final `[24, 25, 26, 27]` and end-of-region.
Retain the exact stale-discard and one-sanitation witnesses. Remove snapshot as readiness authority.

The enqueue-before-drain order is mandatory: it prevents generation-4 submission from racing ahead
of the hold. Add a static assertion/check that neither seek-continuation test uses snapshot return
as immediate authority for a nonzero PCM read.

## Frozen boundary

Only test/test-support portions of `crates/miso-engine-source/src/native_source.rs` may change.
`PendingSeek`, `run_worker`, all non-test behavior, public APIs, diagnostics, queue/resource shapes,
`src/lib.rs`, manifests and lock remain byte/semantically frozen. The existing repeatable gate is
technical input and needs no new behavior. F2–F12 and Issue 073 are forbidden.

## Evidence sequence

Run the exact all-feature corrected test, all all-feature source tests, locked source check, strict
Clippy/rustdoc, format, policies and static production/public/resource/hash/diff gates. Sol High
stops. Sol XHigh focused PASS permits a clean checkpoint.

Root may then authorize one fresh clean broad nonbenchmark seal only. It must retain the exact
workspace all-feature nonbenchmark test command from the spec and complete the remaining ordered
nonbenchmark gates without retry. The Issue-101 seal remains historical FAIL 543/1/8; it is not
reclassified or rerun. Initial Issue-112 seal and all real workload/timing counters are zero.

## Attempt boundary

One Sol High pass plus one bounded Sol XHigh HOLD correction maximum. Focused PASS is not overall
PASS; only the clean sole broad seal can close Issue 112 and unblock Issue 073. Any seal failure,
production diff or broadened scope is STOP/rescope.
