# Sol implementation brief — issue 035 builtin qualification tooling, audits, and benchmark

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1 ONLY AFTER ISSUE 036 PASSES.** This issue has at most two total
implementation attempts: Terra attempt 1 and one bounded Sol correction/review attempt. A second
failure stops and requires a new stateless rescope. Never inspect V1/legacy, relax a retained DSP
threshold, tune expected bytes to an observed defect, or fabricate human listening.

Issue 036's final candidate is the only input. Production coefficients, incremental non-fused
all-`f32` TPT operation order, scalar chain, matrix/pan behavior and meter math are frozen and out
of implementation scope. A discovered DSP defect fails this issue and creates a separate
corrective issue.

Issue 035 is the sole owner of the builtin timed benchmark authorization. Invocation count is
**0**. All work through the final adversarial nonbenchmark review must keep
`workload_launches=0`. Only root Sol may later authorize exactly:

```text
bash scripts/run-builtins-benchmark.sh
```

That one external invocation executes two internal rounds. Failure, interruption or validator
rejection consumes the authorization; there is no retry. Issue 033, not this issue, owns real
human listening after the accepted machine artifact is sealed.

## Frozen response grid and thresholds

The independent crate `miso-engine-dsp-reference` must not depend on production builtins. Use
rates `44100,48000,88200,96000` and quanta `1,127,128,255,1024`. For HPF and LPF separately use
deduplicated cutoffs `10,20,100,1000,min(20000,0.1*Fs),0.45*Fs` Hz. For each cutoff use probes
`0.25*f,f,4*f,0.2*Fs,0.45*Fs`, clipped strictly inside Nyquist, snapped to the nearest 4-Hz
coherent bin and deduplicated; analytic coverage also includes exact cutoff and `0.49*Fs`.

The cascade is always a 100-Hz HPF followed by a 1-kHz LPF. It is never derived as
`0.5*cutoff/2*cutoff`; its probes are the sorted deduplicated union of the two fixed sections'
probe sets. Preparation cutoff is `-3.0102999566 +/- 0.005 dB`. Cast-state analytic error is at
most `0.005 dB`; one-second impulse-DFT error is at most `0.05 dB` where reference magnitude is at
least `-120 dB`. Sustained amplitude-0.5 signals settle `Fs/2` frames and measure `Fs/4` with a
full `f64` three-column DC/sine/cosine fit: at reference gain at least `-90 dB`, fundamental error
is at most `0.05 dB` and residual is at most `-100 dB` relative to input RMS; below `-90 dB`,
total production gain is at most `-88 dB`. Also retain exact DC/Nyquist, dense monotonicity,
finite-state, final-4096 tail energy, partition, recovery/reset/sanitization and L/R isolation
checks.

## Frozen fixture bytes and coverage

`fixtures/builtins/v1/MANIFEST.tsv` has sorted safe relative paths, exact byte lengths and
lowercase SHA-256. It covers and hashes:

- fully expanded canonical `cases.toml` tuples, never declarations;
- ten fully expanded `benchmark/<kind>-<rate>.toml` input bundles with exact parameters/targets,
  session identities and referenced PCM paths/hashes;
- headerless little-endian planar-L-then-R `pcm/*.f32le` production expected bytes;
- sorted `reference/filter-response.csv` with independent RBJ, cast-state, impulse, sustained,
  tail and recovery fields; every decimal `f64` uses 17 significant digits;
- `meters/*.jsonl` with canonical identities/times/counters and lowercase fixed-width IEEE bit
  hex for every floating result;
- sorted `diagnostics.jsonl` with exact code/path for parameter, block/time, meter, resource and
  all eight seal categories; and
- `resources.jsonl` with total, largest and `(size,align,count)` allocation breakdown for tracks
  `1,4,65537` x meter sets `0,1,7`, logical capacity four, on the pinned fixture ABI.

Functional PCM covers asymmetric lanes; gain/polarity/fader/mute; signed-zero identity; all 16
matrix corners; constant-power pan and cosine-balance endpoints/centers; ramp lengths
`0,1,2,127,128,u32::MAX`; all declared splits and mid-ramp retargets; input/fader/filter/matrix
sanitization, pairwise recovery/reset; all seven graph taps with pairwise-distinct values; exact
output and a positive-latency PDC side route. Meter expected output covers partial/multiple
windows, wrap, full/drop, interleaved drain, discontinuity, both resets, overflow, hold/decay and
sanitization.

