# 112 Close native-source seek submission qualification and seal backpressure fix

## Outcome and status

Close the sole test-only synchronization defect left by stopped Issue 101, preserve its native
pending-seek production bytes and semantics, then run one fresh clean broad nonbenchmark seal on the
successor candidate.

**COMPLETE / SOL XHIGH PASS / READY TO CLOSE.** Sol High implemented one focused pass and its sole
bounded HOLD correction; Sol XHigh adversarially verified both and the one authorized broad
nonbenchmark seal. No product behavior, public/resource shape or real workload changed. Issue 073
is unblocked by this local acceptance; root still owns upstream evidence synchronization and remote
closure.

Remote read-only inspection on 2026-08-22 found no Issue 112, so the number is available. Root owns
creation and synchronization under the exact title `112 Close native-source seek submission
qualification and seal backpressure fix` after this docs checkpoint is committed and upstream.
This record claims no Git or GitHub mutation.

## Dependencies and technical input

- **Exact lock-free native source sanitation telemetry handoff** (Issue 043) is the accepted product
  dependency.
- **Close native-source seek backpressure without worker termination** (Issue 101) is stopped, not
  PASS. Its `02bec81` one-file checkpoint is permitted only as frozen technical input.

The route is `043 -> 101 (stopped) -> 112 -> 073`. Issue 073, **Native PCM reference runner and C
ABI qualification**, may consume the pending-seek correction only after Issue 112 passes.

## Frozen predecessor evidence

Issue 101 implemented a private fixed-size pending seek, bounded greatest-generation command
coalescing, nonterminal retention of provider `SourceSeekError::Backpressure`, provider acceptance
before decoder output, exact latest-frame resume, stop handling and unchanged public/resource
shapes. Sol XHigh found no product defect. Preserve these identities:

| Path | SHA-256 |
| --- | --- |
| `crates/miso-engine-source/src/native_source.rs` | `d8fd1762702a5b75a2943b8b99a45724e67afb7a82e5f63bff4f8bcb3f8aa98a` |
| `crates/miso-engine-source/src/lib.rs` | `b11e3c77f603156184c4f0b43832e7d68f13e03f7f92b82d8b6710ed91f15852` |
| `Cargo.lock` | `4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a` |

The Issue-101 broad candidate `dfdefff` / tree `49f03a` passed format and locked workspace
all-target/all-feature check, then stopped its nonbenchmark test selector with 543 passed, 1 failed
and 8 ignored. No broad retry occurred. The bounded test correction at `02bec81` passed 34/34
all-feature source tests, but final Sol XHigh review found the same false synchronization primitive
in a second test. These are historical facts, not Issue-112 invocations or PASS evidence.

## Sole remaining defect

In `worker_coalesces_provider_backpressure_to_latest_exact_frame_without_intermediate_pcm`, after
the generation-4 frame-20 block is consumed, the test calls `sync_worker` once and immediately
expects the final frame-24 block. `sync_worker` waits for `SnapshotSanitation`, which is published
during worker command observation before pending PCM submission or the later decode in that turn.
The snapshot can therefore return before `[24, 25, 26, 27]` is queued; a zero read would advance the
consumer and make the valid block stale. A green run is scheduling luck, not a readiness proof.

## Exact test-only correction

Use the already-repeatable `#[cfg(feature = "test-support")]` audit gate as an explicit successful-
submission handshake:

1. Run the coalescing test through `prepare_native_source_with_audit_gate`; its production request,
   generations 2/3/4, unequal frames 8/13/20, NaN/Inf sanitation witness and PCM assertions remain
   unchanged.
2. While generation 4 is retained behind the occupied provider seek slot, enqueue one `AuditHold`
   before the `while_pending` render read. Because the slot is still occupied, the worker cannot
   submit generation-4 PCM before observing that command.
3. Perform the existing `while_pending` read, proving generation 2 and positive-zero underrun while
   freeing the provider seek slot. Then wait for the audit `held` acknowledgement. It is emitted
   only after the exact generation-4 frame-20 block has been submitted.
4. Read and assert `[+0, 21, 22, 23]`, active generation 4, four copied frames and not-end. Release
   the held worker with `release_and_wait`; its resumed acknowledgement is emitted only after the
   next successful submission, which must be the contiguous final frame-24 block.
5. Read and assert `[24, 25, 26, 27]`, generation 4, four copied frames and end-of-region. Preserve
   the exact stale-discard and single-sanitation watermark assertions.
6. Remove the final snapshot-as-PCM-readiness call. A static test-source check must reject any
   `sync_worker`/snapshot helper used as the immediate readiness authority for a following nonzero
   PCM assertion in either seek-continuation test.

The test may be gated on `test-support` because the mandatory focused and broad commands use all
features. Do not weaken or delete any generation, frame, PCM, sanitation, stale-discard,
contiguity, end-of-region, worker-liveness or terminal assertion.

## Immutable production boundary

No non-test production behavior may change. The Issue-101 private `PendingSeek`, `run_worker`,
provider admission, decoder ordering, stop/wake/snapshot behavior, public APIs/enums, queue
capacities, resource/allocation reports, `src/lib.rs`, manifests and `Cargo.lock` are frozen.

Within `crates/miso-engine-source/src/native_source.rs`, edits are limited to the coalescing test,
its `test-support` annotation/setup, and an audit-gate test-only assertion/helper only if required.
The existing repeatable `release_and_wait` behavior is already accepted technical input and needs no
further semantic change. Any production diff, queue/resource change, sleep/wake work or F2–F12 audit
work is STOP.

## Focused gates and sole seal

