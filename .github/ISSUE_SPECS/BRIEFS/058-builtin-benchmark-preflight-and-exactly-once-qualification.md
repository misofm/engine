# Sol implementation brief — issue 058 builtin benchmark preflight and exactly-once qualification

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1; THE TIMED COMMAND IS NOT AUTHORIZED.** Issue 068 is closed with final
Sol PASS and supplies the target-qualified builtin candidate. Issue 070 supplies the accepted
graph trace; stopped Issues 057 and 069 supply only the exact technical evidence explicitly
accepted by their successors. Do not reopen or rerun those feature, corpus, audit, trace, target or
instruction gates.

The rejected draft `main.rs` patch left no repository mutation, checkpoint, focused gate or
adversarial implementation verdict. It was preimplementation exploration, not an implementation
attempt; Terra attempt 1 remains unused.

Permit one Terra implementation/review attempt and at most one bounded Sol correction/review. A
second failure stops. During implementation, testing, preflight and Sol review, do not invoke
`miso_engine_builtins_bench`, `cargo run -p miso-engine-builtins-bench`, or the real runner. Only
counted hermetic stubs may exercise runner control flow. At briefing:

- `runner_invocations=0`;
- `workload_invocations=0`; and
- `timed_benchmark_invocations=0`.

Root Sol may authorize the sole timed command only after every nonexecuting gate below passes on a
clean committed candidate. A failed or interrupted authorized command consumes the authorization;
there is no retry, resume, tuning run or replacement measurement.

## Immutable dependency identities

Preserve the checked corpus and accepted machine evidence. The preflight must verify, without
authoring any fixture:

- `fixtures/builtins/v1/MANIFEST.tsv` SHA-256
  `bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`;
- graph PCM SHA-256
  `508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19`;
- graph-meter SHA-256
  `958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`;
- accepted direct-audit record SHA-256
  `3581ebf058151a0a0014ff08adcdd7fcd6fe6ad51a5baf41538272d4bba6ce8e`;
- accepted graph functional record SHA-256
  `54103c89b557a72da9c79cd00a636ea64933240a4dcb27c27647fb960b013db4`;
- accepted graph raw trace-set and validator-output SHA-256 values
  `812e7c62cf8963fba1cb6f32615005ec8bd7df6b97f6c72a0c4960fadcf0d4c1` and
  `1c98d033c0c5d156dea887a829cc683d460145c08856c705fdbde7ef8b4324c5`;
- accepted Issue-068 five-package source-manifest SHA-256
  `0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19`;
  and
- preimplementation `Cargo.lock` provenance SHA-256
  `96d0585ab8059905b256f87e7cadd717ae6e790aa140de3a4e7cc9db4791d424`.

The benchmark crate cannot construct and bind the frozen real-tap graph through its old direct
dependency set. Terra may therefore add only the direct benchmark-package graph/effect/
conformance dependencies required by that workload and update only the corresponding
`miso-engine-builtins-bench` dependency stanza in root `Cargo.lock`. Every unrelated lock stanza
and every existing package version, source and checksum must remain byte-for-byte unchanged. The
resulting post-change lockfile SHA-256 becomes the candidate identity sealed by preflight; the
preimplementation hash above remains provenance and is not the post-change expected hash. Any
other lockfile drift stops Issue 058.

All ten `fixtures/builtins/v1/benchmark/<kind>-<rate>.toml` files remain manifest-listed immutable
inputs. Their manifest rows, not duplicated constants, are the authority for byte lengths and
SHA-256 values. The final preflight also seals the clean current candidate commit, benchmark
binary, runner, preflight, both validators and their hermetic test. Any product-source, corpus,
accepted-evidence or lockfile drift stops; do not repair it in Issue 058.

## Exact one-warmup/two-round workload

The frozen JSONL schema belongs to Issue 035 even though Issue 058 owns execution and disposition.
Every measured record therefore retains `schema_version=2`, `issue=35`, and exact
`issue035.<kind>.<rate>hz.q128` IDs. Artifact paths remain under `target/issue35/`. Do not emit the
existing Issue-007 schema or use the corpus manifest as a substitute for a workload input.

One release-binary process performs exactly one global untimed warmup pass, then measured rounds 1
and 2. The warmup pass visits each of the ten workload/rate pairs once. For render pairs it warms
the separately prepared round states with 64 batches of eight operations; preparation uses 16
untimed preparations. It emits no record and is never repeated. Each measured round starts from
its corresponding identically prepared post-warmup state and contains:

