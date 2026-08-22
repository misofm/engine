# Sol implementation brief — issue 108 effect-interchange benchmark migration-fixture repair

## Decision

**SOL XHIGH PASS / COMPLETE / READY TO CLOSE.** This qualification-tool recovery is complete, not
product work and not an Issue-081 retry. Sol High attempt 1 plus bounded corrections passed focused
review; the sole zero-workload preflight and the sole separately authorized descriptive runner both
exited zero. Both one-shot authorities are consumed, and no retry, alternate/direct invocation,
tuning, threshold or comparison is authorized.

Remote #83 and #107 were occupied/open and #108 was available at briefing on 2026-08-22. Root was
required to create the matching GitHub issue with the exact spec H1 title before implementation and
still owns verification of remote synchronization. The initial Issue-108 preflight/runner/workload/
timed counters were `0/0/0/0`.

Attempt-1 focused implementation derives the reachable migration-envelope SHA-256 as
`5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777`; the untimed exact-envelope
regression passes and distinguishes it from Issue 081's unreachable digest. At the focused
checkpoint Issue-108 preflight/runner/workload/timed counters were `0/0/0/0`; Sol XHigh passed that
checkpoint before either real authority was invoked.

## Final terminal evidence — 2026-08-22

Clean immutable candidate HEAD `c4963191310fd39c12e8edf06cae73af1e650622`, tree
`fd58c39418c6ed1d76cdbb6c8a014025b62d0abe`, produced strict terminal PASS. Final Issue-108
preflight/runner/workload/timed counters are `1/1/1/1`, with warmup `1`, rounds `2`, records `8`.
Runner stdout contained only the accepted-artifact path. Its stderr handshake is exactly
workload-started, warmup-complete, timed-started, round-1-complete and round-2-complete in order;
the disposition is `PASS` / `complete`.

All final files are regular and one-link. Repair, binary and preflight identities are respectively
2,350 bytes / `5e791427d6849a415da7ee7f259d8a7ee14f861af1ef0cb163ad441c1ef18ea1`,
837,192 bytes / `bf1ffe9599377c3a1b965eecfb88ea612f14c25e245cd4c198d926bceedfc4e6`, and
1,857 bytes / `f3a5448eec9aa5ba696924bc45b34cc243e72b92e15fb047eb39605366c328bc`.
Raw and accepted are distinct-inode identical copies, each 11,112 bytes with SHA-256
`eb39e3972b50aab45f4d253e20e51d0332eb844af9a7efcb8d86bda95f0776c5`. Stderr is 226 bytes /
`43331d34c536bcdab2f1825f0fae67adfaade37bfde1bfc0b0f379149c5329ad`; disposition is 881 bytes /
`ae026c1536316b077858b3ea7d1c8ecc2bc0fbbbc3b3a6a7c2511f7377f74375`; prelaunch disposition is
absent.

The eight closed-schema records are descriptor/package/state/migration for round 1, then the same
order for round 2, each with 256 observations. The four output hashes remain
`865a0a5a01ba157bea7f3279ad68cc17db0296655998a9b5307cf759c38656f1`,
`02e944154ccdc0315b96a7f493a11f6c60f70993750fb26ed766bc3273685d0f`,
`b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48`, and
`5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777`. Ordered `total_ns` values
are `547946`, `1389054`, `425687`, `718892`, `544929`, `1381496`, `420847`, and `725071`.
All timing fields are positive integral, ordered and arithmetically possible. Shared metadata is
stable: AMD Ryzen 7 9700X, 16 logical/8 physical cores, Linux `6.8.0-138-generic`, Rust 1.97.1,
LLVM 22.1.6, release `x86_64-unknown-linux-gnu`, governor `powersave`, load `0.12,0.13,0.10`,
Instant/nearest-rank. Metadata is incomplete solely because `power_mode` is absent.

Sol XHigh independently reconstructed candidate cleanliness, rehashed every authority and all 36
accepted-manifest rows, validated both seals plus every record/key/type/order/timing/identity
relation, and reconfirmed all frozen Issue-081 files/absences. Results are descriptive only; no
performance claim follows. No further preflight or runner is allowed. Root still owns the docs
commit, single batch push, GitHub synchronization and closure; remote completion is not claimed.

## Immutable boundary

Treat the complete Issue-081 result as historical evidence: nonbenchmark qualification is valid,
but its sole benchmark runner is consumed and Issue 081 has no overall PASS. Preserve the six
retained `target/issue081` files byte-for-byte at the sizes/hashes in the spec, preserve absent
accepted/prelaunch files as absent, and preserve historical preflight/runner/workload/timed
`1/1/1/0`. Never write, link, rename, delete or publish into that namespace.

