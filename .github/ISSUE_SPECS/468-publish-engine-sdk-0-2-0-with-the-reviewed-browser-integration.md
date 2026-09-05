# Publish Engine SDK 0.2.0 with the reviewed browser integration

## Objective and authorization

The user explicitly requires the reviewed SDK and adapter to be published to npm before new website integration, then misofm/app must move from vendored archives to direct npm dependencies. This issue owns only the Engine SDK0.2.0 release; adapter0.3.0 and both consumers are separate dependent slices. Registry currently contains SDK0.0.0 and0.1.0;0.2.0 is unused. Preserve ABI/wire V1 identities.

## Frozen normal integration

Work in isolated /private/tmp/miso-dx-sdk-npm-release on codex/release-sdk-0-2-0, initially clean at current main aba905c0a5ae0bc747a65d1052ba76811fcee3c5. Merge reviewed DX branch9406ae82e44bdf71b8b60aa12d9eb3b28dce515c normally, retaining both histories. Its functional package175755e9/archive0df6 and canonical Wasm271a2bf3 are historical evidence, not the identity of the merged release.

Dedicated Astra medium release audit approves this normal integration. Against preceding main6589c518 the only merge conflicts are three derived files: hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md, hosts/host-web/qualification/results.json and web/miso-engine-v1-audio-worklet-artifact.sha256. Main's subsequent aba905c0 adds native static-scan policy/evidence, not runtime or SDK code. Resolve derived conflicts with explicitly provisional existing main evidence, then replace them only through actual new Linux build/recording. Never label an old digest or browser candidate as the new merged artifact. SDK subtree previously auto-merges identical to reviewed DX; inspect actual merge and preserve native/runtime changes from main. Report any new source conflict before broadening scope.

## Minimal release metadata

Set sdk/package.json and package-lock root versions to0.2.0. In the existing npm-publish.yml, update the version-specific constant, job label, qualification job lookup, packed identity and provenance PURL to the new package version. Update its EXPECTED_WORKLET_SHA256 only when the actual merged Linux digest is earned and equals the authoritative source pin. Preserve main-only dispatch/head checks, accepted/rejected ancestry, immutable-version refusal, prior-qualified exact archive checks, OIDC-only provenance, registry integrity/access/latest convergence and verify-only recovery. No token fallback, force-reset, fabricated asset identity or alternate publishing framework.

## Checkpoint and objective gates

First pause a coherent merge/metadata checkpoint after proportional merge inspection, version consistency, syntax/types and source checks. Root commits the normal merge and pushes before further implementation. The unresolved new artifact identity must be candidly recorded; old records cannot imply qualification PASS.

Use the existing official Ubuntu builder and existing repin/recording/ordinary qualification mechanisms to earn the actual merged artifact and truthful browser records. Run required merged-source CI and existing generated/deletion/type/headless/package gates, including actual first-target suspended/live consumer preparation proofs from the reviewed integration. No new fixture corpus or browser matrix. Independently review the merged source, derived records and exact newly versioned package.

After a concrete green candidate, integrate through normal protected-main review/checks without rewriting history or bypassing protection. Dispatch the existing npm workflow qualify at exact main SHA; inspect its immutable tarball and successful gates. Publish that exact qualifying artifact once through existing GitHub OIDC. Verify registry0.2.0/latest/public access/SHA512/shasum, fresh public imports and enginectl, and cryptographic provenance binding the package, source and trusted workflow. If publication reply is ambiguous, use verify mode rather than republishing. Preserve evidence and synchronize this numbered issue; close only once the public registry release is verified. Adapter publication begins afterward with an exact registry dependency on SDK0.2.0.
