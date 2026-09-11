# Handle unavailable metadata counts in sparse-stem reports

Parent: #734, comparative sparse-stem storage proof of concept.

## Problem

The one authorized benchmark invocation completed successfully with exactly 93 operations and a completion record. Report generation then failed before creating its output directory because the aggregation applies `sum(...)` to `metadata_bytes_read` values that include explicit JSON `null` for dense FLAC. Dense FLAC intentionally records that libsndfile byte counts are unavailable. The existing synthetic reporter test covered a missing field but not an explicitly null field.

The immutable raw JSONL and completed workload remain valid. The frozen protocol requires post-workload report failures to be repaired and republished from the existing raw evidence without rerunning the benchmark.

## Smallest closable outcome

Make the private report adapter treat both a missing `metadata_bytes_read` field and an explicit null value as unavailable data. Generate the report, plots, and distribution bundle from the preserved raw JSONL and existing frozen artifacts. Record the original report failure and bind all outputs to the preserved raw SHA-256.

## Scope

- Change only the private sparse-stem report adapter and its focused synthetic regression, plus the private evidence/decision record needed to publish the repaired report.
- Preserve the raw JSONL, freeze, codec runner, variant set, corpus identities, artifacts, timing values, operation order, dictionary procedure, and benchmark parameters byte-for-byte.
- Do not invoke `benchmark`, any codec encode/decode/seek worker, or any network/Walrus measurement.
- Do not add metadata-read instrumentation, estimate dense-FLAC byte reads, substitute zero for unavailable data, or broaden this into a reporting framework.
- Keep parent issue #734 open until the repaired evidence receives final Sol review.

## Required behavior

1. Nullable aggregation distinguishes a measured integer from unavailable data. Missing and explicit-null `metadata_bytes_read` inputs produce the same unavailable aggregate for the affected variant/round; they are never coerced to zero.
2. Numeric metadata counts continue to sum exactly for variants that provide them. Mixed numeric/null values are rejected as an inconsistent record unless the frozen schema explicitly defines that mixture.
3. Dense FLAC remains labeled `unavailable through libsndfile`; no metadata-byte total or derived byte-amplification value is emitted for it.
4. All other aggregates and the exact 93-operation/completion validation remain unchanged.
5. Report generation uses create-new output semantics. The failed attempt created no output directory; any existing accepted report destination is refused rather than overwritten.
6. Generated human and machine-readable outputs record the preserved raw JSONL SHA-256 and state that this is post-processing of the single completed invocation.

## Objective gates

- A directed synthetic fixture containing explicit null for every dense-FLAC seek row passes and emits an unavailable value/label, with no zero-valued substitute.
- The equivalent fixture with the field missing passes with the same result.
- A numeric-only fixture preserves its exact sum.
- A mixed numeric/null fixture fails closed.
- The existing 93-operation completion, incomplete-run rejection, duplicate-operation rejection, raw persistence, and overwrite-refusal tests remain green.
- A guard test or invocation wrapper proves the repair path calls only report-generation code and cannot dispatch the benchmark or codec workers.
- The report command succeeds once against the preserved raw JSONL, existing freeze/artifacts, qualification records, environment evidence, and padding evidence; its outputs bind the independently recorded raw SHA-256.
- Final Sol review confirms no benchmark rerun, no mutation of frozen codec/workload code, and no unsupported network, realtime, or dense-FLAC byte-read claim.

## Decision record

**Sol scope verdict: PASS to create and synchronize this minimal successor issue/spec.** The failure is isolated to nullable report aggregation after a valid completed benchmark. Repairing post-processing from immutable raw evidence is required by the frozen parent brief and does not authorize another timed invocation. One small implementation attempt should be sufficient; any need to alter the runner, raw record, codec artifacts, or benchmark protocol requires a new ruling rather than scope expansion.

## Implementation and publication evidence

Terra supplied the minimal two-file repair. Sol attempt 1 PASS: explicit-null and missing counts remain unavailable, numeric counts sum exactly, mixed numeric/unavailable rows fail closed, and the adapter has no codec/benchmark dispatch path. Root independently reran the focused synthetic regression and whitespace checks. The repair checkpoint is upstream in the private repository.

The real report then succeeded once against the preserved raw record. All ten complete local download archives were materialized from existing artifacts and matched their prior size model. Publication records bind the original raw digest and demonstrate unchanged codec, workload, freeze, qualification and timing evidence. The earlier report failure is preserved. No benchmark or codec worker was rerun. Final Sol publication review and remote synchronization remain pending.
