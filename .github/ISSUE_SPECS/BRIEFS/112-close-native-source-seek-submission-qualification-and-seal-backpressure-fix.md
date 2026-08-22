# Sol implementation brief — issue 112 seek submission qualification

## Decision

**COMPLETE / SOL XHIGH PASS / READY TO CLOSE.** Sol High completed one test-only pass and its sole
bounded HOLD correction. Sol XHigh verified the correction and the one authorized broad
nonbenchmark seal. No product, workload, benchmark, timing, fuzz, playback or listening work ran.
Issue 073 is unblocked locally; root owns upstream evidence synchronization and remote closure.

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
reclassified or rerun. Before authorization, the Issue-112 seal and all real workload/timing
counters were zero.

## Attempt boundary

One Sol High pass plus one bounded Sol XHigh HOLD correction maximum. Focused PASS is not overall
PASS; only the clean sole broad seal can close Issue 112 and unblock Issue 073. Any seal failure,
production diff or broadened scope is STOP/rescope.

## Final evidence and verdict

Pass 1 installed the explicit frame-20/frame-24 audit acknowledgements and static readiness checks.
Its first default-parallel run stalled for more than 60 seconds. Sol XHigh issued the sole bounded
HOLD because assertions could panic while the worker was held and then hang during worker
drop/join. The correction moved all held-risk assertions after release and added static enforcement.
It also reduced only the coalescing test's valid ring to one quantum after a serial run exposed the
two-slot generation-2 PCM race. Exact zero generation-2 output, stale count one, generation-4
frames 20/24, sanitation, continuation, end and worker liveness are deterministic. The final exact
test passed 1/1; all-feature source tests passed 35/35 in serial and default-parallel modes; focused
format/check/Clippy/rustdoc and workspace/realtime policies passed. Sol XHigh returned strict
focused PASS.

Clean checkpoint `1ed2634ea8fed79e0ededcdf931cdc831a1e5daf`, tree
`75fdc11e7aa543be8c5102a57a58466bb324e050`, freezes:

- native source `f969d2a6175643e6b8b9f1aec14a09aeec6c950dbdea5136a7248679406e3770`;
- production/test-support prefix `68d6d7bb4d7880de7ba552a2bb34cb5a5e93fa25ae8cd15375b9423b7c41c363`;
- `lib.rs` `b11e3c77f603156184c4f0b43832e7d68f13e03f7f92b82d8b6710ed91f15852`;
- `Cargo.lock` `4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a`.

The sole ordered broad seal passed: format; locked workspace all-target/all-feature check; exact
`CARGO_BUILD_JOBS=1 cargo test --workspace --all-features --locked --lib --bins --tests --examples`
with 569 passed, 0 failed and 8 expected ignored/manual rows across 87 groups; doctests 8/0 across
29 groups; warning-denied workspace Clippy/rustdoc; workspace and realtime policy
baselines/mutations; syntax for 111 shell scripts; and final clean/static/diff scans. Test and
doctest transcript SHA-256 values are respectively
`70ae09e114b689adcd571693535d53f52ff55866195f049b894bb4ef6fa8fff7` and
`b1ffbb94781a64ae28451b70f9ceaf771da5a9fbebbfeef77571d13611236e79`.

Issue 101's historical 543/1/8 seal remains FAIL and was not retried. Issue-112 seal count is one;
all prohibited workload/benchmark/timing/fuzz/audio/listening/browser-main/audit-main counts remain
zero. No execution artifact exists. **FINAL SOL XHIGH PASS / COMPLETE / READY TO CLOSE.** Route
`043 -> 101 (stopped technical input) -> 112 (PASS) -> 073`.
