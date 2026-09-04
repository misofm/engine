# Keep browser PCM quantum-shaped across windowed seeks

## Objective

Make the generic browser `CanonicalPcmPump` resume from every valid absolute
source frame without emitting an illegal short, nonterminal chunk when that
frame is not aligned to its private bounded read window.

## Baseline and cause

The live eight-stem Ghost mixer reproduces permanent silence after seeking.
All source seeks are accepted, the Worker continues writing, and the audio
thread continues rendering, but every post-seek source submission is refused.

`CanonicalPcmPump.#writeOne` currently clips every output chunk both to the
engine quantum and to the end of its current read-ahead window. An arbitrary
seek can begin partway through that window. The first post-seek chunk is then
shorter than the prepared quantum without being the end of the source region.
The engine correctly refuses it. Because the drain advances past a malformed
submission, every later chunk is non-contiguous with the engine's expected
seek frame and is refused too.

The read window is private I/O policy. It must never change the public PCM
chunk shape.

## Smallest closable slice

Require `windowFrames >= ring.frameCapacity` for every mounted source. Compute
the next output size independently as `min(frameCapacity, sourceFrames -
cursor)`. Reuse a cached window only when it contains that complete half-open
frame interval; otherwise load a new window beginning exactly at `cursor` and
ending at `min(sourceFrames, cursor + windowFrames)`. Output chunks therefore
remain exactly `frameCapacity`, except for the one legal end-of-region tail.
The maximum cached bytes remain bounded by `windowFrames * frameBytes`; no
codec, transport, OPFS, URL, or application policy enters the pump.

Correct the existing pump transcript from the illegal `[3 nonfinal, 1
nonfinal, 1 final]` shape to `[3 nonfinal, 2 final]`. Add a multichannel 24-bit
regression that seeks to an unaligned frame and records slice ranges, commits,
and sample content. It proves the first and all subsequent nonterminal chunks
retain the configured frame capacity, carry the new generation, start
contiguously at the requested frame without stale samples, and stay within the
one-window memory bound through later reloads.

### Allowed paths

- `.github/ISSUE_SPECS/351-keep-browser-pcm-quantum-shaped-across-windowed-seeks.md`
- `hosts/host-web/web/stem-store/pcm-pump.js`
- `hosts/host-web/tests/stem-pump-v1.mjs`

No other tracked engine path may change.

### Forbidden scope

- application readiness/UI changes;
- FLAC, WebCodecs, R2, HTTP, OPFS layout, or delivery metadata policy;
- render-thread, Rust engine, ABI, Session V1, DSP, or generated artifact
  changes;
- unbounded reads, whole-stem PCM materialization, implicit sample-rate
  conversion, or relaxing the engine's fixed-quantum source contract; and
- unrelated stem-store cleanup or dependencies.

## Objective gates

1. A seek to a frame unaligned with both `windowFrames` and `frameCapacity`
   emits a first chunk beginning at that exact frame with `frameCapacity`
   frames when the source has at least that many frames remaining.
2. Every following nonterminal chunk is contiguous and exactly
   `frameCapacity`; only the final end-of-region chunk may be short.
3. Every post-seek chunk carries the new generation and no pre-seek cached
   window bytes are reused.
4. Every `Blob.slice` is no larger than `windowFrames * channels *
   bytesPerSample`, begins and ends on a whole-frame byte boundary, and never
   ends past `sourceFrames * frameBytes`, including after an unaligned seek.
5. Construction rejects `windowFrames < ring.frameCapacity` before engaging
   any writer. Existing 16-bit/24-bit conversion, backpressure, failure
   teardown, and worker-pump tests remain green after the existing pump oracle
   is corrected to `[3 nonfinal, 2 final]`.
6. The focused host-web JavaScript gate, formatting, workspace policy, and
   `git diff --check` pass; the exact-path diff contains only allowed files.
7. As the cross-repo integration/closure gate, after vendoring into the app, a
   real Chrome Ghost seek shows accepted PCM
   submissions advancing after the seek with no growing refusal/underrun
   counters and audible metering resuming.

## Review and delivery

This is one generic SDK/host PCM-pump correctness slice. Sol briefs it before
implementation, Terra implements attempt 1, and Sol adversarially reviews the
result. Keep the engine checkpoint independent from the app's separate
generation-aware readiness correction. Synchronize and close the matching
GitHub issue only after the evidence commit is upstream.

## Brief evidence and decision record

Sol required and the brief adopted five corrections: enforce the necessary
window/quantum size invariant; define full-interval reuse and cursor-anchored
reload precisely; correct the existing illegal transcript; prove byte offsets,
generation, continuity, and samples with multichannel 24-bit PCM; and bound
slice alignment/end as well as length. The serialized Worker already excludes
seek/read races, so concurrency is deliberately unchanged. Sol approves this
corrected smallest slice for implementation.

## Implementation and Sol review evidence

Pending.
