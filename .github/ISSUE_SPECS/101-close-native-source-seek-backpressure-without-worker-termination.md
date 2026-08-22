# 101 Close native-source seek backpressure without worker termination

## Outcome and status

Close only the critical F1 correctness defect from the remote source-crate audit: a native source
worker must retain a provider seek that encounters the already-full one-slot render seek queue,
coalesce later accepted worker-local seeks deterministically, and continue running rather than
terminating with `NativeSourceWorkerExit::SeekFailed(SourceSeekError::Backpressure)`.

**SOL XHIGH READINESS PASS / READY FOR SOL HIGH PASS 1 / NO WORKLOAD, BENCHMARK OR TIMING
AUTHORIZED.** Sol High implements. Sol XHigh briefs and adversarially verifies. The budget is one
focused implementation pass plus at most one bounded HOLD correction; a second HOLD is terminal
STOP/rescope.

The local briefing baseline is clean `main` commit `e1fbbb65`, tree `d35388fc`. Remote Issue 101 is
OPEN under the audit title `101 Audit: miso-engine-source (worker lifecycle bug, spin, decoder)`.
Root must synchronize it to the exact title `101 Close native-source seek backpressure without
worker termination` and this stateless body after the docs checkpoint is committed and upstream.
This record claims no Git or GitHub mutation.

## Exact dependency and downstream route

- **Exact lock-free native source sanitation telemetry handoff** (Issue 043) is accepted and is the
  sole direct dependency.

Stopped Issue 010, **JIT PCM streaming and host-supplied source rings**, is transitive historical
input through accepted Issue 043, not a PASS dependency. After Issue 101 passes, Issue 073,
**Native PCM reference runner and C ABI qualification**, may consume the corrected native-source
worker. The exact route is `043 -> 101 -> 073`.

## Frozen current defect

At the briefing baseline:

- `NativeSourceController::try_seek` validates a strictly increasing, region-bounded generation,
  enqueues `WorkerCommand::Seek` into the already bounded worker-control SPSC, advances
  `next_requested_generation`, and returns success;
- the worker-control capacity is `NativeSourceCaps::control_queue_items`, while the prepared
  `PcmSourceProducer` has a distinct one-slot seek queue consumed only by
  `PcmSourceConsumer::observe_seek_at_block_boundary`;
- `run_worker` calls `apply_command`, which seeks the decoder and then calls
  `HostChunkProvider::try_seek`; and
- `SourceSeekError::Backpressure` from that provider call is currently mapped to terminal
  `NativeSourceWorkerExit::SeekFailed`, after which only a reason-free terminal event remains and
  future source output is underrun zero.

Thus two valid controller seeks accepted before render drains the provider seek slot can turn
ordinary bounded backpressure into permanent source death. The frozen source identities are:

| Path | SHA-256 |
| --- | --- |
| `crates/miso-engine-source/src/native_source.rs` | `06e9c51c515704b44cfa14d17720adc3f6233a3b94b0e6daa4b176abe68548d0` |
| `crates/miso-engine-source/src/lib.rs` | `b11e3c77f603156184c4f0b43832e7d68f13e03f7f92b82d8b6710ed91f15852` |
| `Cargo.lock` | `4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a` |

Line numbers from the remote audit were recorded at an older candidate; implementation must use
the named types/functions and re-read the live source rather than treating those line numbers as
authority.

## Smallest closable correction

Change only the native worker's private command-to-provider handoff:

1. Add one worker-local, fixed-size pending-seek value containing only generation and source frame.
   It is stack/state storage, not a queue or allocation.
2. At each worker turn, inspect at most the existing worker-command queue capacity. Preserve
   non-seek command behavior, but coalesce accepted seek commands by strictly increasing generation:
   the greatest generation observed in that bounded turn replaces any older worker-local pending
   seek. No unbounded drain is permitted.
3. Seeking the decoder may still fail terminally with the existing `DecodeFailed` variant. Once a
   seek is worker-local, clear any pending decoded block and end-of-region latch. Do not decode or
   submit PCM for that generation until the same seek has been accepted by
   `HostChunkProvider::try_seek`.
4. If provider delivery returns `SourceSeekError::Backpressure`, retain the exact pending seek and
   continue the worker. It emits no terminal event and does not advance the worker's active
   generation. A later accepted controller seek supersedes that retained value.
5. Retry the latest retained value only after bounded command observation. On provider success,
   update the worker generation, clear the pending seek, then resume decoding from its exact frame.
   Provider errors other than `Backpressure` retain their existing typed terminal mapping.
