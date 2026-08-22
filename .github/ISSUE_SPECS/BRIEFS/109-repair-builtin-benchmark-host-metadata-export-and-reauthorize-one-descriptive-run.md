# Sol implementation brief — issue 109 builtin benchmark host-metadata repair

## Decision

**SOL XHIGH BRIEF PASS / READY FOR SOL HIGH PASS 1; ZERO EXECUTION AUTHORIZATION.** Sol High may
make one focused implementation pass and, after one Sol XHigh HOLD only, one bounded correction.
Sol XHigh briefs and adversarially verifies; a second HOLD stops. Initial Issue-109 preflight/
runner/workload/timed counters are `0/0/0/0`, `target/issue109` is absent, and no preflight, runner,
main, workload or timing invocation is authorized. Remote #109 was read-only verified available;
root must create it with the exact spec title before implementation.

Sol High pass 1 has a focused-green live checkpoint using only the five successor scripts and this
tracked evidence. Hermetic complete/unavailable metadata cases, required and malformed discovery,
ambient clearing, every-row projection rejection, phase/counter/no-clobber behavior, direct and
tandem seal mutations, shell syntax and 33 static mutations pass. Real Issue-109 counters remain
`0/0/0/0`, `target/issue109` remains absent and Issue-072 evidence remains byte-exact. This grants
no seal, preflight or runner authorization and is pending Sol XHigh review.

## Exact defect and immutable input

Issue 072's accepted render-only timer/audit correction produced the exact 20-record matrix with
zero render violations, but its sole runner omitted all host/build metadata. The Rust tool reads 16
fixed `MISO_ENGINE_BENCH_*` variables; the runner supplied only candidate and binary identities.
All records therefore carried 16 nulls and the honest 16-name missing list, including facts the
host could supply. Historical counters `1/1/1/1` and every `target/issue72` artifact are consumed
terminal evidence, not PASS and not reusable authority.

Preserve the seven Issue-072 files exactly as the spec table freezes them: hashes beginning
`7c38b068` (seal), `a7bafc45` (binary), `f4e624b8` (preflight), `c44433bc` (raw and accepted),
`7935bf62` (stderr), and `b650449d` (disposition), with their exact sizes, nlink1 state, distinct raw/
accepted inodes and absent prelaunch disposition. Never write beneath `target/issue72`.

Dependencies are exact title **Builtin native, AArch64, and Wasm runtime-selection and instruction
qualification** as accepted product input and exact title **Separate builtin benchmark render timing
from meter evidence collection** as stopped technical input only.

## Literal implementation

Do not edit Rust or existing scripts/validators. Add only five `-109` scripts: runner, preflight,
hermetic lifecycle, static checker and checker-mutation suite.

Before its one launch, the successor runner must clear ambient values and derive/export the exact
16 variables consumed by `Metadata::collect`. Require usable values for architecture (`uname -m`),
logical cores (`getconf _NPROCESSORS_ONLN`), OS/kernel (`uname -s/-r`), Rust/LLVM/host target
(`rustc -V/-vV`), sorted target features (`rustc --print cfg`), and fixed release build facts
profile `release`, opt level `3`, LTO `false`, codegen units `16`. Capture CPU model, physical cores,
scaling governor and prelaunch load average when their documented `/proc`, sysfs or `lscpu` source
is available; only a genuinely unavailable/empty optional source may remain null and appear in the
sorted missing list. Sentinels, invalid counts, malformed output and ambient overrides reject.

Construct an exact expected metadata projection before launch. After the frozen aggregate validator
passes, compare every metadata field and `missing_metadata` in every row to that projection. Bind
the projection SHA-256 in the terminal disposition. Metadata collection and validation are outside
the workload and every measured interval.

The benchmark contract is unchanged: schema v2, records `issue=35`, five workloads, 48/96 kHz,
quantum 128, one warmup, two rounds, 20 records, nearest-rank nanoseconds/operation, unchanged
inputs/output hashes/order and zero audit categories. Tool source, manifests, validators, lock,
render/timer logic and product code are read-only.

## Fake/static checkpoint

The hermetic test uses fake tools, files and benchmark output only. Cover complete available
metadata, each optional-unavailable case, each required-source failure, ambient spoofing, sentinel/
control/malformed/nonpositive values, dishonest row projection/missing list, arguments/tools/dirty
candidate/seal failures, regular/symlink/hardlink no-clobber, partial evidence, exact phases,
distinct-inode accepted copy, raw mutation isolation and second-call refusal. Every prelaunch
failure has zero fake launches and a consumed prelaunch disposition. No real main or clock runs.

The static checker and mutations pin the five allowed scripts, all 16 mappings and source rules,
fixed build values, one launch site, `target/issue109`, issue-109 authorities/counters, unchanged
tool/validators/lock/fixtures and exact Issue-072 historical artifacts/absence. Run only these
fake/static gates, shell syntax, frozen validator checks, and text/diff/artifact sanity. A locked
compile-only benchmark-package check is proportional if needed. No broad, target, audit, trace,
preflight, runner, workload or timing gate.

Sol High pauses at a focused-green checkpoint for Sol XHigh review. One HOLD permits only a bounded
correction; a second is STOP.

## Successor seals and exactly-once barriers

All new files live under `target/issue109`: `metadata-repair.seal.json`, sealed binary, preflight
seal, raw, accepted, stderr, prelaunch disposition and final disposition. Each is regular, one-link,
atomic and no-clobber. Either disposition consumes runner authority.

After the clean implementation commit, root creates only the repair seal binding HEAD/tree, lock,
unchanged tool/validators/fixtures, all five successor scripts, frozen Issue-072 evidence,
metadata-regression count `1` and counters `0/0/0/0`. Sol XHigh must validate it before exactly one:

`bash scripts/preflight-builtins-benchmark-109.sh`

Preflight builds but does not execute the binary and publishes only binary/preflight seal with
counters `1/0/0/0`, warmup `1`, rounds `2`, records `20`. Failure is final; never repeat it.

Sol XHigh separately validates that preflight before exactly one:

`bash scripts/run-builtins-benchmark-109.sh`

No arguments, environment override, direct binary, alternate invocation, retry, resume, tuning or
comparison. Success is 20 exact records, complete honest available metadata, missing only for truly
unavailable optional sources, zero render violations, raw/accepted equality with distinct inodes,
exact five phases, PASS/complete disposition and counters `1/1/1/1`. Any failure is terminal.

## Exact paths and STOP conditions

Allowed tracked paths are the two #109 docs, minimal #72 spec/brief and README routing, plus the five
new `scripts/*-109.sh` paths named in the spec. `target/issue109` is ignored evidence. STOP for any
Rust, Cargo, validator, existing benchmark script, fixture, product, schema, timer, workload, rate,
output-hash, Issue-072 artifact or unrelated policy change.

PASS remains descriptive only and does not complete human listening or release readiness. Root owns
the evidence commit, remote synchronization and closure after final Sol XHigh PASS.
