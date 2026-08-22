# 066 Reconcile builtin graph fixture and dependent benchmark input identities

## Outcome

Seal the corrected seven-boundary graph/PDC fixture and reconcile every accepted benchmark-input
identity that directly pins its graph PCM, without launching or timing any benchmark workload.

## Context

Issue 062 exhausted its Terra-plus-Sol budget after its corrected graph model reached the accepted
benchmark-input identity check. Checkpoint `2bbed6a` is accepted technical input: it corrects the
prepared fader/matrix model, preserves three exact 3-sample rack delays, and proves the early route
begins at output frame 9. It is not an overall PASS. The two existing
`benchmark/meter_success_full-{48000,96000}.toml` files still pin the old graph PCM SHA-256, so the
new graph payload cannot be accepted without atomically reconciling those dependent input bytes.

This stateless issue permits exactly one Terra attempt and one bounded Sol correction/review. A
second failure stops. Benchmark workload, timing and benchmark-runner invocations are forbidden
and remain zero.

## Scope

Consume checkpoint `2bbed6a` and complete only its graph-fixture/checker work. Regenerate exactly
`pcm/graph-taps.f32le` and `meters/graph-taps.jsonl`; update only the existing
`input_pcm_sha256` value in both `benchmark/meter_success_full-{48000,96000}.toml` inputs; and
update the corresponding four `MANIFEST.tsv` rows. The meter payload has no benchmark-input hash
field: its identity changes only in the manifest. All other fixture payload bytes, benchmark-input
fields and manifest rows remain byte-identical to the dependency checkpoint.

The author path may write only an explicit scratch root or the authorized checked corpus. The
valid `--check` path remains read-only. Bounded fixture-tool checker/tests may change only to seal
this exact graph/PDC and dependent-identity surface.

## Required public interfaces/contracts

The production author renders the compiled/bound graph. A separate retained-`f32` model derives
the source, input builtins, three rack-delay stages, prepared nonidentity fader, prepared matrix,
two route transforms, balanced two-input reduction and 9-sample compensation. It derives final
PCM and all seven meter snapshots without reading candidate payload values.

The exact PDC relation is late route `(source=9, compensation=0, destination=9)` and early route
`(source=0, compensation=9, destination=9)`, with exactly one 9-sample inserted early-route delay.
The transformed early contribution is zero through frames 0–8 and first nonzero at frame 9.

## Deliverables

- one corrected `pcm/graph-taps.f32le` and one corrected seven-record
  `meters/graph-taps.jsonl`;
- the two existing `meter_success_full` TOMLs with only their graph-PCM identity repinned;
- exactly four updated manifest rows plus the bounded independent checker and focused mutations;
- strict evidence with all workload/timing counts fixed at zero.

## Explicit non-goals

Benchmark execution, preflight, runner changes, timing, performance claims, new benchmark fields or
inputs, response/scalar PCM, other JSONL classes, production DSP/graph API changes, realtime audit,
targets, instructions, final 24-format seal or listening.

## Dependencies by exact issue title

- Builtin cascade decay and recovery contract
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Production SIMD builtin bank graph retention and reachability qualification

Stopped **Complete builtin graph-tap and PDC fixture semantics** checkpoint `2bbed6a` is technical
input only and is not treated as a passed dependency.

## Acceptance gates with objective measurements

- The seven stable tap identities occur exactly once in canonical order and their left/right
  summary-word tuples are pairwise distinct.
- Independent full PCM and all seven exact snapshot records match the authored payloads. A changed
  output word or tap field is rejected after recomputing its manifest row.
- Compiled PDC metadata matches the frozen two-route relation, and the independently checked
  production output proves the early transformed contribution first appears at frame 9.
- Both accepted `meter_success_full` TOMLs retain every field except `input_pcm_sha256`; that field
  exactly equals the new graph PCM SHA-256, their own new manifest hashes match, and the graph-meter
  payload's new manifest hash matches. Every other corpus byte and manifest row is unchanged.
- Scratch author plus read-only check, checked-corpus read-only validation, focused fixture/graph/
  compiler tests, format, warning-denied relevant-package Clippy and diff/static checks pass.

## Required evidence