6. Stop remains prompt while a seek is pending; wake and sanitation-snapshot commands retain their
   existing bounded behavior. Worker retirement must join normally without requiring render to
   drain the seek queue.

Controller success continues to mean acceptance into the bounded worker-control queue. A seek that
has already crossed into the one-slot render queue cannot be withdrawn. If a later controller seek
is pending, the consumer may observe that earlier generation at one boundary, but no newly decoded
PCM for an intermediate worker-local generation may become audible. Once the slot is available, the
greatest accepted worker-local generation is the next delivered seek and the first newly decoded
audible PCM begins at its exact requested frame. Stale queued/data blocks remain rejected by the
existing generation and start-frame rules.

## Frozen interfaces, resources and diagnostics

Do not change any public type, method, enum variant, result mapping, queue capacity, resource report,
allocation layout, source-region rule or render-consumer algorithm. In particular:

- `NativeSourceController::try_seek` retains its validation, `Backpressure` result for a full
  worker-control queue, and strictly increasing generation contract;
- the provider/render seek queue remains capacity one and its reported bytes remain exact;
- `NativeSourceWorkerExit::SeekFailed` remains public and unchanged, but provider
  `SourceSeekError::Backpressure` must no longer reach it;
- invalid generation, out-of-region seek, decoder failure and genuine non-backpressure invariant
  failures retain their existing diagnostics; and
- render remains allocation/free, lock, I/O, logging and syscall free. The correction performs no
  work on render beyond the unchanged one-command boundary observation.

Any need to resize a queue, expose pending state, change telemetry/terminal-event shapes, change
WAV decoding, add a wake primitive, alter source resource accounting or edit another production
crate is STOP/rescope.

## Required focused evidence

Use deterministic in-memory readers and bounded test gates; no real file workload is needed.

1. Reproduce the former failure with the provider seek slot occupied and at least two newer valid
   seeks accepted by the worker-control queue before a render boundary. Prove no terminal event,
   no `SeekFailed(Backpressure)`, and a live worker.
2. Freeze a schedule with three increasing generations and unequal frames. Before provider
   acceptance, prove no decode/submit for any pending generation. After controlled boundary drains,
   prove intermediate decoded audio is never heard and the greatest accepted pending generation is
   the next newly decoded audible generation at its exact frame.
3. Cover a single seek, a later seek arriving while an older value is retained, controller-queue
   backpressure, wake and sanitation snapshot ordering, decoder failure, and stop/join while the
   provider slot remains full.
4. Prove old-generation queued PCM remains discarded, accepted-generation PCM is contiguous, and
   sanitation/end-of-region watermarks do not leak across the seek.
5. Prove the public API, queue capacities, exact resource report and `Cargo.lock` are unchanged;
   preparation-time allocation accounting is unchanged and the worker-local correction allocates
   nothing after preparation.

Focused source-package tests, warning-denied all-target/all-feature Clippy and rustdoc, format,
applicable source/realtime policies and mutations, static public/resource-diff checks, and
docs/title/dependency/diff sanity must pass. A clean workspace nonbenchmark seal may run once after
the focused checkpoint if root authorizes it. Real source workloads, benchmark binaries, timing,
fuzz execution, audio playback and listening are forbidden; their invocation counts remain zero.

## Allowed paths

- `crates/miso-engine-source/src/native_source.rs`, including colocated tests only;
- this spec and its tracked brief; and
- minimal exact Issue-101 routing in `.github/ISSUE_SPECS/README.md` and
  `docs/IMPLEMENTATION_PLAN.md`.

`crates/miso-engine-source/src/lib.rs`, public headers, Cargo manifests/lock, other production
crates, fixtures and unrelated scripts are read-only. If the correction cannot close within this
surface, stop and rebrief.

## Explicitly deferred audit findings

Remote Issue 101's F2 through F12 are preserved findings but are not acceptance gates or permitted
implementation in this tranche: idle/full-ring spinning, one-thread-per-source topology, decoder
read/seek and conversion performance, duplicate source functions, render-path copy/zero-fill,
arbitrary host-chunk geometry, broader WAV compatibility, host-PCM sanitation policy, resource-
accounting simplification, worker-event publication behavior, and late-block catch-up/telemetry.
Cross-crate duplication findings remain coordinated with Issue 083. Each needs a separate stateless
owner before implementation; none may be smuggled into Issue 101.

## Readiness decision

The defect is live, reproducible from the existing bounded queues, independent of external input,
and closable without public or resource-shape changes. **SOL XHIGH READINESS PASS / READY FOR SOL
HIGH PASS 1.** This docs-only decision authorizes no product edit until root commits the exact docs
checkpoint and synchronizes the issue workflow.