Before checkpoint, run the exact all-feature coalescing test, all source-package all-feature tests,
locked all-target/all-feature source check, warning-denied Clippy and rustdoc, format, applicable
source/realtime policies and static production/public/resource/hash/diff checks. Sol High then stops;
Sol XHigh focused PASS authorizes a clean exact-path checkpoint only.

After that committed candidate is independently verified clean, root may authorize exactly one
fresh broad nonbenchmark seal. It includes format; locked workspace all-target/all-feature check;
the exact retained-session command
`CARGO_BUILD_JOBS=1 cargo test --workspace --all-features --locked --lib --bins --tests --examples`;
workspace doctests; warning-denied workspace all-target/all-feature Clippy and rustdoc; applicable
policy baselines/mutations; and clean/static/diff scans. Benchmark targets and ignored/manual tests
are not executed. The first nonzero exit stops the seal. There is no retry under Issue 112.

Before seal authorization, the Issue-112 broad-seal invocation count was zero. Real source
workload, benchmark, timing, fuzz, playback and listening invocation counts were also zero and had
to remain zero. No persistent `target/issue112` artifact or execution namespace was required.

## Allowed paths and deferred work

- `crates/miso-engine-source/src/native_source.rs`, test-support/test portions only;
- this spec and its tracked brief; and
- minimal exact route/status updates in `.github/ISSUE_SPECS/README.md` and
  `docs/IMPLEMENTATION_PLAN.md`.

Issue 101's deferred F2–F12 findings remain outside scope, including worker spin/topology, decoder
performance, duplication, copy geometry, WAV breadth, sanitation policy, accounting and telemetry.
No Issue-073 runner/provider/platform implementation begins here.

## Acceptance

Issue 112 passes only if Sol XHigh verifies the deterministic explicit-submit test evidence and the
sole fresh broad nonbenchmark seal completes on one clean immutable candidate. Focused green alone
is not overall PASS. No benchmark, performance, workload or human claim is possible.

## Final implementation and acceptance evidence (2026-08-22)

Sol High pass 1 changed only the test/test-support portion of
`crates/miso-engine-source/src/native_source.rs`. It replaced the coalescing test's final snapshot
readiness proxy with the audit-gate acknowledgements and added nonvacuous source checks for all four
nonzero continuation reads. One default-parallel focused run stalled for more than 60 seconds and
was interrupted. Although later serial and default-parallel runs passed, Sol XHigh returned the
sole bounded HOLD: assertions executed while the audit gate held the worker, so assertion unwind
could block in worker drop/join and mask a mismatch as that observed hang.

The correction captured every held-risk read/report, released the corresponding hold, and only then
evaluated assertions. Its static rows cover the single-seek initial, while-held and first-block
reads plus the coalescing while-pending and latest-block reads; the original readiness rows still
require a gate acknowledgement immediately before each of the four nonzero reads. During the
correction, a first default-parallel run passed 35/35, then a serial run exposed generation-2 PCM
`[8, 9, 10, 11]` occupying the second transfer slot instead of the intended zero. The final test
uses the exact valid one-quantum `frame_capacity = 4`: the sole old block is discarded, the
generation-2 boundary read is deterministically zero with exact stale count one, and held/resumed
acknowledgements prove generation-4 frames 20 and 24 before their reads. The final exact selector
passed 1/1 and the all-feature source suite passed 35/35 in both default-parallel and serial modes.
Format, locked source check, warning-denied Clippy/rustdoc and workspace/realtime policy baselines
and mutations passed. Sol XHigh then issued strict focused PASS.

The committed clean candidate is `1ed2634ea8fed79e0ededcdf931cdc831a1e5daf`, tree
`75fdc11e7aa543be8c5102a57a58466bb324e050`. Its frozen identities are:

| Authority | SHA-256 |
| --- | --- |
| `crates/miso-engine-source/src/native_source.rs` | `f969d2a6175643e6b8b9f1aec14a09aeec6c950dbdea5136a7248679406e3770` |
| production/test-support prefix, lines 1–1350 before `#[cfg(test)]` | `68d6d7bb4d7880de7ba552a2bb34cb5a5e93fa25ae8cd15375b9423b7c41c363` |
| `crates/miso-engine-source/src/lib.rs` | `b11e3c77f603156184c4f0b43832e7d68f13e03f7f92b82d8b6710ed91f15852` |
| `Cargo.lock` | `4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a` |

Root authorized and executed the sole broad nonbenchmark seal on that immutable candidate. In
order, format and locked workspace all-target/all-feature check passed; exact command
`CARGO_BUILD_JOBS=1 cargo test --workspace --all-features --locked --lib --bins --tests --examples`
passed 569 tests, failed 0 and retained 8 expected ignored/manual rows across 87 result groups;
workspace doctests passed 8/0 across 29 groups; warning-denied workspace all-target/all-feature
Clippy and warning-denied workspace all-feature rustdoc passed; workspace and realtime policy
baselines/mutations passed; syntax validation passed for 111 shell scripts; and final clean
HEAD/tree/index/worktree, conflict-marker, trailing-whitespace and artifact scans passed. Preserved
transcript hashes are:

- workspace tests: `70ae09e114b689adcd571693535d53f52ff55866195f049b894bb4ef6fa8fff7`;
- doctests: `b1ffbb94781a64ae28451b70f9ceaf771da5a9fbebbfeef77571d13611236e79`.

Issue 101's historical 543-pass/1-fail/8-ignored seal remains strict FAIL and was not retried or
reclassified. Issue-112 broad-seal invocation count is exactly one. Real source workload,
benchmark, timing, fuzz, audio/playback, listening, browser-main and audit-main invocation counts
are all exactly zero. No execution artifact was created. **FINAL SOL XHIGH PASS / COMPLETE / READY
TO CLOSE.** The accepted route is now `043 -> 101 (stopped technical input) -> 112 (PASS) -> 073`.
