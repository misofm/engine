# #511 final Sol attempt 3 report

## Result

The final bounded source correction and all required proportional local gates PASS at clean,
pushed source HEAD `35dc8d4dc90c352bc1b328ec57852fa520a1e197` on
`codex/runtime-slot-reservation`, equal to `origin/codex/runtime-slot-reservation` when the gates
ran. This is the third and final implementation attempt. Consolidated Astra acceptance and
immutable native/Wasm/current-artifact delivery remain with root and are not claimed here.

The correction is exactly two source lines:

- `crates/builtins-compiler/Cargo.toml`: the existing `test-support` feature now forwards the
  already existing dependency feature as `["graph/test-support"]`.
- `crates/builtins-compiler/tests/allocation_tracker.rs`: complete conversion coexistence is
  `n*f + n*b + (n+1)*w`, including the original chain mask.

No `Cargo.lock`, default feature, dependency, production path, fixture corpus, benchmark/preflight,
seal, digest, artifact pin, or test filter changed. Root made the checkpoint/spec commit, push and
GitHub evidence update. This agent made no Git or GitHub mutation.

## Final source identity

Every immutable-head command recorded cwd, exact argv, effective `PATH`, clean porcelain status,
HEAD and these seven SHA256 values before execution:

- `crates/graph/src/lib.rs`: `9bdac963068795bc8a33f93b18ba5bbe5221ab628bdf739ec8694a919bbbd432`
- `crates/graph/src/runtime.rs`: `b92af988c6a8a602a7f93bf8cd11f4fb62e1f68831acca0eac8e39aaf75c23a2`
- `crates/graph-compiler/src/compile.rs`: `7f435cff8087b7e66dc74b57873cd343ff178fe8f849b5c9cf61e14c8513ff13`
- `crates/graph-compiler/src/lib.rs`: `fc4a43306b56b0ddb6d4f85ff86ee926fc14b98a9543b18e5e420b9218f444d8`
- `crates/builtins-compiler/src/lib.rs`: `6642d7fa25ca317e133b1705fe62fb6148f0e6894e28a51ba48353e152021ebc`
- `crates/builtins-compiler/tests/allocation_tracker.rs`: `dac42c6c4659ace55c442618578182548e36df1379327f964e0b1f96a8099a0e`
- `crates/builtins-compiler/Cargo.toml`: `39e2d1e28b374be61c747b61ec7f45d05822f7d9d8c04c6b324129372790f086`

The effective command environment prefixes the existing path with `/home/bl/.cargo/bin`; all
Cargo gates use `--locked` except the repository format command, which has no locked option.

## Correct physical bound and retained facts

On the measured native fixture, `N=3`, `F=16`, `B=32`, and `W=8`. The reservation remains
`C=N*(F+3B+3W)=408`, and the largest single request is
`L=max(NF,NB,W)=max(48,96,8)=96`.

The prior `48+96+24=168` value remains an accurate observed conversion subset: incoming stage
vector, slot array, and three cloned slot masks. It is not the full named component coexistence.
The complete stage/slot/mask coexistence includes the independently identified original 8-byte
chain mask and is `48+96+24+8=176 <= 408`. Retained slots plus the three cloned masks and original
mask remain `96+4*8=128 <= N*(B+2W)=144`. Destruction still observes one 96-byte slot array, four
8-byte masks and two 32-byte scratch planes: seven frees, zero allocations, and a checked zero
closing balance.

The preserved compiled construction facts remain paired `N=6/R=6/S=5`, max per-chain `R=3/S=2`,
and unpaired `N=6/R=6/S=6`, max per-chain `R=3/S=3`. Attempt 3 changed neither the construction
seam nor those facts.

## Gate results

All 18 captured attempt-3 status files contain numeric status 0. The two pre-checkpoint dirty-source
runs also passed, but the following immutable-head reruns are the qualification evidence:

- Existing isolated consumer command: 1 passed, 0 failed; its other integration-test binaries
  selected zero tests. Raw `/tmp/511-sol3-old-consumer-immutable.{meta,stdout,stderr,status}`.
- Previously failing combined strict all-targets Clippy: status 0. Raw
  `/tmp/511-sol3-clippy-combined.{meta,stdout,stderr,status}`.
- Frozen graph exact: debug 1/0 and release 1/0. Raw
  `/tmp/511-sol3-focused-graph-debug.*` and `/tmp/511-sol3-focused-graph-release.*`.
- Frozen physical exact: debug 1/0 and release 1/0. Raw
  `/tmp/511-sol3-focused-physical-debug.*` and `/tmp/511-sol3-focused-physical-release.*`.
- Full graph-compiler lib: debug 64/0 and release 64/0. Raw
  `/tmp/511-sol3-full-graph-debug.*` and `/tmp/511-sol3-full-graph-release.*`.
- Full allocation tracker: debug 7/0 and release 7/0. Raw
  `/tmp/511-sol3-full-physical-debug.*` and `/tmp/511-sol3-full-physical-release.*`.
- Strict builtins all-targets Clippy with explicit `test-support,graph/test-support`: status 0.
  Raw `/tmp/511-sol3-clippy-test-support.*`.
- Format, diff, graph policy, realtime policy and workspace policy: each status 0. Raw
  `/tmp/511-sol3-fmt.*`, `/tmp/511-sol3-diff-check.*`, `/tmp/511-sol3-policy-graph.*`,
  `/tmp/511-sol3-policy-realtime.*`, and `/tmp/511-sol3-policy-workspace.*`.

The pre-checkpoint records are preserved at `/tmp/511-sol3-old-consumer.*` and
`/tmp/511-sol3-physical-debug.*`; both have status 0 and contemporaneous seven-file hashes. The
first old-consumer metadata did not print the effective environment, so the immutable rerun is the
complete provenance record rather than a reconstruction of the earlier record.

## Failures and evidence limits

Attempt 3 has no failing source or proportional local gate. The attempt-2 feature regression,
combined-Clippy failure and all earlier failed attempts remain unmodified in
`artifacts/issue511-sol-attempt2/` and `/tmp/astra-511-attempt2-review.md`; their failures are not
reclassified or overwritten. The forwarding correction is directly exercised by the exact
existing isolated consumer and by the formerly failing combined Clippy command.

The historical full sealed/timed preflight was deliberately not run. No timing, benchmark, new
fixture corpus, broad target matrix, seal/digest edit or artifact repin occurred. Final consolidated
Astra review must decide source acceptance. Only after that acceptance may root perform the
already-defined immutable native/Wasm/current-artifact qualification and synchronize final issue
delivery.
