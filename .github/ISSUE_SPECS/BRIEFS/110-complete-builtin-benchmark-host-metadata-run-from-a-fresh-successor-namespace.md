# Sol implementation brief — issue 110 fresh builtin benchmark metadata namespace

## Decision

**COMPLETE / SOL XHIGH PASS / READY TO CLOSE / NO FURTHER EXECUTION AUTHORIZATION.** Sol High used
one focused pass and the one bounded correction; Sol XHigh independently verified the clean seal,
sole preflight and sole runner. Final preflight/runner/workload/timed counters are `1/1/1/1`, warmup
`1`, measured rounds `2`, records `20`. Root owns upstream evidence synchronization and closure.

## Why this successor exists

Issue 109 accepted the runner metadata semantics at `5b2744f` and corrected its hermetic lifecycle
at `f6e8a6b5936f9d578e050a6b85242c4234a1a886`, final lifecycle SHA-256
`14dd9ec48921fefdba8a57afa827f27222dc881311929f55a44d0edc89c97ef4`. During the intervening
handoff, its sole no-clobber repair seal was created against the pre-correction lifecycle. Preserve
that regular nlink1, 2,538-byte file at `target/issue109/metadata-repair.seal.json`, SHA-256
`1e8cec4904d8987ddca581e5b23870629d734127ad3f9e010f6a5c2d178b69c6`. Every later Issue-109 output
is absent and its counters remain `0/0/0/0`; no Issue-109 workload ran. Never alter or use that
namespace.

Dependencies are exact titles **Builtin native, AArch64, and Wasm runtime-selection and instruction
qualification**, **Separate builtin benchmark render timing from meter evidence collection**, and
**Repair builtin benchmark host metadata export and reauthorize one descriptive run**. Issues 072
and 109 are stopped technical inputs, not PASS.

## Literal implementation

Copy/route the accepted behavior into only five new successor scripts:

- `scripts/run-builtins-benchmark-110.sh`;
- `scripts/preflight-builtins-benchmark-110.sh`;
- `scripts/test-builtins-benchmark-110.sh`;
- `scripts/check-builtins-benchmark-110.sh`;
- `scripts/test-builtins-benchmark-110-policy.sh`.

Use only `issue=110`, `target/issue110`, successor script identities and the new completion-seal
authority. Do not edit Issue-109 scripts. Preserve exact metadata sources and all 16 environment
mappings, ambient clearing, required/optional/null rules, validation, canonical 20-row projection
comparison and projection digest. Preserve the frozen Rust tool, record/aggregate validators,
Cargo lock, fixtures, Issue-068 identity, Issue-072 timer/audit boundary, schema v2 `issue=35`, five
workloads, 48/96 kHz, quantum 128, one warmup, two rounds, 20 records and output hashes.

No Rust/product/timer/schema/input/rate/workload/digest change is permitted. No tuning, threshold,
comparison, optimization, listening or release claim is in scope.

## Focused fake/static checkpoint

Retain the reviewed Issue-109 hermetic matrix under successor names: complete and optional-missing
metadata, required and malformed discovery, all ambient spoof keys, every-row projection mismatch,
authority drift, direct/tandem seal mutation, argument/tool/dirty failures, phase counters,
regular/symlink/hardlink no-clobber, partial evidence, distinct accepted inode, raw isolation and
second-call refusal. Add exact read-only proof that the Issue-109 stale seal and its seven absences
remain unchanged. The static checker/mutations pin the successor namespace, one launch site, all
source mappings, fixed build values and counters `0/0/0/0`.

Allowed precheckpoint gates are shell syntax, fake lifecycle, static checker/mutations, frozen
validator read-only validation, compile-only locked benchmark-package check if needed, and text/
diff/artifact sanity. Real preflight/runner/main/workload/timing, inherited matrices, target, audit,
trace, fuzz and listening execution remain zero.

## Fresh authority sequence

Persistent Issue-110 paths are exactly completion seal, sealed binary, preflight seal, raw,
accepted, stderr, prelaunch disposition and final disposition under `target/issue110`. Every file
is regular, nlink1 and no-clobber; seals, binary, accepted and dispositions are atomically
published, while raw/stderr are created once to preserve partial evidence. Either disposition
consumes runner authority.

After Sol XHigh focused PASS and root's clean exact-path commit, root creates only
`target/issue110/completion.seal.json`, binding HEAD/tree, all frozen/current authorities,
Issue-072 and stale Issue-109 evidence, metadata regression `1`, and counters `0/0/0/0`. Sol XHigh
must independently review it before exactly one:

`bash scripts/preflight-builtins-benchmark-110.sh`

