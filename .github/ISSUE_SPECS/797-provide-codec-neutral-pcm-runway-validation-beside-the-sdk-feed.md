# Provide codec-neutral PCM runway validation beside the SDK feed

Boundary correction sibling of misofm/engine#796. Downstream:
misofm/engine-web-adapter#101. This does not block engine misofm/engine#794,
adapter misofm/engine-web-adapter#95 or app misofm/app#210's plotting delivery.

## Outcome and source facts

A direct PCM producer can await a bounded, contiguous runway at an acknowledged
source generation/frame through `@misofm/engine/browser`. No codec or storage
adapter is required. This moves an existing generic algorithm, not source I/O.

Inspected engine main `69c268f240bf30b2a43b43dd521120cd89dcc0b9` already owns
`sdk/src/browser/pcm-ring.ts`, `pcm-feed.ts` and `scratch.ts`. Adapter main
`f833303f146de7cbe1705fe88ae68a6d6e0d4e45` owns `src/session.ts::waitForRunway`:
it reads SDK ring capacities, uses `Msb1RingObserver`, checks generation and
contiguous frames, and waits with a two-second deadline. `waitForPrefill` and
`waitForSeekPrefill` supply opening/seek policy and adapter error translation.
These are source facts; no runtime regression or new test result is asserted.

## Frozen scope

1. Export `waitForPcmRunway(options): Promise<void>` beside the existing feed.
   Options: `sources: readonly { sourceId: string; frames: bigint;
   ring: SharedArrayBuffer }[]`, `targetFrame: bigint`, `generation: bigint`,
   `timeoutMs: number`, optional `minimumFrames: bigint`, and optional
   `signal: AbortSignal`. `frames` is each source's declared total. An omitted
   minimum selects that ring's full capacity in frames (`frameCapacity × capacity`), as the adapter does today;
   an explicit positive minimum cannot exceed any relevant ring's capacity.
   Cap the required end at EOF; sources already at/past EOF need no PCM.
2. Validate options and bind valid existing SDK rings before waiting. Require
   unique nonempty source IDs, nonnegative frame/target values, positive
   generation/minimum within existing SDK ring representable domains, and finite positive timeout. Use `MisoUsageError` for
   invalid requests. Snapshot the small source/options structure before awaits.
   Use bigint frame arithmetic, existing ring validation and observer semantics;
   never substitute the low-word generation tag for the full generation.
3. Own one temporary SDK observer per relevant source for this call and close
   every observer in `finally`. Observe without advancing consumer read indices
   or changing producer/seek state. Pull at most the existing 32 chunks per
   source per polling turn, then yield. Preserve contiguous start/end checks,
   full generation equality and source-bound checks. Return only when every
   relevant source has supplied the required run; no partially ready ack.
4. Export `PcmRunwayError` with `reason: "mismatch" | "timeout"` and optional
   `sourceId`; use it for received wrong/noncontiguous/out-of-range PCM or
   deadline expiry. Preserve the caller's abort reason. Reuse ordinary SDK
   control-thread waits; cancellation and errors release observer resources.
5. The producer/composition must serialize seeks, obtain the producer seek ack,
   and complete existing `feed.prepareSeek()` before checking the new runway.
   It selects minimum/deadline, does reads/refills, suspends/resumes context and
   owns aggregate failure cleanup. This helper neither renders nor resumes audio,
   seeks a producer, attaches a feed, waits for network, or closes caller objects.
   Readiness is a proof under that choreography, not a new queue reservation.

## Files and focused gates

Change `sdk/src/browser/pcm-feed.ts` (or a small adjacent module exported there),
browser exports and README. Extend `sdk/test/browser-pcm-evals.mjs`; explicitly
provenance useful adapter source/tests to the SHA above. Do not edit adapter
production under this issue.

Use the existing ring writer/observer fixtures to prove: full default runway;
smaller explicit minimum; short-source EOF and past-EOF handling; two sources
where only one is ready; wrong full generation with matching low tag;
noncontiguous/out-of-range chunks; timeout; abort; and invalid request refusal.
Prove consumer indices/seek state remain unchanged and a subsequent wait works
after cancellation. Reuse existing fake-clock seams where available; no new
timing framework. Run focused PCM tests, SDK types/generated-surface checks and
the existing package import smoke. Keep existing feed-attach/prepareSeek cases
green; no browser/DSP matrix, benchmark or new ring protocol.

## Delivery and bounds

Luna XHIGH implements; a fresh Astra MEDIUM verifies and may fix concrete bugs
within scope only. A separate fresh Astra MEDIUM coordinates. Maximum five
coherent attempts, one verdict per attempt, then stop/rebrief if still failing.
Checkpoint the focused-green helper before more work. Root integrates current
main and closes the synchronized local/GitHub source issue after independent
PASS, merged delivery, required CI and upstream evidence; verify remote CLOSED.

If ready before misofm/engine#794's frozen qualification cut, it may share that publication;
misofm/engine#794 must not wait for it. Otherwise record its exact registry release as a
dependency in adapter misofm/engine-web-adapter#101 using the next ordinary bounded release
issue. No unnecessary standalone publish is required to close this source slice,
and source closure must not be reported as registry availability. No codec,
storage, float/source-policy expansion or optimization is included.

Decision record: scoped only; no fresh execution evidence yet.


## Implementation checkpoint

Luna XHIGH implemented the bounded SDK helper, typed mismatch/timeout errors,
request validation, full-generation contiguous readiness checks and observer
cleanup in the existing PCM feed module. Existing browser exports expose its
public types/function. README records producer choreography and adapter source
provenance. Focused PCM suite 18/18, locked dependency install, SDK type/generated
checks, artifact-backed package/import smoke and `git diff --check` pass. Logs
are preserved at `/tmp/miso-796-audit/797-*`. Root checkpoints the exact helper,
existing test and README paths before independent fresh Astra MEDIUM review.
This is attempt 1, not source acceptance or registry availability. It runs in
an isolated worktree during #796 review/CI and cannot delay the plotting release.

## Independent verification — attempt 1

Fresh Astra MEDIUM records **PASS** after correcting one concrete bug: an
explicit `AbortController.abort(null)` reason was replaced by the nullish fallback.
Checkpoint `7e08e956` preserves it exactly; the existing cancellation case proves
in-flight and pre-aborted null identity and subsequent successful reuse. No
scope expansion or further implementation correction was needed.

Independent gates pass: focused PCM 18/18 (including feed attach/prepareSeek),
SDK types/host mirror, artifact-backed package check with generated surface and
package smoke, browser runway exports, and diff whitespace check. Reused
`/tmp/issue793-candidate1-artifact`; no Wasm rebuild or benchmark. Reviewed bigint
domains, full-generation equality, bounded contiguous/EOF proof, all-source
readiness, observer/finally and abort-listener cleanup, and unchanged consumer/seek
state. Full verdict and logs: `/tmp/miso-796-audit/verify-797-attempt1.md` and
`verify-797-{pcm,types,package,import}.log`.

Source integration, required CI and GitHub synchronization remain root delivery
steps. This PASS does not assert registry availability and does not delay #794.
