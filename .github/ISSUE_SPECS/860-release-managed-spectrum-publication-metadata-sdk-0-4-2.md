# Release managed spectrum publication metadata SDK 0.4.2

## Product outcome

Publish one immutable `@misofm/engine` 0.4.2 release containing the managed-spectrum publication-metadata correction delivered by #858 and merged in PR #859.

The accepted runtime source is clean synchronized `main` commit `f2a8ef4f09c7d4428caa6f6a2b80e109e4231099`. Exact-main qualification run 35169580051 passed all required jobs, including the SDK package/generated-surface route and reproducible shipped AudioWorklet artifact. The registry's latest SDK is immutable 0.4.1; a live lookup for 0.4.2 returns parsed E404.

Issue #858 changed only its issue spec, `sdk/src/core/observation-subscriptions.ts`, and `sdk/test/spectrum-evals.mjs`. It did not change Rust, DSP, ABI, host code, generated assets, Wasm, boot policy, session schema, or release machinery. The accepted AudioWorklet SHA-256 remains `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`.

## Smallest closable release slice

Freeze SDK 0.4.2 without a new runtime change. The release-preparation tranche may edit only:

- this issue spec;
- `sdk/package.json`;
- `sdk/package-lock.json`;
- `sdk/README.md`, limited to install and release-record identity;
- `.github/workflows/npm-publish.yml`; and
- matching release-identity fixtures and diagnostics in `scripts/test-npm-publish-modes.py`.

Change the package version and two root lockfile versions from 0.4.1 to 0.4.2 with no dependency/integrity changes. Update the README install and release-record link to 0.4.2/#860. Update exactly five workflow release/job/archive/provenance version literals. Keep the accepted Worklet pin unchanged. In the publish fixture, update only the 0.4.1 release identities and #855 release-diagnostic references to 0.4.2/#860; preserve the historical 0.2.4 normalized baseline, fixed workflow/qualify-step hashes, and accepted-pin literals.

Do not edit runtime source/tests, Rust, DSP, ABI, hosts, generated files, policy defaults, session schema, generic publisher behavior, benchmarks, or V1 contract identities. Preserve exact-main SHA enforcement, qualify→publish→verify archive ownership, OIDC-only publication, immutable-version refusal, one real publication attempt, registry-convergence handling, and trusted SLSA provenance verification.

## Objective gates

Before review, prove:

1. `npm ci --ignore-scripts` in `sdk/`;
2. publish-mode, trust-list, artifact-builder, generated, deletion, and SDK type gates;
3. one fresh seven-file AudioWorklet closure whose Wasm SHA-256 equals the unchanged accepted pin;
4. SDK headless/package, AudioWorklet static/real-receiver/hermetic gates against that closure;
5. a retained 0.4.2 candidate archive smoke-tested from a fresh extraction with all seven artifacts byte-identical to the closure;
6. built-package managed H256 proof that ready followed by pending remains ready/available and metadata matches `readLatest()`;
7. archive byte/file counts, SHA-1/shasum, SHA-256, SHA-512, npm integrity, and all seven artifact identities;
8. `git diff --check` and exact-path audit; and
9. fresh Astra medium PASS before delivery, followed by PR and exact-main qualification.

## Immutable publication sequence

After the reviewed release-preparation PR merges, freeze that exact `main` SHA through registry verification: confirm 0.4.2 is still absent; dispatch `npm-publish.yml` once in qualify mode; require its retained archive; dispatch publish once with the same SHA and qualification run ID; never retry an ambiguous publish; use verify-only after propagation; then independently verify registry version/latest, archive identities, public imports/declarations, `enginectl --version`, managed-spectrum behavior, unchanged Wasm/host identities, and one trusted SLSA v1 subject/PURL/source/workflow binding.

Record all release commits, PR and exact-main qualification runs, npm qualify/publish/verify run IDs, archive/provenance identities, and independent registry evidence here. Close only after verified public registry delivery and synchronized evidence. Adapter publication and app adoption are separate downstream issues.

## Starting evidence

- Accepted source: `f2a8ef4f09c7d4428caa6f6a2b80e109e4231099` from #858 / PR #859.
- Exact-main qualification run 35169580051: PASS.
- Accepted unchanged Worklet SHA-256: `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`.
- Registry latest before work: 0.4.1.
- Registry 0.4.2 before work: parsed E404 / unused.
- Existing release metadata matches the accepted 0.4.1 release at `1f754cb415e5f39123333c526c75110e33ad24df`.
- No 0.4.2 preparation, workflow dispatch, publication, or registry mutation has occurred.
