# 108 Repair effect-interchange benchmark migration fixture and reauthorize one descriptive run

## Outcome and status

Repair only the invalid migration descriptors embedded in the Issue-081 benchmark tool, prove the
real migration fixture without timing, bind its newly reachable canonical output digest through a
successor-only no-clobber lifecycle, and create a fresh authorization barrier for at most one
descriptive run.

**SOL XHIGH BRIEF PASS / READY FOR SOL HIGH ATTEMPT 1.** Sol High implements one coherent attempt;
Sol XHigh adversarially verifies it. At most one bounded Sol High correction is permitted before
preflight. A second failed implementation/review pass, any failed real preflight, or any failure
after the successor runner launches is STOP/rescope, never a weakened gate or retry.

At briefing, Issue-108 counters are
`benchmark_preflight_invocations=0`, `benchmark_runner_invocations=0`,
`benchmark_workload_invocations=0`, and `timed_benchmark_invocations=0`. A focused correctness test
is not a benchmark invocation and emits no timing record. Remote issues 083 and 107 were read-only
verified occupied and open on 2026-08-22; remote issue 108 does not exist. Root must create issue
108 with the H1 title in the docs checkpoint before implementation. This local brief does not
claim remote creation or synchronization.

Attempt-1 focused implementation derives the reachable four-rate D1→D2→D3 final-envelope SHA-256
as `5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777`. The shared untimed Rust
regression validates the exact 283-byte envelope and eleven-byte payload and confirms the digest is
different from the unreachable Issue-081 value. Issue-108 preflight/runner/workload/timed counters
remain `0/0/0/0`; no real benchmark authority has been invoked. This is checkpoint evidence only,
pending Sol XHigh review, not an overall PASS or run authorization.

## Dependencies by exact issue title

- Canonical effect interchange qualification, fuzzing, and benchmark
- Prepared effect state envelope and transactional current-layout restore
- Effect state migration registry and bounded chains
- Close canonical effect descriptor wire, identity, and C inspection ABI

Issue 081 is a terminal evidence dependency, not an accepted overall PASS. Issues 079, 080 and 082
supply the accepted product contracts. No dependency is reopened or changed here.

## Preserved Issue-081 terminal authority

Issue 081's sole runner on clean commit `466b05cbf2bb61e0367d25aa6ca6a0da7643e83f`, tree
`2e1c5c12515e7b16d8a36846130cfe4cde42ad55`, failed before warmup or timing because each local
`MIGRATION_Q1..Q3` table advertised only 48 kHz. The accepted descriptor validator correctly
requires each quality to contain all four launch rates. Historical counters remain reference `1`,
mutation `1`, migration matrix `1`, cross-target `2`, benchmark preflight `1`, runner `1`, workload
`1`, timed `0`.

The following regular, one-link files under `target/issue081` are immutable evidence:

- `nonbenchmark.seal.json`: 833 bytes,
  `6d08e2089e806dc366f5c1171398c241f8dfdc520f97808c4e2f6c7f6b83363c`;
- `miso_engine_effect_interchange_bench`: 827,232 bytes,
  `fad8e39ecd9efa6908b51e7e98c25984f9d97f88b32971581c9a880228758b4c`;
- `benchmark-preflight.seal.json`: 1,577 bytes,
  `da3c537c16d55b1e71b8aa9f8e4d011796b243e4c6c7969020097098a75035a3`;