The generator writes only to an explicit scratch directory. `--check` never writes the repository,
verifies every byte/reference and proves complete Cartesian coverage. Production expected PCM is
not regenerated during comparison. For each format class `cases.toml`, `f32le`, CSV, meter JSONL,
diagnostics JSONL and resources JSONL, independently delete, byte-alter, add and make a
manifest-valid coverage hole: all 24 fail. Also reject missing/corrupt/duplicate/unsorted manifest
entries, wrong length/hash, unsafe paths and unlisted payloads.

## Frozen direct audit

At 48 kHz/128 frames, make exactly 1,000,000 production builtin process calls inside the marker.
The fixed initial schedule starts a 257-update matrix ramp, retargets on the next block boundary
with updates outstanding, injects nonfinite input/target cases, exercises paired filter-state
recovery and both reset paths, then enters deterministic steady state. Two meter sets cover
success/drain and full/drop across seven taps. Compare PCM, matrix state and every recovery,
sanitization, reset and meter count to fixtures.

Count allocations, deallocations, locks, logs, file I/O, network I/O, syscalls, feature detection
and panic/unwind separately and in an exact sum. All are zero. A marker-delimited `strace` also
finds zero syscalls. Exactly one deliberate terminating probe exists for each of the nine detector
categories.

## Frozen graph/swap/retirement audit

Compile the canonical session/effect and issue-036 accepted sealed builtin artifacts with seven meters.
Use production graph and `RealtimePlanOwner` APIs, binding only genuine external source/output and
the fixture processors for the declared racks/PDC route. Never render a direct `BuiltinChain`.
The rack processors are nonidentity so every one of the seven tap values is distinct; a fixed
positive-latency side route exercises integer PDC. Drain off render and compare exact tap snapshots
and PCM to checked fixtures.

The marker contains exactly 1,000,000 renders plus render-side swap decisions. Plan A renders
block 1. Plan B applies before block 2 and displaces A into the capacity-one retirement queue. A
pending Plan C is then deferred because the queue is full. B renders blocks 2–1,000,000. Require
one applied swap, at least one full-queue deferral, `(plan,epoch)` render counts A=1/B=999,999,
seven expected observer windows per declared drain block, exact queue success/full/drop counts,
stable output backing addresses and no render-thread destruction. After the marker, the retirement
owner destroys A after its completed marker and the control owner disposes of never-applied C.

The same nine counters, exact total, `strace` rule and detector probes are zero/pass. Publication,
meter drain and retirement are outside the render marker; this does not excuse render-side swap or
drop work.

## Frozen schema-v2 benchmark binary

The binary produces five kinds x rates `48000,96000` x rounds `1,2` at quantum 128: exactly 20
single-line canonical JSON objects and no other stdout.

| Kind | Tracks/meters/capacity | Timed work |
| --- | --- | --- |
| `full_chain_filters` | 1/0/null | asymmetric 100/200-Hz HPF, 1/2-kHz LPF, fixed gains/non-diagonal matrix, continuous state |
| `identity_chain` | 1/0/null | exact identity chain |
| `matrix_ramp` | 1/0/null | alternate two fixture matrices; one 128-update ramp/operation |
| `meter_success_full` | 1/14/1 | two sets across seven distinct graph taps; drain one, prefill/drop one |
| `prepare_256_tracks` | 256/56/4 | prepare precompiled session; seven taps on eight named tracks |

Render kinds use 64 warm-up and 512 measured batches, eight operations each. Divide each batch's
integer nanoseconds by eight before collecting exactly 4,096 observations; advance sample time by
128 per operation. Preparation uses 16 warm-ups and 128 measured single operations; stop timing
before destruction. Each round resets to the same fixture state. Report nearest-rank integer
nanoseconds/operation `min,p50,p95,p99,p99_9,max`; no speed threshold exists.

Every record's exact common fields are:

```text
schema_version issue workload_kind workload_id sample_rate_hz quantum_frames round
render_scope warmup_batches measured_batches operations_per_batch total_operations
frames_per_operation tracks meter_observers meter_queue_capacity retained_payload_bytes
percentile_method units min_ns p50_ns p95_ns p99_ns p99_9_ns max_ns descriptive_only
candidate_commit binary_sha256 fixture_manifest_id fixture_manifest_sha256
input_fixture_id input_fixture_sha256 output_sha256
render_errors render_allocations render_deallocations render_locks render_logs
render_file_io render_network_io render_syscalls render_feature_detection
render_panic_unwind render_total_forbidden_operations
cpu_model cpu_architecture logical_core_count physical_core_count os kernel
governor_or_power_mode rust_version llvm_version target_triple target_features profile
opt_level lto codegen_units background_load_note missing_metadata
```

