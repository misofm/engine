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

## Attempt 1 review and attempt 2 brief

Astra LOW returned **ATTEMPT-1 FAIL** at exact product checkpoint `6f5c0948` and
record `7359264f`. The inherited product delta, successful gates, realtime-policy
failure, absence of later gates, and manifest all verify. The 41-region result is
a direct policy dependency of deleting a marked implementation, so a separate
issue is unnecessary.

Attempt 2 freezes every product byte and owns only:

- `scripts/check-realtime-policy.sh`: change the marked-region floor and error
  from 42 to 41 and update its comment to explain the complete `LocalRing`
  removal; keep the 12-file floor and all discovery/scanning/error behavior;
- `scripts/test-realtime-policy.sh`: remove exactly the second synthetic SPSC
  `pop()` region so the fixture mirrors the remaining single shared-SPSC region,
  update its count comment and expected diagnostic from 42 to 41, and preserve
  the existing drop-region mutation so the valid fixture has 41 regions and the
  mutant has 40;
- this spec and compact evidence records.

After fresh Astra LOW scope PASS, Hypatia alone may use the fresh paths
`/tmp/issue664-attempt2-source-evidence`,
`/tmp/issue664-attempt2-wasm-realtime`, and
`/tmp/issue664-attempt2-wasm-simd`. Prove exact clean head/upstream, fresh
absence/non-symlink state, frozen product hashes, and capture-control statuses 0
and 1. Apply only the two policy files above, inspect their exact diff, then run
once in order:

1. `bash -n scripts/check-realtime-policy.sh scripts/test-realtime-policy.sh`;
2. `bash scripts/check-realtime-policy.sh`;
3. `bash scripts/test-realtime-policy.sh`;
4. `bash scripts/check-wasm-realtime-atomics.sh /tmp/issue664-attempt2-wasm-realtime`;
5. `bash scripts/test-wasm-realtime-atomics.sh`;
6. `CARGO_TARGET_DIR=/tmp/issue664-attempt2-wasm-simd RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p target-smoke`;
7. working and branch-wide diff checks, exact product freeze and policy-delta
   census, owned-path/payload census, and a verified self-excluding manifest.

Retain attempt 1's behavior, strict-Clippy, format and workspace evidence because
the product is frozen and the two scripts do not change those surfaces. Stop on
the first unexpected failure without correction or retry. Artifact work remains
blocked, and all #659 recovery state remains held.

## Attempt 2 authorization

Astra LOW returned **ATTEMPT-2 SCOPE PASS** at exact clean pushed head/upstream
`277d99e09ef007613e2175afc4c16dd378ca0152`, current main
`6f4a1b1c893ce04fbe8001e9d175702eba9a9d78`, and tracker
`a8a47a3ff369d97c10c21746c7bb13760d91c664`. GitHub #664 matches, product
bytes are frozen, and all three fresh paths are absent including symlinks.

Hypatia alone is authorized to apply the exact two-script recalibration, preserve
the 12-file floor and the discriminating 41-to-40 mutation, and execute the seven
stages once after identity/freshness/capture controls. Attribute carried behavior
and Clippy evidence to #659 attempt 3, and carried format/workspace evidence to
#664 attempt 1. Stop on first unexpected failure. No product correction,
artifact/pin action, retry, or cleanup is authorized; #659 recovery state remains
held.

## Attempt 2 result

Hypatia verified the exact head, frozen hashes, fresh absent/non-symlink paths,
and capture-control statuses 0 and 1. Source checkpoint
`403fdbc4fcc1c3062894b3c4d9760671b64905ab` changes only the two authorized
policy scripts. Shell syntax, realtime policy (41 regions in 12 files), the full
realtime mutation suite, Wasm realtime atomics check (three objects), Wasm
mutation suite (directed cases and two causal mutants), SIMD Wasm cargo check,
and final diff/product-freeze/policy-delta/payload census each returned 0 once.

The self-excluding manifest under
`/tmp/issue664-attempt2-source-evidence` verifies and hashes to
`ba6842e9f63b85aae58ebc3a136a8c13a9bfd1e4620d5e10059c8f401b370a62`.
Behavior and strict-Clippy evidence remains attributed to #659 attempt 3; format
and workspace evidence remains attributed to #664 attempt 1. No product,
artifact, pin, tracker, or unrelated change occurred. Exact-source Astra LOW
review is pending.
