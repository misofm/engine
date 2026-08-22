# 110 Complete builtin benchmark host metadata run from a fresh successor namespace

## Outcome and status

Complete the already reviewed builtin benchmark metadata-runner correction from a fresh,
no-clobber namespace after Issue 109 consumed its only repair-seal path before the final lifecycle
commit. This is authority/lifecycle completion only. Preserve the benchmark product, timer,
schema, workloads, rates, inputs, output digests and metadata semantics byte-for-byte.

**COMPLETE / SOL XHIGH PASS / READY TO CLOSE / NO FURTHER EXECUTION AUTHORIZATION.** Sol High
implemented one focused pass and the one bounded correction authorized by Sol XHigh; Sol XHigh
independently reviewed the completion seal, sole preflight and sole runner evidence. Final
Issue-110 preflight/runner/workload/timed counters are exactly `1/1/1/1`, with warmup `1`, measured
rounds `2` and records `20`. The result is descriptive only.

Remote issue 110 was read-only confirmed absent/available on 2026-08-22. Root owns GitHub evidence
synchronization and closure after this docs checkpoint is upstream. This local record claims no
GitHub mutation or remote synchronization.

## Dependencies by exact issue title

- Builtin native, AArch64, and Wasm runtime-selection and instruction qualification
- Separate builtin benchmark render timing from meter evidence collection
- Repair builtin benchmark host metadata export and reauthorize one descriptive run

Issue 068 is accepted product/target input. Issue 072 is stopped technical timing/audit input with
its immutable terminal artifacts. Issue 109 is stopped technical metadata-runner input, not PASS
and not reusable authority. This issue neither reopens nor weakens those records.

## Immutable predecessor evidence

Preserve every `target/issue72` artifact exactly as Issue 109 freezes it, including seven regular
one-link files, byte-identical distinct-inode raw/accepted output and absent prelaunch disposition.
Never create, delete, truncate, rename, replace, link from or write beneath `target/issue72`.

Preserve `target/issue109` byte-for-byte. Its sole member is the stale, non-authoritative regular
one-link `metadata-repair.seal.json`, 2,538 bytes, SHA-256
`1e8cec4904d8987ddca581e5b23870629d734127ad3f9e010f6a5c2d178b69c6`. The Issue-109 sealed binary,
preflight seal, raw, accepted, stderr, prelaunch disposition and final disposition remain absent.
Historical Issue-109 preflight/runner/workload/timed counters are permanently `0/0/0/0`; no
Issue-109 benchmark main or timing path executed. Never use the stale seal as authority and never
write beneath `target/issue109`.

Issue-109 implementation commits `5b2744f` and
`f6e8a6b5936f9d578e050a6b85242c4234a1a886` are accepted technical input only. The final reviewed
lifecycle SHA-256 is `14dd9ec48921fefdba8a57afa827f27222dc881311929f55a44d0edc89c97ef4`.

## Smallest closable implementation

Create successor-named copies/routes of the five reviewed Issue-109 scripts:

- `scripts/run-builtins-benchmark-110.sh`;
- `scripts/preflight-builtins-benchmark-110.sh`;
- `scripts/test-builtins-benchmark-110.sh`;
- `scripts/check-builtins-benchmark-110.sh`; and
- `scripts/test-builtins-benchmark-110-policy.sh`.

The change is mechanical except for successor authority: use `issue=110`, branch/candidate fields
for Issue 110, the exact `target/issue110` paths below, successor script hashes and the frozen
Issue-072/109 evidence. Do not edit or execute any Issue-109 script. Do not change metadata
discovery, validation, environment names, record comparison, phase accounting, workload launch or
accepted-output promotion semantics that Sol XHigh accepted at `f6e8a6b`.

The runner still clears all 16 ambient `MISO_ENGINE_BENCH_*` values before discovery. Required
values remain architecture from `uname -m`, positive logical cores from
`getconf _NPROCESSORS_ONLN`, OS/kernel from `uname -s/-r`, Rust/LLVM/host target from
`rustc -V/-vV`, sorted unique target features from `rustc --print cfg`, and fixed release facts
`release`/`3`/`false`/`16`. CPU model, unique physical core/socket count, scaling governor and the
prelaunch load-average triple with `not-controlled` remain optional only when their fixed source is
genuinely unavailable or empty. Empty/control/sentinel/malformed values, invalid counts, duplicate
features and ambient substitution reject before launch.

