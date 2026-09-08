# Complete formatted `LocalRing` retirement gates

Parent: #560
Supersedes: #659
Baseline: `6f4a1b1c893ce04fbe8001e9d175702eba9a9d78`

## Problem

#659 proved that `engine::realtime::LocalRing<T>` and
`target_smoke::local_realtime_ring_smoke` have no live product caller and that
their removal preserves the production shared SPSC paths. Its final attempt
passed the removed-name census, release and debug `engine`/`target-smoke` suites,
and strict Clippy, then stopped at rustfmt. Three attempts were exhausted, so
#659 closed superseded without delivery or SOURCE PASS.

The remaining correction is exact and mechanical: format the shortened SPSC
re-export and remove the trailing blank line left by the deleted target-smoke
test, then run the policy and Wasm gates that #659 did not reach.

## Smallest closable slice

Own only:

- `crates/engine/src/realtime/spsc.rs`
- `crates/engine/src/realtime/mod.rs`
- `crates/target-smoke/src/lib.rs`
- this issue spec and compact gate records
- concise #559/#560 coordination records

Inherit the three product-file removals from #659 product head
`dbe351cd8ef9b475d657ed01495b5afc2969817e` exactly, with only these two
source-layout differences:

1. rustfmt places `SpscRetainedPayload` on the preceding `pub use spsc` line;
2. rustfmt removes the blank line immediately before the target-smoke test
   module's closing brace.

The resulting live-name census must be zero under `crates/`, `hosts/`, `tools/`,
and `sidecars/`. `bounded_spsc`, `bounded_spsc_move`, retained-payload accounting,
producer/consumer types, ordering, capacity, counters, generations, and all
substantive callers remain unchanged. `TargetSmoke`, `target_smoke()`, and its
per-target backend assertions remain unchanged.

No queue redesign, memory-ordering change, host adoption, allocation or
performance claim, benchmark, artifact, pin, manifest, lockfile, workflow,
unrelated cleanup, compiler dump, `.ll`, assembly, object, archive, binary,
`rlib`, `rmeta`, or generated target output is in scope.

## Inherited evidence

#659 attempt 3 ran at exact product head `dbe351cd` after integrating delivered
main. Its self-excluding 24-entry manifest under
`/tmp/issue659-attempt3-source-evidence` verifies and hashes to
`25ac123c8bac70083514c60369283f8922afb69c200ac215ef90906eb0d1c246`.
The following one-shot gates returned 0:

- removed-name/shared-SPSC census;
- release `engine` plus `target-smoke` tests;
- debug `engine` plus `target-smoke` tests;
- strict Clippy for both packages and all targets/features.

Those results may carry only after this issue proves that its three product files
differ from #659 solely by the two exact rustfmt changes above. The #659 format
failure, unexecuted gates, wrong-directory manifest verification, hard-stop
verdict, branch, history, evidence, and targets remain preserved without rewrite.

## Attempt 1 authorization gates

Hypatia, Luna HIGH agent `issue583_luna_impl`, is the sole executor after Astra
LOW returns scope PASS. Before editing, prove the exact clean pushed head/upstream
and current main, confirm #663 is the only other active issue and path-disjoint,
and prove these fresh paths are absent and are not symlinks:

- `/tmp/issue664-attempt1-source-evidence`
- `/tmp/issue664-attempt1-inherited-source`
- `/tmp/issue664-attempt1-wasm-realtime`
- `/tmp/issue664-attempt1-wasm-simd`

Exercise the capture wrapper with harmless status-0 and expected status-1
controls and independently read both statuses before source work. Apply only the
inherited three-file removal and two exact formatting corrections. Prove the
product delta against #659 is formatting-only, the live-name census is zero, the
shared-SPSC callers remain, and the inherited gate evidence is intact.

Run once in order, stopping at the first unexpected failure:

1. `cargo fmt --all -- --check`;
2. branch-wide `git diff --check` plus exact product-path and inherited-delta
   census;
3. `bash scripts/check-workspace-policy.sh`;
4. `bash scripts/check-realtime-policy.sh`;
5. `bash scripts/test-realtime-policy.sh`;
6. `bash scripts/check-wasm-realtime-atomics.sh /tmp/issue664-attempt1-wasm-realtime`;
7. `bash scripts/test-wasm-realtime-atomics.sh`;
8. `CARGO_TARGET_DIR=/tmp/issue664-attempt1-wasm-simd RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p target-smoke`;
9. final owned-path/payload census.

Retain exact identities, literal commands/environment, separate complete
stdout/stderr, numeric statuses, and a verified self-excluding manifest with its
hash recorded separately. Do not correct or retry after any unexpected failure.

## Review and delivery

Astra LOW reviews the exact source checkpoint. Any correction consumes the next
of at most three attempts and requires fresh scope authorization. After SOURCE
PASS, root separately rules whether the shipped AudioWorklet artifact is affected;
artifact qualification and pin changes remain blocked until that ruling.

Then use exact-head/current-main PR readiness, required qualification, guarded
merge-parent review, post-main qualification, GitHub synchronization, and clean
delivered-worktree removal. #659 evidence and worktree remain held until this
successor's source review explicitly releases them.

## Scope authorization

Astra LOW returned **SCOPE PASS** at exact clean pushed head/upstream
`900f51f00492850e0c594e5b85b62642c6c835ba`, current main
`6f4a1b1c893ce04fbe8001e9d175702eba9a9d78`, and tracker
`6d4aba09734c208df8e1d6cb4cbcc7948512ce64`. GitHub #664 matches; #659 is
closed, #663 is open and documentation-only, all four declared temporary paths
are absent including symlinks, and this branch contains only the brief.

Hypatia alone is authorized for one bounded attempt: import exactly the three
#659 product files, apply only the two prescribed formatting changes, prove that
exact difference before carrying the four successful gates, and run the listed
gates once with capture controls and complete records. Branch-wide whitespace
must check both uncommitted changes and `origin/main...HEAD`; tracker changes
remain root-owned. Stop on first unexpected failure. No artifact or pin action is
authorized, and all #659 recovery state remains held through source review.

## Attempt 1 result

Hypatia's preflight proved exact identities, four fresh absent/non-symlink paths,
and capture-control statuses 0 and 1. The three inherited product files plus only
the two authorized format hunks have formatting-delta hash
`8be2b7753c3b468b3e05b43f3ca23979ec694f0147fdca1b3fd2888a4564cd5c`.
Format, full working/branch diff and census, and workspace policy returned 0.
Realtime policy returned 1 because removing `LocalRing` also removes one genuine
marked realtime region: the delivered tree has 41 regions in 12 files while the
policy floor still requires 42. Gates 5-9 did not run; no correction or retry
occurred, and no Wasm or Cargo target path was created.

Product source checkpoint `6f5c094895551b9f259f0ba503d0d8b5ed9fb3c6` is clean and
pushed. The self-excluding manifest under
`/tmp/issue664-attempt1-source-evidence` verifies and hashes to
`744d8ba24b72cff27fb2efb6a332d3ddedfdfee1aca791b5a7462829b98fe83b`.
No compiler payload or artifact work ran. Attempt 1 is **FAIL** pending Astra LOW
review of the direct realtime-policy count dependency; no policy edit or further
gate is authorized yet.
