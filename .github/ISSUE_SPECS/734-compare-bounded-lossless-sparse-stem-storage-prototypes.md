# Compare bounded lossless sparse-stem storage prototypes

## Outcome and boundary

Build external research prototypes and produce a reproducible comparison of dense and sparse stem storage, exact PCM reconstruction, random access, and bounded decoder resources. This is a decision experiment, not an engine implementation or delivery-codec addition. The user authorized real-corpus proof of concepts and chose to align file beginnings for the activity model. Every required asset must be downloaded before session start: the network objective is complete offline-session availability, not progressive playback startup.

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

Complete: final Sol attempt 2 PASS covers all ten prototypes, real-corpus qualification, the single descriptive run, repaired reporting and complete local download bundles. The private evidence and public decision record are upstream. Historical checkpoints below preserve earlier provisional/failing verdicts.

## Implementation checkpoint 1

The private codec tranche is committed and upstream. Synthetic PCM24 reconstruction, sparse windows, a Zstd payload corruption control, invalid decoded-length rejection, create-new output behavior, and Python compilation pass. Zstd content frames spool to a file. All-corpus qualification, complete corruption/cap controls, dense comparison readers, the frozen benchmark runner and independent adversarial review remain outstanding. No real-corpus benchmark invocation has run.

## Implementation checkpoint 2

The private harness now exposes corpus freeze, all-variant preparation/qualification, and one warmup plus two measured rounds through isolated worker processes. Root independently reran the synthetic selftest and create-new persistence preflight successfully; both attest zero timed-workload invocations. The private checkpoint is upstream. Sol is reviewing attempt 1 before real-corpus qualification/timing. These passing controls are provisional evidence, not a claim that every issue gate has passed.

## Attempt 1 adversarial verdict

**Sol: FAIL; real-corpus qualification and timing remain gated.** A directed synthetic fixture reproduced an incorrect gap origin when an exact-zero run crossed an internal source-read boundary. Review also found incomplete adversarial controls, recomputed rather than universally frozen seek schedules, weak freeze admission, and a worker-failure path that returned apparent CLI success. These findings are preserved; the passing initial selftest does not satisfy the issue gates.

Sol owns attempt 2, bounded to the gap correction, discriminating boundary/corruption controls, one shared frozen-window qualification path, strict freeze/record validation, truthful failure exits, and preflight/output/resource accounting. No corpus, format, compression parameter, or benchmark-protocol change is authorized by this revision.

The private aligned-input census and original-prefix padding verification are upstream. They confirm the dataset preparation independently of the faulty prototype; they supply no codec or realtime performance evidence.

## Delivery objective clarification

The user clarified that all files required by a session are downloaded before startup. Compare complete package bytes, including every required manifest, index and dictionary; keep transfer bundling/request overhead distinct from local decoder-unit size. Existing dense files serve as baselines. A new sparse package remains the intended product candidate.

An existing corrected lossless DEFLATE archive is approximately 147.3 MB. Its exact size may be reported as an untimed existing-artifact reference, so storage/transfer savings are not presented only against expanded WAV bytes. This does not add a timed variant or change the frozen workload. No actual network or progressive-startup claim is introduced.

## Attempt 2 readiness evidence

Sol corrected the reproduced gap defect and completed the bounded parser, frozen-schedule, corruption/locality and runner-lifecycle controls. Sol recorded readiness PASS for real-corpus untimed qualification. Root independently reran selftest and preflight; both passed with zero timed-workload invocations. The correction and adapted reporting are upstream.

The corrected corpus was admitted, and every PCM identity matches the independent input census. Artifact preparation is in progress. Full reconstruction, common seeks, representative corruption qualification and the single timed invocation remain outstanding; this is not an issue-completion verdict. Reporting is frozen around complete-session bytes, local decode cost, and an explicitly untimed conservative single-archive body-size model.

## Real-corpus qualification

All 490 full-stem PCM reconstructions, all 3,920 common frozen seek windows, and all 43 representative-copy corruption/locality cases passed. Root preserved the private correctness evidence and an execution freeze binding code, corpus, environment, reporter and exact command, then pushed the checkpoint. The single benchmark invocation has started with one warmup and two measured rounds. No timing result or completion claim is made until the complete record passes validation and final Sol review.

## Completed measurements; report-only successor