Dependency checkpoint; graph topology and exact latency; old/new graph PCM, graph meter, two input
TOML and manifest identities; seven distinct summary tuples; output/tap/PDC and dependent-hash
mutation results; unchanged-file comparison; strict Terra/Sol verdicts;
`workload_invocations=0`; `timed_benchmark_invocations=0`.

## Terra attempt 1 evidence — PASS, pending Sol review

Dependency technical input: stopped Issue 062 checkpoint `2bbed6a`.

- The regenerated graph PCM is SHA-256
  `508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19`; the seven-record
  graph meter payload is SHA-256
  `958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`.
- The independent retained-`f32` PCM/snapshot model accepts exactly seven canonical, pairwise
  distinct graph-tap summaries. Compiled route timing remains late `(9,0,9)`, early `(0,9,9)`,
  with one inserted early-route delay of 9 samples; the early transformed contribution is
  positive-zero through frames 0–8 and first nonzero at frame 9.
- Only `input_pcm_sha256` changed in
  `benchmark/meter_success_full-{48000,96000}.toml`; both now pin the regenerated graph PCM.
  Their manifest rows, plus the graph PCM and graph-meter rows, were regenerated. Corpus
  cardinality remains 50 payloads and 10 benchmark inputs.
- Exact dirty artifacts are `tools/miso-engine-builtins-fixture/src/main.rs`,
  `fixtures/builtins/v1/pcm/graph-taps.f32le`,
  `fixtures/builtins/v1/meters/graph-taps.jsonl`,
  `fixtures/builtins/v1/benchmark/meter_success_full-48000.toml`,
  `fixtures/builtins/v1/benchmark/meter_success_full-96000.toml`, and
  `fixtures/builtins/v1/MANIFEST.tsv`; no other payload or benchmark input changed.
- Scratch author plus read-only validation and checked-corpus read-only validation passed.
  The focused manifest-valid graph-word/tap/dependent-hash/PDC mutations passed rejection.
  Fixture package tests passed 9/9 in 50.63s; format check, warning-denied all-target Clippy,
  and diff/static checks passed.
- `workload_invocations=0`; `timed_benchmark_invocations=0`; no benchmark runner, preflight,
  workload, or timing invocation occurred.

## Final Sol adversarial review — PASS

**PASS. No Sol correction was required.** The implementation delta is exactly six owned paths:
the fixture tool, graph PCM, graph meter, two `meter_success_full` TOMLs and `MANIFEST.tsv`; this
spec is the only additional evidence path. The checker independently derives every PCM word and
complete snapshot from the retained-`f32` source/filter/delay/fader/matrix/route/reduction model.
The actual report is checked at late `(9,0,9)`, early `(0,9,9)` and one inserted 9-sample delay;
the full-PCM check plus the focused word-9 mutation proves runtime placement rather than metadata
alone. All seven canonical tap records have distinct peak/energy/RMS summary tuples.

Artifact identities are:

- graph PCM: `508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19`;
- graph meter: `958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`;
- 48-kHz meter input: `ded3579ee8ffbf79d920648a33a7e2f35fa9c9b386e98ef469d583830ef992de`;
- 96-kHz meter input: `aa1c4d8835753ce290d7abcf1cbf3ffdb98b79a58f0ec6cd0cce6614f5befef9`;
- manifest: `bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`.

Both TOML diffs change only `input_pcm_sha256`; exactly four manifest rows change. The corpus
remains exactly 50 manifest payloads, 10 benchmark inputs, one 1,024-byte graph PCM and seven graph
meter records. Manifest-valid PCM-word, tap-field and dependent-hash mutations reject, as do both
wrong PDC relations.

Final Sol gates:

- checked-corpus read-only validation: PASS;
- exact `issue066_graph_pdc_and_dependent_identity_mutations_are_rejected`: PASS, `1/1`;
- warning-denied all-target fixture-package Clippy: PASS;
- format, six-path, four-row, TOML-field, cardinality and diff checks: PASS.

This strict PASS unblocks **Seal independent builtin corpus corruption and read-only
qualification**. `workload_invocations=0`; `timed_benchmark_invocations=0`; benchmark runner,
preflight, workload and timing invocations remain zero.