Construct one canonical expected metadata projection before raw creation. After the frozen
aggregate validator passes, every one of the 20 records must equal that projection for all 16
fields and the exact sorted `missing_metadata` list. Bind the projection SHA-256 in the terminal
disposition. Metadata collection, projection construction and post-run validation stay outside all
measured intervals.

## Frozen product and benchmark contract

No Rust, Cargo manifest/lock, validator, existing benchmark script, fixture, product, timer,
workload, rate, record-schema or output-digest change is allowed. Preserve:

- benchmark tool source SHA-256
  `b520e3d14bd4fa2985d18f273e515261a53b4ea69ac1a2a38aba9bc77bf6e7fe` and manifest
  `f361c26b6a59c984a9fc60484748b5a2fd0bd0c35079e83ee72d3932f118cf97`;
- record and aggregate validators
  `c3db1d9574360bdab0d9ac335615787446e5537439d6accdded4fdd0a4479467` and
  `6085e740f15d7902fca4443d761cfb8e29df7168ba12f632c7946db56a3e1b63`;
- `Cargo.lock` SHA-256
  `4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a`;
- builtin manifest, graph PCM and graph meter SHA-256 values
  `bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`,
  `508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19` and
  `958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`;
- accepted Issue-068 source identity
  `0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19`; and
- schema version 2 records with `issue=35`, five workloads, 48/96 kHz, quantum 128, one untimed
  warmup, two measured rounds, 20 records, nearest-rank nanoseconds per operation, frozen
  inputs/output hashes/order and zero render-audit categories.

The eventual result is descriptive only. No threshold, comparison, tuning, optimization, capacity,
quality, listening or release claim follows from its values.

## Fresh namespace and no-clobber lifecycle

Persistent Issue-110 paths are exactly:

- `target/issue110/completion.seal.json`;
- `target/issue110/miso_engine_builtins_bench`;
- `target/issue110/builtins-benchmark.preflight.json`;
- `target/issue110/builtins-benchmark.raw.jsonl`;
- `target/issue110/builtins-benchmark.jsonl`;
- `target/issue110/builtins-benchmark.validator.stderr`;
- `target/issue110/builtins-benchmark.prelaunch.disposition.json`; and
- `target/issue110/builtins-benchmark.disposition.json`.

Every persistent file is regular, one-link and no-clobber. Seals, the binary, accepted output and
dispositions use same-directory atomic publication. Raw and stderr are each created once with
noclobber so postlaunch partial evidence remains available. The runner accepts no arguments, path
override, environment-selected binary, retry, resume, alternate entrypoint or direct binary
execution. A preexisting prelaunch or final disposition blocks before scratch creation. Every
prelaunch failure consumes authority through the prelaunch disposition; every postlaunch failure
preserves raw/stderr and publishes the final FAIL disposition. Either disposition is terminal.

## Focused nonexecuting gates

The successor hermetic lifecycle uses temporary fake tools, metadata sources and binary only. It
must retain the reviewed complete/optional-unavailable/required-failure/malformed metadata matrix,
ambient clearing, canonical all-row projection rejection, authority drift, direct/tandem seal
mutations, argument/tool/dirty failures, exact phase-derived counters, regular/symlink/hardlink
no-clobber, partial preservation, distinct accepted inode, raw mutation isolation and second-call
refusal. It must additionally prove all real Issue-110 counters remain `0/0/0/0`, the Issue-109
stale seal is read-only and no Issue-109 future artifact appears.

The static checker and mutation suite pin the five allowed scripts, fixed frozen identities,
Issue-072 and Issue-109 evidence, all 16 metadata mappings/source rules, fixed build values, one
launch site, the exact Issue-110 namespace and initial counters. Only shell syntax, the fake/static
gates, frozen validator read-only checks, a compile-only locked benchmark-package check if needed,
and text/diff/artifact sanity are allowed before checkpoint review. Do not execute a real
preflight, runner, benchmark main, workload, clock, inherited matrix, target, audit, trace, fuzz or
listening gate.

## Checkpoints and authorization barriers

### Checkpoint 1 — fresh successor routes

Sol High lands the five successor scripts and these two docs in one focused pass, records exact
nonexecuting evidence and pauses. Sol XHigh returns focused PASS or one bounded HOLD. Root commits
the exact paths before any ignored authority artifact is created.