Preflight is zero-workload, builds but never executes the benchmark, publishes only binary/seal and
records counters `1/0/0/0`, warmup `1`, rounds `2`, records `20`. Failure is terminal.

After another independent Sol XHigh review, exactly one runner may be authorized:

`bash scripts/run-builtins-benchmark-110.sh`

No retry, direct/alternate invocation, tuning or preflight repeat. Success requires exact 20-row
projection equality, zero render violations, raw/accepted equality with distinct inodes, exact five
phases, PASS/complete disposition and counters `1/1/1/1`. Any failure is terminal STOP.

## Paths and verdict boundary

Allowed tracked paths are these two Issue-110 docs, minimal Issue-109/README routing, and the five
new `*-110.sh` scripts. Any other change is STOP. Sol High pauses after one coherent pass; one HOLD
allows one bounded correction only.

Only complete terminal evidence permits `SOL XHIGH PASS / COMPLETE / READY TO CLOSE`. Root owns
commits, remote synchronization and closure. Results remain descriptive only.

## Sol High checkpoint-1 handoff

Sol High completed the five-script successor tranche on base
`d46e5a96d3f294c601674f5d2b0205d0a96a4ac2` / tree
`186db283e386c11edc2e705a8164da5e6b7f9627`. Runner/preflight/lifecycle/checker/mutation SHA-256
values are respectively `a014ae7fa90ab140b2d7529564a19a9c5d3d7105da1bb00541943f1c26df2089`,
`20faec32e735e9f314d21729cb7737e202b8a9637d600491308ac98fee4a3893`,
`5de8eceebfb1ae1265c6a0ab0adc7eacefa4104ef1856cb89cd7084f96b71675`,
`9b6d808604f919f226a24de1bcf99e3e2e90395839e143beb07a62c9d824855a` and
`2dce3f5b8189dbf8667d2d5c25aa6bd53408722336228e019f3c5496b8eabde1`.

Shell syntax, static checking, 37 static rejection mutations, the hermetic lifecycle, read-only
20-record validator check, locked package compilation and format check passed. Exact Issue-072 and
stale Issue-109 evidence remains unchanged, all seven Issue-109 future artifacts and the entire
Issue-110 namespace remain absent, and real preflight/runner/workload/timed counters remain
`0/0/0/0`. The bounded review correction additionally proves exact sole-member Issue-109 namespace
membership and rejects an arbitrary extra member before launch/build. This is a focused review
handoff record; the separately reviewed execution evidence follows.

## Terminal evidence

Candidate `47daeda00683acb6e0fd29bafd3ee6d6403cd782`, tree
`1f51a7bba86bbe34afb18567272faa2dc86bc397`, produced exactly one successful zero-workload
preflight and one successful runner. Completion seal, binary and preflight are respectively
2,988/3,200,296/1,893 bytes with SHA-256 values `3ce39b2653d6b912b6ede083fe8479e46bcbce665095190bd94d15fe82ca238d`,
`a7bafc459b69fb8bdfd7d9195e4ff8d1febf8602a57540498cb579d04a486912` and
`9a7a78748b32d8a7cdee1bf7e886e38e6a358f6dfd093d93bbd51bdac2eddaa0`.

Raw and accepted are distinct-inode, byte-identical 38,477-byte files with SHA-256
`8a2d3f2f9f6d5a6f2edb4513fd304b121c934f6dcc1f5379b96f4256b54aa2dc`. Exact five-phase stderr is
211 bytes, SHA-256 `7935bf62063c0e9d2bfaac91d02db6f448dbf0636fbf16d3c49660738f55b396`.
The 1,075-byte PASS/complete disposition hashes to
`361f3a4f612e88dcc8a6dcb9f810528b175a64fbf3eea07122024df7971f274f`, records counters `1/1/1/1`,
warmup `1`, rounds `2`, and projection SHA-256
`59efa293fd6781d8da916490621f0973475b4dc4cbff7f5bd92baccb833d095f`. Prelaunch disposition is
absent and final target membership is exactly seven regular nlink1 files.

All 20 ordered records pass the frozen schema, identity, output, percentile and audit contracts.
All 16 metadata values are identical and populated with `missing_metadata=[]`; the independently
reconstructed canonical projection matches the disposition digest. Render audit fields are all
zero and preparation audit fields are `not_applicable`. Rough descriptive p50 ranges are full chain
2,298–2,306, identity 1,009–1,029, matrix 1,442–1,446, meter 25,408–31,430, and preparation
890,644–957,490 ns/op. These are descriptive period observations only.

Issue-072 and Issue-109 namespaces remain exact. No retry, alternate invocation or tuning occurred.
No further preflight, runner, benchmark-main, workload or timing execution is authorized. Issue 110
is `COMPLETE / SOL XHIGH PASS / READY TO CLOSE`.
