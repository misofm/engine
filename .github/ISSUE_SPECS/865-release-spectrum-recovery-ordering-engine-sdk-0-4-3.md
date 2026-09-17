# Release spectrum recovery ordering in Engine SDK 0.4.3

## Product outcome

Publish one immutable `@misofm/engine` 0.4.3 release containing the managed-spectrum recovery publication correction delivered by #863 and merged in PR #864.

The accepted runtime source is clean synchronized `main` commit `c78e1fbed267b8e1382a59beaf266eac015335bc`. Exact-main qualification run 35178072805 passed the exact shipped AudioWorklet artifact, the SDK package/generated-surface route, and the final qualification verdict. Registry 0.4.2 is current; a live lookup for 0.4.3 returns parsed E404.

Issue #863 changed only its issue spec, `sdk/src/core/observation-subscriptions.ts`, and `sdk/test/spectrum-evals.mjs`. It did not change Rust, DSP, ABI, hosts, generated assets, Wasm, analysis parameters, session schema, or release machinery. The accepted AudioWorklet SHA-256 remains `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`.

## Smallest closable release slice

Freeze SDK 0.4.3 without a new runtime change. The release-preparation tranche may edit only:

- this issue spec;
- `sdk/package.json`;
- `sdk/package-lock.json`;
- `sdk/README.md`, limited to install and release-record identity;
- `.github/workflows/npm-publish.yml`; and
- matching release-identity fixtures and diagnostics in `scripts/test-npm-publish-modes.py`.

Change package and root lockfile identities from 0.4.2 to 0.4.3 without dependency or integrity changes. Update the README install and release-record link to 0.4.3 and this issue. Update only the five workflow release/job/archive/provenance version literals. Keep the accepted Worklet pin unchanged. Update only current release identities and diagnostics in the publish fixture; preserve the historical normalized baseline, fixed workflow/qualify-step hashes, and accepted-pin literals.

Do not edit runtime source/tests, Rust, DSP, ABI, hosts, generated files, policy defaults, session schema, generic publisher behavior, benchmarks, or V1 contract identities. Preserve exact-main SHA enforcement, qualify→publish→verify archive ownership, OIDC-only publication, immutable-version refusal, one real publication attempt, registry-convergence handling, and trusted SLSA provenance verification.

## Objective gates

Before review, prove package install; release-mode/trust/artifact/generated/deletion/type gates; one fresh seven-file AudioWorklet closure with the unchanged accepted Wasm pin; headless/package and AudioWorklet gates; retained candidate archive extraction and artifact identity; a built-package repeated H256 gap/recovery pressure test showing every recovered publication reaches the managed callback; archive identities; exact-path/diff audit; and independent review before PR/exact-main qualification.

## Immutable publication sequence

After the reviewed preparation PR merges, freeze that exact `main` SHA through registry verification. Confirm 0.4.3 is still absent; dispatch `npm-publish.yml` once in qualify mode; retain its archive; dispatch publish once with the same SHA and qualification run ID; never retry an ambiguous publish; use verify-only after registry propagation if required. Independently verify registry version/latest, byte-identical archive, public imports/declarations, `enginectl --version`, repeated H256 recovery behavior, unchanged Wasm/host identities, signatures, and one trusted SLSA v1 subject/PURL/source/workflow binding.

Record all commits, PR and exact-main qualification runs, release workflow IDs, archive/provenance identities, and independent public evidence here. Close only after verified registry delivery and synchronized evidence. Adapter publication and app adoption are separate downstream issues.

## Starting evidence

- Accepted source: `c78e1fbed267b8e1382a59beaf266eac015335bc` from #863 / PR #864.
- Exact-main qualification run 35178072805: PASS.
- Accepted unchanged Worklet SHA-256: `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`.
- Registry latest before work: 0.4.2.
- Registry 0.4.3 before work: parsed E404 / unused.
- No 0.4.3 preparation, workflow dispatch, publication, or registry mutation has occurred.
