# 112 Close native-source seek submission qualification and seal backpressure fix

## Outcome and status

Close the sole test-only synchronization defect left by stopped Issue 101, preserve its native
pending-seek production bytes and semantics, then run one fresh clean broad nonbenchmark seal on the
successor candidate.

**SOL XHIGH READINESS PASS / READY FOR SOL HIGH PASS 1 / NO PRODUCT OR REAL WORKLOAD CHANGE.** Sol
High implements. Sol XHigh briefs and adversarially verifies. The budget is one focused pass plus at
most one bounded HOLD correction before the seal. A second HOLD, any product change or any failure
of the sole successor broad seal is terminal STOP/rescope.

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

Initial Issue-112 broad-seal invocation count is zero. Real source workload, benchmark, timing,
fuzz, playback and listening invocation counts are also zero and must remain zero. No persistent
`target/issue112` artifact or execution namespace is required.

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