The single frozen invocation completed all 93 operations and its completion marker; closed record validation passed. Raw evidence is preserved upstream, and the codec/workload/configuration hashes remained unchanged through completion. The report exporter then failed on an explicitly unavailable dense-FLAC metadata count. The completed measurements are valid; issue #735 owns the minimal nullable-field repair and publication from the preserved raw record. No benchmark rerun is authorized. This parent remains open through repaired evidence and final Sol review.

## Final measured evidence

The corrected corpus contains 49 aligned stems, each 6,187,569 stereo PCM24 frames at 44.1 kHz. Exact zeros occupy 88.02% of source frames; 87.95% of frames lie in omittable runs of at least 20 ms. All 490 full reconstructions, 3,920 common seek windows and 43 corruption/locality cases passed.

Actual complete ZIP_STORED archives were produced from every already-measured representation, with no codec rerun or outer recompression. Every archive's actual byte count matches the precomputed body-size model; manifests, indexes, dictionary bytes and conservative inspection copies are included.

| Representation | Actual complete bundle MB | Decode and hash seconds, round 1 / round 2 |
|---|---:|---:|
| Dense WAV | 1819.195 | 1.240 / 1.243 |
| Dense FLAC | 93.874 | 12.179 / 12.153 |
| Dense independent Zstd, 1 second | 162.215 | 4.962 / 4.932 |
| Sparse WAV files | 219.977 | 1.593 / 1.591 |
| Sparse packed PCM | 219.698 | 1.410 / 1.411 |
| Sparse FLAC units | 91.959 | 3.364 / 3.397 |
| Sparse Zstd, 1 second | 167.299 | 1.983 / 1.970 |
| Sparse Zstd with dictionary, 1 second | 168.226 | 1.968 / 1.939 |
| Sparse Zstd, 250 milliseconds | 169.463 | 2.929 / 2.966 |
| Sparse Zstd with dictionary, 250 milliseconds | 169.512 | 2.936 / 2.912 |

MB is decimal. The existing corrected DEFLATE ZIP is 147.256 MB. The sparse-FLAC complete bundle is 37.55% smaller than that reference and 2.04% smaller than dense FLAC's bundle. Its mean full reconstruction/hash time is 3.380 seconds versus 12.166 seconds for dense FLAC on this local prototype. Sparse-FLAC encoding took 11.415 / 11.440 seconds versus dense FLAC's 8.336 / 8.324 seconds.

Both dictionary variants failed the frozen 1% held-out net-saving gate: total size increased 0.808% for one-second units and 0.399% for 250-ms units. This is within-song evidence only. Dense-FLAC byte-read accounting remains unavailable, never zero.

## Decision and production scope

Recommend exact-zero intervals plus independent FLAC units in a complete offline-session package; omit dictionaries from the first product slice. Most download savings here come from FLAC compression, while explicit sparsity provides a further small size reduction and substantial measured prototype decode savings. Decoder units need not equal network objects: all 1,399 FLAC units are packaged into one local downloadable archive in this experiment.

Keep production work separate: (1) an external canonical sparse asset packer/reader with byte-exact reconstruction and bounded validation; (2) one worker/host adapter feeding the existing bounded PCM ingress, with generation-tagged seeks, render-quantum reblocking, known-zero versus underrun semantics and duration-independent decoded memory; (3) actual complete-session transport qualification. Compact gap queue entries and DSP skipping require later independent evidence. Source silence alone cannot skip effect state/tails, lookahead/PDC, sidechains, automation or partially active SIMD banks.

All claims are local descriptive evidence from exactly one warmup and two measured rounds. There was no machine isolation or cold-cache control. No actual network, Walrus, browser/mobile or render-callback timing was performed. No production engine code changed. Private reproducibility artifacts include raw operation records, corpus qualification, plots, bundle-size receipts and a decision/integration note; public records contain only sanitized aggregates.

## Final adversarial verdict

**Sol attempt 2: PASS.** Independent review reproduced raw/frozen evidence identities, all 93 successful operations and the completion record, correctness/corruption counts, all ten actual archive sizes/hashes/member inventories, publication identities and decision arithmetic. No benchmark rerun or frozen workload mutation occurred. The public aggregates are accurate and contain no private corpus identity. The external proof-of-concept outcome is complete; future production integration remains outside this issue. Delivery is recorded in PR #736 and the private companion PR.
