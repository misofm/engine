# 064 Seal independent builtin corpus corruption and read-only qualification

## Outcome

Join the three completed builtin corpus semantics slices and seal one immutable, read-only checked
corpus through the exact 24/24 corruption matrix and final nonbenchmark policy gates.

## Context

Stopped Issue 060 could not combine response/scalar PCM, graph/PDC, typed JSONL and final seal in
one correction. Issues 061 and 063 close response/scalar PCM and typed JSONL; stopped Issue 062's
graph/PDC technical input is completed by Issue 067. This issue changes no expected DSP value; it
validates their one joined corpus and becomes the sole corpus dependency of Issue 057.

It permits exactly one Terra attempt and one bounded Sol correction/review. A second failure stops.
Workload, timing and benchmark invocations are forbidden and remain zero.

## Scope

Join the exact accepted Issue-061/063/067 payloads without regeneration or tuning. Execute the
frozen six-class, four-mutation corruption matrix; prove the supplied-root checker cannot reach
generation/production-render/write APIs and leaves a valid tree byte-identical; then run the
focused nonbenchmark repository/policy seal.

## Required public interfaces/contracts

`miso_engine_builtins_fixture --check FIXTURE_DIRECTORY` reads and validates supplied regular
files only. It never calls `generated`, authoring, production rendering or filesystem writes. One
accepted manifest identifies exactly 50 payloads and all checker reports are deterministic.

## Deliverables

Exactly 24 meaningful semantic corruption results, read-only/no-production-reachability proof,
complete candidate/manifest coverage report and strict final corpus verdict.

## Explicit non-goals

Expected-value changes, new cases/payloads/formats, production DSP, realtime audits, graph
lifecycle, targets, instructions, benchmark runner/workload/timing, performance or listening.

## Dependencies by exact issue title

- Complete builtin response cases and scalar PCM semantics
- Reconcile builtin graph fixture and dependent benchmark input identities
- Complete builtin meter, diagnostic, and resource corpus semantics

## Acceptance gates with objective measurements

- The joined corpus has exactly 50 manifest-listed payloads and all accepted semantic checks pass
  without changing bytes.
- For each of TOML, `f32le`, CSV, meter JSONL, diagnostics JSONL and resources JSONL, delete, byte
  alter, unlisted add and manifest-valid semantic coverage hole reject: exactly 24/24.
- Every coverage hole removes one required tuple/path while leaving the payload syntactically
  valid and recomputing its manifest entry; an empty file or stale-manifest rejection does not
  count.
- Static/unit call-graph proof shows `--check` cannot reach generation, production rendering or
  writes; complete tree hashes before and after a valid check are identical.
- Focused fixture/reference tests, format, warning-denied package Clippy and applicable
  nonbenchmark workspace/policy checks pass on one clean candidate.

## Required evidence

Accepted dependency commits; candidate, manifest and payload hashes; exact case/row/path/record
counts; all 24 class/mutation/error identities; read-only proof; strict Terra/Sol verdicts;
`workload_invocations=0`; `timed_benchmark_invocations=0`.

## Terra attempt 1 and final Sol correction evidence

Accepted inputs are Issue 061 commit `f86a6d2`, Issue 063 commit `9533d36` and Issue 067 commit
`092ded7`. No accepted payload changed. The joined `MANIFEST.tsv` SHA-256 is
`bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff` and identifies exactly 50
payloads: 1,652 cases, 1,630 response rows, 33 PCM files, two meter files with 7/15 records,
13 diagnostics, nine resource rows and ten benchmark inputs. Representative sealed payload hashes
remain cases `3f097580addf28280cf0c2aa3709610974e0a92d4ad00ea7267e5359a9ac7091`, response
`c2173a06aa9c2f37c7966d576f7d34dde349e05633941d9e8e4eb6d888fbf53d`, graph PCM
`508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19`, graph meters
`958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`, window/drop meters
`474a89159cb7cd867b01bf84649bf32982a0795ad48979db8f70affa6453c402`, diagnostics
`f8b43cf86100485711f213608bd3a3bfeade6ee4493b6413eb01dcea4582d6dc`, resources
`429b2a1a413eef7dfc7b80f3763bbdb04ada3eaa5435207c72f34deca2ec316e` and metadata
`767c49694e16154ca7a5bfce5c08a3c0cd6df327e1cd752320216e664cb67501`.

Terra replaced generated baselines with exact copies of the accepted checked-in corpus, made all
six semantic holes syntactically valid and manifest-valid, recorded typed class/mutation/rejection
rows, and exercised the real `--check` dispatch over a before/after full-tree hash. Sol correction
updated the TOML hole's truthful stable identity to `cases.toml coverage count differs:
cases=1651 responses=1629`, expanded the static source-region proof across every checker/reference
helper, and repaired the legacy shell mutation's stale cascade-case ID. Neither correction changed
the checker, reference math or corpus.

The exact 24/24 rejection matrix is:

| Class/path | Delete | Stale byte | Unlisted add | Manifest-valid semantic hole |
| --- | --- | --- | --- | --- |
| TOML / `cases.toml` | `FixtureTree` | `ManifestSha256(cases.toml)` | `FixtureTree` | `CasesCoverage(1651/1629)` |
| `f32le` / identity PCM | `FixtureTree` | `ManifestSha256(identity PCM)` | `FixtureTree` | `PcmTuple` |
| CSV / response | `FixtureTree` | `ManifestSha256(response CSV)` | `FixtureTree` | `ReferenceCoverage(1629/1630)` |
| meter JSONL / window-drop | `FixtureTree` | `ManifestSha256(window-drop)` | `FixtureTree` | `MeterTuple(14/15)` |
| diagnostics JSONL | `FixtureTree` | `ManifestSha256(diagnostics)` | `FixtureTree` | `DiagnosticTuple(12/13)` |
| resources JSONL | `FixtureTree` | `ManifestSha256(resources)` | `FixtureTree` | `ResourceGrid(8/9)` |

Every delete/add identity is exactly `fixture tree has missing or unlisted payload files`; every
stale-byte identity is exactly `fixture sha256 mismatch: <selected path>`. Each semantic-hole test
first passes manifest-byte verification, then reaches the named independent semantic validator.
The PCM hole removes one complete dual-mono frame and remains valid planar `f32le`; each JSONL hole
removes one record while leaving a nonempty canonical payload.

The static/unit proof pins `--check` directly to `check_read_only_fixture_root_v1`, scans the whole
checker/reference region against authoring, production render and filesystem mutation entry points,
and executes the accepted corpus through that dispatch. Path/length/bytes including the manifest
are identical before and after.

Final nonbenchmark gates:

- exact 24/24 accepted-corpus corruption test: PASS;
- exact accepted-corpus read-only/no-author-reachability test: PASS;
- checked-in `--check` and `scripts/check-builtins-fixtures.sh`: PASS, 50 files;
- fixture shell mutation suite: PASS after the stale fixture-ID repair;
- focused DSP-reference tests: PASS, seven unit and three integration tests, with the two separately
  frozen EQ execution tests ignored;
- warning-denied all-target fixture Clippy, format, shell syntax, workspace policy/mutations and
  diff checks: PASS.

**FINAL SOL VERDICT: PASS.** This immutable corpus unblocks **Builtin direct and graph realtime
audit closure**. No authoring or expected-byte regeneration was rerun;
`workload_invocations=0`; `timed_benchmark_invocations=0`; benchmark runner, preflight, workload
and timing invocations remain zero.
