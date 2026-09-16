# Release configurable spectrum-hop SDK 0.4.1 for testnet adoption

## Product outcome

Publish one immutable `@misofm/engine` 0.4.1 release so the web adapter and testnet app can adopt the configurable prepared spectrum hop delivered by #851 and #852 and merged in PR #854. The accepted runtime base is clean synchronized `main` commit `9fe7157e3485f016cd62963cb84189fddac9c2b3`; exact-main qualification run 35140015285 passed all required jobs.

The registry's latest SDK is immutable 0.4.0, which predates the spectrum-hop control. Live registry lookup returns parsed E404 for unused 0.4.1. The accepted AudioWorklet source pin is SHA256 `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`.

## Smallest closable release slice

Freeze SDK 0.4.1 without changing runtime behavior. One implementation tranche may edit only:

- this numbered issue spec;
- `sdk/package.json`;
- `sdk/package-lock.json`;
- `sdk/README.md` release/install identity only;
- `.github/workflows/npm-publish.yml`; and
- matching release-identity and accepted-pin fixtures in `scripts/test-npm-publish-modes.py`.

Update the package and lock identities from 0.4.0 to 0.4.1. Update the README install command and release-record link to 0.4.1 and this issue. In the publish workflow, update exactly the five 0.4.0 release/job/packed-archive/provenance literals to 0.4.1 and replace the prior accepted Worklet digest with `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`. Update only the corresponding version, PURL, pin, normalization, fixture CLI/package, mutation, issue-number, and diagnostic literals in `scripts/test-npm-publish-modes.py`; preserve its historical 0.2.4 normalized baseline and its fixed workflow/step hashes.

No Rust, DSP, ABI, host/runtime, generated asset/layout, policy-default, session-schema, generic publisher, benchmark, or listening work is allowed. Do not alter the compatible omitted-option default established by #852. Preserve V1 wire identities, exact-main SHA enforcement, qualify→publish→verify archive ownership, OIDC-only publication, immutable-version refusal, and SLSA provenance verification.

## Objective gates

Before review, prove:

1. `npm ci --ignore-scripts` in `sdk/`;
2. `python3 -B scripts/test-npm-publish-modes.py`;
3. `node scripts/test-parse-npm-trust-list.mjs`;
4. `bash scripts/test-sdk-artifact-builder-output-contract.sh`;
5. `bash scripts/check-sdk-generated.sh`;
6. `python3 -B scripts/check-sdk-deletions.py`;
7. `bash scripts/check-sdk-types.sh`;
8. headless, package, AudioWorklet, declaration-mirror, and packed-tarball smoke gates against one freshly built seven-file artifact closure whose Wasm SHA256 is the accepted pin; and
9. `git diff --check` plus an exact-path diff audit showing no change outside the allowed paths.

Retain the candidate archive evidence and compare its packaged host, ABI layout, declarations, and Wasm with the qualified closure. Exercise both the default omitted `spectrumHopFrames` path and an explicit supported hop so the published package proves legacy behavior and the new control. A fresh independent Astra MEDIUM review must return PASS before delivery. The required pull-request qualification must then pass on the bounded release-preparation commit.

## Immutable publication sequence

After the release-preparation PR merges, hold that exact merged `main` SHA unchanged through publication:

1. confirm `@misofm/engine@0.4.1` is still absent;
2. dispatch `npm-publish.yml` once in `qualify` mode with the exact merged SHA and an empty `qualification_run_id`;
3. after successful qualification, dispatch `publish` once with the same SHA and the successful qualification run ID;
4. never retry `publish` after an ambiguous response; after registry propagation use only `verify` with the same SHA and qualification run ID; and
5. independently verify public version/latest convergence, exact tarball SHA256/SHA512/integrity, public imports and types, `enginectl --version`, packaged default and explicit spectrum-hop behavior, one trusted SLSA v1 subject/PURL/source/workflow binding, and the actual Wasm/host hashes.

Record the release commit, PR, exact-main qualification run, npm qualification/publish/verify run IDs, archive identities, and independent registry evidence in this issue/spec. Close only after verified registry delivery; only then may the adapter exact-dependency release advance.

## Starting evidence

- Accepted runtime base: `9fe7157e3485f016cd62963cb84189fddac9c2b3`.
- Required qualification: run 35140015285, PASS.
- Accepted Worklet Wasm SHA256: `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`.
- Registry latest before work: 0.4.0.
- Registry 0.4.1 before work: parsed E404 / unused.
- No release-preparation implementation or publication has occurred under this issue.
