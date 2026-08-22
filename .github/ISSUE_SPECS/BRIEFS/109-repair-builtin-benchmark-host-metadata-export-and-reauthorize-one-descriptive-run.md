# Sol implementation brief — issue 109 builtin benchmark host-metadata repair

## Decision

**TERMINAL STOP / NO OVERALL PASS / ZERO EXECUTION.** Sol High landed the focused metadata-runner
checkpoint at `5b2744f` and the one permitted lifecycle correction at
`f6e8a6b5936f9d578e050a6b85242c4234a1a886`. Sol XHigh accepted both as technical checkpoints.
During the first handoff, however, root created the sole exact-path repair seal while the lifecycle
correction was still being applied. The seal binds lifecycle SHA-256
`bbdb316775895e8d82e9d6d0696653466fb26cdf423502b23040ee748ff381ca` instead of the final
`14dd9ec48921fefdba8a57afa827f27222dc881311929f55a44d0edc89c97ef4`; it is immutable stale
evidence at `target/issue109/metadata-repair.seal.json`, regular nlink1, 2,538 bytes, SHA-256
`1e8cec4904d8987ddca581e5b23870629d734127ad3f9e010f6a5c2d178b69c6`.

No-clobber forbids replacement or movement, the exact namespace forbids a second repair seal, and
the issue's one-correction budget forbids another implementation pass. All seven future Issue-109
outputs remain absent and preflight/runner/workload/timed counters remain `0/0/0/0`; no benchmark
main or timing path executed. Preserve the entire Issue-109 namespace. Issue 110, **Complete builtin
benchmark host metadata run from a fresh successor namespace**, owns stateless completion.

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

## Consumed Issue-109 seal barrier

All new files live under `target/issue109`: `metadata-repair.seal.json`, sealed binary, preflight
seal, raw, accepted, stderr, prelaunch disposition and final disposition. Each is regular, one-link,
atomic and no-clobber. Either disposition consumes runner authority.

The intended sequence required root to create only the repair seal after the final clean implementation commit, binding HEAD/tree, lock,
unchanged tool/validators/fixtures, all five successor scripts, frozen Issue-072 evidence,
metadata-regression count `1` and counters `0/0/0/0`. Sol XHigh would then have validated it before exactly one:

`bash scripts/preflight-builtins-benchmark-109.sh`

That preflight was never authorized or executed. It would have built but not executed the binary and published only binary/preflight seal with
counters `1/0/0/0`, warmup `1`, rounds `2`, records `20`. Failure is final; never repeat it.

The separately reviewed runner command was also never authorized or executed:

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

Issue 109 cannot attain PASS without violating its no-clobber, exact-path or pass-budget contract.
Its status is terminal STOP; Issue 110 owns the fresh namespace. Root owns evidence commit and
remote synchronization. No result here completes human listening or release readiness.
