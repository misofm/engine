## TOOL-3 audit result

TOOL-3 remains partially open, but the historical blanket claim is false.

The original six subjects are identifiable exactly from the audited baseline’s 4,720 lines:

| Subject | Module at baseline | Current disposition |
|---|---:|---|
| `builtins-fixture` | 550 | Retain as fixture author/check maintenance interface |
| `capi` | 342 | **Unresolved: unique audit has no caller; wire into qualification** |
| `fixture-builtins-listening` | 850 | Live operator caller |
| `fixture-source` | 1,174 | Retain; its complete check runs through release unit tests |
| `source-duration` | 336 | **Unresolved: unique full-duration audit has no caller; wire into qualification** |
| `unfused-fma` | 1,468 | Retain as ruling-backed reproducer and required seal evidence source |

`builtins` and `builtins-graph`, cited by the handoff as newly wired by #319, are not members of the original six. Their wiring does not settle TOOL-3.

No original subject is presently justified for wholesale deletion. The smallest complete implementation is to execute the two unique, currently uncalled audits—`capi` and `source-duration`—in the existing release audit job. No dispatcher, audit module, fixture, script, DSP, or orphan-tool cleanup is needed.

### Current evidence

- All six remain declared and dispatched in [tools/audit/src/main.rs](/home/bl/misofm/engine/tools/audit/src/main.rs:8), [SUBJECTS](/home/bl/misofm/engine/tools/audit/src/main.rs:30), and [run_subject](/home/bl/misofm/engine/tools/audit/src/main.rs:52).
- `builtins-fixture` exposes a guarded author/check interface at [builtins_fixture_check.rs](/home/bl/misofm/engine/tools/audit/src/builtins_fixture_check.rs:33). Its checker and mutation test remain active at [line 519](/home/bl/misofm/engine/tools/audit/src/builtins_fixture_check.rs:519), while the resulting accepted fixtures are consumed by the real builtins audit at [builtins.rs](/home/bl/misofm/engine/tools/audit/src/builtins.rs:13).
- `capi` uniquely executes 100,000 exported C render calls under the realtime audit scope and asserts zero render errors, address changes, and forbidden operations at [capi.rs](/home/bl/misofm/engine/tools/audit/src/capi.rs:149). Its CLI entry is [line 268](/home/bl/misofm/engine/tools/audit/src/capi.rs:268), but the current qualification job only builds/tests the audit binary and separately checks C linkage; it never invokes this subject ([qualification.yml](/home/bl/misofm/engine/.github/workflows/qualification.yml:529)).
- `fixture-builtins-listening` has a real current caller at [prepare-builtins-listening.sh](/home/bl/misofm/engine/scripts/operator/prepare-builtins-listening.sh:44), invoking the subject at [line 57](/home/bl/misofm/engine/scripts/operator/prepare-builtins-listening.sh:57).
- `fixture-source`’s CLI and release test call the same complete `check()` path: [source_fixture.rs](/home/bl/misofm/engine/tools/audit/src/source_fixture.rs:43) and [line 1177](/home/bl/misofm/engine/tools/audit/src/source_fixture.rs:1177). Release audit tests run in qualification at [qualification.yml](/home/bl/misofm/engine/.github/workflows/qualification.yml:531). Its dispatcher route adds no untested 1,181-line implementation.
- `source-duration` uniquely compares a materialized one-minute WAVE with a sparse three-hour WAVE and requires identical allocation layouts and source/graph resource reports at [source_duration.rs](/home/bl/misofm/engine/tools/audit/src/source_duration.rs:25). Its unit test only checks a one-quantum accounting pin at [line 316](/home/bl/misofm/engine/tools/audit/src/source_duration.rs:316); it does not execute the duration comparison.
- `unfused-fma` is explicitly retained evidence code at [unfused_fma.rs](/home/bl/misofm/engine/tools/audit/src/unfused_fma.rs:1), with its runnable modes at [line 1258](/home/bl/misofm/engine/tools/audit/src/unfused_fma.rs:1258). The current ruling names it as the reproducible audit instrument at [unfused-multiply-add-audit.md](/home/bl/misofm/engine/docs/rulings/unfused-multiply-add-audit.md:246), and the required seal registers seven intentional fused-reference calls from it at [check-unfused-seal.sh](/home/bl/misofm/engine/scripts/check-unfused-seal.sh:475). The seal is required qualification at [qualification.yml](/home/bl/misofm/engine/.github/workflows/qualification.yml:350).

