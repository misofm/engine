# Compare bounded lossless sparse-stem storage prototypes

## Outcome and boundary

Build external research prototypes and produce a reproducible comparison of dense and sparse stem storage, exact PCM reconstruction, random access, and bounded decoder resources. This is a decision experiment, not an engine implementation or delivery-codec addition. The user authorized real-corpus proof of concepts and chose to align file beginnings for the activity model.

All implementation, audio, dictionaries, detailed corpus identities, raw records and plots remain in the private artist-agent repository or local private artifact storage. Public evidence is limited to generic methods, aggregate results and limitations. No artist identity, filename, source hash, private path, recoverable audio or dictionary bytes may enter this issue/repository.

## Sol-approved frozen experiment

Use the replacement equal-length stereo PCM24/44.1-kHz corpus. Preserve input bytes. Omit only exact-zero runs of at least 882 sample frames (20 ms); there is no amplitude threshold, gain change, fade, resampling or time shift.

Compare original dense WAV, dense FLAC, dense independent-frame Zstd, sparse WAV chunk files, sparse packed PCM, sparse FLAC chunks, and sparse independent-frame Zstd with/without one shared dictionary. Primary chunks contain at most 44,100 frames; the sole sensitivity is the same sparse-Zstd pair at 11,025 frames. Zstd level is 3. Dictionary capacity is 65,536 bytes; freeze a deterministic 20/80 stem split, bounded training samples and fixed trainer parameters before execution. Report training and held-out bytes separately; a split within one song is not cross-song evidence. Count every manifest, header, index and dictionary in storage.

A canonical sparse manifest binds source shape, original PCM identity, sample intervals, codec units, sizes, offsets and payload identities. The Zstd variant embeds the manifest in a skippable frame; other variants retain equivalent metadata. Chunks remain independently seekable with the fixed dictionary supplied where required.

## Objective gates

- Validate corpus shape and identities; confirm padding preserved original PCM prefixes.
- Reconstruct every variant exactly to the original packed PCM bytes and full duration.
- Verify one frozen common seek schedule including starts, ends, random locations and silence boundaries.
- Reject malformed manifests, invalid ranges/lengths, truncated/corrupt payloads, missing/wrong dictionaries and expansion beyond declared caps. Independently readable unaffected chunks remain testable.
- Synthetic fixtures discriminate exact silence, least-significant nonzero samples, minimum-gap boundaries, all-silent/all-active data, partial chunks and boundary-crossing seeks.
- Bound manifests, entries, dictionaries, decoder units and output space. Stream sources; no complete decoded stem/corpus residency. Record logical retained buffers separately from process RSS.

## Measurement and delivery

Freeze workload, parameters, input/seek manifests, result schema and validator before timing. Preflight arguments, output persistence/flushes, schema, failure exit semantics and overwrite refusal with tiny synthetic controls and zero timed-workload invocations.

Run exactly one benchmark invocation: one warmup, two measured rounds, fixed forward/reverse variant ordering, no tuning or rerun. Preserve flushed raw JSONL records, including failures. Report stored bytes/overheads, encoding and dictionary training cost, full reconstruction/hash cost, local warm-filesystem seek/read/decode amplification, and peak RSS. Report both rounds. Post-workload reporting defects use a bounded tooling successor and do not authorize another timed run.

Sol supplied the initial brief; Terra implements attempt 1, pauses at a focused-green checkpoint for root's exact-path commit/push; Sol adversarially reviews the evidence. Maximum five attempts, with rebriefing for any workload or boundary change. Keep the public issue/spec synchronized and close only after PASS and upstream evidence.

This evidence makes no claim about realtime DSP savings, audio callback deadlines, browser/mobile performance, cold storage, HTTP, Walrus networking/prices, float PCM, other rates or other songs. Production integration requires a separate issue.

## Status

Sol scope PASS recorded. Root verified existing numbered local specs have remote counterparts and prepared isolated public-record and private-prototype branches. The private prototype implementation is upstream; real-corpus qualification and timing remain pending.

## Implementation checkpoint 1

The private codec tranche is committed and upstream. Synthetic PCM24 reconstruction, sparse windows, a Zstd payload corruption control, invalid decoded-length rejection, create-new output behavior, and Python compilation pass. Zstd content frames spool to a file. All-corpus qualification, complete corruption/cap controls, dense comparison readers, the frozen benchmark runner and independent adversarial review remain outstanding. No real-corpus benchmark invocation has run.

## Implementation checkpoint 2

The private harness now exposes corpus freeze, all-variant preparation/qualification, and one warmup plus two measured rounds through isolated worker processes. Root independently reran the synthetic selftest and create-new persistence preflight successfully; both attest zero timed-workload invocations. The private checkpoint is upstream. Sol is reviewing attempt 1 before real-corpus qualification/timing. These passing controls are provisional evidence, not a claim that every issue gate has passed.