- four render workloads × two rates: 512 batches of eight 128-frame operations; and
- `prepare_256_tracks` × two rates: 128 single preparations, with destruction outside timing.

This produces exactly 20 records: five kinds × rates 48,000/96,000 × rounds 1/2. The other launch
rates remain covered by Issue 068; this descriptive benchmark makes no claim at 44.1 or 88.2 kHz.
There is no threshold, comparison, tuning decision or launch-capacity claim.

Each measured pair consumes its exact TOML and referenced PCM bytes after verifying their hashes.
The workloads are exactly:

1. `full_chain_filters`: the checked asymmetric HPF/LPF, trim/fader and matrix chain;
2. `identity_chain`: the checked signed-zero identity input and exact identity chain;
3. `matrix_ramp`: the checked alternating matrices with 128 updates per operation;
4. `meter_success_full`: two separately prepared otherwise-identical accepted single-track graph
   plans, each with one unique request at every real tap. One seven-tap set drains every operation;
   the other is prefilled at logical capacity one and proves full/drop behavior. Both plans have
   identical tap IDs/order, input, PDC and continuation before queue outcome divergence. This
   preserves production `builtin.meter.duplicate`; no graph or meter API change is allowed; and
5. `prepare_256_tracks`: the checked 256-track session with 56 meters and exact retained-resource
   projection.

Timing covers only the named product operation or preparation. Input generation, evidence hashing,
queue draining, metadata collection and destruction stay outside the measured interval. Where a
render batch needs all eight outputs for its canonical hash, sum eight individually delimited
operation durations into the one batch observation and update the streaming output hash between
those intervals. Divide the checked batch nanoseconds by eight using integer arithmetic. Record
nearest-rank `min/p50/p95/p99/p99_9/max` in integer `ns_per_operation`.

Output SHA-256 covers every measured planar PCM block in canonical operation order. The meter row
also covers every emitted tap identity/snapshot/counter from both plans. Preparation hashes the
address-free processor/meter/resource projection. Output identity must match across rounds for each
workload/rate pair.

## Record and validator contract

Implement the exact Issue-035 required key set and types. In particular, include
`total_operations`, `units="ns_per_operation"`, `descriptive_only=true`, clean
`candidate_commit`, `binary_sha256`, exact per-pair input ID/SHA, deterministic output SHA, all
nine render audit categories plus their checked total, and the complete typed host metadata.
Preparation uses JSON null where Issue 035 requires null and exact `not_applicable` strings for
render-only audit fields. Missing metadata is JSON null plus the sorted unique exact field-name
list; `unknown`, `default`, empty strings and stringified numbers are not discovered values.

The record validator rejects every missing/extra key, wrong type, dishonest null/missing mapping,
bad ID/rate/round/shape/count, unordered percentile, wrong audit sum, nonzero render violation, or
bad hash. The aggregate validator requires the exact 20-record Cartesian set, no duplicates,
stable candidate/binary/manifest identity and stable per-pair input/output identity across rounds.
Mutation coverage must exercise every required field/class plus omitted/duplicated pair,
cross-round output drift and candidate/binary/manifest drift. Synthetic records only; no real
benchmark process.

## Preflight and sole runner lifecycle

Reuse the accepted Issue-030 shell lifecycle rather than inventing a second publication model.
The no-workload preflight:

1. requires a clean candidate and verifies the dependency identities above;
2. runs the read-only corpus checker and only proportional nonbenchmark package/policy gates;
3. runs all validator and hermetic runner mutations with counted stubs;
4. builds the release binary in a fresh unique `CARGO_TARGET_DIR`, copies it without overwrite to
   `target/issue35/miso_engine_builtins_bench`, and seals its bytes; and
5. writes a checksummed `target/issue35/builtins-benchmark.preflight.json` containing candidate,
   source/lock/corpus/evidence/tool/binary hashes and exact zero real-launch counters.

Preflight must never execute the binary. The public runner accepts no arguments or path overrides,
requires that exact seal and candidate, and directly launches the sealed binary exactly once. It
must not use `cargo run`, rebuild, retry or accept an environment-selected executable.

Before launch it refuses any existing raw, accepted, validator-stderr or disposition path,
including symlinks/aliases. Exact output paths are:

