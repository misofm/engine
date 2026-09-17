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

## Implementation attempt 1 evidence

The release-preparation tranche changes exactly the five authorized metadata paths. Package and lockfile identities are 0.4.2; the README install command and release record identify 0.4.2/#860; the publish workflow contains exactly five 0.4.2 literals and retains the accepted Worklet pin; the fixture updates only release identities and diagnostics while preserving the historical 0.2.4 normalization and fixed workflow/step hashes. No runtime, generated, ABI, dependency, integrity, or artifact content changed.

`npm ci --ignore-scripts` passed in `sdk/`. The publish-mode reachability/trust gate, trust-list parser, artifact-builder contract, generated-surface checks, deletion gate and its 37-mutation self-test, SDK type gate, exact-path/literal audit, and `git diff --check` passed. Fresh closure/package qualification, retained archive evidence, independent review, PR/exact-main qualification, and immutable registry delivery remain.

## Candidate qualification evidence

Exactly one fresh seven-file closure was built at `/tmp/issue860-closure-20260917` and exactly one 0.4.2 archive was packed at `/tmp/issue860-qualification-20260917/candidate.tgz`. The archive is 1,331,339 bytes and 98 files, with shasum `bf17c31d80129264bcc8d885b27818b7f8958c9b`, SHA-256 `8f28af09f1fb6f31295e82ba9cb97350cb2f56be21e1db1c5bb028d9e128880d`, SHA-512 `fb0c74620bef68378a3b9d293d8e2e1ee35d00c3093b2ad57f47e6dc7c5acef7b5fb0d58bc95c3fb042441e034e83d3a9ce7225eea0dc9da217b85e8dbb17666`, and npm integrity `sha512-+wx0YgvvaDeKO50pPY4uHuNdAMMJOyrVf0fm3Hxazve1+w1YvJXD+wQkQeA06D06nOciXuoNydohe4Xo27F2Zg==`.

The fresh closure's Wasm SHA-256 is the unchanged accepted pin `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`. SDK headless passed 279 tests; package, static AudioWorklet, real-Wasm receiver, hermetic AudioWorklet, fresh-extraction smoke, and all seven byte-for-byte artifact comparisons passed. The built-package H256 probe observed a ready/available publication for samples 0..2048 and retained matching result identity through a pending poll. Its first harness invocation compared the full status object to a string and failed before exercising the product assertion; the corrected field assertion passed without rebuilding or repacking, and both records are preserved under `/tmp/issue860-qualification-20260917/09*`. Release-mode, trust, deletion/type, and diff audits also passed. Independent review and remote delivery remain.

## Fresh Astra medium review

PASS at clean checkpoint `d1598f23`, with no blockers. The reviewer confirmed the exact authorized path/literal set, unchanged dependencies/integrities/runtime/artifacts/ABI, exactly five workflow version changes, unchanged accepted pin and historical fixture hashes, and preserved exact-SHA/archive/OIDC/single-publish/provenance safeguards. Independent archive hashes/counts, fresh-extraction smoke, all seven artifact comparisons, 279/279 headless tests, release/trust/type/generated/deletion/diff gates, and a reconstructed built-package H256 ready→pending probe passed. The recorded first probe failure was confirmed as harness misuse before the product assertion. Remote qualification and immutable registry delivery remain.
