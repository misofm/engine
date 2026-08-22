# Sol implementation brief — issue 108 effect-interchange benchmark migration-fixture repair

## Decision

**SOL XHIGH BRIEF PASS / READY FOR SOL HIGH ATTEMPT 1.** This is a qualification-tool recovery,
not product work and not an Issue-081 retry. Sol High gets one coherent implementation pass and at
most one bounded correction; Sol XHigh verifies both the focused checkpoint and the later
zero-workload preflight. Any second failed implementation pass, real-preflight failure or runner
postlaunch failure is terminal STOP.

Remote #83 and #107 are occupied/open and #108 is available as of 2026-08-22. Root creates the
matching GitHub issue with the exact spec H1 title before implementation and later owns remote
synchronization. The initial Issue-108 preflight/runner/workload/timed counters are `0/0/0/0`.

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

- Checkpoint 1: strict focused PASS/HOLD on the repaired fixture, non-timed exact-envelope test,
  new digest binding and fake/static lifecycle. Real Issue-108 counters remain `0/0/0/0`.
- Checkpoint 2: strict GO/HOLD for exactly one runner only after the sole zero-workload preflight is
  independently sealed and all output/disposition paths remain absent.
- Final: PASS only if the one authorized runner returns eight validator-valid records and honest
  `1/1/1` runner/workload/timed counters. Any failure is candid terminal STOP, never a retry.