On that final clean commit, root creates exactly one no-clobber
`target/issue110/completion.seal.json`. Its closed schema binds branch/HEAD/tree, lock, frozen
tool/validators/fixtures, all five Issue-110 scripts, exact Issue-072 evidence, exact stale
Issue-109 evidence/absences, `metadata_regressions=1` and Issue-110 counters `0/0/0/0`. Sol XHigh
must independently validate it before any execution authorization.

### Checkpoint 2 — sole zero-workload preflight

Only a strict Sol XHigh GO may authorize exactly one:

`bash scripts/preflight-builtins-benchmark-110.sh`

The no-argument preflight validates the clean candidate and completion seal, runs only the frozen
nonexecuting gates, builds but never executes the unchanged benchmark binary, and atomically
publishes only the binary and preflight seal. The seal binds every authority, warmup `1`, rounds
`2`, records `20`, and counters `1/0/0/0`. Failure is terminal; there is no repeat or alternate.

Sol XHigh then independently validates exact seal/hash/schema/membership state, unchanged
Issue-072/109 evidence, and absent raw/accepted/stderr/dispositions. Only a separate strict GO may
authorize exactly one:

`bash scripts/run-builtins-benchmark-110.sh`

The runner launches only the sealed binary once. Success requires 20 frozen-validator-valid
records, exact metadata-projection equality, zero render violations, byte-identical distinct-inode
raw/accepted output, exact five-phase stderr and PASS/complete disposition counters `1/1/1/1` with
warmup `1` and rounds `2`. Any failure is terminal STOP. No more benchmark activity is authorized.

## Exact allowed paths

- this spec and its tracked brief;
- minimal Issue-109 spec/brief and `ISSUE_SPECS/README.md` terminal/successor routing;
- `scripts/run-builtins-benchmark-110.sh`;
- `scripts/preflight-builtins-benchmark-110.sh`;
- `scripts/test-builtins-benchmark-110.sh`;
- `scripts/check-builtins-benchmark-110.sh`; and
- `scripts/test-builtins-benchmark-110-policy.sh`.

No other tracked path may change. `target/issue110` contains ignored authority/evidence only after
the explicit barriers above. Any need to edit Rust, an existing script/validator, product, schema,
timer, workload, rate, digest, fixture or predecessor artifact is STOP and rebrief.

## PASS boundary

PASS requires the frozen pass count and Sol XHigh verdicts; exact clean completion/preflight seals;
all authority hashes; preserved Issue-072/109 evidence; one authorized preflight and one separately
authorized runner; 20 exact records with honest available metadata; raw/accepted/stderr/disposition
hashes, sizes and inode counts; phase-derived counters `1/1/1/1`; warmup `1`; rounds `2`; zero
render violations; and an explicit descriptive-only statement.

Only then may Issue 110 be marked `SOL XHIGH PASS / COMPLETE / READY TO CLOSE`. Root owns commits,
GitHub synchronization and closure. This issue does not close listening or release readiness.

## Sol High checkpoint-1 evidence

On base commit `d46e5a96d3f294c601674f5d2b0205d0a96a4ac2` / tree
`186db283e386c11edc2e705a8164da5e6b7f9627`, Sol High implemented the five fresh successor
scripts without editing Rust, Cargo, validators, fixtures, product code, predecessor scripts or
predecessor artifacts. Their checkpoint SHA-256 identities are:

- runner `a014ae7fa90ab140b2d7529564a19a9c5d3d7105da1bb00541943f1c26df2089`;
- preflight `20faec32e735e9f314d21729cb7737e202b8a9637d600491308ac98fee4a3893`;
- lifecycle `5de8eceebfb1ae1265c6a0ab0adc7eacefa4104ef1856cb89cd7084f96b71675`;
- checker `9b6d808604f919f226a24de1bcf99e3e2e90395839e143beb07a62c9d824855a`;
- mutation suite `2dce3f5b8189dbf8667d2d5c25aa6bd53408722336228e019f3c5496b8eabde1`.

Focused nonexecuting evidence passed: shell syntax; static checker; 37 rejected static mutations;
the complete hermetic metadata/lifecycle matrix; read-only validation of all 20 inherited records;
locked all-target benchmark-package compilation; and workspace format checking. The checker and
fakes prove the exact seven Issue-072 artifacts and distinct raw/accepted inodes, exact stale
Issue-109 seal and seven absences, closed successor authority, one launch site, no-clobber and
initial counters `0/0/0/0`. `target/issue110` remains absent. Real preflight, runner, benchmark
main, workload and timed invocation counts remain `0/0/0/0`.