- empty `benchmark.raw.jsonl`:
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`;
- `benchmark.stderr.log`: 361 bytes,
  `442f071fb23e57a9cb4616c6df7683bee669d8114eacce43b16af812e86d1a93`;
  and
- `benchmark.disposition.json`: 817 bytes,
  `8c833293bb3e9f2e981e0be1d379819786d92706627b3fa3fbc64e93b188a5de`.

`benchmark.accepted.jsonl` and `benchmark.prelaunch.disposition.json` remain absent. Issue-108 code
may verify these paths read-only but must never create, delete, truncate, replace, rename, link from,
or write beneath `target/issue081`. All successor artifacts live only under `target/issue108`.

## Immutable product and qualification boundary

Preserve accepted descriptor/package/CID/state/migration wire bytes, diagnostics, APIs, source,
golden fixtures, manifests, Python references, C header/ABI and Issue-002 benchmark byte-for-byte.
The accepted interchange manifest remains
`6403ae6205dbc86a57483f44723cfc107f7f49654532fc648516b7cfed7ae3a5`.
No production crate, runtime path, public export, workspace dependency or lockfile changes.

Do not rerun the Issue-081 100-process reference runner, 30,000-trial mutation campaign, 48-row
migration matrix, five-target matrix, broad workspace seal, doctests, fuzz execution, audit,
browser, benchmark preflight, runner or binary. Their candid evidence is inherited, not regenerated.

## Smallest closable product vertical

The only Rust change is the benchmark-local fixture and focused tests in
`tools/miso-engine-effect-interchange-bench/src/main.rs`:

1. `MIGRATION_Q1`, `MIGRATION_Q2` and `MIGRATION_Q3` each become an exact four-entry table ordered
   `44_100`, `48_000`, `88_200`, `96_000`. Every entry remains `EffectQuality::Normal`, latency
   zero, finite tail zero, fixed scratch two bytes and zero scratch bytes per frame. D1 entries use
   layout/sizes `1/(1,2,2)`, D2 `2/(2,3,3)`, and D3 `3/(3,4,4)`.
2. The actual benchmark request remains Normal quality at 48 kHz, quantum 128, no bypass, Average
   link, no sidechain, initial values left `-0.25` and right `0.75`, the existing limits, W4
   `WasmSimd128` bank and member index 1. The measured migration interval remains exactly the
   two-step restore followed by the final snapshot; setup, validation and hashing remain outside.
3. A shared untimed execution seam must let an ordinary Rust test exercise the same descriptor
   binding, registry, resolution, restore, snapshot and postvalidation path without calling `main`,
   emitting benchmark records or consulting a clock. It verifies all three descriptor wires,
   sorted complete rates, the two adjacent steps, current-layout and replay validation, unaffected
   sibling lanes, and full canonical re-encode equality.
4. The exact final payload is eleven bytes
   `10 82 83 11 12 82 83 13 14 82 83`; the final envelope is 283 bytes: the accepted 224-byte V1
   header, 15-byte `bench.migration` effect ID, one zero alignment byte, two accepted 16-byte initial
   records and those three ordered payload sections. The test compares every byte, not only the
   payload or a digest.
5. Recompute the SHA-256 of that independently constructed canonical final envelope. It must differ
   from unreachable Issue-081 value
   `350acfa6e348c27a01afcb9efbd40c51a697aac8bbb6a5fe19dc1eb3c52bf441`.
   Freeze the new lowercase 64-hex value exactly once in the tool and in each successor validator,
   checker, preflight, runner and fake authority. The other three output digests remain exactly
   `865a0a5a01ba157bea7f3279ad68cc17db0296655998a9b5307cf759c38656f1`,
   `02e944154ccdc0315b96a7f493a11f6c60f70993750fb26ed766bc3273685d0f`, and
   `b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48`.

The benchmark record contract otherwise remains frozen: no-argument main; one untimed warmup;
exactly two measured rounds; four workload IDs in the existing order; 256 positive observations per
workload/round; exactly eight address/PID/path-free JSONL records; `issue=108`; nearest-rank
p50/p95/p99/p99.9/min/max; nanoseconds per operation; honest toolchain/machine metadata and missing-
metadata list; descriptive-only and no threshold.

## Successor-only authorities and atomic lifecycle

Add distinct `-108` validator, checker, checker-mutation, hermetic lifecycle, preflight and runner
scripts. They must not dispatch to or mutate the Issue-081 lifecycle. The new checker pins the
accepted manifest, the six Issue-081 evidence identities/absence facts, exact four-rate source
shape, exact four output digests, `issue=108`, target namespace, allowed paths and forbidden product
edits. Its mutation suite proves stale/duplicate/missing rate, old/new digest divergence, issue 81,
`target/issue081`, accepted-manifest refresh and a production edit all reject.

The strict stdlib validator retains the existing closed record key set and rejects missing/extra/
duplicate/unordered records, wrong issue/workload/round/count/unit/identity/digest, Boolean-as-
integer, nonpositive/nonfinite/unordered timing values, dishonest totals/percentiles/metadata,
uppercase or nonhex hashes, and addresses/PIDs/absolute paths. Self-tests use synthetic records only.

The hermetic lifecycle uses only inert fake binaries in temporary directories. It proves no launch
during preflight; arguments and missing-tool failures; clean candidate and every bound hash;
regular-file, symlink and hardlink refusal; raw/stderr preservation; exact five-line phase handshake;
missing/extra/truncated/duplicate/malformed/wrong-digest rejection; loader, warmup, round-1, final-
round and post-output failure disposition; distinct-inode copy+fsync atomic accepted publication;
raw mutation isolation; and no-clobber on any second call. It never invokes the real benchmark.

All persistent Issue-108 files are regular, one-link, atomically published and never overwritten:
`repair.seal.json`, `miso_engine_effect_interchange_bench`, `benchmark-preflight.seal.json`,
`benchmark.raw.jsonl`, `benchmark.stderr.log`, `benchmark.accepted.jsonl`,
`benchmark.prelaunch.disposition.json` and `benchmark.disposition.json` under `target/issue108`.
The runner must preserve partial evidence on failure and derive counters only from the exact stderr
phase lines.

## Checkpoints and authorization barriers

### Checkpoint 1 — repair and non-timed proof

Sol High implements the fixture/test and successor authorities, then runs only the exact focused
unit test, tool-package locked check/Clippy/rustdoc/format, validator self-test, fake lifecycle,
new static checker/mutations, shell syntax and text/diff/artifact scans. No broad or inherited real
matrix runs. Evidence records the new migration digest and confirms Issue-108 preflight/runner/
workload/timed counters remain `0/0/0/0`. Sol XHigh must return focused PASS before root commits.

On the clean committed candidate, root creates `target/issue108/repair.seal.json` with exact
candidate HEAD/tree, accepted manifest, Cargo lock, tool manifest/source, six successor authority
hashes, new output digest map, inherited Issue-081 artifact identities, focused-regression count `1`,
and Issue-108 preflight/runner/workload/timed counters `0/0/0/0`. Creation is no-clobber and the seal
is independently validated before preflight.

### Checkpoint 2 — sole zero-launch preflight

Root invokes exactly once:

`bash scripts/preflight-effect-interchange-benchmark-108.sh`

It accepts no arguments, verifies a clean exact HEAD/tree and the repair seal, runs only synthetic
validator/lifecycle gates, builds but never executes the real binary, and publishes a successor
preflight seal bound to binary/tool/source/input/lock/validator/checker/preflight/runner/lifecycle/
repair hashes and the four output digests. It reports preflight `1`, runner/workload/timed `0/0/0`,
planned warmup `1`, rounds `2`, records `8`. Failure is STOP; there is no alternate preflight.

Sol XHigh independently recomputes the seal, cleanliness, regular/one-link/no-clobber state and
absence of raw/accepted/stderr/disposition/prelaunch files. Only an explicit Sol XHigh PASS naming
the candidate and seal may authorize exactly one separate command:

`bash scripts/run-effect-interchange-benchmark-108.sh`

No direct binary, alternate command or retry is authorized. Success is runner/workload/timed
`1/1/1`, warmup `1`, rounds `2`, records `8`, strict validation and atomic accepted/disposition
publication. Any postlaunch failure is terminal evidence and stops Issue 108 without overall PASS.
Numbers are descriptive only; no tuning, comparison, threshold or optimization follows.

## Exact allowed paths

- this spec, its tracked brief, and the minimal Issue-081/README successor routing correction;
- `tools/miso-engine-effect-interchange-bench/src/main.rs`;
- `scripts/effect-interchange-benchmark-108-validator.py`;
- `scripts/check-effect-interchange-benchmark-108.sh`;
- `scripts/test-effect-interchange-benchmark-108-policy.sh`;
- `scripts/test-effect-interchange-benchmark-108.sh`;
- `scripts/preflight-effect-interchange-benchmark-108.sh`;
- `scripts/run-effect-interchange-benchmark-108.sh`; and
- only the minimal historical `scripts/check-effect-interchange-qualification.sh` routing change
  needed to distinguish terminal Issue-081 constants from current Issue-108 tool constants.

`target/issue108` is ignored evidence, never committed. No manifest/lock or other source path is
allowed. If implementation needs a product, accepted fixture/reference/C ABI, dependency, second
benchmark framework, target harness, or unrelated policy change, STOP and rebrief.

## Gates and non-goals

Before the real preflight: exact focused regression PASS; locked tool check and warning-denied
Clippy; warning-denied tool rustdoc; format; validator self-test; fake lifecycle; static checker and
mutations; exact-script `bash -n`; conflict/trailing-whitespace/diff; accepted-baseline and Issue-081
artifact identity/absence checks. Sol XHigh inspects source and evidence read-only.

No product/ref/accepted-fixture/C ABI edit; no existing real qualification matrix or broad seal;
no fuzz, cross-target, audit, browser, reference process, mutation campaign, migration matrix,
benchmark preflight/runner/main/workload/timing during implementation; no performance claim;
no Issue-081 artifact mutation; no Issue-109 or unrelated scope.