- `target/issue35/builtins-benchmark.raw.jsonl`;
- `target/issue35/builtins-benchmark.jsonl`;
- `target/issue35/builtins-benchmark.validator.stderr`; and
- `target/issue35/builtins-benchmark.disposition.json`.

Stdout is written directly to a newly created raw regular file and those bytes are never moved,
edited or deleted. Workload failure, interruption or validator failure preserves raw/stderr and
writes a checksummed FAIL disposition; it never publishes accepted output. Success validates raw,
creates an atomic no-clobber byte-identical accepted copy while retaining raw, and writes PASS
disposition. Disposition records exact candidate/tool/raw/accepted hashes and sizes,
`runner_invocations=1`, `workload_invocations=1`, `warmup_passes=1`,
`measured_rounds_completed=2`, and `timed_benchmark_invocations=1`. Any post-launch failure is
strict FAIL and moves artifact repair/promotion to a new tooling issue; it does not authorize a
rerun.

The sole command, only after root Sol authorization, is:

```sh
bash scripts/run-builtins-benchmark.sh
```

## Bounded implementation surface and ordered gates

Limit implementation to the existing benchmark crate and its direct tooling:

- `tools/miso-engine-builtins-bench/{Cargo.toml,src/main.rs}`;
- root `Cargo.lock`, limited to the benchmark-package dependency-stanza transition frozen above;
- `scripts/{builtins-benchmark-record-validator.jq,builtins-benchmark-validator.jq}`;
- `scripts/{run-builtins-benchmark.sh,preflight-builtins-benchmark.sh,test-builtins-benchmark.sh}`;
  and
- concise attempt evidence in the Issue-058 spec/brief.

No production DSP, core, graph, runtime, session, corpus, audit, trace, target, instruction,
benchmark-input, listening or unrelated runner file may change. If truthful real-tap execution
requires a production API change or the correction escapes this surface, stop and rescope rather
than broadening.

Ordered gates:

1. Static review proves all existing Issue-007 identities and fake single-buffer tap paths are
   gone, production duplicate rejection is preserved, and the binary has one warmup pass plus two
   measured rounds.
2. A static lock comparison proves the permitted benchmark-package-only transition and freezes
   the post-change lock hash. Format, locked package check/tests, warning-denied all-target
   Clippy/rustdoc and focused fixture/reference/graph tests pass without invoking benchmark
   `main`.
3. Both validators and the complete synthetic mutation matrix pass. Hermetic scratch tests prove
   argument/missing-tool/seal mismatch, counted success, workload failure, interruption/partial
   raw, validator failure, existing/symlink/alias artifacts, atomic no-clobber publication and
   status propagation with real workload launches zero.
4. Applicable workspace/realtime/builtin/graph policies, shell syntax, mutation suites and static
   no-artifact/no-workload scans pass.
5. Run the no-workload preflight and adversarially verify its clean identity seal and zero counters.
   Sol records PASS TO RUN or strict FAIL. No timed command occurs during Terra attempt 1.
6. Only after the preflight correction is committed and the candidate remains clean may root Sol
   authorize the sole runner command. Validate exact 20 records, raw/accepted byte identity,
   disposition and all counters/hashes, then record final PASS/FAIL.

At the first numerical, identity, real-tap, schema, runner-lifecycle or preflight failure, stop the
current attempt. Never weaken the schema, consume an old raw artifact, delete a partial artifact,
or optimize based on descriptive timing. A second failed attempt is final STOP/RESCOPE.

## Required evidence and disposition

Record dependency identities; clean candidate/source/lock/tool/binary seals; exact ten input IDs;
package/policy/validator/hermetic/preflight results; zero preauthorization counters; the sole
authorization and command; raw/accepted/stderr/disposition sizes and SHA-256 values; exact 20-row,
one-warmup/two-round counts; strict Terra/Sol verdicts; and no-threshold statement.

PASS sets `machine_qualification=PASS`, `human_listening_status=pending` with exact Issue 033 title
**Issue-007 builtin filter and matrix human listening qualification**,
`runner_invocations=1`, `workload_invocations=1`, and `timed_benchmark_invocations=1`. It does not
claim human listening, launch readiness, capacity or performance superiority.

## Terra attempt 1 verdict — 2026-08-22