The bounded Sol XHigh correction added exact sole-member Issue-109 namespace enumeration to the
runner, preflight and checker, plus zero-launch/zero-build arbitrary-extra-member fake mutations.
Sol XHigh returned focused PASS, then separately authorized and verified the sole preflight and sole
runner.

## Terminal Sol XHigh evidence and verdict

The clean tracked candidate is commit `47daeda00683acb6e0fd29bafd3ee6d6403cd782`, tree
`1f51a7bba86bbe34afb18567272faa2dc86bc397`. The sole preflight exited zero and published only the
sealed binary plus preflight seal with counters `1/0/0/0`; the sole runner then exited zero and
printed exactly the accepted-output path. No retry, alternate, direct invocation or tuning occurred.

Final `target/issue110` membership is exactly seven regular nlink1 files; the prelaunch disposition
is absent:

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| `completion.seal.json` | 2,988 | `3ce39b2653d6b912b6ede083fe8479e46bcbce665095190bd94d15fe82ca238d` |
| `miso_engine_builtins_bench` | 3,200,296 | `a7bafc459b69fb8bdfd7d9195e4ff8d1febf8602a57540498cb579d04a486912` |
| `builtins-benchmark.preflight.json` | 1,893 | `9a7a78748b32d8a7cdee1bf7e886e38e6a358f6dfd093d93bbd51bdac2eddaa0` |
| `builtins-benchmark.raw.jsonl` | 38,477 | `8a2d3f2f9f6d5a6f2edb4513fd304b121c934f6dcc1f5379b96f4256b54aa2dc` |
| `builtins-benchmark.jsonl` | 38,477 | `8a2d3f2f9f6d5a6f2edb4513fd304b121c934f6dcc1f5379b96f4256b54aa2dc` |
| `builtins-benchmark.validator.stderr` | 211 | `7935bf62063c0e9d2bfaac91d02db6f448dbf0636fbf16d3c49660738f55b396` |
| `builtins-benchmark.disposition.json` | 1,075 | `361f3a4f612e88dcc8a6dcb9f810528b175a64fbf3eea07122024df7971f274f` |

Raw and accepted are byte-identical distinct-inode files. Stderr contains exactly the five ordered
phase lines `workload_started`, `warmup_complete`, `timed_started`, `round_1_complete` and
`round_2_complete`. The closed disposition is `PASS` / `complete`, workload exit status zero,
counters `1/1/1/1`, warmup `1`, rounds `2`, and metadata-projection SHA-256
`59efa293fd6781d8da916490621f0973475b4dc4cbff7f5bd92baccb833d095f`.

The raw file is the exact ordered two-round matrix of five workload kinds at 48/96 kHz. All 20
records retain the frozen candidate, binary, fixture/input/output identities and schema, monotonic
nearest-rank percentiles, and identical complete metadata with `missing_metadata=[]`: AMD Ryzen 7
9700X, `x86_64`, 16 logical/8 physical cores, Linux `6.8.0-138-generic`, `powersave`, Rust 1.97.1,
LLVM 22.1.6, target `x86_64-unknown-linux-gnu`, features `fxsr,sse,sse2`, release/opt 3/LTO false/
16 codegen units, and load `0.76,0.42,0.21;not-controlled`. The independently reconstructed
canonical projection hashes exactly to the disposition value.

All four render workloads report zero errors and zero in every forbidden-operation field; the
preparation workload reports the frozen `not_applicable` audit values. Descriptive p50 ranges across
the two rates and rounds are: full chain 2,298–2,306 ns/op, identity 1,009–1,029 ns/op, matrix
1,442–1,446 ns/op, meter 25,408–31,430 ns/op, and 256-track preparation 890,644–957,490 ns/op.
These are rough period observations only, with no threshold, comparison, tuning, optimization,
capacity, quality, listening or release claim.

Issue-072 artifacts/inode separation and exact sole-member Issue-109 namespace were preserved after
the run. All acceptance conditions are met. Issue 110 is `COMPLETE / SOL XHIGH PASS / READY TO
CLOSE`; no further preflight, runner, benchmark-main, workload or timing execution is authorized.