## Proposed stateless issue

**Title:** Close TOOL-3 by executing the two unique uncalled audit subjects

**Body:**

> TOOL-3 originally identified six `tools/audit` dispatcher subjects: `builtins-fixture`, `capi`, `fixture-builtins-listening`, `fixture-source`, `source-duration`, and `unfused-fma`. The historical 4,720-line total identifies that population but is not a current dead-code measure.
>
> Current inspection establishes:
>
> - `fixture-builtins-listening` has a live operator caller.
> - `fixture-source` executes its complete checker through the required release audit tests.
> - `builtins-fixture` remains the guarded author/check maintenance interface for fixtures consumed by the live builtins audit, with its read-only checker mutation-tested.
> - `unfused-fma` remains the ruling-backed reproducer and supplies the intentionally fused reference arms registered by the required unfused seal.
> - `capi` and `source-duration` contain unique executable claims that neither required qualification nor their unit tests currently exercise.
>
> Add exactly two invocations to the existing `audit-native` release job in `.github/workflows/qualification.yml`:
>
> 1. Run `target/release/audit capi` and validate its single JSON record: schema/kind, 100,000 calls, 48 kHz/128 frames, stable output address, zero render errors, and all nine forbidden-operation counters plus total violations equal zero.
> 2. Run `target/release/audit source-duration` and validate its single JSON record: one-minute and three-hour frame identities, equal allocation layout/source report/graph report, the current exact 17-entry/6,416-byte accounting contract, and `timed_benchmark_invocations == 0`. RSS remains descriptive; introduce no unstable RSS threshold.
>
> Allowed implementation path: `.github/workflows/qualification.yml` only, plus the root-owned numbered spec/evidence.
>
> Do not edit `tools/audit` source, its dispatcher, Cargo dependencies, fixtures, DSP code, scripts, historical rulings, benchmark artifacts, or unrelated orphan-tool cleanup. Do not delete or rename any of the six subjects. Do not add a generic reachability framework.
>
> Implementation: Luna high. Non-audio verification: Sol xhigh. Audio coordination: Sol medium. Audio/correctness verification: Astra medium. Root owns issue creation, worktrees, checkpoints, GitHub synchronization, delivery, and cleanup.

### Objective gates

- `cargo test --locked --release -p audit` remains green.
- The `capi` record satisfies every field above and exits successfully.
- The `source-duration` record satisfies every field above and exits successfully.
- `cargo fmt --all -- --check` and workflow syntax validation pass.
- Required `qualification` succeeds on the exact reviewed commit.
- Diff inspection proves only `.github/workflows/qualification.yml` and root-owned issue evidence changed.
- Zero benchmark invocations, fixture regeneration, human listening, or performance claims.

### Audio impact

No DSP equations, arithmetic order, PCM fixtures, session behavior, latency, state, resource calculation, or render implementation changes. The C-ABI audit hashes PCM but does not establish a new PCM pin; the source-duration audit performs preparation/accounting and reports RSS descriptively. No listening work is required.

### Overlap

- **CP20:** none; TOOL-3 must not edit audit/production hex encoders.
- **IO1:** none; no protocol source changes.
- **TOOL14:** none if TOOL14 stays within its orphan binary/validator inventory. TOOL-3 expressly excludes FLAC cleanup, deleted `sweep.sh` references, and general tool-orphan removal.
- **#359 qualification design:** direct workflow overlap; preserve triggers, routing, leaf expectations, verdict logic, and the single required `qualification` context.
- **#411/#438/#402:** no changes to `check-unfused-seal.sh` or its registry.
- **#509:** no changes to the previously consolidated audit hash helpers.

No files were edited, no builds/tests/timing were run, and no agents or Git/GitHub mutations were launched.