Accepted Issue-079 state, Issue-080 migration, Issue-082 descriptor, Issue-078 package, references,
fixtures/manifests, C ABI, product sources and Issue-002 benchmark remain read-only. Do not rerun the
100-process, 30,000-mutation, 48-row migration, five-target or broad workspace qualification.

## Literal implementation order

1. In benchmark `main.rs`, replace each one-row D1/D2/D3 quality table with sorted launch-rate rows
   `44100/48000/88200/96000`. Preserve Normal quality, layout-specific `(common,left,right)` sizes
   `(1,2,2)/(2,3,3)/(3,4,4)`, zero latency/tail, scratch 2+0/frame and the unchanged 48 kHz request.
2. Extract a shared execution seam so a Rust unit test runs the exact descriptor-bind, two-edge
   resolve, W4 member-1 restore, final snapshot and validations without `main`, clock reads or
   benchmark records. Assert complete descriptor rates, sibling isolation, exact payload
   `1082831112828313148283`, exact 283-byte independently encoded envelope and full byte equality.
3. Recompute that envelope's SHA-256 and prove it is not Issue-081's unreachable `350acfa6...f441`.
   Freeze the new digest across the tool and every Issue-108 authority; keep the descriptor/package/
   state output digests exactly unchanged. Change emitted benchmark records to `issue=108` only.
4. Add distinct `-108` stdlib validator, static checker plus mutations, hermetic fake lifecycle,
   no-argument preflight and no-argument public runner. Every persistent path is under
   `target/issue108`; old Issue-081 scripts/artifacts are never dispatched or reused. Make only the
   minimal old qualification-checker routing change necessary for historical/current constants.
5. Fake-test the exact closed eight-record schema, five-phase handshake, all prelaunch/postlaunch
   failure dispositions, regular/symlink/hardlink refusal, partial evidence, copy+fsync publication,
   distinct inode, raw mutation isolation and second-call no-clobber. Fakes must never execute real
   benchmark main.
6. Run only the focused regression, locked tool check, strict Clippy/rustdoc/fmt, validator/fakes,
   checker/mutations, shell syntax and static/diff scans. Record the new digest and counters
   `0/0/0/0`; pause for Sol XHigh PASS and root's exact-path commit.

## Preflight and one-shot barrier

After the clean repair commit, root creates and validates a no-clobber
`target/issue108/repair.seal.json` binding candidate HEAD/tree, accepted/lock/tool/source and all
successor authorities, inherited Issue-081 evidence, exact output map, focused regression `1`, and
Issue-108 counters `0/0/0/0`.

Root then invokes exactly one
`bash scripts/preflight-effect-interchange-benchmark-108.sh`. It launches zero real workloads,
builds but does not execute the sealed binary, publishes only Issue-108 binary/preflight files and
leaves raw/accepted/stderr/dispositions absent. A failed preflight stops; do not rerun it.

Sol XHigh must independently verify the committed candidate and preflight seal before authorizing
exactly one `bash scripts/run-effect-interchange-benchmark-108.sh`. That runner alone may perform
one untimed warmup and two measured rounds, 256 observations per workload/round and eight validated
descriptive records. No direct binary, alternate invocation, tuning or retry. Preserve terminal
artifacts on every outcome.

## Allowed paths and STOP conditions

Allowed paths are exactly the two #108 docs; minimal #81 spec/brief and ISSUE_SPECS README routing;
benchmark `src/main.rs`; six new `-108` validator/checker/checker-mutation/lifecycle/preflight/runner
scripts; and the minimal historical qualification-checker routing edit named in the spec.

STOP for any accepted product/reference/fixture/C ABI or manifest/lock edit; public API/wire/
diagnostic change; production dependency/reachability; Issue-081 artifact mutation; inherited real
matrix/broad/fuzz/target execution; real benchmark activity before authorization; a second
preflight/runner or alternate/direct invocation; or any scope beyond local benchmark correctness
and atomic lifecycle evidence.

## Verdict checkpoints

- Checkpoint 1: Sol XHigh focused PASS on the repaired fixture, non-timed exact-envelope test, new
  digest binding and fake/static lifecycle; real Issue-108 counters were `0/0/0/0`.
- Checkpoint 2: Sol XHigh GO for exactly one runner after independently sealing the sole
  zero-workload preflight and confirming all output/disposition paths absent.
- Final: Sol XHigh PASS. The one authorized runner returned eight validator-valid records and
  honest `1/1/1` runner/workload/timed counters. The authority is consumed and cannot be retried.