**FAIL; NO PREFLIGHT OR TIMED COMMAND AUTHORIZED.** On clean candidate
`f15a7aefc7379b585508673823bbdaf89c238cd2`, Terra found multiple coupled frozen-workload defects,
not one bounded correction: checked TOML/referenced PCM do not drive the direct or graph-meter
inputs; matrix alternation is wrong at batch boundaries; meter plans differ in queue capacity and
use reset generation 35 instead of seven; the full/drop snapshots/counters are absent from the
meter hash; direct hashes cover only the final PCM block; the preparation projection is incomplete;
validators do not bind the exact manifest/per-pair input hashes; the hermetic matrix omits the
missing-tool case; and partial failures claim completed warmup/round counters.

Format, locked package check/test (5/5), warning-denied Clippy/rustdoc, JQ and shell syntax,
synthetic validator/stub-runner tests, five applicable policies, the read-only 50-row manifest
identity check and the benchmark-stanza-only lock comparison passed. Lock identities are
preimplementation `96d0585ab8059905b256f87e7cadd717ae6e790aa140de3a4e7cc9db4791d424`,
frozen diff `5ebc70f8a35208d50ff4d9afd92602462180b345125263a0a4916aa3bb08940e`, and candidate
`da662dd70c21ae844f551e5f2ed6ef97c52982fc9f8b86d19c1776e57e0a576f`. The fixture executable,
actual preflight, public runner, benchmark binary, workload, timing, audit, trace and target gates
were not invoked. Exact final counters: `runner_invocations=0`, `workload_invocations=0`,
`timed_benchmark_invocations=0`. Terra made no implementation correction; one bounded Sol
correction/review remains.

## Sol attempt 2 nonexecuting checkpoint — 2026-08-22

**PASS TO PREFLIGHT; NO PUBLIC RUNNER OR TIMED COMMAND IS AUTHORIZED YET.** Clean candidate
`3f4fd34f81e7e2205503887c03ad27f3aad69c8a` passed the frozen package/workspace, fixture,
validator, scratch runner/preflight lifecycle, warning-denied Clippy/rustdoc, policy/mutation,
identity and static no-workload gates. Candidate source SHA-256 is
`34e40ddcce0b51b53aa58629894332a0ee045e4bf4ea5a5a7ca0fffbb59c4a62`; candidate lock remains
`da662dd70c21ae844f551e5f2ed6ef97c52982fc9f8b86d19c1776e57e0a576f`, with the frozen permitted
lock diff unchanged. The accepted Issue-068 source manifest reconstructs exactly, all 50 corpus
files and ten benchmark inputs validate read-only, and no Issue-035 artifact existed.

Actual preflight, benchmark main, public runner, workload, timing, audit, trace and target workloads
were not invoked. Counters remain `runner_invocations=0`, `workload_invocations=0`, and
`timed_benchmark_invocations=0`. After this evidence is committed cleanly, root may authorize one
no-workload preflight; its seal and zero counters require a separate Sol check before the sole
runner command can be authorized.

## Final Sol verdict after the sole runner invocation — 2026-08-22

**FAIL / STOPPED / RESCOPE REQUIRED; NO OVERALL PASS.** The one authorized runner invocation on
candidate `79c1872753aa4943761f31a77aac98eaa633c31e` exited 134 before producing any JSONL. Raw and
stderr are preserved empty, accepted output is absent, and the 974-byte FAIL disposition SHA-256
is `e722148752733cb16cbfa1534c7bc10d048cea31182ea58c8af4eb1627ee44ce`. It binds the unchanged
binary SHA-256 `242f6789ea994c4147205396bb10c10dbef85a48681160037680bb5b745b8944`
and preflight-seal SHA-256 `85fcfcfb1c72e2dfd1128667c583dfc2aae74b5f183bb4d04dd8604fa07a195d`,
with exact counters runner/workload/timed `1/1/1`, warmup zero, and completed rounds zero.

Read-only source review proves the abort boundary: the first audited `meter_success_full` warmup
calls `drain_all().collect()` inside `audit::in_render_scope`; the audited global allocator aborts
on that `Vec` allocation. Queue draining and evidence hashing are also incorrectly inside the
timed/audited operation. No retry or correction remains in Issue 058. A stateless successor is
required for the launch dependency chain and must own only off-scope/preallocated drain and
evidence separation, nonexecuting armed-render proof, clean reseal, and one newly frozen
exactly-once run while preserving every product, corpus, schema, workload and timing contract.
