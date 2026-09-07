# Restore builtin endpoint cap preflight and bitwise PCM proof

GitHub: #580 (https://github.com/misofm/engine/issues/580)

Bounded successor to hard-stopped #579 and #576 under parent #444 and audit lane A #559. #579 exhausted three attempts at final source `d67becc3d4dd7100faf3b172c6d18e87d963bfaa`, preserved by hard-stop record `3f662230`. Its final Astra LOW review accepted cancellation exact-once/token/gating, test-only backend preparation, dev-dependency witness support, restored production meter behavior, actual queue-retention measurement, forced-scalar state/PostFader evidence, and direct zero-pair witnesses. This successor owns only the two residual defects and delivery of the preserved source; it does not repeat #579 attempts.

## Smallest closable outcome

Restore report-only endpoint retained/largest projection and cap validation before any delivery or outcome queue allocation. A capacity that exceeds either configured endpoint limit must return the existing typed `ResourceLimit` before host or endpoint queue preparation can allocate. Preserve the factored actual queue helper for the already accepted retained-allocation measurement, but invoke it only after preflight succeeds. Preserve checked composition and exact/one-below behavior without adding another ledger, report authority, allocator, or cap.

Replace the native-bank and forced-scalar endpoint/reference PCM comparisons with mapped `f32::to_bits` arrays. Add a focused signed-zero discriminator proving the comparator rejects `+0.0` versus `-0.0`; ordinary `f32` equality may not satisfy this gate.

## Exact ownership

Allowed paths are `crates/host-core/src/builtin_batch_endpoint.rs`, `crates/host-core/tests/builtin_batch_endpoint.rs`, this numbered spec, #579's spec only for successor/delivery linkage, and focused audit/review records. Do not edit `crates/host-core/src/prepare.rs` or `crates/host-core/Cargo.toml`; their accepted #579 test-only seam/dev-feature state is frozen. Do not edit any other host-core module, protocol, graph, engine, builtins, builtins-compiler, manifests/lockfile, hosts, C ABI, browser, SDK, artifacts, policies, workflows, or lane B #578 paths. Lane B retains artifact qualification/pinning.

## Objective gates

1. Independently compute endpoint delivery/outcome reports before construction. Exact caps proceed; one-below endpoint retained and one-below endpoint largest caps return `ResourceLimit` before `prepare_endpoint_queues` or host preparation.
2. Warm allocator TLS, surround each endpoint-only cap refusal with current-thread counters, and require zero allocations, frees, reallocations, and byte traffic. Retain a positive allocator control. Mutation that allocates queues before the cap check must fail this assertion.
3. Preserve the accepted actual queue-retention measurement after successful preflight: exact requested bytes/count with zero frees/reallocations while all owners live, independent layout/largest validation, and matching off-render allocation-count reclamation.
4. Map both native-bank and forced-scalar endpoint/reference PCM arrays through `to_bits` before comparison. A signed-zero control and mutation back to `f32` equality must fail the bitwise discriminator.
5. Preserve all #576/#579 cancellation, post-claim, sticky-fault, resource, state, PostFader, zero-pair, mutation, realtime, thread-ownership and low-meter-cap gates byte-for-byte unless a line must change for the two corrections.
6. Run complete affected host-core suites in debug/release, strict Clippy/rustdoc, formatting/diff, workspace/host policies, CI routing, native x86-64-v3 and Wasm scalar/simd128 compilation. No benchmark, performance, artifact, pairing, lifecycle, or broader product claim is authorized.

## Workflow and completion

Luna HIGH implements attempt 1. Astra LOW performs every adversarial source and exact-head/current-base review. Root checkpoints each coherent exact-path tranche, pushes promptly, synchronizes this issue plus #576/#579/#444/#559/#560, and obtains required CI on the exact reviewed head. On PASS, deliver the preserved #576/#579 chain, close all three child issues after upstream evidence, verify post-main qualification, and remove their clean delivered worktrees. The successor has at most three attempts; stop and rescope after a third FAIL without weakening gates.

## Attempt 1 implementation record

Attempt 1 restored report-only endpoint queue projections and validates endpoint retained/largest
caps before host preparation or actual queue allocation. Exact and one-below behavior remains
covered; warmed current-thread allocator checks prove both one-below refusals have zero allocation,
free, realloc, and byte traffic, while the accepted positive retention test remains green. Moving
queue construction before the cap check fails with 11 allocations, 11 frees, and 23,256 requested
bytes; the mutation was restored.

Native-bank and forced-scalar endpoint/reference PCM comparisons now compare mapped `f32::to_bits`
arrays. A signed-zero discriminator rejects `+0.0` versus `-0.0`; changing it back to `f32`
equality fails with the exact `[0.0]`/`[-0.0]` assertion. All mutations were restored.

Debug all-target host-core tests, release endpoint tests, strict Clippy/rustdoc, formatting/diff,
host/workspace policy, CI routing, and Wasm scalar/simd128 checks pass. Cargo.lock is restored;
root owns checkpoint, review, GitHub synchronization, and final delivery gates.

## Attempt 1 adversarial review

Astra LOW reviewed exact pushed source head
`42d9e09f5dd1f4d706af90f2f6d4d4e9b1defea3` and returned **PASS** with no blocking
findings. Independent debug and release all-target host-core suites, doctests, strict Clippy and
rustdoc, formatting/diff, workspace and host policies, CI routing checks, native AVX2/FMA, and Wasm
scalar/simd128 compilation passed. The review confirmed that both endpoint cap refusals occur before
host or queue allocation with zero allocator traffic, successful queue retention and off-render
reclamation remain intact, both PCM paths compare `to_bits`, and the signed-zero discriminator is
effective. Accepted #579 cancellation, state, PostFader, pairing, fault, and realtime behavior is
unchanged. The committed lockfile, manifest, and frozen preparation seam are unchanged.

Current-main integration, an Astra LOW exact integrated-head review, required pull-request
qualification, merge, post-main qualification, GitHub synchronization, and clean delivered-worktree
removal remain delivery gates.

## Artifact qualification

Lane B qualified candidate commit `517bbf486f84fdfd6d59c42f7683348b55cea781` after the pre-pin
repin probe and independent pre-pin static/object/ABI/resource/PCM checks. The ordinary locked
AudioWorklet build produced the exact candidate SHA-256
`29abe2fa838ad4c24cbf19db9ac4185ac95c98d8e4f9e49ed8d668a35e577226` and reproduced it after
applying the shipped pin; all six shipped files matched the pre-pin candidate byte-for-byte.
(The candidate hash is recorded in the pin and durable evidence; the repeated digest is
`29abe2fa838ad4c24cbf19db9ac4185ac95c98d8e4f9e49ed8d668a35e577226`.)

Final static/object/ABI checks and expected-resource/native-PCM checks passed, including the
26-resource red mutations. SDK dependency installation and publishable package/self-test passed
with all 11 tests. Playwright 1.62.1 qualification with artifact-set and semantic red mutations
passed for Chromium 151.0.7922.34, Firefox 153.0, and WebKit 26.5. `results.json` and
`BROWSER_DEPLOYMENT_MATRIX.md` now carry this candidate and artifact lineage. Exact commands,
raw streams, statuses, and checksums are preserved under
`artifacts/issue580-artifact-qualification/`; `Cargo.lock` is restored and no target output is
owned by the tranche.