Require `schema_version=2`, `issue=35`, `workload_id=issue035.<kind>.<rate>hz.q128`, quantum 128,
`percentile_method=nearest_rank`, `units=ns_per_operation`, and `descriptive_only=true`. Render
fields are integer zero and total equals the nine forbidden-operation fields; preparation uses
exact `not_applicable` strings. Render `frames_per_operation=128`; preparation uses null. The
first three workload meter capacities are null; the table freezes all other shapes.

The manifest ID is exactly `fixtures/builtins/v1/MANIFEST.tsv`. Each input ID is exactly
`fixtures/builtins/v1/benchmark/<kind>-<rate>.toml`; the manifest-listed file is fully expanded and
self-contained except for referenced PCM whose exact path/hash it includes. Its manifest SHA is
the input hash. Never use the manifest itself as the input ID. Render output hashes cover all
measured PCM and, for the meter workload, canonical snapshots/counters. Preparation hashes a
canonical address-free track/tail/meter/resource projection. Candidate, binary and manifest hashes
match across all 20 records. Each workload/rate input and output hash matches across rounds.

Metadata values are correctly typed discovered strings/integers or JSON null. The sorted unique
`missing_metadata` list equals exactly the null metadata keys. Reject empty or `unknown/default`
sentinels as discovered values. The single-record validator enforces the exact key set, types,
ID/kind/rate mapping, workload shape, observation counts, percentiles, fixture/audit and metadata
rules. The aggregate validator enforces the exact Cartesian set/cardinality and stable hashes.
Mutate every field and type plus every kind/rate/round duplicate/omission/cardinality, workload
shape, percentile order, missing-metadata honesty, audit total/nonzero, fixture/input/output
mismatch and cross-round output: each mutation fails without launching work.

## Frozen safe runner

`scripts/run-builtins-benchmark.sh` has `set -euo pipefail`, no arguments, quoted paths derived
from `BASH_SOURCE`, no `eval`, source, unsafe splitting or user-built command. It checks every tool
and refuses before launch if any of these exists:
`target/issue35/builtins-benchmark.raw.jsonl`,
`target/issue35/builtins-benchmark.jsonl`,
`target/issue35/builtins-benchmark.validator.stderr` or
`target/issue35/builtins-benchmark.disposition.json`. It gathers metadata before the one workload
launch and passes fixed environment variables.

Stdout goes directly to a newly created raw JSONL file. Never rewrite/delete raw bytes. On process
or validation failure, preserve raw and write a separate disposition with exit status, validator
reason/stderr, validator/tool hash and raw hash. On success, preserve raw and atomically create a
byte-identical accepted copy plus PASS disposition. Refuse overwrite and expose no retry/resume/
tuning flag. Preflight builds/hashes the release binary, checks input bundles, runner and all
synthetic mutations, but reports `workload_launches=0` and creates no raw/accepted artifact.

## Ordered nonbenchmark gates

1. Complete fixture/reference/coverage/corruption checks and issue-036 contract tests.
2. Direct and graph audits, exact output/lifecycle checks, traces and nine probes.
3. Native debug/pinned scalar release; Android/iOS pure-Rust release; Wasm `-simd128/+simd128`.
4. Locked workspace tests; warning-denied all-target Clippy/rustdoc; format; all workspace,
   realtime, research, graph and builtin policies/mutations.
5. Exact schema/runner/validator mutation suite and zero-launch preflight.
6. Seal candidate, binary, fixture, audit and target hashes on one clean committed tree.
7. Sol adversarial review finds no unresolved nonbenchmark gate.

Only then may root Sol issue the one exact authorization. If any earlier gate fails, record FAIL
without invoking the workload. If the authorized run fails or its 20 records reject, preserve all
bytes/disposition, record FAIL and do not retry.

Passing produces a **machine-qualified, human-listening pending** candidate. Update issue 033 with
the exact candidate, fixture, render and benchmark identities; do not add listener responses.
Issue 026 remains blocked until issue 033 independently passes.
